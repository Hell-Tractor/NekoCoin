<script setup lang="ts">
import { computed, ComputedRef, nextTick, onBeforeUnmount, onMounted, ref, Ref, watch } from 'vue';
import BackTitleBar from './components/BackTitleBar.vue';
import { useI18n } from 'vue-i18n';
import { rules } from '../common/Rules';
import Constants from '../common/Constants';
import Tag, { TagType, TagTypeFromString, TagTypeNames, TagTypeToString } from '../common/Tag';
import { useDate } from 'vuetify';
import { Wallet } from './Wallets.vue';
import { invoke } from '@tauri-apps/api/core';
import { formatDatetime, formatTime } from '../common/Utils';
import { load_settings, settings } from '../common/Settings';
import WalletSelector from './components/WalletSelector.vue';
import TagSelector from './components/TagSelector.vue';
import { Transaction } from './components/TransactionList.vue';
import { onBeforeRouteLeave, useRouter } from 'vue-router';
import { clear_transaction_draft, transaction_draft } from '../common/TransactionDraft';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    init?: Transaction
}>();

const id: Ref<number | undefined> = ref(undefined);
const split_id: Ref<number | undefined> = ref(undefined);
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
const tags: Ref<Tag[]> = ref([]);
const has_split: Ref<boolean> = ref(false);
const split_count: Ref<number> = ref(2);
const split_expense: Ref<number> = ref(0);
const split_receive_wallet: Ref<Wallet | undefined> = ref(undefined);
let tag_request_id = 0;
let restoring_draft = false;

const others_expense: ComputedRef<number> = computed(() => {
    return Number.parseInt(Math.ceil((amount.value ?? 0) * 100 / split_count.value).toFixed(0)) / 100;
});

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
    save_draft();
}
const resetDate = function() {
    selecting_date.value = new Date(time.value);
}
const updateTime = function() {
    var [hour, minute] = selecting_time.value.split(':');
    time.value.setHours(parseInt(hour), parseInt(minute));
    save_draft();
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

const save_draft = function() {
    if (props.init) {
        return;
    }
    transaction_draft.active = true;
    transaction_draft.remark = remark.value;
    transaction_draft.amount = amount.value;
    transaction_draft.time = time.value.toISOString();
    const tag_type = typeof selected_tag_type.value === 'string'
        ? TagTypeFromString(selected_tag_type.value)
        : selected_tag_type.value;
    transaction_draft.selected_tag_type = TagTypeToString(tag_type) as 'Expense' | 'Income' | 'Transfer';
    transaction_draft.wallet_id = selected_wallet.value?.id;
    transaction_draft.to_wallet_id = selected_to_wallet.value?.id;
    transaction_draft.tag_id = selected_tag.value?.id;
    transaction_draft.tag = selected_tag.value ? { ...selected_tag.value } : undefined;
    transaction_draft.has_split = has_split.value;
    transaction_draft.split_count = split_count.value;
    transaction_draft.split_expense = split_expense.value;
    transaction_draft.split_receive_wallet_id = split_receive_wallet.value?.id;
};

const restore_draft = function() {
    remark.value = transaction_draft.remark;
    amount.value = transaction_draft.amount;
    time.value = new Date(transaction_draft.time);
    selecting_date.value = new Date(time.value);
    resetTime();
    selected_tag_type.value = TagTypeFromString(transaction_draft.selected_tag_type);
    selected_wallet.value = wallets.value.find(wallet => wallet.id === transaction_draft.wallet_id);
    selected_to_wallet.value = wallets.value.find(wallet => wallet.id === transaction_draft.to_wallet_id);
    selected_tag.value = tags.value.find(tag => tag.id === transaction_draft.tag_id) ?? transaction_draft.tag;
    has_split.value = transaction_draft.has_split;
    split_count.value = transaction_draft.split_count;
    split_expense.value = transaction_draft.split_expense;
    split_receive_wallet.value = wallets.value.find(wallet => wallet.id === transaction_draft.split_receive_wallet_id);
};

const restore_draft_after_tags_loaded = async function() {
    restoring_draft = true;
    selected_tag_type.value = TagTypeFromString(transaction_draft.selected_tag_type);
    await retrieve_tags();
    await nextTick();
    restore_draft();
    await nextTick();
    restoring_draft = false;
};

const maxTime: Ref<string> = ref(getMaxTime());
const time_picker_format = computed(() => settings.time_format === '12hr' ? 'ampm' : '24hr');
const retrieve_wallets = async function() {
    try {
        wallets.value = await invoke('retrieve_wallets');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const retrieve_tags = async function() {
    const current_request_id = ++tag_request_id;
    try {
        const tag_type = typeof selected_tag_type.value === 'string'
            ? TagTypeFromString(selected_tag_type.value)
            : selected_tag_type.value;
        const result = await invoke('retrieve_tags', { filter: '', kind: TagTypeToString(tag_type) }) as Tag[];
        if (current_request_id === tag_request_id) {
            tags.value = result;
        }
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const confirm = async function() {
    try {
        let spilt = {
            id: split_id.value,
            count: split_count.value,
            expense: Math.round(split_expense.value * 100),
            receiveWalletId: split_receive_wallet.value?.id,
        };
        let params = {
            id: id.value,
            remark: remark.value,
            walletId: selected_wallet.value!.id,
            toWalletId: selected_to_wallet.value?.id,
            tagId: selected_tag.value!.id,
            amount: Math.round(amount.value! * 100),
            time: formatDatetime(time.value),
            split: has_split.value && selected_tag_type.value == TagType.EXPENSE ? spilt : undefined,
        };
        console.log('params:', params);
        if (!params.id)
            await invoke('create_transaction', { vo: params });
        else
            await invoke('update_transaction', { vo: params });
        clear_transaction_draft();
        router.back();
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
}
const isFormValid = function() {
    let basic = form.value && selected_wallet.value != undefined && selected_tag.value != undefined &&
        (selected_tag.value.type != 'Transfer' || selected_to_wallet.value != undefined);
    let split = !has_split.value || split_receive_wallet.value != undefined;
    return basic && split;
}
const update_split_expense = function(count: number) {
    split_expense.value = Number.parseFloat(((amount.value ?? 0) - (count - 1) * others_expense.value).toFixed(2));
}
watch(split_count, function(newValue) {
    update_split_expense(newValue);
});
watch(has_split, function(newValue) {
    if (newValue) {
        update_split_expense(split_count.value);
        if (split_receive_wallet.value == undefined) {
            split_receive_wallet.value = selected_wallet.value;
        }
    }
});
watch(amount, function(_newValue) {
    update_split_expense(split_count.value);
})
watch(selected_tag_type, function(newValue, oldValue) {
    if (restoring_draft || newValue === oldValue) {
        return;
    }
    selected_tag.value = undefined;
    retrieve_tags();
});
watch([
    remark,
    amount,
    selected_tag_type,
    selected_wallet,
    selected_to_wallet,
    selected_tag,
    has_split,
    split_count,
    split_expense,
    split_receive_wallet,
], () => {
    if (!restoring_draft) {
        save_draft();
    }
});
onMounted(async () => {
    await load_settings();
    await retrieve_wallets();

    if (props.init) {
        clear_transaction_draft();
        selected_tag_type.value = TagTypeFromString(props.init.tag.type);
        await retrieve_tags();
        id.value = props.init.id;
        amount.value = props.init.amount / 100;
        remark.value = props.init.remark || '';
        time.value = props.init.time;
        selected_wallet.value = wallets.value.find(wallet => wallet.name == props.init!.wallet_name);
        selected_tag.value = tags.value.find(tag => tag.id == props.init!.tag.id) ?? props.init.tag;
        if (props.init!.to_wallet_name) {
            selected_to_wallet.value = wallets.value.find(wallet => wallet.name == props.init!.to_wallet_name);
        }
        has_split.value = props.init!.split != undefined;
        // console.log(props.init);
        if (props.init!.split) {
            split_id.value = props.init!.split.id;
            split_count.value = props.init!.split.count;
            split_expense.value = props.init!.split.expense;
            split_receive_wallet.value = wallets.value.find(wallet => wallet.name == props.init!.split!.receive_wallet_name);
        }
    } else if (transaction_draft.active) {
        await restore_draft_after_tags_loaded();
    } else {
        await retrieve_tags();
    }
});

onBeforeUnmount(() => {
    save_draft();
});

onBeforeRouteLeave(() => {
    save_draft();
});
</script>
<template>
    <BackTitleBar :title="t(id == undefined ? 'transaction.add' : 'transaction.update')" @back="router.back()"></BackTitleBar>
    <v-main class="main">
        <v-form class="fill-height" v-model="form">
            <v-chip-group mandatory v-model="selected_tag_type" :rules="[rules.required]">
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
                                            <v-btn :text="t('actions.cancel')" @click="show_calendar_picker=false; resetDate()"></v-btn>
                                            <v-btn :text="t('actions.confirm')" @click="show_calendar_picker=false; updateDate()"></v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </v-btn>
                        </v-col>
                        <v-col class="flex-grow-1">
                            <v-btn variant="text" prepend-icon="mdi-clock" width="95%">
                                {{ formatTime(time) }}
                                <v-dialog activator="parent" width="auto" v-model="show_time_picker">
                                    <v-card>
                                        <v-card-text style="padding: 0px;">
                                            <v-time-picker v-model="selecting_time" :format="time_picker_format" :max="maxTime"></v-time-picker>
                                        </v-card-text>
                                        <v-card-actions>
                                            <v-spacer></v-spacer>
                                            <v-btn :text="t('actions.cancel')" @click="show_time_picker=false; resetTime()  "></v-btn>
                                            <v-btn :text="t('actions.confirm')" @click="show_time_picker=false; updateTime()"></v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </v-btn>
                        </v-col>
                    </v-row>
                </v-card-text>
            </v-card>
            <WalletSelector :title="selected_tag_type == TagType.TRANSFER ? 'account.select_from' : undefined" v-model="selected_wallet" :wallets="wallets"></WalletSelector>
            <WalletSelector v-if="selected_tag_type == TagType.TRANSFER" :title="'account.select_to'" v-model="selected_to_wallet" :wallets="wallets"></WalletSelector>
            <TagSelector v-model="selected_tag" :tags="tags"></TagSelector>
            <v-card v-if="selected_tag_type == TagType.EXPENSE" :variant="has_split ? 'flat' : 'text'" density="compact" color="surface-lighten-1">
                <v-card-text style="padding: 0px;">
                    <v-row class="d-flex align-center">
                        <v-col class="flex-grow-0">
                            <v-checkbox class="text-body-2" v-model="has_split" hide-details density="compact" color="secondary" base-color="black"></v-checkbox>
                        </v-col>
                        <v-col style="margin-left: 5px; color: black">{{ t('transaction.has_split') }}</v-col>
                    </v-row>
                    <v-sheet v-if="has_split" style="margin: 0px 7px 0px 7px;" color="surface-lighten-1">
                        <v-row class="d-flex align-center">
                            <v-col>{{ t('transaction.split.count') }}</v-col>
                            <v-col>
                                <v-text-field v-model.number="split_count" variant="outlined" type="number" min="2" :rules="[rules.required, rules.min(2)]" density="compact" hide-details="auto"></v-text-field>
                            </v-col>
                        </v-row>
                        <v-row class="d-flex align-center">
                            <v-col>{{ t('transaction.split.your') }}</v-col>
                            <v-col>
                                <v-text-field v-model.number="split_expense" variant="outlined" type="number" min="0" :max="amount ?? 0" :rules="[rules.required, rules.min(0), rules.max(amount ?? 0)]" density="compact" hide-details="auto"></v-text-field>
                            </v-col>
                        </v-row>
                        <v-row class="d-flex align-center">
                            <v-col class="text-end">{{ t('transaction.split.other', { each: (Math.ceil(((amount ?? 0) - split_expense) / (split_count - 1) * 100) / 100).toFixed(2), total: ((amount ?? 0) - split_expense).toFixed(2) }) }}</v-col>
                        </v-row>
                    </v-sheet>
                    <WalletSelector v-if="has_split" :title="t('transaction.split.select_wallet')" v-model="split_receive_wallet" :wallets="wallets"></WalletSelector>
                </v-card-text>
            </v-card>
            <div style="height: 50px;"></div>
            <v-btn @click="confirm" color="primary" width="95%" style="position: fixed; bottom: 10px;" :disabled="!isFormValid()">{{ t('actions.save') }}</v-btn>
        </v-form>
    </v-main>
</template>