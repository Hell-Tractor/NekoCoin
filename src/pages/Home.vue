<script setup lang="ts">
import { ref } from 'vue';
import { Money } from '../common/Money';
import { useI18n } from 'vue-i18n';
const { t, d } = useI18n();

const totalBalance = ref(new Money(1234567.89, '￥'));
const today = new Date();
</script>

<template>
    <div>{{ d(today, 'short') }}</div>
    <div>{{ t('welcome') }}</div>
    <v-card>
        <v-card-text>
            <div>{{ t('total_balance') }}</div>
            <p class="text-h5 font-weight-black">{{ totalBalance }}</p>
        </v-card-text>
    </v-card>
    <v-card variant="text">
        <v-card-text>
            <div>{{ t('this_month') }}</div>
        </v-card-text>
    </v-card>
</template>