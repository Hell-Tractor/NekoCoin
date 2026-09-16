<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import Tag from '../../common/Tag';
import { SummaryType, SummaryTypeList } from '../../common/SummaryType';
import { entity_accent_color } from '../../common/Utils';
import AmountText from './AmountText.vue';

type Grouping = SummaryType | 'All';

interface GroupingOption {
    value: Grouping;
    label: string;
}

interface TagSummary {
    tag: Tag;
    summary: number;
    currency_code: string;
    period: string;
}

const { t } = useI18n();
const router = useRouter();
const data = ref<TagSummary[]>([]);
const loading = ref(false);
const loaded = ref(false);
const load_failed = ref(false);
const grouping = ref<Grouping>(SummaryType.Monthly);
const grouping_items = computed<GroupingOption[]>(() => [
    { value: 'All', label: t('report.all') },
    ...SummaryTypeList.map(value => ({
        value,
        label: t(`report.summary_type.${value}`),
    })),
]);

const groups = computed(() => {
    const result = new Map<string, TagSummary[]>();
    for (const item of data.value) {
        const group = result.get(item.period) ?? [];
        group.push(item);
        result.set(item.period, group);
    }
    return [...result.entries()].map(([period, items]) => ({ period, items }));
});

const get_data = async function() {
    loading.value = true;
    loaded.value = false;
    load_failed.value = false;
    data.value = [];
    try {
        data.value = await invoke('get_expense_summary_by_tag', {
            summaryType: grouping.value === 'All' ? null : grouping.value,
            begin: null,
            end: null,
        }) as TagSummary[];
    } catch (error) {
        console.error(error);
        load_failed.value = true;
    } finally {
        loading.value = false;
        loaded.value = true;
    }
};

const max_for_currency = function(group: TagSummary[], currency_code: string) {
    return group
        .filter(item => item.currency_code === currency_code)
        .reduce((max, item) => Math.max(max, item.summary), 1);
};

const open_tag = function(item: TagSummary) {
    router.push({ path: `/tag/${item.tag.id}` });
};

onMounted(() => {
    get_data();
});

watch(grouping, () => {
    get_data();
});
</script>

<template>
    <v-card variant="flat" rounded="xl">
        <v-card-title class="title-row text-body-1 font-weight-bold">
            <span>{{ t('report.expense_by_tag') }}</span>
            <v-select
                v-model="grouping"
                :items="grouping_items"
                item-title="label"
                item-value="value"
                class="grouping-select"
                density="compact"
                variant="outlined"
                hide-details
            />
        </v-card-title>
        <v-progress-linear v-if="loading" indeterminate />
        <v-card-text v-if="groups.length > 0" class="pt-0">
            <section v-for="group in groups" :key="group.period || 'all'" class="period-group">
                <button v-for="item in group.items" :key="`${group.period}-${item.currency_code}-${item.tag.id}`" class="tag-row" type="button" @click="open_tag(item)">
                    <div class="tag-label">
                        <v-icon :color="entity_accent_color(item.tag.color)" size="small">{{ item.tag.icon }}</v-icon>
                        <span>{{ item.tag.name }}</span>
                        <span class="tag-amount"><AmountText :cents="item.summary" :currency="item.currency_code" /></span>
                    </div>
                    <v-progress-linear
                        :model-value="(item.summary / max_for_currency(group.items, item.currency_code)) * 100"
                        :color="entity_accent_color(item.tag.color)"
                        rounded
                        height="8"
                    />
                </button>
            </section>
        </v-card-text>
        <v-card-text v-else-if="load_failed">{{ t('report.load_failed') }}</v-card-text>
        <v-card-text v-else-if="loaded">{{ t('report.no_data') }}</v-card-text>
    </v-card>
</template>

<style scoped>
.title-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
}

.grouping-select {
    flex: 0 0 auto;
    width: fit-content;
    min-width: 92px;
    /* max-width: 112px; */
}

.period-group + .period-group {
    margin-top: 12px;
    padding-top: 8px;
    border-top: 1px solid rgba(var(--v-theme-on-surface), 0.1);
}

.tag-row {
    display: block;
    width: 100%;
    padding: 8px 0;
    border: 0;
    color: inherit;
    text-align: left;
    cursor: pointer;
}

.tag-row:hover .tag-label {
    opacity: 0.72;
}

.tag-label {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 5px;
}

.tag-amount {
    margin-left: auto;
    font-variant-numeric: tabular-nums;
}
</style>
