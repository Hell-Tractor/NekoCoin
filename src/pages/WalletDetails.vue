<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import WalletCard from './components/WalletCard.vue';
import { Wallet } from './Wallets.vue';
import { invoke } from '@tauri-apps/api/core';
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import SummaryBar from './components/SummaryBar.vue';
import { Money } from '../common/Money';
import TransactionList from './components/TransactionList.vue';
import AddWallet from './AddWallet.vue';
import ConfirmSheet from './components/ConfirmSheet.vue';
import StackDiagram from './components/StackDiagram.vue';
import { TagType } from '../common/Tag';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    id: number,
}>();

const wallet: Ref<Wallet | undefined> = ref(undefined);
const sum_balance: Ref<{ income: number, expense: number }> = ref({ income: 0, expense: 0 });
const show_confirm_sheet: Ref<boolean> = ref(false);

const get_wallet = async function() {
    try {
        wallet.value = await invoke('get_wallet_by_id', { id: props.id });
    } catch (error) {
        console.log(error);
    }
}

const get_sum_balance = async function() {
    try {
        sum_balance.value = await invoke('get_sum_balance_in_wallet', { walletId: props.id });
    } catch (error) {
        console.log(error);
    }
}

const edit_wallet = async function() {
    if (router.hasRoute("wallet_edit")) {
        router.removeRoute("wallet_edit");
    }
    router.addRoute({ path: '/wallet/edit', name: 'wallet_edit', props: { init: wallet.value }, component: AddWallet });
    await router.push({ path: '/wallet/edit' });
}

const delete_wallet = async function() {
    try {
        await invoke('delete_wallet', { id: props.id });
        router.back();
    } catch (error) {
        console.log(error);
    }
}

onMounted(() => {
    get_wallet();
    get_sum_balance();
});
</script>

<template>
    <BackTitleBar :title="t('account.details')" @back="router.back()"/>
    <v-main v-if="wallet" class="main">
        <WalletCard variant="flat" :wallet="wallet" />
        <SummaryBar variant="flat" rounded="lg" :title="t('account.summary')" :current-expense="new Money(sum_balance.expense, { symbol: '', code: wallet.currency_code })" :current-income="new Money(sum_balance.income, { symbol: '', code: wallet.currency_code })" />
        <StackDiagram class="mt-2" variant="flat" rounded="lg" :kind="TagType.EXPENSE" :item_id="{ type: 'wallet', value: props.id }" :currency_code="wallet.currency_code" />
        <v-row class="d-flex" style="margin: 0px;">
            <v-col><v-btn block variant="tonal" rounded="xl" prepend-icon="mdi-pencil" color="secondary-darken-1" :text="t('actions.edit')" @click="edit_wallet"></v-btn></v-col>
            <v-col><v-btn block variant="outlined" rounded="xl" prepend-icon="mdi-delete" color="error" :text="t('actions.delete')" @click="show_confirm_sheet = true"></v-btn></v-col>
        </v-row>
        <TransactionList variant="flat" :title="t('account.transactions')" :filter="{ by: 'wallet', id: props.id }"/>
        <ConfirmSheet v-model="show_confirm_sheet" :title="t('warning.cascade_and_irrevertible.title')" :text="t('warning.cascade_and_irrevertible.content')" @confirm="delete_wallet" />
    </v-main>
</template>