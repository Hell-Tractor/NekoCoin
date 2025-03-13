<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import ConfirmDialog from '../common/ConfirmDialog.vue';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const CARD_MIN_HEIGHT = 100;
const CARD_MIN_WIDTH = 200;

export interface Wallet {
    id: number;
    name: string;
    remark?: string;
    balance: number;
    currency: string;
    icon: string;
    color: string;
};

const wallets: Ref<Wallet[]> = ref([]);

const retrieveWallets = async function() {
    try {
        wallets.value = await invoke('retrieve_wallets');
        console.log(wallets.value);
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const deleteWallet = function(id: number) {
    try {
        invoke('delete_wallet', { id });
        wallets.value = wallets.value.filter(wallet => wallet.id !== id);
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
};

onMounted(() => {
    retrieveWallets();
});
</script>

<template>
    <v-row v-if="wallets.length > 0">
        <v-col v-for="wallet in wallets">
            <v-card :min-width="CARD_MIN_WIDTH" :min-height="CARD_MIN_HEIGHT" class="fill-height d-flex flex-column" :color="wallet.color">
                <v-card-title>
                    <v-icon style="margin-right: 10px;">{{ wallet.icon }}</v-icon>
                    <span>{{ wallet.name }}</span>
                    <span style="float: right;">{{ (wallet.balance / 100).toFixed(2) + ' ' + wallet.currency }}</span>
                </v-card-title>
                <v-card-text>{{ wallet.remark }}</v-card-text>
                <v-card-actions id="actions">
                    <v-spacer></v-spacer>
                    <ConfirmDialog title="确认删除？" @confirm="deleteWallet(wallet.id)">
                        <template v-slot:activator="{ props: confirmDialogActivatorProps }">
                            <v-btn v-bind="confirmDialogActivatorProps" density="comfortable" border="thin error">删除</v-btn>
                        </template>
                        <template v-slot:default>
                            <span>删除后将删除所有关联的收支记录且<span style="color: red;">无法恢复</span></span>
                        </template>
                    </ConfirmDialog>
                </v-card-actions>
            </v-card>
        </v-col>
    </v-row>
    <v-row v-else>
        <v-col>
            <v-card class="fill-height d-flex flex-column">
                <v-card-title>{{ t('account.no_account') }}</v-card-title>
                <v-card-text>{{ t('account.no_account_tip') }}</v-card-text>
            </v-card>
        </v-col>
    </v-row>
</template>

<style scoped>
#actions {
    margin-top: -10px;
    margin-bottom: -5px;
}
</style>