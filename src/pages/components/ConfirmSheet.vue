<script setup lang="ts">
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const model = defineModel<boolean>();

const props = defineProps<{
    title?: string;
    text?: string;
}>();

const emits = defineEmits<{
    confirm: [],
}>();
</script>

<template>
    <v-bottom-sheet v-model="model" >
        <v-card :title="props.title" :text="props.text">
            <v-card-actions>
                <v-btn rounded="xl" @click="emits('confirm'); model = false;">{{ t('actions.confirm') }}</v-btn>
                <v-btn rounded="xl" variant="tonal" @click="model = false">{{ t('actions.cancel') }}</v-btn>
            </v-card-actions>
        </v-card>
    </v-bottom-sheet>
</template>