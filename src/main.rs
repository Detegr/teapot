#[macro_use]
extern crate glium;

#[macro_use]
extern crate ecs;
use ecs::World;

extern crate glium_text;

extern crate cgmath;
extern crate genmesh;
extern crate obj;

mod support;

use std::default::Default;
use std::io::Read;
use glium::Surface;

use cgmath::FixedArray;

#[derive(Eq,PartialEq)]
enum LightDir {
    Left,
    Right,
}

fn main() {

    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
                     .with_depth_buffer(24)
                     .build_glium()
                     .unwrap();
    let text_system = glium_text::TextSystem::new(&display);
    let font_file = match std::fs::File::open("/usr/share/fonts/TTF/arial.ttf") {
        Ok(f) => f,
        Err(_) => {
            panic!("Could not open a font");
        }
    };
    let font = glium_text::FontTexture::new(&display, &font_file, 24).unwrap();
    let text = glium_text::TextDisplay::new(&text_system, &font, "Hello Teapot world!");

    let mut camera = support::camera::CameraState::new();
    let mut vert_shader = String::new();
    match std::fs::File::open("assets/cel.vert") {
        Ok(mut f) => {
            f.read_to_string(&mut vert_shader).unwrap();
        },
        Err(_) => {
            panic!("Vertex shader not found");
        }
    };
    let mut frag_shader = String::new();
    match std::fs::File::open("assets/cel.frag") {
        Ok(mut f) => {
            f.read_to_string(&mut frag_shader).unwrap();
        }
        Err(_) => {
            panic!("Fragment shader not found");
        }
    }
    let prog = match glium::Program::from_source(&display, &vert_shader, &frag_shader, None) {
        Ok(prog) => prog,
        Err(e) => {
            println!("{}", e);
            panic!("Failed to compile shaders");
        }
    };
    let vbuffer = load_wavefront(&display, include_bytes!("../assets/teapot.obj"));
    let ibuffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut light_dir = LightDir::Right;
    let mut light_x: f32 = -10.0;

    loop {
        light_x += if light_dir == LightDir::Right { 0.1 } else { -0.1 };
        if light_x >= 10.0 {
            light_dir = LightDir::Left;
        }
        if light_x <= -10.0 {
            light_dir = LightDir::Right;
        }
        println!("{}", light_x);

        let mut tgt = display.draw();
        tgt.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);
        let model = cgmath::Matrix4::from_fixed([
            [0.005, 0.0,   0.0,   0.0],
            [0.0,   0.005, 0.0,   0.0],
            [0.0,   0.0,   0.005, 0.0],
            [0.0,   0.0,   0.0,   1.0],
        ]);
        tgt.draw(&vbuffer, &ibuffer, &prog, &uniform! {
            m_perspective: camera.get_perspective(),
            m_view: camera.get_view(),
            m_model: model,
            in_lightpos: [light_x, 0.0, -3.0],
        }, &glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            ..Default::default()
        }).unwrap();
        glium_text::draw(&text, &text_system, &mut tgt, cgmath::Matrix4::identity().into_fixed(), (1.0, 1.0, 0.3, 1.0));
        tgt.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                ev => camera.process_input(&ev),
            }
        }
        camera.update();
    }
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
// This function is from glutin's (https://github.com/tomaka/glutin) examples
pub fn load_wavefront(display: &glium::Display, data: &[u8]) -> glium::vertex::VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        in_position: [f32; 3],
        in_normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, in_position, in_normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::Obj::load(&mut data);

    let mut vertex_data = Vec::new();

    for shape in data.object_iter().next().unwrap().group_iter().flat_map(|g| g.indices().iter()) {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position()[v.0];
                    let texture = v.1.map(|index| data.texture()[index]);
                    let normal = v.2.map(|index| data.normal()[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        in_position: position,
                        in_normal: normal,
                        texture: texture,
                    })
                }
            },
            _ => unimplemented!()
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into_vertex_buffer_any()
}
