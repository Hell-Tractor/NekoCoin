<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import Tag from '../common/Tag';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
const { t } = useI18n();
const router = useRouter();

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

onMounted(() => {
    retrieve_tags();
});
</script>
<template>
    <v-card id="card">
        <v-text-field height="50px" clearable density="compact" :placeholder="t('tag.search.hint')" append-inner-icon="mdi-magnify" v-model="filter_key" variant="outlined" @update:model-value="retrieve_tags"></v-text-field>
        <v-card variant="text" block v-for="item in tags" :key="item.id" @click="router.push({ path: `/tag/${item.id}`})">
            <v-card-text style="padding: 12px;">
                <v-row class="flex-nowrap align-center">
                    <v-col class="flex-grow-0">
                        <v-icon :color="item.color" size="x-large">{{ item.icon }}</v-icon>
                    </v-col>
                    <v-col>
                        <v-row><v-col class="text-body-1" style="padding: 0px;">{{ item.name }}</v-col></v-row>
                        <v-row v-if="item.remark"><v-col class="on-surface-lighten-1" style="padding: 2px 0px 0px 0px;">{{ item.remark }}</v-col></v-row>
                    </v-col>
                    <v-col class="flex-grow-0">
                        <v-icon size="large" v-if="item.type == 'Income'" color="green">mdi-chart-line-variant</v-icon>
                        <v-icon size="large" v-else-if="item.type == 'Expense'" color="red" class="v-flipped">mdi-chart-line-variant</v-icon>
                        <v-icon size="large" v-else color="blue">mdi-swap-horizontal</v-icon>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
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