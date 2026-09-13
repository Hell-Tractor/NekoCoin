<script setup lang="ts">
import { onMounted } from 'vue';
import { useTheme } from 'vuetify';
import { load_settings, settings } from './common/Settings';

const theme = useTheme();

onMounted(async () => {
    await load_settings();
    theme.global.name.value = settings.theme;
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
