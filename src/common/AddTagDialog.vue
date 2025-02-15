<script setup lang="ts">
import { Ref, ref } from 'vue';
import { Tag } from '../pages/Tags.vue';
import { rules } from './Rules.ts';
import ColorPickDialog from './ColorPickDialog.vue';

const emits = defineEmits<{
    confirm: [tag: Tag]
}>();

const getRandomColor = function() {
    return '#' + Math.floor(Math.random() * 0xffffff).toString(16).padStart(6, '0');
};

const getDefaultTag = function() : Tag {
    return { name: '', color: getRandomColor() };
}

const form: Ref<boolean> = ref(false);
const tag: Ref<Tag> = ref(getDefaultTag());
</script>

<template>
    <v-dialog max-width="60%">
        <template v-slot:activator="{ props }">
            <slot name="activator" :props="props"></slot>
        </template>
        <template v-slot:default="{ isActive }">
            <v-form v-model="form">
                <v-card>
                    <v-card-title>添加标签</v-card-title>
                    <v-card-text>
                        <v-text-field label="名称" variant="underlined" v-model="tag.name" :rules="[rules.required]"></v-text-field>
                        <div>
                            <span>颜色：</span>
                            <ColorPickDialog v-model="tag.color">
                                <template v-slot:activator="{ props: colorPickDialogActivatorProps }">
                                    <v-btn v-bind="colorPickDialogActivatorProps" class="right" :style="{ backgroundColor: tag.color }"></v-btn>
                                </template>
                            </ColorPickDialog>
                        </div>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn @click="isActive.value = false; emits('confirm', tag); tag = getDefaultTag();" :disabled="!form">添加</v-btn>
                        <v-btn @click="isActive.value = false">取消</v-btn>
                    </v-card-actions>
                </v-card>
            </v-form>
        </template>
    </v-dialog>
</template>

<style scoped>
.right {
    float: right;
}
</style>