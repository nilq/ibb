use opengl_graphics::GlGraphics;
use piston::input::*;


pub struct View {
  gl: GlGraphics,
}

impl View {
  pub fn new(gl: GlGraphics) -> Self {
    View {
      gl
    }
  }

  pub fn render(&mut self, args: &RenderArgs) {
    use graphics::*;

    self.gl.draw(args.viewport(), |context, gl| {
      clear([1.0, 1.0, 1.0, 1.0], gl);
    })
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    
  }
}