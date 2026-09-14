<script setup lang="ts">
import { computed } from 'vue';
import { Wallet } from '../Wallets.vue';
import { useI18n } from 'vue-i18n';
import { entity_accent_color, entity_avatar_style, entity_card_style, formatAmount } from '../../common/Utils';
const { t } = useI18n();

const props = defineProps<{
    wallet: Wallet,
    variant?: "flat" | "text" | "elevated" | "tonal" | "outlined" | "plain",
    layout?: 'list' | 'hero',
}>();

const layout = computed(() => props.layout ?? 'list');
const balance_text = computed(() => `${props.wallet.currency_code} ${formatAmount(props.wallet.balance)}`);
const card_style = computed(() => entity_card_style(props.wallet.color));
const avatar_style = computed(() => entity_avatar_style(props.wallet.color));
const accent_color = computed(() => entity_accent_color(props.wallet.color));
</script>

<template>
    <v-card :variant="variant" class="entity-card mb-2" rounded="xl" :style="card_style">
        <div v-if="layout === 'list'" class="entity-row">
            <div class="entity-avatar" :style="avatar_style">
                <v-icon :color="accent_color" size="22">{{ wallet.icon }}</v-icon>
            </div>
            <div class="entity-copy">
                <div class="entity-name">{{ wallet.name }}</div>
                <div v-if="wallet.remark" class="entity-meta">{{ wallet.remark }}</div>
            </div>
            <div class="entity-amount">{{ balance_text }}</div>
        </div>
        <div v-else class="entity-hero">
            <div class="entity-row">
                <div class="entity-avatar entity-avatar-lg" :style="avatar_style">
                    <v-icon :color="accent_color" size="28">{{ wallet.icon }}</v-icon>
                </div>
                <div class="entity-copy">
                    <div class="entity-name">{{ wallet.name }}</div>
                    <div v-if="wallet.remark" class="entity-meta entity-meta-wrap">{{ wallet.remark }}</div>
                </div>
            </div>
            <div class="entity-hero-balance">
                <div class="entity-meta">{{ t('total_balance') }}</div>
                <div class="entity-hero-amount">{{ balance_text }}</div>
            </div>
        </div>
    </v-card>
</template>
