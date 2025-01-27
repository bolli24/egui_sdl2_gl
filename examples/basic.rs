use egui::Checkbox;
use egui_backend::DpiScaling;
use std::time::Instant;
// Alias the backend to something less mouthful
use egui_sdl2_gl as egui_backend;
use sdl2::{
    event::Event,
    video::{GLProfile, SwapInterval},
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    // Set per-process DPI awareness
    // https://docs.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows
    #[cfg(windows)]
    unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    }

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Compatibility);
    // On linux, OpenGL ES Mesa driver 22.0.0+ can be used like so:
    // gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);

    let window = video
        .window(
            "Demo: Egui backend for SDL2 + GL",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    let gl: gl::Gl = gl::Gl::load_with(|s| window.subsystem().gl_get_proc_address(s) as *const _);

    // Init egui stuff
    let (mut painter, mut egui_state) = egui_backend::with_sdl2(&gl, &window, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut test_str: String =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let mut enable_vsync = false;
    let mut quit = false;
    let mut slider = 0.0;

    let start_time = Instant::now();

    'running: loop {
        if enable_vsync {
            window
                .subsystem()
                .gl_set_swap_interval(SwapInterval::VSync)
                .unwrap()
        } else {
            window
                .subsystem()
                .gl_set_swap_interval(SwapInterval::Immediate)
                .unwrap()
        }

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(" ");
            ui.add(egui::Slider::new(&mut slider, 0.0..=50.0).text("Slider"));
            ui.label(" ");
            ui.add(Checkbox::new(&mut enable_vsync, "Reduce CPU Usage?"));
            ui.separator();
            if ui.button("Quit?").clicked() {
                quit = true;
            }
        });

        let full_output = egui_ctx.end_frame();

        // Process ouput
        egui_state.process_output(&window, &full_output.platform_output);

        let primitives = egui_ctx.tessellate(full_output.shapes);

        // An example of how OpenGL can be used to draw custom stuff with egui
        // overlaying it:
        // First clear the background to something nice.
        unsafe {
            // Clear the screen to green
            gl.ClearColor(0.3, 0.6, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // TODO: correct replacement for "needs_repaint"?
        if !full_output.repaint_after.is_zero() {
            if let Some(event) = event_pump.wait_event_timeout(5) {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {
                        // Process input event
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }
        painter.paint_and_update_textures(primitives.as_slice(), &full_output.textures_delta);

        // TODO: draw something here using OpenGL directly (draws over the gui)

        window.gl_swap_window();

        if quit {
            break;
        }
    }
}
