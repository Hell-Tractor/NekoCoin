export enum TagType {
    EXPENSE,
    INCOME,
    TRANSFER,
}

export const TagTypeNames = {
    [TagType.EXPENSE]: 'expense',
    [TagType.INCOME]: 'income',
    [TagType.TRANSFER]: 'transfer',
};

export default interface Tag {
    id: number;
    name: string;
    remark?: string;
    color: string;
    icon: string;
    type: TagType;
    parentId: number;
};