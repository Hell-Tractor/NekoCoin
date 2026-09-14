import { reactive } from 'vue';
import Tag from './Tag';

export interface ActivityDraftState {
    active: boolean;
    name: string;
    remark: string;
    color: string;
    icon: string;
    tag_id?: number;
    tag?: Tag;
}

export const activity_draft = reactive<ActivityDraftState>({
    active: false,
    name: '',
    remark: '',
    color: '',
    icon: 'mdi-flag',
});

export const clear_activity_draft = function() {
    activity_draft.active = false;
    activity_draft.name = '';
    activity_draft.remark = '';
    activity_draft.color = '';
    activity_draft.icon = 'mdi-flag';
    activity_draft.tag_id = undefined;
    activity_draft.tag = undefined;
};
