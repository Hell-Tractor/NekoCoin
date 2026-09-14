<script lang="ts" setup>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import TransactionList from './components/TransactionList.vue';

const { t } = useI18n();
const keyword = ref('');
const begin_date = ref('');
const end_date = ref('');

const parsed_begin = () => begin_date.value ? new Date(`${begin_date.value}T00:00:00`) : undefined;
const parsed_end = () => end_date.value ? new Date(`${end_date.value}T00:00:00`) : undefined;
const list_key = () => `${keyword.value}|${begin_date.value}|${end_date.value}`;
</script>

<template>
    <v-card class="mb-2" variant="flat" rounded="xl">
        <v-card-text class="pb-2">
            <v-text-field
                v-model="keyword"
                :placeholder="t('transaction.search.hint')"
                variant="outlined"
                density="compact"
                hide-details
                clearable
                append-inner-icon="mdi-magnify"
            />
            <div class="date-row">
                <v-text-field
                    v-model="begin_date"
                    :label="t('transaction.search.begin')"
                    type="date"
                    variant="outlined"
                    density="compact"
                    hide-details
                    clearable
                />
                <v-text-field
                    v-model="end_date"
                    :label="t('transaction.search.end')"
                    type="date"
                    variant="outlined"
                    density="compact"
                    hide-details
                    clearable
                />
            </div>
        </v-card-text>
    </v-card>
    <TransactionList
        :key="list_key()"
                :keyword="keyword || ''"
        :begin-date="parsed_begin()"
        :end-date="parsed_end()"
    />
</template>

<style scoped>
.date-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    margin-top: 8px;
}
</style>
