<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';
import BackTitleBar from '../common/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import Constants from '../common/Constants';
import Tag, { TagType, TagTypeNames, TagTypeToString } from '../common/Tag';
import { useDate } from 'vuetify';
import { Wallet } from './Wallet.vue';
import { invoke } from '@tauri-apps/api/core';
import AddWallet from './AddWallet.vue';
import AddTag from './AddTag.vue';
import { formatDatetime } from '../common/Utils';
import WalletSelector from '../common/WalletSelector.vue';
import TagSelector from '../common/TagSelector.vue';
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const form: Ref<boolean> = ref(false);
const remark: Ref<string> = ref('');
const amount: Ref<number | null> = ref(null);
const time: Ref<Date> = ref(new Date());
const selected_tag_type: Ref<TagType> = ref(TagType.EXPENSE);
const show_time_picker: Ref<boolean> = ref(false);
const show_calendar_picker: Ref<boolean> = ref(false);
const selecting_date: Ref<Date> = ref(new Date());
const selecting_time: Ref<string> = ref(`${new Date().getHours()}:${new Date().getMinutes()}`);
const wallets: Ref<Wallet[]> = ref([]);
const selected_wallet: Ref<Wallet | undefined> = ref(undefined);
const selected_to_wallet: Ref<Wallet | undefined> = ref(undefined);
const selected_tag: Ref<Tag | undefined> = ref(undefined);
const page: Ref<string> = ref('main');
const tags: Ref<Tag[]> = ref([]);

const formatter = new Intl.NumberFormat('en-US', { minimumIntegerDigits: 2 });
const updateDate = function() {
    time.value.setFullYear(
        selecting_date.value.getFullYear(),
        selecting_date.value.getMonth(),
        selecting_date.value.getDate()
    );
    maxTime.value = getMaxTime();
    var [maxHour, maxMinute] = maxTime.value.split(':');
    if (time.value.getHours() == parseInt(maxHour)) {
        time.value.setMinutes(Math.min(time.value.getMinutes(), parseInt(maxMinute)));
    } else if (time.value.getHours() > parseInt(maxHour)) {
        time.value.setHours(parseInt(maxHour), parseInt(maxMinute));
    }
    resetTime();
}
const resetDate = function() {
    selecting_date.value = new Date(time.value);
}
const updateTime = function() {
    var [hour, minute] = selecting_time.value.split(':');
    time.value.setHours(parseInt(hour), parseInt(minute));
}
const resetTime = function() {
    selecting_time.value = `${formatter.format(time.value.getHours())}:${formatter.format(time.value.getMinutes())}`;
}
const adapter = useDate();
const allowedDates = function(date: unknown): boolean {
    return adapter.isBefore(date, new Date());
}
const getMaxTime = function() {
    var today = new Date();
    if (today.getDate() == time.value.getDate() && today.getMonth() == time.value.getMonth() && today.getFullYear() == time.value.getFullYear()) {
        return `${formatter.format(today.getHours())}:${formatter.format(today.getMinutes())}`;
    }
    return '23:59';
}
const maxTime: Ref<string> = ref(getMaxTime());
const retrieve_wallets = async function() {
    try {
        wallets.value = await invoke('retrieve_wallets');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const retrieve_tags = async function() {
    try {
        tags.value = await invoke('retrieve_tags', { filter: '', kind: TagTypeToString(selected_tag_type.value) });
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const backFromAddWallet = async function() {
    page.value = 'main';
    await retrieve_wallets();
}
const backFromAddTag = async function() {
    page.value = 'main';
    await retrieve_tags();
}
const addTransaction = async function() {
    try {
        let params = {
            remark: remark.value,
            walletId: selected_wallet.value!.id,
            toWalletId: selected_to_wallet.value?.id,
            tagId: selected_tag.value!.id,
            amount: Math.round(amount.value! * 100),
            time: formatDatetime(time.value)
        };
        await invoke('create_transaction', params);
        emits('back');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const isFormValid = function() {
    return form.value && selected_wallet.value != undefined && selected_tag.value != undefined &&
        (selected_tag.value.type != 'Transfer' || selected_to_wallet.value != undefined);
}
onMounted(() => {
    retrieve_wallets();
    retrieve_tags();
});
</script>
<template>
    <div v-if="page == 'main'">
        <BackTitleBar :title="t('transaction.add')" @back="emits('back')"></BackTitleBar>
        <v-form class="fill-height" v-model="form">
            <v-chip-group mandatory v-model="selected_tag_type" :rules="[rules.required]" @update:model-value="selected_tag = undefined; retrieve_tags()">
                <v-chip v-for="tag in TagTypeNames" :value="tag.type" :key="tag.type" variant="flat" color="secondary">{{ t(`tag.type.${tag.name}`) }}</v-chip>
            </v-chip-group>
            <v-text-field v-model.number="amount" :placeholder="t('transaction.enter.amount')" variant="outlined" density="comfortable" :rules="[rules.required, rules.isValidMoney]"></v-text-field>
            <v-text-field v-model="remark" :placeholder="t('transaction.enter.remark')" variant="outlined" density="comfortable" :rules="[rules.maxLength(Constants.MAX_TRANSACTION_REMARK_LENGTH)]"></v-text-field>
            <v-card variant="text" density="compact">
                <v-card-text>
                    <div style="margin: -10px 0px 5px -10px;">{{ t('transaction.datetime') }}</div>
                    <v-row class="flex-nowrap" style="margin-left: -20px;">
                        <v-col class="flex-grow-1">
                            <v-btn variant="text" prepend-icon="mdi-calendar" width="95%">
                                {{ `${time.getFullYear()}/${time.getMonth() + 1}/${time.getDate()}` }}
                                <v-dialog activator="parent" width="auto" v-model="show_calendar_picker">
                                    <v-card>
                                        <v-card-text style="padding: 0px;">
                                            <v-date-picker v-model="selecting_date" show-adjacent-months :allowed-dates="allowedDates"></v-date-picker>
                                        </v-card-text>
                                        <v-card-actions>
                                            <v-spacer></v-spacer>
                                            <v-btn :text="t('cancel')" @click="show_calendar_picker=false; resetDate()"></v-btn>
                                            <v-btn :text="t('confirm')" @click="show_calendar_picker=false; updateDate()"></v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </v-btn>
                        </v-col>
                        <v-col class="flex-grow-1">
                            <v-btn variant="text" prepend-icon="mdi-clock" width="95%">
                                {{ `${formatter.format(time.getHours())}:${formatter.format(time.getMinutes())}` }}
                                <v-dialog activator="parent" width="auto" v-model="show_time_picker">
                                    <v-card>
                                        <v-card-text style="padding: 0px;">
                                            <v-time-picker v-model="selecting_time" format="24hr" :max="maxTime"></v-time-picker>
                                        </v-card-text>
                                        <v-card-actions>
                                            <v-spacer></v-spacer>
                                            <v-btn :text="t('cancel')" @click="show_time_picker=false; resetTime()  "></v-btn>
                                            <v-btn :text="t('confirm')" @click="show_time_picker=false; updateTime()"></v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </v-btn>
                        </v-col>
                    </v-row>
                </v-card-text>
            </v-card>
            <WalletSelector :title="selected_tag_type == TagType.TRANSFER ? t('account.select_from') : undefined" v-model="selected_wallet" :wallets="wallets" @create="page = 'add_wallet'"></WalletSelector>
            <WalletSelector v-if="selected_tag_type == TagType.TRANSFER" :title="t('account.select_to')" v-model="selected_to_wallet" :wallets="wallets" @create="page = 'add_wallet'"></WalletSelector>
            <TagSelector v-model="selected_tag" :tags="tags" @create="page = 'add_tag'"></TagSelector>
            <v-btn @click="addTransaction" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!isFormValid()">{{ t('save') }}</v-btn>
        </v-form>
    </div>
    <AddWallet v-else-if="page == 'add_wallet'" @back="backFromAddWallet"></AddWallet>
    <AddTag v-else-if="page == 'add_tag'" @back="backFromAddTag"></AddTag>
</template>