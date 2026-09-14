const AXIS_LOCK_PX = 8;

export const create_chart_pan = function(get_element: () => HTMLElement | undefined) {
    let pointer_start_x = 0;
    let pointer_start_y = 0;
    let pointer_start_scroll_left = 0;
    let dragging = false;
    let axis: 'x' | 'y' | undefined;

    const on_pointer_down = function(event: PointerEvent) {
        const element = get_element();
        if (element === undefined) {
            return;
        }
        dragging = true;
        axis = undefined;
        pointer_start_x = event.clientX;
        pointer_start_y = event.clientY;
        pointer_start_scroll_left = element.scrollLeft;
    };

    const on_pointer_move = function(event: PointerEvent) {
        const element = get_element();
        if (!dragging || element === undefined) {
            return;
        }
        const dx = event.clientX - pointer_start_x;
        const dy = event.clientY - pointer_start_y;
        if (axis === undefined) {
            if (Math.abs(dx) < AXIS_LOCK_PX && Math.abs(dy) < AXIS_LOCK_PX) {
                return;
            }
            axis = Math.abs(dx) > Math.abs(dy) ? 'x' : 'y';
            if (axis === 'x') {
                element.setPointerCapture(event.pointerId);
            }
        }
        if (axis === 'x') {
            element.scrollLeft = pointer_start_scroll_left - dx;
        }
    };

    const on_pointer_up = function(event: PointerEvent) {
        const element = get_element();
        dragging = false;
        axis = undefined;
        if (element?.hasPointerCapture(event.pointerId)) {
            element.releasePointerCapture(event.pointerId);
        }
    };

    return { on_pointer_down, on_pointer_move, on_pointer_up };
};
