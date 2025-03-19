export default {
    app_name: 'NekoCoin',
    welcome: '欢迎回来！',
    total_balance: '总余额',
    this_month: '本月',
    income: '收入',
    expense: '支出',
    WIP: '开发中...',
    select_icon: '选择图标',
    save: '保存',
    confirm: '确认',
    cancel: '取消',
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
    },
    account: {
        add: '添加账户',
        select: '选择账户',
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
            income: '收入',
            expense: '支出',
            transfer: '转账',
        }
    },
    transaction: {
        add: '添加交易',
        datetime: '日期 & 时间',
        enter: {
            remark: '输入备注',
            amount: '输入金额',
        },
        list: {
            title: '近期交易',
        }
    },
    list: {
        no_more_data: '没有更多数据',
    },
    date: {
        today: '今天',
        yesterday: '昨天',
    }
}