<script setup lang="ts">
import { onMounted } from 'vue';
import { useTheme } from 'vuetify';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { load_settings, settings } from './common/Settings';

const theme = useTheme();
const router = useRouter();
const { locale } = useI18n();

onMounted(async () => {
    await load_settings();
    locale.value = settings.locale;
    theme.global.name.value = settings.theme;
    if (window.location.pathname === '/' && settings.default_page !== 'home') {
        await router.replace(`/main/${settings.default_page}`);
    }
});
</script>

<template>
    <v-app>
        <router-view />
    </v-app>
</template>

<style>
.main {
    margin: 10px;
}
</style>
