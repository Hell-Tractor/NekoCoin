<script lang="ts" setup>
import { ref, Ref } from 'vue';
import TransactionList from '../common/TransactionList.vue';

const page: Ref<string> = ref('main');
</script>

<template>
    <TransactionList v-if="page == 'main'"></TransactionList>
</template>