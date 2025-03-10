export const getRandomColor = function(type: 'rgb' | 'rgba') : string {
    const r = Math.floor(Math.random() * 256);
    const g = Math.floor(Math.random() * 256);
    const b = Math.floor(Math.random() * 256);
    if (type === 'rgb') {
        return `rgb(${r},${g},${b})`;
    } else {
        const a = Math.random().toFixed(1);
        return `rgba(${r},${g},${b},${a})`;
    }
}