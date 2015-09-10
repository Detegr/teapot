mod position;
use position::Position;

mod console;


#[macro_use]
extern crate glium;

#[macro_use]
extern crate ecs;
components! {
    struct Components {
        #[hot] position: Position,
        #[cold] light_dir: LightDir,
    }
}
systems! {
    struct Systems<Components, Services> {
        light_mover: ecs::system::EntitySystem<LightMover> = ecs::system::EntitySystem::new(
            LightMover,
            aspect!(<Components> all: [position, light_dir])
        ),
    }
}
pub struct Services {
    console: console::Console,
}
impl Default for Services {
    fn default() -> Self {
        Services {
            console: console::Console::new()
        }
    }
}
impl ecs::ServiceManager for Services {}

pub struct LightMover;
impl ecs::System for LightMover { type Components = Components; type Services = Services; }
impl ecs::system::EntityProcess for LightMover {
    fn process(&mut self, entities: ecs::EntityIter<Components>, data: &mut ecs::DataHelper<Components, Services>) {
        for e in entities {
            let mut dir = data.light_dir[e];
            data.position[e].x += if dir == LightDir::Right { -0.1 } else { 0.1 };
            {
                let pos = data.position[e];
                if pos.x >= 10.0 {
                    dir = LightDir::Right;
                    data.services.console.log("Light direction changed");
                }
                if pos.x <= -10.0 {
                    dir = LightDir::Left;
                    data.services.console.log("Light direction changed");
                }
            }
            data.light_dir[e] = dir;
        }
    }
}

extern crate glium_text;
extern crate cgmath;
extern crate support;

use std::default::Default;
use std::io::Read;

use glium::DisplayBuild;
use glium::Surface;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum LightDir {
    Left,
    Right,
}

fn scale_matrix(scale: f32) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        scale, 0.0, 0.0, 0.0,
        0.0, scale, 0.0, 0.0,
        0.0, 0.0, scale, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}


fn main() {
    let mut world = ecs::World::<Systems>::new();
    let display = glium::glutin::WindowBuilder::new()
                     .with_depth_buffer(24)
                     .build_glium()
                     .unwrap();

    let text_system = glium_text::TextSystem::new(&display);
    let font_file = match ::std::fs::File::open("/usr/share/fonts/TTF/arial.ttf") {
        Ok(f) => f,
        Err(_) => {
            panic!("Font not found");
        }
    };
    let font = glium_text::FontTexture::new(&display, &font_file, 24).ok().expect("Could not create fount");
    world.services.console.set_gfx(text_system, font);

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
    let mut teapot = vec![];
    match std::fs::File::open("assets/teapot.obj") {
        Ok(mut pot) => pot.read_to_end(&mut teapot).unwrap(),
        Err(e) => {
            println!("{}", e);
            panic!("Failed to open teapot");
        }
    };
    let vbuffer = support::load_wavefront(&display, &teapot[..]);
    let ibuffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let light = world.create_entity(
        |entity: ecs::BuildData<Components>, data: &mut Components| {
            data.position.add(&entity, Position { x: 0.0, y: 0.0 });
            data.light_dir.add(&entity, LightDir::Right);
        }
    );

    loop {
        let mut tgt = display.draw();
        tgt.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);
        let model = scale_matrix(0.005);
        world.with_entity_data(&light, |entity, data| {
            tgt.draw(&vbuffer, &ibuffer, &prog, &uniform! {
                m_perspective: camera.get_perspective(),
                m_view: camera.get_view(),
                m_model: model,
                in_lightpos: [data.position[entity].x, 0.0, -3.0],
            }, &glium::DrawParameters {
                depth_test: glium::DepthTest::IfLess,
                depth_write: true,
                ..Default::default()
            }).unwrap();
        });
        world.services.console.draw(&mut tgt);
        tgt.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                ev => camera.process_input(&ev),
            }
        }
        camera.update();
        world.update();
    }
}
