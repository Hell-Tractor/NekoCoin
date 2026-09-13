<script lang="ts" setup>
import { ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag, { TagType, TagTypeToString } from '../../common/Tag';
import { invoke } from '@tauri-apps/api/core';
import { formatDate, formatDatetimeRelative, formatTime } from '../../common/Utils';
import { useRouter } from 'vue-router';
import AddTransaction from '../AddTransaction.vue';
import ConfirmSheet from './ConfirmSheet.vue';
const { t } = useI18n();
const router = useRouter();

interface TransactionFilter {
    by: "tag" | "wallet",
    id: number,
}

const props = defineProps<{
    title?: string,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
    filter?: TransactionFilter,
}>();

const emits = defineEmits<{
    deleted: [Transaction]
}>();

export interface Transaction {
    id?: number;
    remark?: string;
    wallet_name: string;
    to_wallet_name?: string;
    currency_code: string;
    tag: Tag;
    amount: number;
    time: Date;
    split?: {
        id?: number;
        count: number;
        expense: number;
        receive_wallet_name: string;
    }
}

const transactions: Ref<Transaction[]> = ref([]);
const current_page: Ref<number> = ref(0);
const show_confirm_sheet: Ref<boolean> = ref(false);
const PAGE_SIZE = 20;

const retrieve_transactions = async function(page: number, pageSize: number): Promise<Transaction[]> {
    try {
        let transactions: Transaction[];
        if (props.filter == undefined)
            transactions = await invoke('retrieve_transactions', { page, pageSize });
        else if (props.filter!.by === 'tag')
            transactions = await invoke('retrieve_transactions_with_tag', { page, pageSize, tagId: props.filter!.id });
        else { // props.filter.by === 'wallet'
            transactions = await invoke('retrieve_transactions_in_wallet', { page, pageSize, walletId: props.filter!.id });
        }
        for (let id in transactions) {
            transactions[id].time = new Date(transactions[id].time);
        }
        return transactions;
    } catch (e) {
        console.error(e);
    }
    return [];
}

const load_transactions = async function({ done } : { done: (arg0: any) => void }) {
    const result = await retrieve_transactions(current_page.value, PAGE_SIZE);
    // console.log(result);
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
        console.error(e);
    }
}

const get_color_with_type = function(type: string) {
    if (type == 'Expense') {
        return '#ff3333';
    } else if (type == 'Income') {
        return '#009900';
    } else {
        return '#3399ff';
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

const get_actual_expense = function(transaction: Transaction) {
    if (transaction.split) {
        return transaction.split.expense;
    } else {
        return transaction.amount;
    }
}
</script>
<template>
    <v-card :variant="variant" rounded="lg">
        <v-card-text style="padding: 10px;">
            <div v-if="!!props.title" style="padding-top: 0px; padding-bottom: 0px; padding-left: 10px;">{{ props.title }}</div>
            <v-infinite-scroll :items="transactions" @load="load_transactions">
                <template v-for="transaction in transactions" :key="transaction.id">
                    <v-bottom-sheet>
                        <template v-slot:activator="{ props }">
                            <!-- add inner border box -->
                            <v-card variant="text" rounded="0" v-bind="props" style="border-bottom: 1px solid #e0e0e0">
                                <v-card-text>
                                    <v-row class="flex-nowrap">
                                        <v-col class="flex-grow-0" style="padding-left: 0px;">
                                            <v-icon :color="transaction.tag.color">{{ transaction.tag.icon }}</v-icon>
                                        </v-col>
                                        <v-col style="padding-bottom: 0px;">
                                            <v-row class="flex-nowrap">
                                                <v-col class="no-pad" style="font-size: 2ch;">{{ transaction.tag.name }}</v-col>
                                                <v-col class="no-pad text-end" :style="{ color: get_color_with_type(transaction.tag.type) }">{{ `${transaction.currency_code} ${(get_actual_expense(transaction) / 100).toFixed(2)}` }}</v-col>
                                            </v-row>
                                            <v-row class="flex-nowrap">
                                                <v-col class="no-pad on-surface-lighten-2">{{ formatDatetimeRelative(transaction.time, new Date()) }}</v-col>
                                                <v-col class="no-pad text-end on-surface-lighten-2">{{ formatTime(transaction.time) }}</v-col>
                                            </v-row>
                                        </v-col>
                                    </v-row>
                                </v-card-text>
                            </v-card>
                        </template>
                        <v-card style="padding: 10px">
                            <v-card-text>
                                <v-row class="flex-nowrap">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon :color="transaction.tag.color">{{ transaction.tag.icon }}</v-icon>
                                    </v-col>
                                    <v-col style="padding-bottom: 0px;">
                                        <v-row class="flex-nowrap">
                                            <v-col class="no-pad" style="font-size: 2ch;">{{ transaction.tag.name }}</v-col>
                                            <v-col class="no-pad text-end" :style="{ color: get_color_with_type(transaction.tag.type) }">{{ `${transaction.currency_code} ${(get_actual_expense(transaction) / 100).toFixed(2)}` }}</v-col>
                                        </v-row>
                                        <v-row class="flex-nowrap">
                                            <v-col class="no-pad on-surface-lighten-2">{{ formatDate(transaction.time) }}</v-col>
                                        </v-row>
                                    </v-col>
                                </v-row>
                                <v-divider style="margin-top: 20px; margin-bottom: 20px;"></v-divider>
                                <v-row class="flex-nowrap align-center">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon color="on-surface-lighten-1">mdi-bank</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad on-surface-lighten-1" style="font-size: 0.9em;">{{ t('transaction.account') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.wallet_name }}</v-col></v-row>
                                    </v-col>

                                    <v-col v-if="transaction.tag.type == 'Transfer'">
                                        <v-icon color="on-surface-lighten-1">mdi-chevron-double-right</v-icon>
                                    </v-col>
                                    <v-col v-if="transaction.tag.type == 'Transfer'">
                                        <v-row><v-col class="no-pad on-surface-lighten-1" style="font-size: 0.9em">{{ t('transaction.account') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.to_wallet_name! }}</v-col></v-row>
                                    </v-col>

                                            <v-col v-if="transaction.split" class="no-pad text-end" :style="{ color: get_color_with_type(TagTypeToString(TagType.EXPENSE)) }">{{ `${transaction.currency_code} ${(transaction.amount / 100).toFixed(2)}` }}</v-col>
                                </v-row>
                                <v-row class="flex-nowrap align-center" v-if="transaction.split">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon color="on-surface-lighten-1">mdi-account-multiple</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad on-surface-lighten-1" style="font-size: 0.9em;">{{ t('transaction.split.title') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ t('transaction.split.people', transaction.split!.count) }}</v-col></v-row>
                                    </v-col>
                                    <v-col>
                                        <v-icon color="on-surface-lighten-1">mdi-arrow-right-bold</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad on-surface-lighten-1" style="font-size: 0.9em;">{{ t('transaction.account') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.split!.receive_wallet_name }}</v-col></v-row>
                                    </v-col>
                                            <v-col v-if="transaction.split" class="no-pad text-end" :style="{ color: get_color_with_type(TagTypeToString(TagType.INCOME)) }">{{ `${transaction.currency_code} ${((transaction.amount - transaction.split!.expense) / 100).toFixed(2)}` }}</v-col>
                                </v-row>
                                <v-row class="flex-nowrap" v-if="!!transaction.remark">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon color="on-surface-lighten-1">mdi-file-document</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad on-surface-lighten-1" style="font-size: 0.9em;">{{ t('transaction.remark') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.remark }}</v-col></v-row>
                                    </v-col>
                                </v-row>
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
                    {{ t('list.summary', [transactions.length]) }}
                </template>
            </v-infinite-scroll>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.no-pad {
    padding: 0px;
}
</style>