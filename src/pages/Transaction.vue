<script lang="ts" setup>
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDate } from 'vuetify';
import TransactionList from './components/TransactionList.vue';
import { formatDate, formatDisplayDate } from '../common/Utils';

const { t } = useI18n();
const adapter = useDate();
const keyword = ref('');
const begin_date = ref<Date | undefined>();
const end_date = ref<Date | undefined>();
const show_begin_picker = ref(false);
const show_end_picker = ref(false);
const selecting_begin = ref(new Date());
const selecting_end = ref(new Date());

const allowedDates = function(date: unknown): boolean {
    return adapter.isBefore(date, new Date());
};

const list_key = () => `${keyword.value}|${begin_date.value ? formatDate(begin_date.value) : ''}|${end_date.value ? formatDate(end_date.value) : ''}`;

watch(show_begin_picker, (open) => {
    if (open) {
        selecting_begin.value = begin_date.value ? new Date(begin_date.value) : new Date();
    }
});
watch(show_end_picker, (open) => {
    if (open) {
        selecting_end.value = end_date.value ? new Date(end_date.value) : new Date();
    }
});

const confirm_begin = function() {
    begin_date.value = new Date(selecting_begin.value);
    show_begin_picker.value = false;
};
const confirm_end = function() {
    end_date.value = new Date(selecting_end.value);
    show_end_picker.value = false;
};
const open_picker = function(which: 'begin' | 'end', event?: MouseEvent) {
    const target = event?.target as HTMLElement | undefined;
    if (target?.closest('.v-field__clearable')) {
        return;
    }
    if (which === 'begin') {
        show_begin_picker.value = true;
    } else {
        show_end_picker.value = true;
    }
};
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
                    :model-value="begin_date ? formatDisplayDate(begin_date) : ''"
                    :label="t('transaction.search.begin')"
                    variant="outlined"
                    density="compact"
                    hide-details
                    readonly
                    :clearable="!!begin_date"
                    append-inner-icon="mdi-calendar"
                    @click="open_picker('begin', $event)"
                    @click:append-inner="open_picker('begin')"
                    @click:clear="begin_date = undefined"
                />
                <v-text-field
                    :model-value="end_date ? formatDisplayDate(end_date) : ''"
                    :label="t('transaction.search.end')"
                    variant="outlined"
                    density="compact"
                    hide-details
                    readonly
                    :clearable="!!end_date"
                    append-inner-icon="mdi-calendar"
                    @click="open_picker('end', $event)"
                    @click:append-inner="open_picker('end')"
                    @click:clear="end_date = undefined"
                />
            </div>
            <v-dialog v-model="show_begin_picker" width="auto">
                <v-card>
                    <v-card-text style="padding: 0px;">
                        <v-date-picker v-model="selecting_begin" show-adjacent-months :allowed-dates="allowedDates"></v-date-picker>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn :text="t('actions.cancel')" @click="show_begin_picker = false"></v-btn>
                        <v-btn :text="t('actions.confirm')" @click="confirm_begin"></v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
            <v-dialog v-model="show_end_picker" width="auto">
                <v-card>
                    <v-card-text style="padding: 0px;">
                        <v-date-picker v-model="selecting_end" show-adjacent-months :allowed-dates="allowedDates"></v-date-picker>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn :text="t('actions.cancel')" @click="show_end_picker = false"></v-btn>
                        <v-btn :text="t('actions.confirm')" @click="confirm_end"></v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
        </v-card-text>
    </v-card>
    <TransactionList
        :key="list_key()"
        :keyword="keyword || ''"
        :begin-date="begin_date"
        :end-date="end_date"
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
