<script lang="ts" setup>
import { ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag from '../../common/Tag';
import { invoke } from '@tauri-apps/api/core';
import { entity_accent_color, entity_avatar_style, flow_color_for_tag, formatAmount, formatDate, formatDatetimeRelative, formatDisplayDate, formatTime } from '../../common/Utils';
import { useRouter } from 'vue-router';
import AddTransaction from '../AddTransaction.vue';
import ConfirmSheet from './ConfirmSheet.vue';
import { show_error } from '../../common/Notify';
const { t } = useI18n();
const router = useRouter();

interface TransactionFilter {
    by: "tag" | "wallet" | "activity",
    id: number,
}

const props = defineProps<{
    title?: string,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    filter?: TransactionFilter,
    keyword?: string,
    beginDate?: Date,
    endDate?: Date,
}>();

const emits = defineEmits<{
    deleted: [Transaction]
}>();

export interface Transaction {
    id?: number;
    remark?: string;
    wallet_id: number;
    wallet_name: string;
    to_wallet_name?: string;
    currency_code: string;
    tag: Tag;
    activity?: {
        id: number;
        name: string;
        color: string;
        icon: string;
        open: boolean;
    };
    amount: number;
    time: Date;
    split?: {
        id?: number;
        count: number;
        expense: number;
        receive_wallet_id: number;
        receive_wallet_name: string;
    }
}

const transactions: Ref<Transaction[]> = ref([]);
const current_page: Ref<number> = ref(0);
const show_confirm_sheet: Ref<boolean> = ref(false);
const PAGE_SIZE = 20;

const retrieve_transactions = async function(page: number, pageSize: number): Promise<Transaction[]> {
    try {
        const query = {
            page,
            pageSize,
            keyword: props.keyword?.trim() || null,
            begin: props.beginDate ? formatDate(props.beginDate) : null,
            end: props.endDate ? formatDate(props.endDate) : null,
        };
        let transactions: Transaction[];
        if (props.filter == undefined)
            transactions = await invoke('retrieve_transactions', query);
        else if (props.filter!.by === 'tag')
            transactions = await invoke('retrieve_transactions_with_tag', { ...query, tagId: props.filter!.id });
        else if (props.filter!.by === 'activity')
            transactions = await invoke('retrieve_transactions_in_activity', { ...query, activityId: props.filter!.id });
        else {
            transactions = await invoke('retrieve_transactions_in_wallet', { ...query, walletId: props.filter!.id });
        }
        for (let id in transactions) {
            transactions[id].time = new Date(transactions[id].time);
        }
        return transactions;
    } catch (e) {
        show_error(e);
    }
    return [];
}

const load_transactions = async function({ done } : { done: (arg0: any) => void }) {
    const result = await retrieve_transactions(current_page.value, PAGE_SIZE);
    current_page.value += 1;
    transactions.value = transactions.value.concat(result);
    if (result.length != PAGE_SIZE) {
        done('empty');
    } else {
        done('ok');
    }
}

const delete_transaction = async function(transaction: Transaction) {
    try {
        await invoke('delete_transaction', { id: transaction.id });
        transactions.value = transactions.value.filter(t => t.id != transaction.id);
        emits('deleted', transaction);
    } catch (e) {
        show_error(e);
    }
}

const operate_transaction = async function(transaction: Transaction, operation: 'copy' | 'edit') {
    if (operation == 'copy') {
        transaction.id = undefined;
    }
    if (router.hasRoute('transaction_operate')) {
        router.removeRoute('transaction_operate');
    }
    router.addRoute({ path: '/transaction/operate', name: 'transaction_operate', component: AddTransaction, props: { init: transaction } });
    await router.push({ path: '/transaction/operate' });
}

const is_split_income = function(transaction: Transaction) {
    return props.filter?.by === 'wallet'
        && transaction.split != undefined
        && transaction.split.receive_wallet_id === props.filter.id
        && transaction.wallet_id !== props.filter.id;
}

const get_actual_expense = function(transaction: Transaction) {
    if (is_split_income(transaction) && transaction.split) {
        return transaction.amount - transaction.split.expense;
    }
    if (transaction.split) {
        return transaction.split.expense;
    }
    return transaction.amount;
}

const display_flow_type = function(transaction: Transaction) {
    if (is_split_income(transaction)) {
        return 'Income';
    }
    return transaction.tag.type;
}

const display_title = function(transaction: Transaction) {
    if (is_split_income(transaction)) {
        return t('transaction.split.income');
    }
    return transaction.tag.name;
}

const display_meta = function(transaction: Transaction) {
    if (is_split_income(transaction) && transaction.split) {
        return t('transaction.split.from', {
            account: transaction.wallet_name,
            tag: transaction.tag.name,
        });
    }
    const activity = transaction.activity ? `${transaction.activity.name} · ` : '';
    return `${activity}${formatDatetimeRelative(transaction.time, new Date())}`;
}

const money_text = function(cents: number, currency_code: string) {
    return `${currency_code} ${formatAmount(cents)}`;
}
</script>
<template>
    <v-card :variant="variant" rounded="xl">
        <v-card-text class="pa-3">
            <div v-if="!!props.title" class="list-section-title">{{ props.title }}</div>
            <v-infinite-scroll :items="transactions" @load="load_transactions">
                <template v-for="transaction in transactions" :key="transaction.id">
                    <v-bottom-sheet content-class="tx-sheet">
                        <template v-slot:activator="{ props: sheet_props }">
                            <button class="tx-row" type="button" v-bind="sheet_props">
                                <div class="entity-avatar" :style="entity_avatar_style(transaction.tag.color)">
                                    <v-icon :color="entity_accent_color(transaction.tag.color)" size="20">{{ transaction.tag.icon }}</v-icon>
                                </div>
                                <div class="entity-copy">
                                    <div class="entity-name">{{ display_title(transaction) }}</div>
                                    <div class="entity-meta">{{ display_meta(transaction) }}</div>
                                </div>
                                <div class="tx-amount">
                                    <div class="entity-amount" :style="{ color: flow_color_for_tag(display_flow_type(transaction)) }">
                                        {{ money_text(get_actual_expense(transaction), transaction.currency_code) }}
                                    </div>
                                    <div class="entity-meta" style="text-align: right;">{{ formatTime(transaction.time) }}</div>
                                </div>
                            </button>
                        </template>
                        <v-card class="pa-2 tx-sheet-card" rounded="t-xl">
                            <v-card-text>
                                <div class="sheet-row">
                                    <div class="entity-avatar" :style="entity_avatar_style(transaction.tag.color)">
                                        <v-icon :color="entity_accent_color(transaction.tag.color)">{{ transaction.tag.icon }}</v-icon>
                                    </div>
                                    <div class="entity-copy">
                                        <div class="entity-name">{{ display_title(transaction) }}</div>
                                        <div class="entity-meta">{{ formatDisplayDate(transaction.time) }}</div>
                                    </div>
                                    <div class="entity-amount" :style="{ color: flow_color_for_tag(display_flow_type(transaction)) }">
                                        {{ money_text(get_actual_expense(transaction), transaction.currency_code) }}
                                    </div>
                                </div>
                                <v-divider class="my-2"></v-divider>
                                <div class="sheet-row" v-if="is_split_income(transaction)">
                                    <v-icon class="sheet-row-icon" color="success">mdi-call-split</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('transaction.split.income') }}</div>
                                        <div>{{ t('transaction.split.from', { account: transaction.wallet_name, tag: transaction.tag.name }) }}</div>
                                    </div>
                                </div>
                                <div class="sheet-row">
                                    <v-icon class="sheet-row-icon" color="on-surface-lighten-1">mdi-bank</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('transaction.account') }}</div>
                                        <div>{{ transaction.wallet_name }}</div>
                                    </div>
                                    <template v-if="transaction.tag.type == 'Transfer'">
                                        <v-icon color="on-surface-lighten-1">mdi-chevron-double-right</v-icon>
                                        <div class="entity-copy">
                                            <div class="entity-meta">{{ t('transaction.account') }}</div>
                                            <div>{{ transaction.to_wallet_name }}</div>
                                        </div>
                                    </template>
                                    <div v-if="transaction.split" class="entity-amount" :style="{ color: flow_color_for_tag('Expense') }">
                                        {{ money_text(transaction.amount, transaction.currency_code) }}
                                    </div>
                                </div>
                                <div class="sheet-row" v-if="transaction.split">
                                    <v-icon class="sheet-row-icon" color="on-surface-lighten-1">mdi-account-multiple</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('transaction.split.title') }}</div>
                                        <div>{{ t('transaction.split.people', transaction.split.count) }}</div>
                                    </div>
                                    <v-icon color="on-surface-lighten-1">mdi-arrow-right-bold</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('transaction.account') }}</div>
                                        <div>{{ transaction.split.receive_wallet_name }}</div>
                                    </div>
                                    <div class="entity-amount" :style="{ color: flow_color_for_tag('Income') }">
                                        {{ money_text(transaction.amount - transaction.split.expense, transaction.currency_code) }}
                                    </div>
                                </div>
                                <div class="sheet-row" v-if="transaction.activity">
                                    <v-icon class="sheet-row-icon" :color="entity_accent_color(transaction.activity.color)">{{ transaction.activity.icon }}</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('activity.title') }}</div>
                                        <div>{{ transaction.activity.name }}</div>
                                    </div>
                                </div>
                                <div class="sheet-row" v-if="!!transaction.remark">
                                    <v-icon class="sheet-row-icon" color="on-surface-lighten-1">mdi-file-document</v-icon>
                                    <div class="entity-copy">
                                        <div class="entity-meta">{{ t('transaction.remark') }}</div>
                                        <div class="entity-meta-wrap">{{ transaction.remark }}</div>
                                    </div>
                                </div>
                            </v-card-text>
                            <v-card-actions class="d-flex justify-space-around">
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-pencil" variant="tonal" color="primary-darken-1" @click="operate_transaction(transaction, 'edit')">{{ t('actions.edit') }}</v-btn>
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-content-copy" variant="tonal" color="primary-darken-1" @click="operate_transaction(transaction, 'copy')">{{ t('actions.copy') }}</v-btn>
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-delete" variant="outlined" color="error" @click="show_confirm_sheet = true">{{ t('actions.delete') }}</v-btn>
                                <ConfirmSheet @confirm="delete_transaction(transaction)" v-model="show_confirm_sheet" :title="t('warning.irrevertible.title')" :text="t('warning.irrevertible.content')"></ConfirmSheet>
                            </v-card-actions>
                        </v-card>
                    </v-bottom-sheet>
                </template>
                <template v-slot:empty>
                    <div v-if="transactions.length === 0" class="empty-state">
                        <div class="empty-state-title">{{ t('transaction.no_transaction') }}</div>
                        <div class="empty-state-tip">{{ t('transaction.no_transaction_tip') }}</div>
                    </div>
                    <div v-else class="entity-meta">{{ t('list.summary', [transactions.length]) }}</div>
                </template>
            </v-infinite-scroll>
        </v-card-text>
    </v-card>
</template>

<style>
.tx-sheet,
.tx-sheet .v-bottom-sheet__content,
.tx-sheet-card {
    border-bottom-left-radius: 0 !important;
    border-bottom-right-radius: 0 !important;
}
</style>
