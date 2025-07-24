extern crate wu_clib;

use std::rc::Rc;
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::*;

use easy_imgui_renderer::*;
use easy_imgui::{self as imgui, image::GenericImage, lbl_id, CustomRectIndex};
use easy_imgui_sys::*;
use easy_imgui_opengl::{self as glr};

pub struct Data {
    render: Renderer,
    app: App,
    last_time: f32,
}

#[wasm_bindgen]
pub unsafe fn init_demo() -> *mut Data {
    let _ = log::set_logger(&wu_clib::JsLog)
                .map(|()| log::set_max_level(log::LevelFilter::Debug));
    std::panic::set_hook(Box::new(|info| {
        log::error!("{info}");
    }));

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);
    let gl = Rc::new(gl);
    let mut render = Renderer::new(gl.clone()).unwrap();

    let rose_rect;

    // Read a JPEG and write it to the log, just for show.
    unsafe {
        static JPEG_DATA: &[u8] = include_bytes!("rose.jpg");
        use jpeg::bindings::*;
        use std::mem::MaybeUninit;
        use imgui::image::Rgba;

        let mut cinfo: jpeg_decompress_struct = MaybeUninit::zeroed().assume_init();
        let mut jerr: jpeg_error_mgr = MaybeUninit::zeroed().assume_init();
        cinfo.err = jpeg_std_error(&mut jerr);
        //jpeg_create_decompress is a macro
        jpeg_CreateDecompress(&mut cinfo, JPEG_LIB_VERSION as i32, std::mem::size_of::<jpeg_decompress_struct>());
        jpeg_mem_src(&mut cinfo, JPEG_DATA.as_ptr(), JPEG_DATA.len());
        jpeg_read_header(&mut cinfo, 1);
        jpeg_start_decompress(&mut cinfo);

        let size = [cinfo.image_width, cinfo.image_height];
        rose_rect = render.imgui().io_mut().font_atlas_mut().add_custom_rect(size,
            |img| {
                log::info!("{cinfo:?}");
                let mut line = vec![0; cinfo.output_width as usize * cinfo.output_components as usize];
                let mut lines = vec![line.as_mut_ptr(); 1];
                while cinfo.output_scanline < cinfo.output_height {
                    let y = cinfo.output_scanline;
                    let _num_scanlines = jpeg_read_scanlines(&mut cinfo, lines.as_mut_ptr(), 1);
                    for x in 0 .. cinfo.image_width {
                        let p = &line[3 * x as usize ..][..3];
                        img.put_pixel(x, y, Rgba([p[0], p[1], p[2], 0xff]));
                    }
                }
            }
        );
        log::info!("Rect: {rose_rect:?}");
        jpeg_finish_decompress(&mut cinfo);
        jpeg_destroy_decompress(&mut cinfo);
    }

    let app = App {
        _gl: gl.clone(),
        rose_rect,
    };
    let data = Box::new(Data {
        render,
        app,
        last_time: 0.0,
    });
    Box::into_raw(data)
}

#[wasm_bindgen]
pub unsafe fn do_frame(data: *mut Data, time: f32, w: i32, h: i32) {
    let data = &mut *data;
    data.render.set_size([w as f32, h as f32].into(), 1.0);
    let io = &mut *ImGui_GetIO();
    io.DeltaTime = (time - data.last_time) / 1000.0;
    if io.DeltaTime <= 0.0 {
        return;
    }
    data.last_time = time;
    data.render.do_frame(&mut data.app);
}

#[wasm_bindgen]
pub unsafe fn do_mouse_move(_data: *mut Data, x: i32, y: i32) {
    let io = &mut *ImGui_GetIO();
    ImGuiIO_AddMousePosEvent(io, x as f32, y as f32);
}
#[wasm_bindgen]
pub unsafe fn do_mouse_button(_data: *mut Data, btn: i32, down: bool) {
    let io = &mut *ImGui_GetIO();
    let btn = match btn {
        1 => 2,
        2 => 1,
        x => x,
    };
    ImGuiIO_AddMouseButtonEvent(io, btn, down);
}
#[wasm_bindgen]
pub unsafe fn do_mouse_wheel(_data: *mut Data, x: i32, y: i32) {
    let io = &mut *ImGui_GetIO();
    ImGuiIO_AddMouseWheelEvent(io, x as f32, y as f32);
}
#[wasm_bindgen]
pub unsafe fn do_text(_data: *mut Data, text: &str) {
    let io = &mut *ImGui_GetIO();
    for c in text.chars() {
        ImGuiIO_AddInputCharacter(io, c as u32);
    }
}

#[wasm_bindgen]
pub unsafe fn do_key(
    _data: *mut Data,
    key_string: &str,
    code_string: &str,
    press: bool,
    ctrl: bool,
    shift: bool,
    alt: bool,
    meta: bool,
    _repeat: bool,
) {
    let io = &mut *ImGui_GetIO();

    // Modifer Keys
    ImGuiIO_AddKeyEvent(io, imgui::Key::ModCtrl.bits(), ctrl);
    ImGuiIO_AddKeyEvent(io, imgui::Key::ModShift.bits(), shift);
    ImGuiIO_AddKeyEvent(io, imgui::Key::ModAlt.bits(), alt);
    ImGuiIO_AddKeyEvent(io, imgui::Key::ModSuper.bits(), meta);

    // Alphanumerics with `key_string`, can be different in non-english languages.
    // With `code_string`, we can catch them all.
    let key = match code_string {
        // Navigation Keys
        "ArrowLeft" => imgui::Key::LeftArrow,
        "ArrowRight" => imgui::Key::RightArrow,
        "ArrowUp" => imgui::Key::UpArrow,
        "ArrowDown" => imgui::Key::DownArrow,
        "Home" => imgui::Key::Home,
        "End" => imgui::Key::End,
        "PageUp" => imgui::Key::PageUp,
        "PageDown" => imgui::Key::PageDown,
        "ContextMenu" => imgui::Key::Menu,

        // Editing & Control Keys
        // TODO: "Print" => imgui::Key::PrintScreen,
        "Backspace" => imgui::Key::Backspace,
        "Enter" => imgui::Key::Enter,
        "Tab" => imgui::Key::Tab,
        "Escape" => imgui::Key::Escape,
        "Insert" => imgui::Key::Insert,
        "Delete" => imgui::Key::Delete,
        "CapsLock" => imgui::Key::CapsLock,
        "NumLock" => imgui::Key::NumLock,
        "ScrollLock" => imgui::Key::ScrollLock,
        "Pause" => imgui::Key::Pause,

        // Function keys
        // TODO: Fn (function key)
        "F1" => imgui::Key::F1,
        "F2" => imgui::Key::F2,
        "F3" => imgui::Key::F3,
        "F4" => imgui::Key::F4,
        "F5" => imgui::Key::F5,
        "F6" => imgui::Key::F6,
        "F7" => imgui::Key::F7,
        "F8" => imgui::Key::F8,
        "F9" => imgui::Key::F9,
        "F10" => imgui::Key::F10,
        "F11" => imgui::Key::F11,
        "F12" => imgui::Key::F12,

        // Alphanumeric and Panctuations Keys
        "Space" => imgui::Key::Space,
        "Digit0" => imgui::Key::Num0,
        "Digit1" => imgui::Key::Num1,
        "Digit2" => imgui::Key::Num2,
        "Digit3" => imgui::Key::Num3,
        "Digit4" => imgui::Key::Num4,
        "Digit5" => imgui::Key::Num5,
        "Digit6" => imgui::Key::Num6,
        "Digit7" => imgui::Key::Num7,
        "Digit8" => imgui::Key::Num8,
        "Digit9" => imgui::Key::Num9,
        "Quote" => imgui::Key::Apostrophe,
        "Comma" => imgui::Key::Comma,
        "Minus" => imgui::Key::Minus,
        "Period" => imgui::Key::Period,
        "Slash" => imgui::Key::Slash,
        "Semicolon" => imgui::Key::Semicolon,
        "Equal" => imgui::Key::Equal,
        "BracketLeft" => imgui::Key::LeftBracket,
        "BracketRight" => imgui::Key::RightBracket,
        "Backslash" => imgui::Key::Backslash,
        "Backquote" => imgui::Key::GraveAccent,
        "IntlBackslash" => imgui::Key::Oem102,
        "KeyA" => imgui::Key::A,
        "KeyB" => imgui::Key::B,
        "KeyC" => imgui::Key::C,
        "KeyD" => imgui::Key::D,
        "KeyE" => imgui::Key::E,
        "KeyF" => imgui::Key::F,
        "KeyG" => imgui::Key::G,
        "KeyH" => imgui::Key::H,
        "KeyI" => imgui::Key::I,
        "KeyJ" => imgui::Key::J,
        "KeyK" => imgui::Key::K,
        "KeyL" => imgui::Key::L,
        "KeyM" => imgui::Key::M,
        "KeyN" => imgui::Key::N,
        "KeyO" => imgui::Key::O,
        "KeyP" => imgui::Key::P,
        "KeyQ" => imgui::Key::Q,
        "KeyR" => imgui::Key::R,
        "KeyS" => imgui::Key::S,
        "KeyT" => imgui::Key::T,
        "KeyU" => imgui::Key::U,
        "KeyV" => imgui::Key::V,
        "KeyW" => imgui::Key::W,
        "KeyX" => imgui::Key::X,
        "KeyY" => imgui::Key::Y,
        "KeyZ" => imgui::Key::Z,
        "Numpad0" => imgui::Key::Keypad0,
        "Numpad1" => imgui::Key::Keypad1,
        "Numpad2" => imgui::Key::Keypad2,
        "Numpad3" => imgui::Key::Keypad3,
        "Numpad4" => imgui::Key::Keypad4,
        "Numpad5" => imgui::Key::Keypad5,
        "Numpad6" => imgui::Key::Keypad6,
        "Numpad7" => imgui::Key::Keypad7,
        "Numpad8" => imgui::Key::Keypad8,
        "Numpad9" => imgui::Key::Keypad9,
        "NumpadDecimal" => imgui::Key::KeypadDecimal,
        "NumpadDivide" => imgui::Key::KeypadDivide,
        "NumpadMultiply" => imgui::Key::KeypadMultiply,
        "NumpadSubtract" => imgui::Key::KeypadSubtract,
        "NumpadAdd" => imgui::Key::KeypadAdd,
        "NumpadEnter" => imgui::Key::KeypadEnter,
        "NumpadEqual" => imgui::Key::KeypadEqual,

        "CtrlLeft" | "CtrlRight" | "ShiftLeft" | "ShiftRight" | "AltLeft" | "AltRight" => return, // we already handled it
        code => {
            log::warn!("Key code `{code}` is not supported yet!");
            // web_sys::console::warn_1(&format!("Key code `{code}` is not supported yet!").into());
            // TODO: Add a link to create a github issue for key code support
            return
        },
    };

    ImGuiIO_AddKeyEvent(io, key.bits(), press);
}

struct App {
    _gl: glr::GlContext,
    rose_rect: CustomRectIndex,
}

impl imgui::UiBuilder for App {
    fn do_ui(&mut self, ui: &imgui::Ui<Self>) {
        //ui.dock_space_over_viewport(imgui::DockNodeFlags::None);
        ui.show_demo_window(None);

        ui.window_config(lbl_id("wu-clib-demo", "main"))
            .with(|| {
                ui.text("This is a demo for `wu-clib-rs`, a contraption to build");
                ui.text("wasm32 application with C/C++ library dependencies.");
                ui.set_cursor_pos_y(ui.get_cursor_pos_y() + 16.0);
                ui.text("The UI is built with `Dear ImGui`, a C++ library,");
                ui.text("using the `easy-imgui` bindings, and `glow` for the rendering.");
                ui.set_cursor_pos_y(ui.get_cursor_pos_y() + 16.0);
                ui.text("This image is decoded using IJG's libjpeg, a C library.");

                ui.image_with_custom_rect_config(self.rose_rect, 4.0)
                    .build();
            });
    }
}


