<script setup lang="ts">
defineProps<{
    title: string;
    tip: string;
}>();
</script>

<template>
    <v-card rounded="xl">
        <v-card-text class="empty-state">
            <div class="empty-state-title">{{ title }}</div>
            <div class="empty-state-tip">{{ tip }}</div>
        </v-card-text>
    </v-card>
</template>
