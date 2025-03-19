<script lang="ts" setup>
import { ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag from '../common/Tag';
import { invoke } from '@tauri-apps/api/core';
import { formatDatetimeRelative, formatTime } from '../common/Utils';
const { t } = useI18n();

const props = defineProps<{
    title?: string,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain"
}>();

export interface Transaction {
    id: number;
    remark?: string;
    wallet_name: string;
    currency: string;
    tag: Tag;
    amount: number;
    time: Date;
}

const transactions: Ref<Transaction[]> = ref([]);
const current_page: Ref<number> = ref(0);
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
</script>
<template>
    <v-card :variant="variant">
        <v-card-text style="padding: 10px;">
            <div v-if="!!props.title" style="padding-top: 0px; padding-bottom: 0px; padding-left: 10px;">{{ props.title }}</div>
            <v-infinite-scroll :items="transactions" @load="load_transactions">
                <template v-for="transaction in transactions" :key="transaction.id">
                    <v-card variant="text">
                        <v-card-text>
                            <v-row class="flex-nowrap">
                                <v-col class="flex-grow-0" style="padding-left: 0px;">
                                    <v-icon :color="transaction.tag.color">{{ transaction.tag.icon }}</v-icon>
                                </v-col>
                                <v-col style="padding-bottom: 0px;">
                                    <v-row class="flex-nowrap">
                                        <v-col class="no-pad" style="font-size: 2ch;">{{ transaction.tag.name }}</v-col>
                                        <v-col class="no-pad" :style="{ textAlign: 'right', color: transaction.tag.type == 'Expense' ? '#ff3333' : '#009900' }">{{ `${transaction.currency}${(transaction.amount / 100).toFixed(2)}` }}</v-col>
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
                <template v-slot:empty>
                    {{ t('list.no_more_data') }}
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