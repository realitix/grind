extern crate egl;
extern crate image;
extern crate opengles2 as gl;
extern crate wayland_client;
extern crate grindtest;


use std::ptr;
use std::{thread, time};
use wayland_client::egl::WlEglSurface;
use wayland_client::protocol::wl_compositor::RequestsTrait as CompositorRequests;
use wayland_client::protocol::wl_display::RequestsTrait as DisplayRequests;
use wayland_client::protocol::wl_shell::RequestsTrait as ShellRequests;
use wayland_client::protocol::wl_shell_surface::RequestsTrait as ShellSurfaceRequests;
use wayland_client::protocol::{wl_compositor, wl_shell, wl_shell_surface, wl_shm};
use wayland_client::{Display, GlobalManager, Proxy};

use grindtest::utils;

fn get_triangle_vbo(program: &utils::ShaderProgram) -> gl::Uint {
    let vbos = gl::gen_buffers(1);
    let vbo = vbos[0];

    let vertex_data: Vec<f32> = vec![
        0.75, 0.75, 0.0,
        -0.75, -0.75, 0.0,
        0.75, -0.75, 0.0
    ];

    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl::ARRAY_BUFFER, 4*vertex_data.len() as i64, vertex_data.as_slice().as_ptr() as *const gl::Void, gl::STATIC_DRAW);
    gl::enable_vertex_attrib_array(program.get_attrib_location(&"vin_position") as u32);
    gl::vertex_attrib_pointer(program.get_attrib_location(&"vin_position") as u32, 3, gl::FLOAT, gl::FALSE as u8, 0, ptr::null());

    gl::bind_buffer(gl::ARRAY_BUFFER, 0);
    vbo
}

fn with_gles2<F>(f:F)
    where
        F: FnOnce(i32, i32)
{
    let width = 300;
    let height = 300;

    // Create Wayland window
    let (wl_display, mut event_queue) = Display::connect_to_env().unwrap();
    let globals = GlobalManager::new(wl_display.get_registry().unwrap());
    event_queue.sync_roundtrip().unwrap();

    let compositor = globals
        .instantiate_auto::<wl_compositor::WlCompositor>()
        .unwrap()
        .implement(|_, _| {});
    let _shm = globals
        .instantiate_auto::<wl_shm::WlShm>()
        .unwrap()
        .implement(|_, _| {});
    let shell = globals
        .instantiate_auto::<wl_shell::WlShell>()
        .unwrap()
        .implement(|_, _| {});

    let surface = compositor.create_surface().unwrap().implement(|_, _| {});
    let shell_surface = shell.get_shell_surface(&surface).unwrap().implement(
        |event, shell_surface: Proxy<wl_shell_surface::WlShellSurface>| {
            use wayland_client::protocol::wl_shell_surface::{Event, RequestsTrait};
            // This ping/pong mechanism is used by the wayland server to detect
            // unresponsive applications
            if let Event::Ping { serial } = event {
                shell_surface.pong(serial);
            }
        },
    );
    shell_surface.set_toplevel();

    let egl_surface = WlEglSurface::new(&surface, width, height);

    // Create EGL context
    let egl_display = egl::get_display(wl_display.c_ptr() as _);
    let (_major, _minor) = egl::initialize(egl_display);
    let mut configs = egl::choose_config(
        egl_display,
        vec![
            egl::SURFACE_TYPE,
            egl::WINDOW_BIT,
            egl::RENDERABLE_TYPE,
            egl::OPENGL_ES2_BIT,
            egl::RED_SIZE,
            8,
            egl::GREEN_SIZE,
            8,
            egl::BLUE_SIZE,
            8,
            egl::NONE,
        ],
    );    

    let config = configs.remove(0);
    let surface =
        egl::create_window_surface(egl_display, config, egl_surface.ptr() as _, vec![egl::NONE]);
    let context = egl::create_context(
        egl_display,
        config,
        egl::NO_CONTEXT,
        vec![egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE, egl::NONE],
    );

    egl::make_current(egl_display, surface, surface, context);
    
    f(width, height);

    egl::swap_buffers(egl_display, surface);
    egl::make_current(
        egl_display,
        egl::NO_SURFACE,
        egl::NO_SURFACE,
        egl::NO_CONTEXT,
    );
}


#[test]
fn basic_clear() {
    with_gles2(|width, height| {
        let program = utils::ShaderProgram::new(include_str!("../test.vert"), include_str!("../test.frag"));
        let vbo = get_triangle_vbo(&program);

        // Rendering    
        gl::clear_color(1., 1., 0., 1.);
        gl::clear(gl::COLOR_BUFFER_BIT);
        gl::use_program(program.program);
        gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
        gl::draw_arrays(gl::TRIANGLES, 0, 3);

        thread::sleep(time::Duration::from_secs(10));

        // Get image buffer
        utils::assert_gl(width, height, include_bytes!("opengles2/output/test_image.png"));
    });
}
