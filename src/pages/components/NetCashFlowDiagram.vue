<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ApexCharts from 'apexcharts';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
import { SummaryType } from '../../common/SummaryType';
import { formatDate, format_chart_number } from '../../common/Utils';
import { privacy_mode } from '../../common/Settings';
import { apex_chart_theme } from '../../common/ChartTheme';
import { create_chart_pan } from '../../common/chartPan';

interface SummaryData {
    currency_code: string;
    summary: {
        income: number;
        expense: number;
    }[];
}

interface SummaryPage {
    dates: string[];
    data: SummaryData[];
}

const props = defineProps<{
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain";
    rounded?: string | boolean;
}>();

const { t } = useI18n();
const vuetify_theme = useTheme();
const chart_element = ref<HTMLElement>();
const scroll_element = ref<HTMLElement>();
const width = ref(0);
const loading = ref(false);
const has_data = ref(false);
let chart: ApexCharts | undefined;
let request_id = 0;
const { on_pointer_down, on_pointer_move, on_pointer_up } = create_chart_pan(() => scroll_element.value);

const load_chart = async function() {
    const current_request_id = ++request_id;
    loading.value = true;
    chart?.destroy();
    chart = undefined;
    has_data.value = false;
    try {
        const begin_date = new Date();
        begin_date.setFullYear(begin_date.getFullYear() - 1);
        const result = await invoke('get_summary', {
            summaryType: SummaryType.Monthly,
            begin: formatDate(begin_date),
            end: null,
            offset: 0,
            limit: 12,
        }) as SummaryPage;
        if (current_request_id !== request_id) {
            return;
        }

        const dates = result.dates;
        const series = result.data.map(item => ({
            name: item.currency_code,
            data: item.summary.map(summary => (summary.income - summary.expense) / 100),
        }));
        width.value = Math.max(window.innerWidth - 50, 150 + 50 * dates.length);
        has_data.value = dates.length > 0 && series.length > 0;
        await nextTick();
        if (!has_data.value || chart_element.value === undefined) {
            return;
        }

        const chart_theme = apex_chart_theme();
        const options = {
            chart: {
                type: 'line',
                height: 260,
                width: width.value,
                toolbar: { show: false },
                selection: { enabled: false },
                zoom: { enabled: false },
                animations: { enabled: false },
                redrawOnWindowResize: true,
                ...chart_theme.chart,
            },
            theme: chart_theme.theme,
            series,
            colors: chart_theme.line_colors,
            stroke: {
                curve: 'smooth',
                width: 2,
            },
            markers: {
                size: 3,
                hover: { size: 5 },
                ...chart_theme.markers,
            },
            xaxis: {
                categories: dates,
                tickAmount: Math.min(8, dates.length),
                ...chart_theme.xaxis,
            },
            yaxis: {
                ...chart_theme.yaxis,
                labels: {
                    ...chart_theme.yaxis.labels,
                    formatter: (value: number) => format_chart_number(value, 0),
                },
            },
            grid: chart_theme.grid,
            tooltip: {
                ...chart_theme.tooltip,
                y: {
                    formatter: (value: number) => format_chart_number(value, 2),
                },
            },
            dataLabels: { enabled: false },
            legend: {
                horizontalAlign: 'left',
                ...chart_theme.legend,
            },
            noData: { text: t('loading') },
        };

        chart = new ApexCharts(chart_element.value, options);
        await chart.render();
    } catch (error) {
        console.error(error);
        has_data.value = false;
    } finally {
        if (current_request_id === request_id) {
            loading.value = false;
        }
    }
};

onMounted(() => {
    load_chart();
});

watch(() => vuetify_theme.global.name.value, async () => {
    await nextTick();
    await load_chart();
});

watch(privacy_mode, async () => {
    if (has_data.value) {
        await load_chart();
    }
});

onBeforeUnmount(() => {
    request_id++;
    chart?.destroy();
    chart = undefined;
});
</script>

<template>
    <v-card :variant="props.variant" :rounded="props.rounded">
        <v-card-title>{{ t('report.net_cash_flow') }}</v-card-title>
        <v-progress-linear v-if="loading" indeterminate />
        <v-card-text v-if="has_data" class="pa-0">
            <div
                ref="scroll_element"
                class="chart-container"
                @pointerdown="on_pointer_down"
                @pointermove="on_pointer_move"
                @pointerup="on_pointer_up"
                @pointercancel="on_pointer_up"
            >
                <div ref="chart_element" class="chart" :style="{ width: `${width}px` }"></div>
            </div>
        </v-card-text>
        <v-card-text v-else-if="!loading">{{ t('report.no_data') }}</v-card-text>
    </v-card>
</template>

<style scoped>
.chart-container {
    max-width: 100%;
    min-height: 260px;
    overflow-x: auto;
    overflow-y: hidden;
    -webkit-overflow-scrolling: touch;
    overscroll-behavior-x: contain;
    scrollbar-width: none;
    -ms-overflow-style: none;
    touch-action: pan-y;
    cursor: grab;
}

.chart-container :deep(.apexcharts-canvas),
.chart-container :deep(svg) {
    touch-action: pan-y;
}

.chart-container::-webkit-scrollbar {
    display: none;
}

.chart-container:active {
    cursor: grabbing;
}

.chart {
    min-width: 100%;
}
</style>
