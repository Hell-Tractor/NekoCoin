<script setup lang="ts">
import PrivacyToggle from './PrivacyToggle.vue';

const emits = defineEmits<{
    back: []
}>();

const props = defineProps<{
    title: string;
}>();
</script>

<template>
    <v-app-bar density="compact" color="primary">
        <v-btn icon @click="emits('back')">
            <v-icon>mdi-arrow-left</v-icon>
        </v-btn>
        <v-toolbar-title>{{ props.title }}</v-toolbar-title>
        <template v-slot:append>
            <slot name="append"></slot>
            <PrivacyToggle />
        </template>
    </v-app-bar>
</template>