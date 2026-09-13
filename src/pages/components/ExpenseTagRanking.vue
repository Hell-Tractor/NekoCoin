<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import Tag from '../../common/Tag';

interface TagSummary {
    tag: Tag;
    summary: number;
    currency_code: string;
}

const { t } = useI18n();
const router = useRouter();
const data = ref<TagSummary[]>([]);
const loading = ref(false);

const max_by_currency = computed(() => {
    const result = new Map<string, number>();
    for (const item of data.value) {
        result.set(item.currency_code, Math.max(result.get(item.currency_code) ?? 0, item.summary));
    }
    return result;
});

const get_data = async function() {
    loading.value = true;
    try {
        data.value = await invoke('get_expense_summary_by_tag') as TagSummary[];
    } catch (error) {
        console.error(error);
    } finally {
        loading.value = false;
    }
};

const format_amount = function(item: TagSummary) {
    return `${item.currency_code} ${(item.summary / 100).toFixed(2)}`;
};

const open_tag = function(item: TagSummary) {
    router.push({ path: `/tag/${item.tag.id}` });
};

onMounted(() => {
    get_data();
});
</script>

<template>
    <v-card variant="flat" rounded="xl">
        <v-card-title>{{ t('report.expense_by_tag') }}</v-card-title>
        <v-progress-linear v-if="loading" indeterminate />
        <v-card-text v-if="data.length > 0" class="pt-0">
            <button v-for="item in data" :key="`${item.currency_code}-${item.tag.id}`" class="tag-row" type="button" @click="open_tag(item)">
                <div class="tag-label">
                    <v-icon :color="item.tag.color" size="small">{{ item.tag.icon }}</v-icon>
                    <span>{{ item.tag.name }}</span>
                    <span class="tag-amount">{{ format_amount(item) }}</span>
                </div>
                <v-progress-linear
                    :model-value="(item.summary / (max_by_currency.get(item.currency_code) ?? 1)) * 100"
                    :color="item.tag.color"
                    rounded
                    height="8"
                />
            </button>
        </v-card-text>
        <v-card-text v-else-if="!loading">{{ t('loading') }}</v-card-text>
    </v-card>
</template>

<style scoped>
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
