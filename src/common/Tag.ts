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

export interface TagNode extends Tag {
    children: TagNode[];
}

export const buildTagForest = (tags: Tag[]): TagNode[] => {
    const nodes = new Map<number, TagNode>();
    for (const tag of tags) {
        nodes.set(tag.id, { ...tag, children: [] });
    }
    const roots: TagNode[] = [];
    for (const node of nodes.values()) {
        const parent = node.parent_id == null ? undefined : nodes.get(node.parent_id);
        if (parent) {
            parent.children.push(node);
        } else {
            roots.push(node);
        }
    }
    return roots;
};

export const tagPath = (forest: TagNode[], id: number): TagNode[] => {
    const walk = (nodes: TagNode[], acc: TagNode[]): TagNode[] | null => {
        for (const node of nodes) {
            const next = [...acc, node];
            if (node.id === id) {
                return next;
            }
            const found = walk(node.children, next);
            if (found) {
                return found;
            }
        }
        return null;
    };
    return walk(forest, []) ?? [];
};

export const findTagNode = (forest: TagNode[], id: number): TagNode | undefined => {
    const path = tagPath(forest, id);
    return path[path.length - 1];
};

export const toTag = (node: TagNode): Tag => {
    const { children: _children, ...tag } = node;
    return tag;
};