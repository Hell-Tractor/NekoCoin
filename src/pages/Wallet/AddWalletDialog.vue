<script setup lang="ts">
import { ref, Ref } from 'vue';
import { Wallet } from '../Wallet.vue';
import { rules } from '../../common/Rules.ts';

const emits = defineEmits<{
    confirm: [wallet: Wallet]
}>();

const name: Ref<string> = ref('');
const balance: Ref<string> = ref('0');
const remark: Ref<string> = ref('');
const form: Ref<boolean> = ref(false);
</script>

<template>
    <v-dialog width="100%" height="100%">
        <template v-slot:activator="{ props }">
            <slot name="activator" :props="props"></slot>
        </template>
        <template v-slot:default="{ isActive }">
            <v-form v-model="form">
                <v-card>
                    <v-card-title>添加钱包</v-card-title>
                    <v-card-text>
                        <v-text-field label="名称" variant="underlined" v-model="name" :rules="[rules.required]"></v-text-field>
                        <v-text-field label="余额" variant="underlined" v-model="balance" :rules="[rules.isValidMoney]"></v-text-field>
                        <v-textarea label="备注" variant="underlined" v-model="remark" rows="2"></v-textarea>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn @click="isActive.value = false; emits('confirm', { name, balance: Number.parseFloat(balance), remark })" :disabled="!form">添加</v-btn>
                        <v-btn @click="isActive.value = false">取消</v-btn>
                    </v-card-actions>
                </v-card>
            </v-form>
        </template>
    </v-dialog>
</template>