<script lang="ts" setup>
import { ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag from '../common/Tag';
import { invoke } from '@tauri-apps/api/core';
import { formatDate, formatDatetimeRelative, formatTime } from '../common/Utils';
import { useRouter } from 'vue-router';
import AddTransaction from '../pages/AddTransaction.vue';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    title?: string,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
}>();

export interface Transaction {
    id?: number;
    remark?: string;
    wallet_name: string;
    to_wallet_name?: string;
    currency: string;
    tag: Tag;
    amount: number;
    time: Date;
    split?: {
        id?: number;
        count: number;
        expense: number;
        recieve_wallet_name: string;
    }
}

const transactions: Ref<Transaction[]> = ref([]);
const current_page: Ref<number> = ref(0);
const show_confirm_sheet: Ref<boolean> = ref(false);
const PAGE_SIZE = 20;

const retrieve_transactions = async function(page: number, pageSize: number): Promise<Transaction[]> {
    try {
        let transactions: Transaction[] = await invoke('retrieve_transactions', { page, pageSize });
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
    let removeRoute = router.addRoute({ path: '/transaction/operate', component: AddTransaction, props: { init: transaction } });
    await router.push({ path: '/transaction/operate' });
    removeRoute();
}
</script>
<template>
    <v-card :variant="variant">
        <v-card-text style="padding: 10px;">
            <div v-if="!!props.title" style="padding-top: 0px; padding-bottom: 0px; padding-left: 10px;">{{ props.title }}</div>
            <v-infinite-scroll :items="transactions" @load="load_transactions">
                <template v-for="transaction in transactions" :key="transaction.id">
                    <v-bottom-sheet>
                        <template v-slot:activator="{ props }">
                            <v-card variant="text" v-bind="props">
                                <v-card-text>
                                    <v-row class="flex-nowrap">
                                        <v-col class="flex-grow-0" style="padding-left: 0px;">
                                            <v-icon :color="transaction.tag.color">{{ transaction.tag.icon }}</v-icon>
                                        </v-col>
                                        <v-col style="padding-bottom: 0px;">
                                            <v-row class="flex-nowrap">
                                                <v-col class="no-pad" style="font-size: 2ch;">{{ transaction.tag.name }}</v-col>
                                                <v-col class="no-pad" :style="{ textAlign: 'right', color: get_color_with_type(transaction.tag.type) }">{{ `${transaction.currency}${(transaction.amount / 100).toFixed(2)}` }}</v-col>
                                            </v-row>
                                            <v-row class="flex-nowrap">
                                                <v-col class="no-pad" style="color: #666666;">{{ formatDatetimeRelative(transaction.time, new Date()) }}</v-col>
                                                <v-col class="no-pad" style="text-align: right; color: #666666;">{{ formatTime(transaction.time) }}</v-col>
                                            </v-row>
                                            <v-divider style="margin-top: 20px;"></v-divider>
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
                                            <v-col class="no-pad" :style="{ textAlign: 'right', color: get_color_with_type(transaction.tag.type) }">{{ `${transaction.currency}${(transaction.amount / 100).toFixed(2)}` }}</v-col>
                                        </v-row>
                                        <v-row class="flex-nowrap">
                                            <v-col class="no-pad" style="color: #666666;">{{ formatDate(transaction.time) }}</v-col>
                                        </v-row>
                                    </v-col>
                                </v-row>
                                <v-divider style="margin-top: 20px; margin-bottom: 20px;"></v-divider>
                                <v-row class="flex-nowrap">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon color="#444444">mdi-bank</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad" style="font-size: 0.9em; color: #444444;">{{ t('transaction.account') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.wallet_name }}</v-col></v-row>
                                    </v-col>
                                    <v-col v-if="transaction.tag.type == 'Transfer'">
                                        <v-icon>mdi-chevron-double-right</v-icon>
                                    </v-col>
                                    <v-col v-if="transaction.tag.type == 'Transfer'">
                                        <v-row><v-col class="no-pad" style="font-size: 0.9em; color: #444444;">{{ t('transaction.account') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.to_wallet_name! }}</v-col></v-row>
                                    </v-col>
                                </v-row>
                                <v-row class="flex-nowrap" v-if="!!transaction.remark">
                                    <v-col class="flex-grow-0" style="padding-left: 0px;">
                                        <v-icon color="#444444">mdi-file-document</v-icon>
                                    </v-col>
                                    <v-col>
                                        <v-row><v-col class="no-pad" style="font-size: 0.9em; color: #444444;">{{ t('transaction.remark') }}</v-col></v-row>
                                        <v-row><v-col class="no-pad">{{ transaction.remark }}</v-col></v-row>
                                    </v-col>
                                </v-row>
                            </v-card-text>
                            <v-card-actions class="d-flex justify-space-around">
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-pencil" variant="tonal" color="primary-darken-1" @click="operate_transaction(transaction, 'edit')">{{ t('actions.edit') }}</v-btn>
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-content-copy" variant="tonal" color="primary-darken-1" @click="operate_transaction(transaction, 'copy')">{{ t('actions.copy') }}</v-btn>
                                <v-btn rounded="xl" class="flex-grow-1" prepend-icon="mdi-delete" variant="outlined" color="error" @click="show_confirm_sheet = true">{{ t('actions.delete') }}</v-btn>
                                <v-bottom-sheet v-model="show_confirm_sheet" >
                                    <template>
                                    </template>
                                    <v-card :title="t('warning.irrevertible.title')" :text="t('warning.irrevertible.content')">
                                        <v-card-actions>
                                            <v-btn rounded="xl" @click="delete_transaction(transaction); show_confirm_sheet = false;">{{ t('actions.confirm') }}</v-btn>
                                            <v-btn rounded="xl" variant="tonal" @click="show_confirm_sheet = false">{{ t('actions.cancel') }}</v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-bottom-sheet>
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