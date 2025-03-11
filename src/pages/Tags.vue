<script setup lang="ts">
import { computed, ref, Ref } from 'vue';
import ConfirmDialog from '../common/ConfirmDialog.vue';
import Tag from '../common/Tag';
import { useI18n } from 'vue-i18n';
const { t } = useI18n();

const emits = defineEmits<{
    createTag: []
}>();

const tags: Ref<Tag[]> = ref([]);
const filter_key: Ref<string> = ref('');

const filtered_tags = computed<Tag[]>(() => {
    if (!!filter_key.value)
        return tags.value.filter(tag => tag.name.includes(filter_key.value));
    return tags.value;
});

const deleteTag = function(tag: string) {
    tags.value = tags.value.filter(t => t.name !== tag);
};

const addTag = function(tag: Tag) {
    tags.value.push(tag);
};
</script>
<template>
    <v-card id="card">
        <v-text-field height="50px" clearable density="compact" :placeholder="t('tag.search.hint')" append-inner-icon="mdi-magnify" v-model="filter_key" variant="outlined"></v-text-field>
        <div id="summary">
            <span>{{ t('tag.search.count', filtered_tags.length) }}</span>
            <v-btn @click="emits('createTag')" icon="mdi-plus" density="compact" class="right" variant="flat" size="medium"></v-btn>
        </div>
        <v-divider></v-divider>
        <v-list density="compact">
            <v-list-item v-for="tag in filtered_tags" :key="tag.name" color="primary">
                <template v-slot:append>
                    <ConfirmDialog title="确认删除？" @confirm="deleteTag(tag.name)">
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