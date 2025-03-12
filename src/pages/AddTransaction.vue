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
const { t } = useI18n();

const emits = defineEmits<{
    back: []
}>();

const form: Ref<boolean> = ref(false);
const remark: Ref<string> = ref('');
const amount: Ref<number | null> = ref(null);
const time: Ref<Date> = ref(new Date());
const selected_tag_type: Ref<string> = ref(TagTypeToString(TagTypeNames[0].type));
const show_time_picker: Ref<boolean> = ref(false);
const show_calendar_picker: Ref<boolean> = ref(false);
const selecting_date: Ref<Date> = ref(new Date());
const selecting_time: Ref<string> = ref(`${new Date().getHours()}:${new Date().getMinutes()}`);
const wallets: Ref<Wallet[]> = ref([]);
const selected_wallet: Ref<number | null> = ref(null); // index in array
const selected_tag: Ref<number | null> = ref(null); // index in array
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
        tags.value = await invoke('retrieve_tags', { filter: '', kind: selected_tag_type.value });
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
const offsetColor = function(color: string, offset: number, alpha: number) {
    // format: rgb(r,g,b)
    var [r, g, b] = color.substring(4, color.length - 1).split(',');
    return `rgba(${Math.min(255, parseInt(r) + offset)}, ${Math.min(255, parseInt(g) + offset)}, ${Math.min(255, parseInt(b) + offset)}, ${alpha})`;
}
const addTransaction = async function() {
    try {
        let params = {
            remark: remark.value,
            walletId: wallets.value[selected_wallet.value!].id,
            tagId: tags.value[selected_tag.value!].id,
            amount: amount.value!,
            time: formatDatetime(time.value)
        };
        await invoke('create_transaction', params);
        emits('back');
    } catch (error) {
        // TODO: handle error
        console.error(error);
    }
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
            <v-chip-group mandatory v-model="selected_tag_type" :rules="[rules.required]" @update:model-value="selected_tag = null; retrieve_tags()">
                <v-chip v-for="tag in TagTypeNames" :value="TagTypeToString(tag.type)" :key="tag.type" variant="flat" color="secondary" :disabled="tag.type == TagType.TRANSFER">{{ t(`tag.type.${tag.name}`) }}</v-chip>
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
            <v-card variant="text" density="compact">
                <v-card-text style="padding-bottom: 0;">
                    <v-row class="flex-nowarp">
                        <v-col style="padding-left: 3px;">
                            <span>{{ t('transaction.accounts') }}</span>
                        </v-col>
                        <v-col class="d-flex justify-end">
                            <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="page = 'add_wallet'"></v-btn>
                        </v-col>
                    </v-row>
                    <v-slide-group class="pa-4" style="margin-left: -20px;" mandatory v-model="selected_wallet">
                        <v-slide-group-item v-for="wallet in wallets" :key="wallet.id" v-slot="{ isSelected, toggle }">
                            <v-card @click="toggle" :border="isSelected ? 'opacity-100 primary md' : ''" width="100" height="100" class="ma-1">
                                <v-card-text style="padding: 10px;">
                                    <v-icon :color="wallet.color">{{ wallet.icon }}</v-icon>
                                    <div>{{ (wallet.remark?.length ?? 0) > 5 ? (wallet.remark!.substring(0, 4) + '...') : (wallet.remark?.substring(0, 5) || '') }}</div>
                                    <div style="position: absolute; bottom: 10px;" class="font-weight-black">{{ wallet.name }}</div>
                                </v-card-text>
                            </v-card>
                        </v-slide-group-item>
                    </v-slide-group>
                </v-card-text>
            </v-card>
            <v-card variant="text" density="compact">
                <v-card-text>
                    <v-row class="flex-nowarp">
                        <v-col style="padding-left: 3px;">
                            <span>{{ t('transaction.tags') }}</span>
                        </v-col>
                        <v-col class="d-flex justify-end">
                            <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="page = 'add_tag'"></v-btn>
                        </v-col>
                    </v-row>
                    <v-chip-group mandatory column v-model="selected_tag">
                        <v-chip v-for="tag in tags" :key="tag.id" :color="tag.color" :style="(!selected_tag || tags[selected_tag].id != tag.id) ? { backgroundColor: offsetColor(tag.color, 50, 0.7) } : { borderWidth: '1px', borderColor: offsetColor(tag.color, -50, 1) }" label :prepend-icon="tag.icon">{{ tag.name }}</v-chip>
                    </v-chip-group>
                </v-card-text>
            </v-card>
            <v-btn @click="addTransaction" color="primary" width="93%" style="position: fixed; bottom: 10px;" :disabled="!form || selected_wallet == null || selected_tag == null">{{ t('save') }}</v-btn>
        </v-form>
    </div>
    <AddWallet v-else-if="page == 'add_wallet'" @back="backFromAddWallet"></AddWallet>
    <AddTag v-else-if="page == 'add_tag'" @back="backFromAddTag"></AddTag>
</template>