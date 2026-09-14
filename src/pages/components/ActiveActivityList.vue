<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { Activity } from '../../common/Activity';
import { load_settings, settings } from '../../common/Settings';
import { color_with_alpha, format_net_cash_flow, net_cash_flow_color } from '../../common/Utils';

const PREVIEW_COUNT = 3;
const { t } = useI18n();
const router = useRouter();
const activities = ref<Activity[]>([]);
const net_by_id = ref<Record<number, number>>({});
const expanded = ref(false);
const closing_id = ref<number | undefined>(undefined);

const visible_activities = computed(() => {
    if (expanded.value || activities.value.length <= PREVIEW_COUNT) {
        return activities.value;
    }
    return activities.value.slice(0, PREVIEW_COUNT);
});
const hidden_count = computed(() => Math.max(0, activities.value.length - PREVIEW_COUNT));

const avatar_style = function(color: string) {
    return { backgroundColor: color_with_alpha(color, 0.22) };
};

const retrieve_open_activities = async function() {
    try {
        await load_settings();
        activities.value = await invoke('retrieve_activities', { openOnly: true });
        const balances = await invoke('get_activity_balances', {
            currencyCode: settings.primary_currency_code,
            openOnly: true,
        }) as { id: number, income: number, expense: number }[];
        const next: Record<number, number> = {};
        for (const item of balances) {
            next[item.id] = item.income - item.expense;
        }
        net_by_id.value = next;
        if (activities.value.length <= PREVIEW_COUNT) {
            expanded.value = false;
        }
    } catch (error) {
        console.error(error);
    }
};

const close_activity = async function(activity: Activity, event: Event) {
    event.stopPropagation();
    if (closing_id.value !== undefined) {
        return;
    }
    closing_id.value = activity.id;
    try {
        await invoke('set_activity_open', { id: activity.id, open: false });
        activities.value = activities.value.filter(item => item.id !== activity.id);
        if (activities.value.length <= PREVIEW_COUNT) {
            expanded.value = false;
        }
    } catch (error) {
        console.error(error);
    } finally {
        closing_id.value = undefined;
    }
};

onMounted(retrieve_open_activities);
</script>

<template>
    <v-card v-if="activities.length > 0" rounded="xl" class="mb-2">
        <v-card-title class="text-body-1 font-weight-bold">{{ t('activity.active_on_home') }}</v-card-title>
        <v-card-text class="pt-0 px-3 pb-3">
            <div
                v-for="item in visible_activities"
                :key="item.id"
                class="activity-row"
                @click="router.push({ path: `/activity/${item.id}` })"
            >
                <div class="activity-avatar" :style="avatar_style(item.color)">
                    <v-icon :color="item.color" size="20">{{ item.icon }}</v-icon>
                </div>
                <div class="activity-copy">
                    <div class="activity-name">{{ item.name }}</div>
                    <div class="activity-net" :style="{ color: net_cash_flow_color(net_by_id[item.id] ?? 0) || undefined }">
                        {{ format_net_cash_flow(net_by_id[item.id] ?? 0) }}
                    </div>
                </div>
                <v-btn
                    size="small"
                    variant="tonal"
                    color="error"
                    :loading="closing_id === item.id"
                    @click.stop="close_activity(item, $event)"
                >
                    {{ t('activity.close') }}
                </v-btn>
            </div>
            <v-btn
                v-if="hidden_count > 0"
                variant="text"
                color="primary"
                size="small"
                block
                class="mt-1"
                :prepend-icon="expanded ? 'mdi-chevron-up' : 'mdi-chevron-down'"
                @click="expanded = !expanded"
            >
                {{ expanded ? t('activity.show_less') : t('activity.show_more', { count: hidden_count }) }}
            </v-btn>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.activity-row {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
    padding: 10px 8px;
    border-radius: 14px;
    cursor: pointer;
}

.activity-row:active {
    background: rgba(var(--v-theme-on-surface), 0.06);
}

.activity-avatar {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    width: 38px;
    height: 38px;
    border-radius: 12px;
}

.activity-copy {
    min-width: 0;
    flex: 1 1 auto;
}

.activity-name {
    overflow: hidden;
    font-weight: 600;
    line-height: 1.3;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.activity-net {
    overflow: hidden;
    margin-top: 2px;
    color: rgba(var(--v-theme-on-surface), 0.58);
    font-size: 0.75rem;
    text-overflow: ellipsis;
    white-space: nowrap;
}
</style>
