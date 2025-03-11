export enum TagType {
    EXPENSE,
    INCOME,
    TRANSFER,
}

export const TagTypeToString = (type: TagType): string => {
    switch (type) {
        case TagType.EXPENSE:
            return 'Expense';
        case TagType.INCOME:
            return 'Income';
        case TagType.TRANSFER:
            return 'Transfer';
    }
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
    type: string;
    parentId: number | null;
};