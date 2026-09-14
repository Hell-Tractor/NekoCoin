<script setup lang="ts">
withDefaults(defineProps<{
    title: string;
    tip: string;
    variant?: 'flat' | 'text' | 'elevated' | 'tonal' | 'outlined' | 'plain';
}>(), {
    variant: 'elevated',
});
</script>

<template>
    <v-card rounded="xl" :variant="variant">
        <v-card-text class="empty-state">
            <div class="empty-state-title">{{ title }}</div>
            <div class="empty-state-tip">{{ tip }}</div>
        </v-card-text>
    </v-card>
</template>
