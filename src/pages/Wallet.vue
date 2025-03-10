<script setup lang="ts">
import { ref, Ref } from 'vue';
import ConfirmDialog from '../common/ConfirmDialog.vue';

const CARD_MIN_HEIGHT = 100;
const CARD_MIN_WIDTH = 200;

export interface Wallet {
    name: string;
    remark?: string;
    balance: number;
};

const wallets: Ref<Wallet[]> = ref([
    { name: '微信', balance: 0, remark: "一些备注" },
    { name: '现金', balance: 0 },
    { name: '支付宝', balance: 0 },
]);

const deleteWallet = function(name: string) {
    wallets.value = wallets.value.filter(wallet => wallet.name !== name);
};
</script>

<template>
    <v-row>
        <v-col v-for="wallet in wallets">
            <v-card :min-width="CARD_MIN_WIDTH" :min-height="CARD_MIN_HEIGHT" class="fill-height d-flex flex-column">
                <v-card-title>
                    <span>{{ wallet.name }}</span>
                    <span style="float: right;">{{ wallet.balance.toFixed(2) }}</span>
                </v-card-title>
                <v-card-text>{{ wallet.remark }}</v-card-text>
                <v-card-actions id="actions">
                    <v-spacer></v-spacer>
                    <ConfirmDialog title="确认删除？" @confirm="deleteWallet(wallet.name)">
                        <template v-slot:activator="{ props: confirmDialogActivatorProps }">
                            <v-btn v-bind="confirmDialogActivatorProps" density="comfortable" border="thin error" color="error">删除</v-btn>
                        </template>
                        <template v-slot:default>
                            <span>删除后将删除所有关联的收支记录且<span style="color: red;">无法恢复</span></span>
                        </template>
                    </ConfirmDialog>
                </v-card-actions>
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