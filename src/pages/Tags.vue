<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import ConfirmDialog from '../common/ConfirmDialog.vue';
import Tag from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
const { t } = useI18n();

const emits = defineEmits<{
    createTag: []
}>();

const filter_key: Ref<string> = ref('');
const tags: Ref<Tag[]> = ref([]);

const retrieve_tags = async function() {
    try {
        tags.value = await invoke('retrieve_tags', { filter: filter_key.value });
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}

const deleteTag = async function(id: number) {
    try {
        await invoke('delete_tag', { id });
        await retrieve_tags();
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
};

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <v-card id="card">
        <v-text-field height="50px" clearable density="compact" :placeholder="t('tag.search.hint')" append-inner-icon="mdi-magnify" v-model="filter_key" variant="outlined" @update:model-value="retrieve_tags"></v-text-field>
        <div id="summary">
            <span>{{ t('tag.search.count', tags.length) }}</span>
            <v-btn @click="emits('createTag')" icon="mdi-plus" density="compact" class="right" variant="flat" size="medium"></v-btn>
        </div>
        <v-divider></v-divider>
        <v-list density="compact">
            <v-list-item v-for="tag in tags" :key="tag.name" color="primary">
                <template v-slot:append>
                    <ConfirmDialog title="确认删除？" @confirm="deleteTag(tag.id)">
                        <template v-slot:activator="{ props: confirmDialogActivatorProps }">
                            <v-btn v-bind="confirmDialogActivatorProps" icon="mdi-delete" color="error" density="compact" size="medium"></v-btn>
                        </template>
                        <template v-slot:default>
                            <span>删除后将删除所有关联的收支记录且<span style="color: red;">无法恢复</span></span>
                        </template>
                    </ConfirmDialog>
                </template>
                <template v-slot:prepend>
                    <div :style="{ backgroundColor: tag.color, width: '10px', height: '10px', marginRight: '10px'}"></div>
                </template>
                <v-list-item-title v-text="tag.name"></v-list-item-title>
            </v-list-item>
        </v-list>
    </v-card>
</template>
<style scoped>
#card {
    padding: 10px;
}

.right {
    float: right;
}

#summary {
    margin-top: -10px;
}
</style>