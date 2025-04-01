export default {
    app_name: 'NekoCoin',
    welcome: '欢迎回来！',
    total_balance: '总余额',
    this_month: '本月',
    income: '收入',
    expense: '支出',
    WIP: '开发中...',
    loading: '加载中...',
    actions: {
        save: '保存',
        confirm: '确认',
        cancel: '取消',
        edit: '编辑',
        delete: '删除',
        copy: '复制',
    },
    page: {
        home: '首页',
        transactions: '交易',
        reports: '报表',
        accounts: '账户',
        tags: '标签',
        settings: '设置',
    },
    validation: {
        required: '必填',
        isValidMoney: '请输入正确的金额。(至多两位小数)',
        maxLength: '最多{max}个字符',
        isValidSearchText: '只允许输入字母、数字和中文',
        min: '至少{min}',
        max: '至多{max}',
    },
    account: {
        update: '更新账户',
        add: '添加账户',
        select: '选择账户',
        select_from: '选择转出账户',
        select_to: '选择转入账户',
        enter: {
            name: '输入账户名',
            amount: '输入金额',
            remark: '输入备注',
        },
        no_account: '暂无账户',
        no_account_tip: '点击右下角按钮添加一个账户吧！',
    },
    tag: {
        add: '添加标签',
        update: '更新标签',
        select: '选择标签',
        no_available_parent: '没有可用的父标签',
        enter: {
            name: '输入标签名',
            remark: '输入备注',
            parent_tag: '选择父标签(可选)',
        },
        search: {
            hint: '搜索...',
            count: '找到{count}个标签',
        },
        type: {
            Income: '收入',
            Expense: '支出',
            Transfer: '转账',
        }
    },
    transaction: {
        add: '添加交易',
        update: '更新交易',
        datetime: '日期 & 时间',
        remark: '备注',
        account: '账户',
        has_split: '分账？',
        split: {
            title: '分账',
            count: '分账人数(包括你)',
            your: '你的金额',
            other: '其他人每人 {each}, 共 {total}',
            select_wallet: '选择收款账户',
            people: '{count} 人',
        },
        enter: {
            remark: '输入备注',
            amount: '输入金额',
        },
        list: {
            title: '近期交易',
        }
    },
    list: {
        summary: '共 {0} 条记录',
    },
    date: {
        today: '今天',
        yesterday: '昨天',
    },
    icon: {
        select: '选择图标',
        foodAndDrink: '饮食',
        shopping: '购物',
        transportation: '交通',
        bank: '银行',
        dailyLife: '日常',
        travel: '旅行',
        education: '教育',
        pets: '宠物',
        hobbies: '爱好',
        gifts: '礼物',
        health: '健康',
        sports: '运动',
    },
    warning: {
        irrevertible: {
            title: '警告',
            content: '该操作不可逆，确认继续？',
        },
        cascade_and_irrevertible: {
            title: '警告',
            content: '该操作不可逆且会删除所有相关数据，确认继续？',
        },
    }
}