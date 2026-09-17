import { type ThemeDefinition } from "vuetify"
import calico from "./calico";
import midnight from "./midnight";
import neon from "./neon";
import pinkPad from "./pinkPad";

export const THEME_NAMES = ['pinkPad', 'midnight', 'calico', 'neon'] as const;

const themes: Record<string, ThemeDefinition> = {
    pinkPad,
    midnight,
    calico,
    neon,
};

const theme = {
    defaultTheme: 'pinkPad',
    themes,
};

export const is_theme_dark = function(name: string): boolean {
    return themes[name]?.dark === true;
};

export const get_theme_background = function(name: string): string {
    return themes[name]?.colors?.background ?? '#FFFFFF';
};

export default theme;
