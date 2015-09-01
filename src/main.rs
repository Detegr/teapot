#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate genmesh;
extern crate obj;

mod support;

use std::default::Default;
use std::io::Read;
use glium::Surface;

fn to_matrix(m: &[[f32; 4]; 4]) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        m[0][0], m[0][1], m[0][2], m[0][3],
        m[1][0], m[1][1], m[1][2], m[1][3],
        m[2][0], m[2][1], m[2][2], m[2][3],
        m[3][0], m[3][1], m[3][2], m[3][3],
    )
}

fn main() {

    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
                     .with_depth_buffer(24)
                     .build_glium()
                     .unwrap();

    let mut camera = support::camera::CameraState::new();
    let mut vert_shader = String::new();
    std::fs::File::open("../assets/cel.vert").unwrap().read_to_string(&mut vert_shader).unwrap();
    let mut frag_shader = String::new();
    std::fs::File::open("../assets/cel.frag").unwrap().read_to_string(&mut frag_shader).unwrap();
    let prog = glium::Program::from_source(&display, &vert_shader, &frag_shader, None).unwrap();
    let vbuffer = load_wavefront(&display, include_bytes!("../assets/teapot.obj"));
    let ibuffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    loop {
        let mut tgt = display.draw();
        tgt.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);
        let model = to_matrix(&[
            [0.005, 0.0,   0.0,   0.0],
            [0.0,   0.005, 0.0,   0.0],
            [0.0,   0.0,   0.005, 0.0],
            [0.0,   0.0,   0.0,   1.0],
        ]);
        tgt.draw(&vbuffer, &ibuffer, &prog, &uniform! {
            m_perspective: camera.get_perspective(),
            m_view: camera.get_view(),
            m_model: model,
            //m_normal: normal,
        }, &glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            ..Default::default()
        }).unwrap();
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
