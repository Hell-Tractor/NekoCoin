<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Activity } from '../../common/Activity';
import { useRouter } from 'vue-router';
const { t } = useI18n();
const router = useRouter();

const props = defineProps<{
    activities: Activity[],
}>();
const selected_activity = defineModel<Activity | undefined>();

const clamp = function(value: number, min: number, max: number) {
    return Math.max(min, Math.min(max, value));
}

const offsetColor = function(color: string, offset: number, alpha: number) {
    var [r, g, b] = color.substring(4, color.length - 1).split(',');
    return `rgba(${clamp(parseInt(r) + offset, 0, 255)}, ${clamp(parseInt(g) + offset, 0, 255)}, ${clamp(parseInt(b) + offset, 0, 255)}, ${alpha})`;
}
</script>

<template>
    <v-card variant="text" density="compact">
        <v-card-text>
            <v-row class="flex-nowarp">
                <v-col style="padding-left: 3px;">
                    <span>{{ t('activity.select') }}</span>
                </v-col>
                <v-col class="d-flex justify-end">
                    <v-btn icon="mdi-plus" size="medium" density="compact" variant="text" @click="router.push({ path: '/activity/add' })"></v-btn>
                </v-col>
            </v-row>
            <div v-if="props.activities.length === 0" class="on-surface-lighten-1 text-body-2 mt-2 mb-2">{{ t('activity.empty_on_select') }}</div>
            <v-chip-group v-else column v-model="selected_activity">
                <v-chip
                    v-for="activity in props.activities"
                    :key="activity.id"
                    :value="activity"
                    :color="activity.color"
                    :style="(!selected_activity || selected_activity.id != activity.id) ? { backgroundColor: offsetColor(activity.color, 50, 0.7) } : { borderWidth: '1px', borderColor: offsetColor(activity.color, -50, 1) }"
                    label
                    :prepend-icon="activity.icon"
                >{{ activity.name }}</v-chip>
            </v-chip-group>
        </v-card-text>
    </v-card>
</template>
