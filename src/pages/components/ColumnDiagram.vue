<script setup lang="ts">
import { onMounted, ref, Ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { SummaryType } from '../../common/SummaryType';
import { formatDate } from '../../common/Utils';
import ApexCharts from 'apexcharts';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

interface ColumnDiagramData {
    currency: string;
    summary: {
        income: number;
        expense: number;
    }[];
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
let chart: ApexCharts | undefined = undefined;

const get_data = async function() {
    try {
        return await invoke('get_summary', {
            summaryType: props.summaryType,
            begin: props.beginDate ? formatDate(props.beginDate) : undefined,
            end: props.endDate ? formatDate(props.endDate) : undefined
        }) as { dates: string[], data: ColumnDiagramData[] };
    } catch (error) {
        console.error(error);
        return undefined;
    }
}

const draw_chart = async function() {
    const data = await get_data();
    if (data === undefined || data.data.length == 0) {
        console.log("no data to draw chart");
        return;
    }
    width.value = 150 + 50 * data.dates.length;

    const options = {
        chart: {
            type: 'bar',
            stacked: true,
            height: '350px',
            width: width.value,
            toolbar: { show: false },
            selection: { enabled: false },
        },
        series: data.data.flatMap(item => {
            return [
                {
                    name: `${t('income')}/${item.currency}`,
                    group: 'income',
                    data: item.summary.map(summary => summary.income / 100)
                }, {
                    name: `${t('expense')}/${item.currency}`,
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
            categories: data.dates,
        },
        yaxis: {
            labels: {
                formatter: (value: number) => {
                    const SIGNS = ['', 'K', 'M', 'B', 'T'];
                    const index = Math.floor(Math.log10(value) / 3);
                    if (index < 0 || index > SIGNS.length - 1) {
                        return value.toString();
                    }
                    const newValue = value / Math.pow(10, index * 3);
                    return newValue.toFixed(0) + SIGNS[index];
                }
            }
        },
        grid: {
            padding: {
                left: 30,
                right: 30,
            }
        },
        legend: {
            horizontalAlign: 'left',
        },
        dataLabels: {
            enabled: false,
        },
        no_data: {
            text: 'loading'
        }
    };
    if (chart == undefined) {
        chart = new ApexCharts(document.querySelector("#chart"), options)
        await chart.render();
    } else {
        chart.updateOptions(options);
    }
}

onMounted(() => {
    draw_chart();
});

watch(props, () => {
    draw_chart();
})
</script>

<template>
    <v-card :variant="props.variant" :rounded="props.rounded" :height="hide_self ? '0px' : 'auto'">
        <v-card-title v-if="props.title">{{ props.title }}</v-card-title>
        <v-card-title v-else><slot name="title" /></v-card-title>
        <v-card-text :width="width" class="pa-0 overflow-x-visible overflow-y-hidden no-scroll-bar">
            <div id="chart"></div>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.no-scroll-bar::-webkit-scrollbar {
    display: none;
}
</style>