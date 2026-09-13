<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTheme } from 'vuetify';
import Constants from '../common/Constants';
import { settings, save_settings } from '../common/Settings';

const { t } = useI18n();
const theme = useTheme();
const saved = ref(false);
const theme_items = [
    { value: 'pinkPad', title: 'Pink Pad' },
    { value: 'midnight', title: 'Midnight' },
    { value: 'calico', title: 'Calico' },
    { value: 'neon', title: 'Neon' },
];
const avatar_items = [
    'mdi-cat',
    'mdi-dog',
    'mdi-owl',
    'mdi-panda',
    'mdi-rabbit',
    'mdi-robot',
];

const update_theme = function(value: string) {
    theme.global.name.value = value;
};

const save = async function() {
    try {
        await save_settings();
        saved.value = true;
        window.setTimeout(() => saved.value = false, 1800);
    } catch (error) {
        console.error(error);
    }
};
</script>

<template>
    <v-card rounded="xl">
        <v-card-title>{{ t('settings.title') }}</v-card-title>
        <v-card-text>
            <v-text-field
                v-model="settings.user_name"
                :label="t('settings.user_name')"
                variant="outlined"
                density="comfortable"
            />
            <v-select
                v-model="settings.primary_currency_code"
                :items="Constants.CURRENCIES"
                item-title="code"
                item-value="code"
                :label="t('settings.primary_currency')"
                variant="outlined"
                density="comfortable"
            />
            <v-select
                v-model="settings.theme"
                :items="theme_items"
                item-title="title"
                item-value="value"
                :label="t('settings.theme')"
                variant="outlined"
                density="comfortable"
                @update:model-value="update_theme"
            />
            <v-select
                v-model="settings.avatar"
                :items="avatar_items"
                :label="t('settings.avatar')"
                variant="outlined"
                density="comfortable"
            >
                <template #selection="{ item }">
                    <v-icon class="mr-2">{{ item.raw }}</v-icon>{{ item.raw }}
                </template>
                <template #item="{ props: item_props, item }">
                    <v-list-item v-bind="item_props">
                        <template #prepend><v-icon>{{ item.raw }}</v-icon></template>
                    </v-list-item>
                </template>
            </v-select>
            <v-btn block color="primary" @click="save">{{ t('actions.save') }}</v-btn>
            <v-alert v-if="saved" class="mt-3" type="success" density="compact" variant="tonal">
                {{ t('settings.saved') }}
            </v-alert>
        </v-card-text>
    </v-card>
</template>
