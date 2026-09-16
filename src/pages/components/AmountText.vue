<script setup lang="ts">
import { computed } from 'vue';
import { Money } from '../../common/Money';
import { privacy_mode, settings } from '../../common/Settings';
import { display_amount } from '../../common/Utils';

const props = defineProps<{
    cents?: number;
    currency?: string;
    money?: Money;
    signed?: boolean;
}>();

const text = computed(() => {
    void privacy_mode.value;
    const cents = props.money?.getAmount() ?? props.cents ?? 0;
    const currency = props.money?.getCurrency().code ?? props.currency ?? settings.primary_currency_code;
    return display_amount(cents, currency, { signed: props.signed });
});
</script>

<template>
    <span>{{ text }}</span>
</template>
