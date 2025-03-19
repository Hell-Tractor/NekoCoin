<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import Tag from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
const { t } = useI18n();

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
        <v-row class="flex-nowrap" v-for="item in tags" :key="item.id">
            <v-col class="flex-grow-0">
                <v-icon :color="item.color">{{ item.icon }}</v-icon>
            </v-col>
            <v-col>{{ item.name }}</v-col>
            <v-col class="flex-grow-0">
                <v-icon v-if="item.type == 'Income'" color="green">mdi-chart-line-variant</v-icon>
                <v-icon v-else-if="item.type == 'Expense'" color="red" class="v-flipped">mdi-chart-line-variant</v-icon>
                <v-icon v-else color="blue">mdi-swap-horizontal</v-icon>
            </v-col>
        </v-row>
    </v-card>
</template>
<style scoped>
#card {
    padding: 10px;
}

.v-flipped {
    transform: scaleY(-1);
}
</style>