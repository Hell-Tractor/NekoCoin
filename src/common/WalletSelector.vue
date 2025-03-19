<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Wallet } from '../pages/Wallet.vue';
const { t } = useI18n();

const selected_wallet = defineModel<Wallet>();
const props = defineProps<{
    wallets: Wallet[];
    title?: string;
}>();
const emits = defineEmits<{
    create: [];
}>()
</script>

<template>
    <v-card variant="text" density="compact">
        <v-card-text style="padding-bottom: 0;">
            <v-row class="flex-nowarp">
                <v-col style="padding-left: 3px;">
                    <span>{{ t(props.title ?? 'account.select') }}</span>
                </v-col>
                <v-col class="d-flex justify-end">
                    <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="emits('create')"></v-btn>
                </v-col>
            </v-row>
            <v-slide-group class="pa-4" style="margin-left: -20px;" mandatory v-model="selected_wallet">
                <v-slide-group-item v-for="wallet in props.wallets" :key="wallet.id" :value="wallet" v-slot="{ isSelected, toggle }">
                    <v-card @click="toggle" :border="isSelected ? 'opacity-100 primary md' : ''" width="100" height="100" class="ma-1">
                        <v-card-text style="padding: 10px;">
                            <v-icon :color="wallet.color">{{ wallet.icon }}</v-icon>
                            <div>{{ (wallet.remark?.length ?? 0) > 5 ? (wallet.remark!.substring(0, 4) + '...') : (wallet.remark?.substring(0, 5) || '') }}</div>
                            <div style="position: absolute; bottom: 10px;" class="font-weight-black">{{ wallet.name }}</div>
                        </v-card-text>
                    </v-card>
                </v-slide-group-item>
            </v-slide-group>
        </v-card-text>
    </v-card>
</template>