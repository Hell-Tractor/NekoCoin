<script lang="ts" setup>
import { onMounted, ref, Ref } from 'vue';
import { useI18n } from 'vue-i18n';
import Tag from '../common/Tag';
import { invoke } from '@tauri-apps/api/core';
const { t } = useI18n();

export interface Transaction {
    id: number;
    remark?: string;
    wallet_name: string;
    currency: string;
    tag: Tag;
    amount: number;
    time: string;
}

const transactions: Ref<Transaction[]> = ref([]);
const current_page: Ref<number> = ref(0);
const PAGE_SIZE = 20;

const retrieve_transactions = async function(page: number, pageSize: number): Promise<Transaction[]> {
    try {
        return await invoke('retrieve_transactions', { page, pageSize });
    } catch (e) {
        console.error(e);
    }
    return [];
}

const load_transactions = async function({ done } : { done: (arg0: any) => void }) {
    const result = await retrieve_transactions(current_page.value, PAGE_SIZE);
    console.log(result);
    current_page.value += 1;
    if (result.length === 0) {
        done('empty');
    } else {
        transactions.value = transactions.value.concat(result);
        done('ok');
    }
}
</script>

<template>
    <v-infinite-scroll :items="transactions" @load="load_transactions">
        <template v-for="transaction in transactions" :key="transaction.id">
            <v-card>
                <v-card-text>{{ transaction }}</v-card-text>
            </v-card>
        </template>
    </v-infinite-scroll>
</template>