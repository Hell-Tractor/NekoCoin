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

export const formatDatetime = function(naive_date: Date) : string {
    // format: %Y-%m-%d %H:%M:%S
    const year = naive_date.getFullYear();
    const month = (naive_date.getMonth() + 1).toString().padStart(2, '0');
    const date = naive_date.getDate().toString().padStart(2, '0');
    const hour = naive_date.getHours().toString().padStart(2, '0');
    const minute = naive_date.getMinutes().toString().padStart(2, '0');
    const second = naive_date.getSeconds().toString().padStart(2, '0');
    return `${year}-${month}-${date} ${hour}:${minute}:${second}`;
}

export const formatDate = function(naive_date: Date) : string {
    // format: %Y-%m-%d
    const year = naive_date.getFullYear();
    const month = (naive_date.getMonth() + 1).toString().padStart(2, '0');
    const date = naive_date.getDate().toString().padStart(2, '0');
    return `${year}-${month}-${date}`;
}