<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import BackTitleBar from './components/BackTitleBar.vue';
import { invoke } from '@tauri-apps/api/core';
import { show_error } from '../common/Notify';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import SummaryBar from './components/SummaryBar.vue';
import { Money } from '../common/Money';
import TransactionList from './components/TransactionList.vue';
import AddActivity from './AddActivity.vue';
import ConfirmSheet from './components/ConfirmSheet.vue';
import StackDiagram from './components/StackDiagram.vue';
import ActivityCard from './components/ActivityCard.vue';
import { TagType } from '../common/Tag';
import { Activity } from '../common/Activity';
import { load_settings, settings } from '../common/Settings';
import Tag from '../common/Tag';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    id: number,
}>();

interface ActivityDetail {
    id: number;
    name: string;
    remark?: string;
    color: string;
    icon: string;
    open: boolean;
    tag_id: number;
    tag: Tag;
}

const activity: Ref<ActivityDetail | undefined> = ref(undefined);
const sum_balance: Ref<{ income: number, expense: number }> = ref({ income: 0, expense: 0 });
const show_confirm_sheet: Ref<boolean> = ref(false);
const toggling_open: Ref<boolean> = ref(false);

const get_activity = async function() {
    try {
        activity.value = await invoke('get_activity_by_id', { id: props.id });
    } catch (error) {
        show_error(error);
    }
}

const get_sum_balance = async function() {
    try {
        sum_balance.value = await invoke('get_sum_balance_in_activity', { activityId: props.id });
    } catch (error) {
        show_error(error);
    }
}

const edit_activity = async function() {
    if (!activity.value) {
        return;
    }
    const init: Activity = {
        id: activity.value.id,
        name: activity.value.name,
        remark: activity.value.remark,
        color: activity.value.color,
        icon: activity.value.icon,
        open: activity.value.open,
        tag_id: activity.value.tag_id,
    };
    if (router.hasRoute('activity_edit')) {
        router.removeRoute('activity_edit');
    }
    router.addRoute({ path: '/activity/edit', name: 'activity_edit', props: { init }, component: AddActivity });
    await router.push({ path: '/activity/edit' });
}

const toggle_open = async function(open: boolean | null) {
    if (!activity.value || toggling_open.value || open === null) {
        return;
    }
    const previous = activity.value.open;
    activity.value.open = open;
    toggling_open.value = true;
    try {
        await invoke('set_activity_open', { id: props.id, open });
    } catch (error) {
        show_error(error);
        activity.value.open = previous;
    } finally {
        toggling_open.value = false;
    }
}

const delete_activity = async function() {
    try {
        await invoke('delete_activity', { id: props.id });
        router.back();
    } catch (error) {
        show_error(error);
    }
}

onMounted(async () => {
    await load_settings();
    await get_activity();
    await get_sum_balance();
});
</script>

<template>
    <BackTitleBar :title="activity?.name ?? t('loading')" @back="router.back()"/>
    <v-main v-if="activity" class="main">
        <ActivityCard variant="flat" layout="hero" :activity="activity">
            <template #extra>
                <div class="activity-category">{{ t('activity.category') }} · {{ activity.tag.name }}</div>
                <v-switch
                    :model-value="activity.open"
                    :label="activity.open ? t('activity.open') : t('activity.closed')"
                    color="primary"
                    hide-details
                    density="compact"
                    :loading="toggling_open"
                    :disabled="toggling_open"
                    class="mt-1"
                    @update:model-value="toggle_open"
                />
            </template>
        </ActivityCard>
        <SummaryBar variant="flat" rounded="xl" :title="t('activity.summary')" :current-expense="new Money(sum_balance.expense, { symbol: '', code: settings.primary_currency_code })" :current-income="new Money(sum_balance.income, { symbol: '', code: settings.primary_currency_code })" />
        <StackDiagram class="mt-2" variant="flat" rounded="xl" :kind="TagType.EXPENSE" :item_id="{ type: 'activity', value: props.id }" :currency_code="settings.primary_currency_code" />
        <div class="detail-actions">
            <v-btn block variant="tonal" rounded="xl" prepend-icon="mdi-pencil" color="secondary-darken-1" :text="t('actions.edit')" @click="edit_activity"></v-btn>
            <v-btn block variant="outlined" rounded="xl" prepend-icon="mdi-delete" color="error" :text="t('actions.delete')" @click="show_confirm_sheet = true"></v-btn>
        </div>
        <TransactionList class="mt-2" variant="flat" :title="t('activity.transactions')" :filter="{ by: 'activity', id: props.id }"/>
        <ConfirmSheet v-model="show_confirm_sheet" :title="t('warning.irrevertible.title')" :text="t('warning.irrevertible.content')" @confirm="delete_activity" />
    </v-main>
</template>

<style scoped>
.activity-category {
    color: rgba(var(--v-theme-on-surface), 0.58);
    font-size: 0.8125rem;
}
</style>
