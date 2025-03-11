<script setup lang="ts">
import { ref, Ref } from 'vue';
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import Constants from '../common/Constants';
import { TagTypeNames } from '../common/Tag';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const form: Ref<boolean> = ref(false);
const remark: Ref<string> = ref('');
const amount: Ref<number | null> = ref(null);
const time: Ref<Date> = ref(new Date());
const selected_tag_type_id: Ref<number> = ref(0);
const show_time_picker: Ref<boolean> = ref(false);
const test: Ref<string> = ref('');

const formatter = new Intl.NumberFormat('en-US', { minimumIntegerDigits: 2 });

const addTransaction = function() {
    emits('back');
}
</script>
<template>
    <BackTitleBar :title="t('transaction.add')" @back="emits('back')"></BackTitleBar>
    <v-form class="fill-height" v-model="form">
        <v-chip-group mandatory v-model="selected_tag_type_id" return-object>
            <v-chip v-for="tag in TagTypeNames" :key="tag.type" variant="flat" color="secondary">{{ t(`tag.type.${tag.name}`) }}</v-chip>
        </v-chip-group>
        <v-text-field v-model="remark" :placeholder="t('transaction.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_TRANSACTION_REMARK_LENGTH)]"></v-text-field>
        <v-text-field v-model.number="amount" :placeholder="t('transaction.enter.amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
        <v-card variant="text" density="compact">
            <v-card-text>
                <div style="margin: -10px 0px 5px -10px;">{{ t('transaction.datetime') }}</div>
                <v-row class="flex-nowrap" style="margin-left: -20px;">
                    <v-col class="flex-grow-1">
                        <v-btn variant="text" prepend-icon="mdi-calendar">{{ `${time.getFullYear()}/${time.getMonth() + 1}/${time.getDay()}` }}</v-btn>
                    </v-col>
                    <v-col class="flex-grow-1">
                        <v-btn variant="text" prepend-icon="mdi-clock">
                            <v-btn-text>{{ `${formatter.format(time.getHours())}:${formatter.format(time.getMinutes())}` }}</v-btn-text>
                            <v-dialog activator="parent" width="auto" v-model="show_time_picker">
                                <v-card>
                                    <v-card-text style="padding: 0px;">
                                        <!-- <v-time-picker style="scale: 0.8; transform: translate(-20%, -15%);" v-if="show_time_picker" v-model="test"></v-time-picker> -->
                                        <v-date-picker></v-date-picker>
                                    </v-card-text>
                                    <v-card-actions>
                                        <v-sapcer></v-sapcer>
                                        <v-btn :text="t('cancel')"></v-btn>
                                        <v-btn :text="t('confirm')"></v-btn>
                                    </v-card-actions>
                                </v-card>
                            </v-dialog>
                        </v-btn>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
        <v-btn @click="addTransaction" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form">{{ t('save') }}</v-btn>
    </v-form>
</template>