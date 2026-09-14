<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { Activity } from '../common/Activity';
import { load_settings, settings } from '../common/Settings';
import ActivityCard from './components/ActivityCard.vue';
const { t } = useI18n();
const router = useRouter();

const activities: Ref<Activity[]> = ref([]);
const net_by_id = ref<Record<number, number>>({});

const retrieve_activities = async function() {
    try {
        await load_settings();
        activities.value = await invoke('retrieve_activities');
        const balances = await invoke('get_activity_balances', {
            currencyCode: settings.primary_currency_code,
        }) as { id: number, income: number, expense: number }[];
        const next: Record<number, number> = {};
        for (const item of balances) {
            next[item.id] = item.income - item.expense;
        }
        net_by_id.value = next;
    } catch (error) {
        console.error(error);
    }
}

onMounted(() => {
    retrieve_activities();
});
</script>

<template>
    <v-card v-if="activities.length === 0" rounded="xl">
        <v-card-title>{{ t('activity.no_activity') }}</v-card-title>
        <v-card-text>{{ t('activity.no_activity_tip') }}</v-card-text>
    </v-card>
    <template v-else>
        <ActivityCard
            v-for="item in activities"
            :key="item.id"
            :activity="item"
            :net="net_by_id[item.id] ?? 0"
            @click="router.push({ path: `/activity/${item.id}` })"
        />
    </template>
</template>
