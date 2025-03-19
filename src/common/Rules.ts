// import t from i18n
import i18n from "../i18n";
const t = i18n.global.t;

export const rules = {
    required: (value: any) => !!(`${value}`) || t('validation.required'),
    isValidMoney: (value: any) => /^\d+(\.\d{1,2})?$/.test(value) || t('validation.isValidMoney'),
    maxLength: (max: number) => (value: any) => (value || '').length <= max || t('validation.maxLength', { max }),
    isValidSearchText: (value: any) => /^[a-zA-Z0-9\u4e00-\u9fa5 ]+$/.test(value) || t('validation.isValidSearchText'),
}