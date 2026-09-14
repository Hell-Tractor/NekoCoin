<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Wallet } from '../Wallets.vue';
import { useRouter } from 'vue-router';
import { entity_accent_color, entity_avatar_style, entity_tint } from '../../common/Utils';
const { t } = useI18n();
const router = useRouter();

const selected_wallet = defineModel<Wallet>();
const props = defineProps<{
    wallets: Wallet[];
    title?: string;
}>();

const avatar_style = function(color: string) {
    return entity_avatar_style(color);
};

const card_style = function(wallet: Wallet, selected: boolean) {
    const accent = entity_accent_color(wallet.color);
    return {
        background: selected
            ? `linear-gradient(180deg, ${entity_tint(wallet.color)} 0%, ${entity_tint(wallet.color)} 100%)`
            : `linear-gradient(180deg, ${entity_tint(wallet.color)} 0%, transparent 78%)`,
        outline: selected ? `2px solid ${accent}` : '2px solid transparent',
    };
};
</script>

<template>
    <v-card variant="text" density="compact">
        <v-card-text style="padding-bottom: 0;">
            <v-row class="flex-nowarp">
                <v-col style="padding-left: 3px;">
                    <span>{{ t(props.title ?? 'account.select') }}</span>
                </v-col>
                <v-col class="d-flex justify-end">
                    <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="router.push({ path: '/account/add' })"></v-btn>
                </v-col>
            </v-row>
            <v-slide-group v-if="props.wallets.length > 0" class="selector-group" mandatory v-model="selected_wallet">
                <v-slide-group-item v-for="wallet in props.wallets" :key="wallet.id" :value="wallet" v-slot="{ isSelected, toggle }">
                    <button class="wallet-tile" :style="card_style(wallet, isSelected)" type="button" @click="toggle">
                        <div class="wallet-tile-icon" :style="avatar_style(wallet.color)">
                            <v-icon :color="entity_accent_color(wallet.color)" size="22">{{ wallet.icon }}</v-icon>
                        </div>
                        <div class="wallet-tile-name">{{ wallet.name }}</div>
                    </button>
                </v-slide-group-item>
            </v-slide-group>
            <div v-else class="on-surface-lighten-1 text-body-2 mt-2 mb-2">{{ t('account.empty_on_select') }}</div>
        </v-card-text>
    </v-card>
</template>

<style scoped>
.selector-group {
    margin: 4px 0 8px -4px;
}

.wallet-tile {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 92px;
    min-height: 96px;
    margin: 4px;
    padding: 12px 8px 10px;
    border: 0;
    border-radius: 16px;
    cursor: pointer;
}

.wallet-tile-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 14px;
}

.wallet-tile-name {
    max-width: 100%;
    overflow: hidden;
    font-size: 0.8125rem;
    font-weight: 600;
    line-height: 1.2;
    text-overflow: ellipsis;
    white-space: nowrap;
}
</style>
