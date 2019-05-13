use crate::{camera::Camera, model::Model, model_pos::ModelPosition, shader::Shader};

use gl;
use glfw::{self, Context};

use cgmath::{perspective, vec3, Deg, Matrix4};
use failure::ensure;

use std::{path::Path, sync::mpsc::Receiver};

pub struct Scene {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,

    camera: Camera,
    wscreen: u32,
    hscreen: u32,

    shader: Shader,
    model: Model,
    models: Vec<ModelPosition>,

    axis_m: Model,
    axis_p: ModelPosition,
}

pub trait SceneObject {
    fn process_input(&mut self, window: &glfw::Window, delta_time: f32);
}

impl Scene {
    pub fn init<P>(
        wscreen: u32,
        hscreen: u32,
        n_models: usize,
        models_config: P,
    ) -> Result<Self, failure::Error>
    where
        P: AsRef<Path>,
    {
        ensure!(
            n_models > 0 && n_models < 10,
            "Number of models should be bigger than 0 and lower than 10"
        );

        let mut camera = Camera::default();
        camera.model_pos.translation = vec3(0., 1., 20.);

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
            .create_window(wscreen, hscreen, "LearnOpenGL", glfw::WindowMode::Windowed)
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

        let (shader, model) = unsafe {
            // configure global opengl state
            // -----------------------------
            gl::Enable(gl::DEPTH_TEST);

            // build and compile shaders
            // -------------------------
            let our_shader = Shader::new("resources/cg_ufpel.vs", "resources/cg_ufpel.fs");

            // load models
            // -----------
            let our_model = Model::new("resources/objects/axis_arrows/axis_arrows.obj");

            (our_shader, our_model)
        };

        let axis_m = Model::new("resources/objects/axis_arrows/axis_arrows.obj");
        let axis_p = ModelPosition::default();

        let mut x_offset = 0.;
        let mut models: Vec<_> = std::iter::repeat(ModelPosition::with_config(models_config)?)
            .take(n_models)
            .map(|mut m| {
                m.translation.x = x_offset;
                x_offset += 2.;
                m
            })
            .collect();
        models[0].is_selected = true;

        Ok(Scene {
            glfw,
            window,
            events,

            camera,
            wscreen,
            hscreen,

            shader,
            model,
            models,

            axis_m,
            axis_p,
        })
    }

    pub fn run(&mut self) -> Result<(), failure::Error> {
        // Camera data
        let mut first_mouse = true;
        let mut last_x: f32 = self.wscreen as f32 / 2.;
        let mut last_y: f32 = self.hscreen as f32 / 2.;

        // timing
        let mut delta_time: f32; // time between current frame and last frame
        let mut last_frame: f32 = 0.;

        // don't forget to enable shader before setting uniforms
        unsafe { self.shader.use_program() };

        // render loop
        // -----------
        while !self.window.should_close() {
            // per-frame time logic
            // --------------------
            let current_frame = self.glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            self.process_events(&mut first_mouse, &mut last_x, &mut last_y);

            self.process_input(delta_time);

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // view/projection transformations
                let projection: Matrix4<f32> = perspective(
                    Deg(self.camera.zoom),
                    self.wscreen as f32 / self.hscreen as f32,
                    0.1,
                    100.,
                );
                let view = self.camera.get_view_matrix();
                self.shader.set_mat4(c_str!("projection"), &projection);
                self.shader.set_mat4(c_str!("view"), &view);

                self.models.iter().for_each(|m| {
                    self.shader.set_mat4(c_str!("model"), &m.matrix());
                    self.model.draw(&self.shader);
                });
                self.shader.set_mat4(c_str!("model"), &self.axis_p.matrix());
                self.axis_m.draw(&self.shader);
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved
            // etc.)
            // -------------------------------------------------------------------------------
            self.window.swap_buffers();
            self.glfw.poll_events();
        }

        Ok(())
    }

    fn process_input(&mut self, delta_time: f32) {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) };

        process_keys!(
            self.window;
            glfw::Key::Escape, glfw::Action::Press => self.window.set_should_close(true),
            glfw::Key::T, glfw::Action::Press => {
                // draw in wireframe
                unsafe{gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)};
            },
            glfw::Key::Num1, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 0 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num2, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 1 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num3, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 2 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num4, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 3 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num5, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 4 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num6, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 5 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num7, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 6 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num8, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 7 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            },
            glfw::Key::Num9, glfw::Action::Press => {
                self.models.iter_mut().enumerate().for_each(|(i, m)| {
                    if i == 8 {
                        m.is_selected = true;
                    } else {
                        m.is_selected = false;
                    }
                });
            }
        );

        let window = &self.window;
        self.models
            .iter_mut()
            .for_each(|model| model.process_input(window, delta_time));

        self.camera.process_input(&self.window, delta_time);
    }

    fn process_events(&mut self, first_mouse: &mut bool, last_x: &mut f32, last_y: &mut f32) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if *first_mouse {
                        *last_x = xpos;
                        *last_y = ypos;
                        *first_mouse = false;
                    }

                    let xoffset = xpos - *last_x;
                    let yoffset = *last_y - ypos;

                    *last_x = xpos;
                    *last_y = ypos;

                    self.camera.process_mouse_movement(xoffset, yoffset, true);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    self.camera.process_mouse_scroll(yoffset as f32);
                }
                _ => {}
            }
        }
    }
}

// /// utility function for loading a 2D texture from file
// /// ---------------------------------------------------
// pub unsafe fn load_texture(path: &str) -> u32 {
//     use image::{DynamicImage::*, GenericImage};
//     use std::{os::raw::c_void, path::Path};

//     let mut id = 0;

//     gl::GenTextures(1, &mut id);
//     let img = image::open(&Path::new(path)).expect("Texture failed to load");
//     let format = match img {
//         ImageLuma8(_) => gl::RED,
//         ImageLumaA8(_) => gl::RG,
//         ImageRgb8(_) => gl::RGB,
//         ImageRgba8(_) => gl::RGBA,
//     };

//     let data = img.raw_pixels();

//     gl::BindTexture(gl::TEXTURE_2D, id);
//     gl::TexImage2D(
//         gl::TEXTURE_2D,
//         0,
//         format as i32,
//         img.width() as i32,
//         img.height() as i32,
//         0,
//         format,
//         gl::UNSIGNED_BYTE,
//         &data[0] as *const u8 as *const c_void,
//     );
//     gl::GenerateMipmap(gl::TEXTURE_2D);

//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
//     gl::TexParameteri(
//         gl::TEXTURE_2D,
//         gl::TEXTURE_MIN_FILTER,
//         gl::LINEAR_MIPMAP_LINEAR as i32,
//     );
//     gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as
// i32);

//     id
// }
