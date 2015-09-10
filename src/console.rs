use glium;
use glium_text;
use cgmath::{Vector3, Matrix4};
use std::collections::VecDeque;
use cgmath::FixedArray;
use std::rc::Rc;

pub struct ConsoleGfx {
    text_system: glium_text::TextSystem,
    font: Rc<glium_text::FontTexture>,
}
pub struct Console {
    buffer: VecDeque<(String, Option<glium_text::TextDisplay<Rc<glium_text::FontTexture>>>)>,
    base_matrix: Matrix4<f32>,
    gfx: Option<ConsoleGfx>,
}

static LINE_HEIGHT: f32 = 0.05;

impl Console {
    pub fn new() -> Console {
        Console {
            buffer: VecDeque::new(),
            base_matrix: Matrix4::from_translation(&Vector3::new(-0.98, -0.98, 0.0)) *
                         ::scale_matrix(0.025),
            gfx: None,
        }
    }
    pub fn set_gfx(&mut self,
                   text_system: glium_text::TextSystem,
                   font: glium_text::FontTexture) {
        self.gfx = Some(ConsoleGfx {
            text_system: text_system,
            font: Rc::new(font),
        })
    }
    pub fn log(&mut self, s: &str) {
        self.buffer.push_front((s.into(), None));
    }
    pub fn draw(&mut self,
                tgt: &mut glium::Frame) {
        if self.gfx.is_none() { return }
        for (i, &mut (ref text, ref mut display)) in self.buffer.iter_mut().enumerate() {
            let font = self.gfx.as_ref().unwrap().font.clone();
            if display.is_none() {
                *display = Some(glium_text::TextDisplay::new(&self.gfx.as_ref().unwrap().text_system, font, &text[..]));
            }
            glium_text::draw(
                &display.as_ref().unwrap(),
                &self.gfx.as_ref().unwrap().text_system,
                tgt,
                (Matrix4::from_translation(&Vector3::new(0.0, i as f32 * LINE_HEIGHT, 0.0)) * self.base_matrix).into_fixed(),
                (1.0, 1.0, 0.3, 1.0)
            );
        }
    }
}
