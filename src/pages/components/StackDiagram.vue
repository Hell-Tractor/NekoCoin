<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import Tag, { TagType, TagTypeToString } from '../../common/Tag';
import { formatDate } from '../../common/Utils';
import { computed, onMounted, Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const props = defineProps<{
    item_id?: { type: 'wallet' | 'tag', value: number };
    currency_code: string;
    // TagType.Transfer is only allowed when item_id.type is 'tag'
    kind: TagType;
    beginDate?: Date;
    endDate?: Date;
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    rounded?: string | boolean;
}>();

const data: Ref<{ tag: Tag, summary: number }[] | undefined> = ref(undefined);
const hide_self: Ref<boolean> = ref(true);
const loading = ref(false);
const total = computed(() => data.value?.reduce((sum, item) => sum + item.summary, 0) ?? 0);
const percentage = (summary: number) => total.value > 0 ? (summary / total.value) * 100 : 0;
const format_percentage = (summary: number) => `${percentage(summary).toFixed(1)}%`;

const get_data = async function() {
    hide_self.value = false;
    loading.value = true;
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
                currencyCode: props.currency_code,
                beginDate: props.beginDate ? formatDate(props.beginDate) : undefined,
                endDate: props.endDate ? formatDate(props.endDate) : undefined
            });
            console.log("data", data.value);
        }
        hide_self.value = data.value === undefined || data.value.length === 0;
    } catch (error) {
        console.error(error);
        hide_self.value = true;
    } finally {
        loading.value = false;
    }
}

onMounted(() => {
    get_data()
});
</script>

<template>
    <v-card :variant="variant" :rounded="rounded" :height="hide_self ? '0px' : undefined">
        <v-card-text class="pa-0">
            <div class="mt-4 ml-4">{{ t(`tag.type.${TagTypeToString(kind)}`) }}</div>
            <v-progress-linear v-if="loading" indeterminate />
            <div v-if="data && total > 0" class="tag-list">
                <div class="segmented-progress" role="img">
                    <v-tooltip
                        v-for="item in data"
                        :key="`segment-${item.tag.id}`"
                        location="top"
                        open-on-click
                        :text="`${item.tag.name} ${format_percentage(item.summary)} - ${props.currency_code} ${(item.summary / 100).toFixed(2)}`"
                    >
                        <template #activator="{ props: tooltip_props }">
                            <div
                                v-bind="tooltip_props"
                                class="progress-segment"
                                :style="{ width: `${percentage(item.summary)}%`, backgroundColor: item.tag.color }"
                                :aria-label="`${item.tag.name} ${format_percentage(item.summary)}`"
                            >
                                <span v-if="percentage(item.summary) >= 7" class="segment-percent">{{ format_percentage(item.summary) }}</span>
                            </div>
                        </template>
                    </v-tooltip>
                </div>
                <div class="tag-legend">
                    <div v-for="item in data" :key="item.tag.id" class="tag-row">
                        <div class="tag-label">
                            <v-icon :color="item.tag.color" size="x-small">{{ item.tag.icon }}</v-icon>
                            <span>{{ item.tag.name }}</span>
                            <span class="tag-amount">{{ props.currency_code }} {{ (item.summary / 100).toFixed(2) }}</span>
                        </div>
                    </div>
                </div>
            </div>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.tag-list {
    position: relative;
    padding: 8px 16px 12px;
}

.segmented-progress {
    display: flex;
    width: 100%;
    height: 20px;
    margin: 4px 0 12px;
    overflow: hidden;
    border-radius: 10px;
    background-color: rgba(var(--v-theme-on-surface), 0.12);
}

.progress-segment {
    position: relative;
    min-width: 2px;
    height: 100%;
    transition: width 180ms ease;
}

.segment-percent {
    position: absolute;
    top: 50%;
    left: 50%;
    color: white;
    font-size: 0.7rem;
    font-variant-numeric: tabular-nums;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
    transform: translate(-50%, -50%);
    white-space: nowrap;
}

.tag-legend {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 2px 12px;
}

.tag-row {
    min-width: 0;
    padding: 3px 0;
}

.tag-label {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
}

.tag-label > span:first-of-type {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.tag-amount {
    margin-left: auto;
    color: rgba(var(--v-theme-on-surface), 0.6);
    font-size: 0.75rem;
    white-space: nowrap;
}
</style>