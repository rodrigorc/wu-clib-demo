import wasm_init, * as w from "./pkg/wu_clib_demo.js";

async function onDocumentLoad() {
    await wasm_init();
    let demo = w.init_demo();
    var canvas = document.getElementById("canvas");
    canvas.width = document.documentElement.clientWidth;
    canvas.height = document.documentElement.clientHeight;
    
    addEventListener("resize", () => {
        canvas.width = document.documentElement.clientWidth;
        canvas.height = document.documentElement.clientHeight;
    });
    function onMouse(e, down) {
        e.preventDefault();
        w.do_mouse_move(demo, e.clientX, e.clientY);
        w.do_mouse_button(demo, e.button, down);
        return false;
    }
    function onMouseMove(e) {
        e.preventDefault();
        w.do_mouse_move(demo, e.clientX, e.clientY);
        return false;
    }
    function onMouseWheel(e) {
        e.preventDefault();
        // this scale is arbitrary
        w.do_mouse_wheel(demo, -e.deltaX / 10.0, -e.deltaY / 100.0);
        return false;
    }

    let touchId = null;
    function onTouch(e, down) {
        e.preventDefault();
        let t = null;
        for (let i = 0; i < e.changedTouches.length; ++i)
            if (touchId == null || touchId == e.changedTouches[i].identifier) {
                t = e.changedTouches[i];
                break;
            }
        if (t === null)
            return false;
        touchId = down ? t.identifier : null;
        w.do_mouse_move(demo, t.clientX, t.clientY);
        w.do_mouse_button(demo, 0, down);
        return false;
    }
    function onTouchMove(e) {
	e.preventDefault();
	let t = null;
	for (let i = 0; i < e.changedTouches.length; ++i)
	    if (touchId == null || touchId == e.changedTouches[i].identifier) {
		t = e.changedTouches[i];
		break;
	    }
	if (t === null)
	    return false;
	touchId = t.identifier;
        w.do_mouse_move(demo, t.clientX, t.clientY);
        return false;
    }
    function onTextInput(e) {
        w.do_text(demo, e.data);
        this.value = "";
    }
    function onKeyDown(e) {
        w.do_key(demo, e.key, true);
    }
    function onKeyUp(e) {
        w.do_key(demo, e.key, false);
    }

    canvas.addEventListener('contextmenu', (e) => { e.preventDefault(); return false; }, false);

    if ('ontouchstart' in canvas) {
        canvas.addEventListener('touchstart', (e) => onTouch(e, true), false);
        canvas.addEventListener('touchend', (e) => onTouch(e, false), false);
        canvas.addEventListener('touchmove', onTouchMove, false);
    } else {
        canvas.addEventListener('mousedown', (e) => onMouse(e, true), false);
        canvas.addEventListener('mouseup', (e) => onMouse(e, false), false);
        canvas.addEventListener('mousemove', onMouseMove, false);
        canvas.addEventListener('wheel', onMouseWheel, false);
    }

    window.addEventListener('keydown', onKeyDown)
    window.addEventListener('keyup', onKeyUp)

    const input = document.querySelector('input[type="text"]');
    input.focus();
    input.addEventListener("input", onTextInput);

    function do_frame(f) {
        w.do_frame(demo, f, canvas.width, canvas.height);
        requestAnimationFrame(do_frame);
    }

    do_frame(0);
}

addEventListener("DOMContentLoaded", onDocumentLoad);

