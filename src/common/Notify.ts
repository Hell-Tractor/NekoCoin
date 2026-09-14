import { reactive } from 'vue';

export const notice = reactive({
    visible: false,
    text: '',
    color: 'error' as 'error' | 'success',
});

export const format_error = function(error: unknown): string {
    if (typeof error === 'string') {
        return error;
    }
    if (error instanceof Error && error.message) {
        return error.message;
    }
    if (error && typeof error === 'object' && 'message' in error) {
        const message = (error as { message: unknown }).message;
        if (message) {
            return String(message);
        }
    }
    try {
        return JSON.stringify(error);
    } catch {
        return String(error);
    }
};

export const show_error = function(error: unknown) {
    console.error(error);
    notice.text = format_error(error);
    notice.color = 'error';
    notice.visible = true;
};

export const show_success = function(text: string) {
    notice.text = text;
    notice.color = 'success';
    notice.visible = true;
};
