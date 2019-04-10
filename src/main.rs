#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
mod learn_opengl;

use gl;
use glfw::{self, Action, Context, Key};

use std::ffi::CStr;

use learn_opengl::{
    camera::Camera,
    common::{processInput, process_events},
    model::Model,
    shader::Shader,
};

use cgmath::{perspective, vec3, Deg, Matrix4, Point3};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut firstMouse = true;
    let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
    let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut deltaTime: f32; // time between current frame and last frame
    let mut lastFrame: f32 = 0.0;

    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
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

    let (ourShader, ourModel) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let ourShader = Shader::new("resources/cg_ufpel.vs", "resources/cg_ufpel.fs");

        // load models
        // -----------
        let ourModel = Model::new("resources/objects/rock/rock.obj");

        (ourShader, ourModel)
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
        let currentFrame = glfw.get_time() as f32;
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // events
        // -----
        process_events(
            &events,
            &mut firstMouse,
            &mut lastX,
            &mut lastY,
            &mut camera,
        );

        // input
        // -----
        processInput(&mut window, deltaTime, &mut camera);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // don't forget to enable shader before setting uniforms
            ourShader.useProgram();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(
                Deg(camera.Zoom),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                100.0,
            );
            let view = camera.GetViewMatrix();
            ourShader.setMat4(c_str!("projection"), &projection);
            ourShader.setMat4(c_str!("view"), &view);

            // draw in wireframe
            if window.get_key(Key::T) == Action::Press {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            was_moving = is_moving;
            is_moving = false;

            let angle = cgmath::dot(vec3(x_pos, 1.0, 1.0), vec3(1.0, 1.0, z_pos));
            rotate += (10.0 * deltaTime) % 360.0;
            let rot = Matrix4::<f32>::from_angle_y(cgmath::Deg(rotate));

            // render the loaded model 1
            if window.get_key(Key::L) == Action::Press {
                x_pos += 0.4 * deltaTime;
                is_moving = true;
            } else if window.get_key(Key::H) == Action::Press {
                x_pos -= 0.4 * deltaTime;
                is_moving = true;
            }
            let mut model1 = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
            model1 = model1 * Matrix4::<f32>::from_translation(vec3(x_pos, 0.0, 0.0));
            model1 = model1 * Matrix4::from_translation(vec3(0.0, 0.0, -1.0));
            model1 = model1 * Matrix4::from_scale(0.2);
            model1 = model1 * rot;

            // render the loaded model 2
            if window.get_key(Key::O) == Action::Press {
                z_pos += 0.4 * deltaTime;
                is_moving = true;
            } else if window.get_key(Key::Y) == Action::Press {
                z_pos -= 0.4 * deltaTime;
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

            ourShader.setMat4(c_str!("model"), &model1);
            ourModel.Draw(&ourShader);
            ourShader.setMat4(c_str!("model"), &model2);
            ourModel.Draw(&ourShader);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved
        // etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
