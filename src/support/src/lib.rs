#[macro_use]
extern crate glium;
extern crate genmesh;
extern crate obj;

pub mod camera;

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
