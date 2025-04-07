<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import Tag, { TagType, TagTypeToString } from '../../common/Tag';
import { formatDate } from '../../common/Utils';
import { onMounted, Ref, ref, watch } from 'vue';
import ApexCharts from 'apexcharts';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const props = defineProps<{
    item_id?: { type: 'wallet' | 'tag', value: number };
    currency: string;
    // TagType.Transfer is only allowed when item_id.type is 'tag'
    kind: TagType;
    beginDate?: Date;
    endDate?: Date;
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    rounded?: string | boolean;
}>();

const data: Ref<{ tag: Tag, summary: number }[] | undefined> = ref(undefined);
const hide_self: Ref<boolean> = ref(true);

const get_data = async function() {
    try {
        if (props.item_id?.type == 'wallet') {
            data.value = await invoke('get_summary_by_tag_in_wallet', {
                walletId: props.item_id?.value,
                kind: TagTypeToString(props.kind),
                beginDate: props.beginDate ? formatDate(props.beginDate) : undefined,
                endDate: props.endDate ? formatDate(props.endDate) : undefined
            });
        } else if (props.item_id?.type == 'tag') {
            data.value = await invoke('get_summary_by_tag_with_tag', {
                tagId: props.item_id?.value,
                currency: props.currency,
                beginDate: props.beginDate ? formatDate(props.beginDate) : undefined,
                endDate: props.endDate ? formatDate(props.endDate) : undefined
            });
            console.log("data", data.value);
        }
    } catch (error) {
        console.error(error);
    }
}

watch(data, function(newValue) {
    hide_self.value = newValue === undefined || newValue.length == 0;
    console.log("hide_self", hide_self.value);
    draw_chart();
});

const draw_chart = function() {
    console.log("drawing chart");
    if (data.value === undefined || data.value.length == 0) {
        console.log("no data to draw chart");
        return;
    }
    const options = {
        chart: {
            type: 'bar',
            stacked: true,
            stackType: '100%',
            height: '170px',
            width: '100%',
            offsetX: -7,
            toolbar: { show: false },
            selection: { enabled: false },
        },
        series: data.value?.map(item => { return { name: item.tag.name, data: [ item.summary ], color: item.tag.color }; }) || [],
        plotOptions: {
            bar: {
                horizontal: true,
                barHeight: '20px',
                borderRadius: 5,
            },
        },
        xaxis: {
            axisBorder: { show: false },
            axisTicks: { show: false },
            labels: { show: false },
        },
        yaxis: {
            show: false,
        },
        grid: {
            show: false,
        },
        tooltip: {
            x: {
                show: false,
            },
            y: {
                formatter: function(val: number) {
                    return `${props.currency} ${(val / 100).toFixed(2)}`;
                }
            }
        },
        states: {
            active: {
                filter: {
                    type: 'none',
                }
            }
        },
        legend: {
            formatter: function(seriesName: string, opts: any) {
                return `${seriesName} ${(opts.w.globals.seriesPercent[opts.seriesIndex][0]).toFixed(2)}%`;
            },
            horizontalAlign: 'left',
            markers: {
                shape: 'circle',
            }
        }
    };
    const chart = new ApexCharts(document.querySelector("#chart"), options);
    chart.render();
}

onMounted(() => {
    get_data()
});
</script>

<template>
    <v-card :variant="variant" :rounded="rounded" :height="hide_self ? '0px' : undefined">
        <v-card-text class="pa-0">
            <div class="mt-4 ml-4">{{ t(`tag.type.${TagTypeToString(kind)}`) }}</div>
            <div id="chart" style="margin-top: -35px; padding-bottom: 10px;"></div>
        </v-card-text>
    </v-card>
</template>