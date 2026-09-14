export const theme_color_palettes: Record<string, string[]> = {
    pinkPad: [
        '#C45B78', '#3F8A76', '#E07A55', '#D4A03A',
        '#5B7FCF', '#8B63B0', '#D36B7A', '#6FA85A',
        '#4D90A8', '#C47A4A', '#A65D8A', '#6B9B8C',
        '#7A6BC4', '#C45B5B', '#8A7B4A', '#5A7A9B',
    ],
    midnight: [
        '#9DC4FF', '#E8B86D', '#6FDC9E', '#FF8AA0',
        '#C9A0FF', '#6FD4D0', '#F0C56A', '#7AA8FF',
        '#E89B6A', '#95D6B8', '#D4A0FF', '#FF9A9A',
        '#8EC4C0', '#D4B06A', '#8D9EC4', '#E07AB0',
    ],
    calico: [
        '#2F6F62', '#D0704A', '#C19A4A', '#6D86B3',
        '#8B6F9E', '#71905C', '#C87555', '#4D8194',
        '#79A66F', '#B9603D', '#5D789F', '#9A7658',
        '#3F7F70', '#806B94', '#D3A24F', '#6D8A83',
    ],
    neon: [
        '#F5C04A', '#3FDBC4', '#74A7FF', '#C59BFF',
        '#FF7A7A', '#7BE08F', '#F28F3B', '#65E0D0',
        '#9BC1FF', '#FF8A8A', '#A2E88D', '#FFD166',
        '#A486E8', '#36BFAE', '#F4B942', '#FF6B6B',
    ],
};

export const get_theme_color_palette = function(theme: string): string[] {
    return theme_color_palettes[theme] ?? theme_color_palettes.pinkPad;
};

export const get_random_theme_color = function(theme: string): string {
    const palette = get_theme_color_palette(theme);
    return palette[Math.floor(Math.random() * palette.length)];
};
