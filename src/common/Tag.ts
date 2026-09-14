export enum TagType {
    EXPENSE,
    INCOME,
    TRANSFER,
    ACTIVITY,
}

export const TagTypeToString = (type: TagType): string => {
    for (let i = 0; i < TagTypeNames.length; i++) {
        if (TagTypeNames[i].type === type) {
            return TagTypeNames[i].name;
        }
    }
    throw new Error(`Unknown tag type: ${type}`);
}

export const TagTypeFromString= (type: String): TagType => {
    for (let i = 0; i < TagTypeNames.length; i++) {
        if (TagTypeNames[i].name === type) {
            return TagTypeNames[i].type;
        }
    }
    throw new Error(`Unknown tag type: ${type}`);
}

export const TagTypeNames = [
    { type: TagType.EXPENSE, name: 'Expense' },
    { type: TagType.INCOME, name: 'Income' },
    { type: TagType.TRANSFER, name: 'Transfer' },
    { type: TagType.ACTIVITY, name: 'Activity' },
]

export const TransactionTagTypeNames = TagTypeNames.filter(item => item.type !== TagType.ACTIVITY);

export default interface Tag {
    id: number;
    name: string;
    remark?: string;
    color: string;
    icon: string;
    type: string;
    parent_id: number | null;
};