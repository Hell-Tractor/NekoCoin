<script setup lang="ts">
withDefaults(defineProps<{
    inputmode?: 'decimal' | 'numeric';
}>(), {
    inputmode: 'decimal',
});

const model = defineModel<number | string | null>();
</script>

<template>
    <v-text-field
        v-model.number="model"
        type="text"
        :inputmode="inputmode"
        autocomplete="off"
        enterkeyhint="done"
    />
</template>
