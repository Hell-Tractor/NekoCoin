<script setup lang="ts">
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
    </v-app-bar>
</template>