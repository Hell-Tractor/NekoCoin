import { reactive } from 'vue';
import Tag from './Tag';

export interface TransactionDraftState {
    active: boolean;
    remark: string;
    amount: number | null;
    time: string;
    selected_tag_type: 'Expense' | 'Income' | 'Transfer';
    wallet_id?: number;
    to_wallet_id?: number;
    tag_id?: number;
    tag?: Tag;
    activity_id?: number;
    has_split: boolean;
    split_count: number;
    split_expense: number;
    split_receive_wallet_id?: number;
}

export const transaction_draft = reactive<TransactionDraftState>({
    active: false,
    remark: '',
    amount: null,
    time: new Date().toISOString(),
    selected_tag_type: 'Expense',
    has_split: false,
    split_count: 2,
    split_expense: 0,
});

export const clear_transaction_draft = function() {
    transaction_draft.active = false;
    transaction_draft.remark = '';
    transaction_draft.amount = null;
    transaction_draft.time = new Date().toISOString();
    transaction_draft.selected_tag_type = 'Expense';
    transaction_draft.wallet_id = undefined;
    transaction_draft.to_wallet_id = undefined;
    transaction_draft.tag_id = undefined;
    transaction_draft.tag = undefined;
    transaction_draft.activity_id = undefined;
    transaction_draft.has_split = false;
    transaction_draft.split_count = 2;
    transaction_draft.split_expense = 0;
    transaction_draft.split_receive_wallet_id = undefined;
};
