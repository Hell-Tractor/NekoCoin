export const theme_color_palettes: Record<string, string[]> = {
    pinkPad: [
        '#C45B78', '#E58A9F', '#D9825B', '#E7B84B',
        '#4F8A78', '#6FA8A0', '#6B7DB3', '#8C72A8',
        '#A65D8A', '#D36B6B', '#C58B57', '#879B5B',
        '#4D8194', '#6574A8', '#8367A8', '#9B6B83',
    ],
    midnight: [
        '#8FB8FF', '#6F98E0', '#E8B86D', '#C9964D',
        '#72D6A0', '#5DB6B0', '#B58AE0', '#E88484',
        '#A9C7FF', '#D9A85B', '#95C9B1', '#6FA9A5',
        '#8D82C4', '#C2768A', '#71869F', '#A6A56B',
    ],
    calico: [
        '#2F6F62', '#5B9688', '#D9825B', '#B9603D',
        '#C19A4A', '#6D86B3', '#8B6F9E', '#71905C',
        '#3F7F70', '#79A66F', '#D3A24F', '#C87555',
        '#5D789F', '#806B94', '#9A7658', '#6D8A83',
    ],
    neon: [
        '#F4B942', '#D99A2B', '#47D7C0', '#20A895',
        '#74A7FF', '#A486E8', '#FF6B6B', '#7BD88F',
        '#FFD166', '#F28F3B', '#36BFAE', '#65E0D0',
        '#9BC1FF', '#C59BFF', '#FF8A8A', '#A2E88D',
    ],
};

export const get_theme_color_palette = function(theme: string): string[] {
    return theme_color_palettes[theme] ?? theme_color_palettes.pinkPad;
};

export const get_random_theme_color = function(theme: string): string {
    const palette = get_theme_color_palette(theme);
    return palette[Math.floor(Math.random() * palette.length)];
};
