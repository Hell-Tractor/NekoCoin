<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, Ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { SummaryType } from '../../common/SummaryType';
import { formatDate, format_chart_axis, format_chart_number } from '../../common/Utils';
import { privacy_mode } from '../../common/Settings';
import { apex_chart_theme } from '../../common/ChartTheme';
import { create_chart_pan } from '../../common/chartPan';
import ApexCharts from 'apexcharts';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
const { t } = useI18n();
const vuetify_theme = useTheme();

interface ColumnDiagramData {
    currency_code: string;
    summary: {
        income: number;
        expense: number;
    }[];
}

interface SummaryPage {
    dates: string[];
    data: ColumnDiagramData[];
    has_more: boolean;
}

const props = defineProps<{
    title?: string;
    summaryType: SummaryType;
    beginDate?: Date;
    endDate?: Date;
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    rounded?: string | boolean;
}>();

const hide_self: Ref<boolean> = ref(false);
const width: Ref<number> = ref(0);
const loading = ref(false);
const chart_element = ref<HTMLElement>();
const scroll_element = ref<HTMLElement>();
const dates = ref<string[]>([]);
const chart_data = ref<ColumnDiagramData[]>([]);
const has_data = ref(false);
const has_more = ref(false);
let chart: ApexCharts | undefined = undefined;
let request_id = 0;
const PAGE_SIZE = 90;
const { on_pointer_down, on_pointer_move, on_pointer_up } = create_chart_pan(() => scroll_element.value);

const get_data = async function(offset: number): Promise<SummaryPage | undefined> {
    try {
        return await invoke('get_summary', {
            summaryType: props.summaryType,
            begin: props.beginDate ? formatDate(props.beginDate) : undefined,
            end: props.endDate ? formatDate(props.endDate) : undefined,
            offset,
            limit: PAGE_SIZE,
        }) as SummaryPage;
    } catch (error) {
        console.error(error);
        return undefined;
    }
}

const merge_data = function(page: SummaryPage, append: boolean) {
    if (!append) {
        dates.value = page.dates;
        chart_data.value = page.data;
    } else {
        const previous_length = dates.value.length;
        dates.value.push(...page.dates);
        const currencies = new Set([
            ...chart_data.value.map(item => item.currency_code),
            ...page.data.map(item => item.currency_code),
        ]);
        chart_data.value = [...currencies].map(currency_code => {
            const existing = chart_data.value.find(item => item.currency_code === currency_code);
            const incoming = page.data.find(item => item.currency_code === currency_code);
            return {
                currency_code,
                summary: [
                    ...(existing?.summary ?? Array.from({ length: previous_length }, () => ({ income: 0, expense: 0 }))),
                    ...(incoming?.summary ?? Array.from({ length: page.dates.length }, () => ({ income: 0, expense: 0 }))),
                ],
            };
        });
    }
    has_more.value = page.has_more;
    width.value = Math.max(window.innerWidth - 50, 150 + 50 * dates.value.length);
    if (dates.value.length === 0 || chart_data.value.length === 0) {
        has_data.value = false;
    } else {
        has_data.value = true;
    }
}

const render_chart = async function() {
    const chart_theme = apex_chart_theme();
    const options = {
        chart: {
            type: 'bar',
            stacked: true,
            height: '350px',
            width: width.value,
            toolbar: { show: false },
            selection: { enabled: false },
            animations: { enabled: false },
            redrawOnWindowResize: false,
            redrawOnParentResize: false,
            ...chart_theme.chart,
        },
        theme: chart_theme.theme,
        colors: chart_data.value.flatMap((_, index) => {
            return chart_theme.income_expense_pairs[index % chart_theme.income_expense_pairs.length];
        }),
        series: chart_data.value.flatMap(item => {
            return [
                {
                    name: `${t('income')}/${item.currency_code}`,
                    group: 'income',
                    data: item.summary.map(summary => summary.income / 100)
                }, {
                    name: `${t('expense')}/${item.currency_code}`,
                    group: 'expense',
                    data: item.summary.map(summary => summary.expense / 100)
                }
            ]
        }),
        plotOptions: {
            bar: {
                columnWidth: '30px',
                borderRadiusWhenStacked: 'all',
                borderRadiusApplication: 'end',
            },
        },
        xaxis: {
            type: 'category',
            categories: dates.value,
            ...chart_theme.xaxis,
        },
        yaxis: {
            ...chart_theme.yaxis,
            labels: {
                ...chart_theme.yaxis.labels,
                formatter: (value: number) => format_chart_axis(value),
            }
        },
        grid: {
            ...chart_theme.grid,
            padding: {
                left: 30,
                right: 30,
            }
        },
        legend: {
            horizontalAlign: 'left',
            ...chart_theme.legend,
        },
        tooltip: {
            ...chart_theme.tooltip,
            y: {
                formatter: (value: number) => format_chart_number(value, 2),
            },
        },
        dataLabels: {
            enabled: false,
        },
        no_data: {
            text: 'loading'
        }
    };
    await nextTick();
    await new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
    if (chart == undefined && chart_element.value !== undefined) {
        chart = new ApexCharts(chart_element.value, options)
        await chart.render();
    } else {
        await chart?.updateOptions(options, false, false);
    }
}

const draw_chart = async function() {
    const current_request_id = ++request_id;
    loading.value = true;
    dates.value = [];
    chart_data.value = [];
    has_more.value = false;
    try {
        const page = await get_data(0);
        if (current_request_id !== request_id || page === undefined || page.dates.length === 0) {
            console.log("no data to draw chart");
            return;
        }
        merge_data(page, false);
        await render_chart();
    } finally {
        if (current_request_id === request_id) {
            loading.value = false;
        }
    }
}

const load_more = async function() {
    if (loading.value || !has_more.value) {
        return;
    }
    const current_request_id = request_id;
    loading.value = true;
    try {
        const page = await get_data(dates.value.length);
        if (current_request_id !== request_id || page === undefined || page.dates.length === 0) {
            return;
        }
        merge_data(page, true);
        await render_chart();
    } finally {
        if (current_request_id === request_id) {
            loading.value = false;
        }
    }
}

const on_scroll = function() {
    const element = scroll_element.value;
    if (element !== undefined && element.scrollLeft + element.clientWidth >= element.scrollWidth - 200) {
        load_more();
    }
}

onMounted(() => {
    draw_chart();
});

watch(props, () => {
    draw_chart();
})

watch(() => vuetify_theme.global.name.value, async () => {
    await nextTick();
    if (has_data.value) {
        await render_chart();
    }
});

watch(privacy_mode, async () => {
    if (has_data.value) {
        await render_chart();
    }
});

onBeforeUnmount(() => {
    request_id++;
    chart?.destroy();
    chart = undefined;
})
</script>

<template>
    <v-card :variant="props.variant" :rounded="props.rounded" :height="hide_self ? '0px' : 'auto'">
        <v-card-title v-if="props.title">{{ props.title }}</v-card-title>
        <v-card-title class="title-row" v-else><slot name="title"/></v-card-title>
        <v-progress-linear v-if="loading" indeterminate />
        <v-card-text v-if="has_data" class="pa-0">
            <div ref="scroll_element" class="chart-scroll" @scroll.passive="on_scroll"
                @pointerdown="on_pointer_down" @pointermove="on_pointer_move"
                @pointerup="on_pointer_up" @pointercancel="on_pointer_up">
                <div ref="chart_element" class="chart" :style="{ width: `${width}px` }"></div>
            </div>
        </v-card-text>
        <v-card-text v-else-if="!loading">{{ t('report.no_data') }}</v-card-text>
    </v-card>
</template>

<style scoped>
.chart-scroll {
    max-width: 100%;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    -ms-overflow-style: none;
    -webkit-overflow-scrolling: touch;
    overscroll-behavior-x: contain;
    touch-action: pan-y;
    cursor: grab;
}

.chart-scroll :deep(.apexcharts-canvas),
.chart-scroll :deep(svg) {
    touch-action: pan-y;
}

.chart-scroll::-webkit-scrollbar {
    display: none;
}

.chart-scroll:active {
    cursor: grabbing;
}

.chart {
    min-width: 100%;
}

.title-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
}
</style>