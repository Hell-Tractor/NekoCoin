<script setup lang="ts">
const emits = defineEmits<{
    confirm: []
}>();

const props = defineProps<{
    title?: string;
    content?: string;
}>();
</script>

<template>
    <v-dialog max-width="max(60%, 260px)">
        <template v-slot:activator="{ props }">
            <slot name="activator" :props="props"></slot>
        </template>
        <template v-slot:default="{ isActive }">
            <v-card>
                <v-card-title v-if="props.title">{{ props.title }}</v-card-title>
                <v-card-text v-if="props.content != undefined">{{ props.content }}</v-card-text>
                <v-card-text v-else>
                    <slot></slot>
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn @click="isActive.value = false; emits('confirm')" color="warning">确定</v-btn>
                    <v-btn @click="isActive.value = false" color="success">取消</v-btn>
                </v-card-actions>
            </v-card>
        </template>
    </v-dialog>
</template>