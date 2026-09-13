<script setup lang="ts">
import { computed } from 'vue';
import { settings } from '../../common/Settings';
import { get_theme_color_palette } from '../../themes/palettes';

const props = defineProps<{
    modelValue: string;
}>();

const emit = defineEmits<{
    'update:modelValue': [value: string];
}>();

const palette = computed(() => get_theme_color_palette(settings.theme));
</script>

<template>
    <div class="palette" role="group">
        <button
            v-for="color in palette"
            :key="color"
            class="color-swatch"
            :class="{ selected: props.modelValue.toUpperCase() === color.toUpperCase() }"
            type="button"
            :style="{ backgroundColor: color }"
            :aria-label="color"
            @click="emit('update:modelValue', color)"
        ></button>
    </div>
</template>

<style scoped>
.palette {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    padding: 4px 0 8px;
}

.color-swatch {
    width: 32px;
    height: 32px;
    border: 2px solid transparent;
    border-radius: 50%;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.16);
    cursor: pointer;
    transition: transform 120ms ease, border-color 120ms ease;
}

.color-swatch:hover {
    transform: scale(1.1);
}

.color-swatch.selected {
    border-color: rgb(var(--v-theme-on-surface));
    box-shadow: 0 0 0 2px rgb(var(--v-theme-surface)), 0 0 0 4px rgb(var(--v-theme-on-surface));
}
</style>
