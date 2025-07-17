extern crate wu_clib;

use std::rc::Rc;
use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::*;

use easy_imgui_renderer::*;
use easy_imgui as imgui;
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
    let render = Renderer::new(gl.clone()).unwrap();
    let app = App {
        _gl: gl.clone(),
    };

    let data = Box::new(Data {
        render,
        app,
        last_time: 0.0,
    });

    // Read a JPEG and write it to the log, just for show.
    unsafe {
        static JPEG_DATA: &[u8] = include_bytes!("rose.jpg");
        use jpeg::bindings::*;
        use std::mem::MaybeUninit;

        let mut cinfo: jpeg_decompress_struct = MaybeUninit::zeroed().assume_init();
        let mut jerr: jpeg_error_mgr = MaybeUninit::zeroed().assume_init();
        cinfo.err = jpeg_std_error(&mut jerr);
        jpeg_CreateDecompress(&mut cinfo, JPEG_LIB_VERSION as i32, std::mem::size_of::<jpeg_decompress_struct>());
        jpeg_mem_src(&mut cinfo, JPEG_DATA.as_ptr(), JPEG_DATA.len());
        jpeg_read_header(&mut cinfo, 1);
        jpeg_start_decompress(&mut cinfo);
        log::info!("{cinfo:?}");
        let mut line = vec![0; cinfo.output_width as usize * cinfo.output_components as usize];
        let mut lines = vec![line.as_mut_ptr(); 1];
        while cinfo.output_scanline < cinfo.output_height {
            let _num_scanlines = jpeg_read_scanlines(&mut cinfo, lines.as_mut_ptr(), 1);
            let mut s = String::new();
            for px in line.chunks_exact(cinfo.output_components as usize) {
                let c = px.iter().map(|x| u32::from(*x)).sum::<u32>() / px.len() as u32;
                let c = if c < 64 {
                    ' '
                } else if c < 128 {
                    '+'
                } else if c < 192 {
                    'o'
                } else {
                    'O'
                };
                s.push(c);
                s.push(c);
            }
            log::info!("{s}");
        }
        jpeg_finish_decompress(&mut cinfo);
        jpeg_destroy_decompress(&mut cinfo);
    }
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
pub unsafe fn do_key(_data: *mut Data, key: &str, press: bool) {
    let io = &mut *ImGui_GetIO();
    let key = match key {
        "ArrowLeft" => imgui::Key::LeftArrow,
        "ArrowRight" => imgui::Key::RightArrow,
        "ArrowDown" => imgui::Key::DownArrow,
        "ArrowUp" => imgui::Key::UpArrow,
        "Enter" => imgui::Key::Enter,
        "Backspace" => imgui::Key::Backspace,
        //TODO: add all the other keys
        _ => return,
    };
    io.AddKeyEvent(key.bits(), press);
}

struct App {
    _gl: glr::GlContext,
}

impl imgui::UiBuilder for App {
    fn do_ui(&mut self, ui: &imgui::Ui<Self>) {
        //ui.dock_space_over_viewport(imgui::DockNodeFlags::None);
        ui.show_demo_window(None);
    }
}


