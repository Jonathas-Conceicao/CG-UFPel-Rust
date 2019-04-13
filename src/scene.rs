use crate::{
    camera::Camera,
    common::{process_events, process_input},
    model::Model,
    shader::Shader,
};

use gl;
use glfw::{self, Action, Context, Key};

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn run() -> Result<(), failure::Error> {
    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (our_shader, our_model) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let our_shader = Shader::new("resources/cg_ufpel.vs", "resources/cg_ufpel.fs");

        // load models
        // -----------
        let our_model = Model::new("resources/objects/rock/rock.obj");

        (our_shader, our_model)
    };

    let mut x_pos = 0.0;
    let mut z_pos = 0.0;
    let mut rotate = 0.0;
    let mut is_moving = false;
    let mut was_moving;

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // events
        // -----
        process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut camera,
        );

        // input
        // -----
        process_input(&mut window, delta_time, &mut camera);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // don't forget to enable shader before setting uniforms
            our_shader.use_program();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(
                Deg(camera.zoom),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                100.0,
            );
            let view = camera.get_view_matrix();
            our_shader.set_mat4(c_str!("projection"), &projection);
            our_shader.set_mat4(c_str!("view"), &view);

            // draw in wireframe
            if window.get_key(Key::T) == Action::Press {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            was_moving = is_moving;
            is_moving = false;

            let angle = cgmath::dot(vec3(x_pos, 1.0, 1.0), vec3(1.0, 1.0, z_pos));
            rotate += (10.0 * delta_time) % 360.0;
            let rot = Matrix4::<f32>::from_angle_y(cgmath::Deg(rotate));

            // render the loaded model 1
            if window.get_key(Key::L) == Action::Press {
                x_pos += 0.4 * delta_time;
                is_moving = true;
            } else if window.get_key(Key::H) == Action::Press {
                x_pos -= 0.4 * delta_time;
                is_moving = true;
            }
            let mut model1 = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model1 = model1 * Matrix4::<f32>::from_translation(vec3(x_pos, 0.0, 0.0));
            model1 = model1 * Matrix4::from_translation(vec3(0.0, 0.0, -1.0));
            model1 = model1 * Matrix4::from_scale(0.2);
            model1 = model1 * rot;

            // render the loaded model 2
            if window.get_key(Key::O) == Action::Press {
                z_pos += 0.4 * delta_time;
                is_moving = true;
            } else if window.get_key(Key::Y) == Action::Press {
                z_pos -= 0.4 * delta_time;
                is_moving = true;
            }
            let mut model2 = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model2 = model2 * Matrix4::from_translation(vec3(0.0, 0.0, z_pos));
            model2 = model2 * Matrix4::from_translation(vec3(0.0, 0.0, -1.0));
            model2 = model2 * Matrix4::from_scale(0.2);
            model2 = model2 * rot;

            if was_moving && !is_moving {
                println!("{}", angle);
            }

            our_shader.set_mat4(c_str!("model"), &model1);
            our_model.draw(&our_shader);
            our_shader.set_mat4(c_str!("model"), &model2);
            our_model.draw(&our_shader);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved
        // etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}
