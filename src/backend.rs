use glium::{self, Surface};
use glium::glutin;
use glium::index::PrimitiveType;
use euclid::Point2D;
use vitral;

pub type Context = vitral::Context<usize, Vertex>;

pub type GliumTexture = glium::texture::CompressedSrgbTexture2d;

pub struct Backend {
    program: glium::Program,
    textures: Vec<GliumTexture>,
}

impl Backend {
    pub fn new(display: &glium::Display) -> Backend {
        let program = program!(
            display,
            150 => {
            vertex: "
                #version 150 core

                uniform mat4 matrix;

                in vec2 pos;
                in vec4 color;
                in vec2 tex_coord;

                out vec4 v_color;
                out vec2 v_tex_coord;

                void main() {
                    gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                    v_color = color;
                    v_tex_coord = tex_coord;
                }
            ",

            fragment: "
                #version 150 core
                uniform sampler2D tex;
                in vec4 v_color;
                in vec2 v_tex_coord;
                out vec4 f_color;

                void main() {
                    vec4 tex_color = texture(tex, v_tex_coord);

                    // Discard fully transparent pixels to keep them from
                    // writing into the depth buffer.
                    if (tex_color.a == 0.0) discard;

                    f_color = v_color * tex_color;
                }
            "})
                          .unwrap();

        Backend {
            program: program,
            textures: Vec::new(),
        }
    }

    pub fn make_texture(&mut self, display: &glium::Display, img: vitral::ImageBuffer) -> usize {
        let dim = (img.width(), img.height());
        let raw = glium::texture::RawImage2d::from_raw_rgba(img.into_raw(), dim);
        let tex = glium::texture::CompressedSrgbTexture2d::new(display, raw).unwrap();
        self.textures.push(tex);
        self.textures.len() - 1
    }

    fn process_events<V>(&self,
                         display: &glium::Display,
                         context: &mut vitral::Context<usize, V>)
                         -> bool
        where V: vitral::Vertex
    {
        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return false,
                glutin::Event::MouseMoved(x, y) => context.input_mouse_move(x, y),
                glutin::Event::MouseInput(state, button) => {
                    context.input_mouse_button(match button {
                                                   glutin::MouseButton::Left => {
                                                       vitral::MouseButton::Left
                                                   }
                                                   glutin::MouseButton::Right => {
                                                       vitral::MouseButton::Right
                                                   }
                                                   _ => vitral::MouseButton::Middle,
                                               },
                                               state == glutin::ElementState::Pressed)
                }
                glutin::Event::ReceivedCharacter(c) => context.input_char(c),
                glutin::Event::KeyboardInput(s, _, Some(vk)) => {
                    let is_down = s == glutin::ElementState::Pressed;
                    use glium::glutin::VirtualKeyCode::*;
                    if let Some(vk) = match vk {
                        Tab => Some(vitral::Keycode::Tab),
                        LShift | RShift => Some(vitral::Keycode::Shift),
                        LControl | RControl => Some(vitral::Keycode::Ctrl),
                        NumpadEnter | Return => Some(vitral::Keycode::Enter),
                        Back => Some(vitral::Keycode::Backspace),
                        Delete => Some(vitral::Keycode::Del),
                        Numpad8 | Up => Some(vitral::Keycode::Up),
                        Numpad2 | Down => Some(vitral::Keycode::Down),
                        Numpad4 | Left => Some(vitral::Keycode::Left),
                        Numpad6 | Right => Some(vitral::Keycode::Right),
                        _ => None,
                    } {
                        context.input_key_state(vk, is_down);
                    }
                }
                _ => (),
            }
        }

        true
    }

    pub fn update<V>(&self,
                     display: &glium::Display,
                     context: &mut vitral::Context<usize, V>)
                     -> bool
        where V: vitral::Vertex + glium::Vertex
    {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        let (w, h) = target.get_dimensions();

        for batch in context.end_frame() {
            // building the uniforms
            let uniforms = uniform! {
                matrix: [
                    [2.0 / w as f32, 0.0, 0.0, -1.0],
                    [0.0, -2.0 / h as f32, 0.0, 1.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                tex: glium::uniforms::Sampler::new(&self.textures[batch.texture])
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            };

            let vertex_buffer = {
                glium::VertexBuffer::new(display, &batch.vertices).unwrap()
            };

            // building the index buffer
            let index_buffer = glium::IndexBuffer::new(display,
                                                       PrimitiveType::TrianglesList,
                                                       &batch.triangle_indices)
                                   .unwrap();

            let params = glium::draw_parameters::DrawParameters {
                scissor: batch.clip.map(|clip| {
                    glium::Rect {
                        left: clip.origin.x as u32,
                        bottom: h - (clip.origin.y + clip.size.height) as u32,
                        width: clip.size.width as u32,
                        height: clip.size.height as u32,
                    }
                }),
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            target.draw(&vertex_buffer,
                        &index_buffer,
                        &self.program,
                        &uniforms,
                        &params)
                  .unwrap();
        }

        target.finish().unwrap();

        self.process_events(display, context)
    }
}


// XXX: An exact copy of Vitral vertex struct, just so that I can derive a
// Glium vertex implementatino for it.
#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub tex_coord: [f32; 2],
}
implement_vertex!(Vertex, pos, color, tex_coord);

impl vitral::Vertex for Vertex {
    fn new(pos: [f32; 2], color: [f32; 4], tex_coord: [f32; 2]) -> Self {
        Vertex {
            pos: pos,
            color: color,
            tex_coord: tex_coord,
        }
    }
}

/*
fn main() {
    // Construct Glium backend.
    let display = glutin::WindowBuilder::new()
                      .build_glium()
                      .unwrap();

    let mut backend = Backend::new(&display);

    // Construct Vitral context.
    let mut context: Context;
    let mut builder = vitral::Builder::new();
    let image = builder.add_image(&image::open(&Path::new("julia.png")).unwrap());
    context = builder.build(|img| backend.make_texture(&display, img));

    let font = context.default_font();

    let mut test_input = String::new();

    // Run the program.
    loop {
        context.begin_frame();

        context.draw_image(image, Point2D::new(100.0, 100.0), [1.0, 1.0, 1.0, 1.0]);

        if context.button("Hello, world") {
            println!("Click");
        }

        if context.button("Another button") {
            println!("Clack {}", test_input);
        }

        context.text_input(font,
                           Point2D::new(10.0, 120.0),
                           [0.8, 0.8, 0.8, 1.0],
                           &mut test_input);

        if !backend.update(&display, &mut context) {
            return;
        }
    }
}
*/