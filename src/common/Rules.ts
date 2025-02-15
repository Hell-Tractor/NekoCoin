export const rules = {
    required: (value: any) => !!value || '必填',
    isValidMoney: (value: any) => /^\d+(\.\d{1,2})?$/.test(value) || '请输入正确的金额',
}