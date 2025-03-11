export enum TagType {
    EXPENSE,
    INCOME,
    TRANSFER,
}

export const TagTypeNames = [
    { type: TagType.EXPENSE, name: 'expense' },
    { type: TagType.INCOME, name: 'income' },
    { type: TagType.TRANSFER, name: 'transfer' },
]

export default interface Tag {
    id: number;
    name: string;
    remark?: string;
    color: string;
    icon: string;
    type: TagType;
    parentId: number | null;
};