extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::process;
use std::io::prelude::*;
use std::io;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    clues: Vec<Vec<u32>>,  // Sudoku clues.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let radius = 1.0;
        let grid_len = 9.0;
        let font_size = 2;
        let grid = self.clues.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform;

            for i in 0..grid_len as u32 {
                let i = i as f64;
                let foreach_w = i * (args.window_size[0] / grid_len);
                let foreach_h = i * (args.window_size[1] / grid_len);
                line(BLACK, radius, [foreach_w, 0.0, foreach_w, args.window_size[1]], transform, gl);
                line(BLACK, radius, [0.0, foreach_h, args.window_size[0], foreach_h], transform, gl);
            }

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    text::Text::new(font_size)
                        .draw();
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    let mut clue = String::new();

    // Read stdin for Sudoku clues
    print!("Enter clues: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut clue)
        .expect("Failed to read line");
    trim_newline(&mut clue);

    // Check for clues size
    if clue.len() > 81 {
        process::exit(1);
    }
    while clue.len() < 81 {
        clue.push('0');
    }

    // Convert clues string into list of list of u32
    let grid_chars: Vec<char> = clue.to_string().chars().collect();
    let grid_intlen = 9;
    let mut grid = vec![vec![0; grid_intlen]; grid_intlen];
    for i in 0..grid_intlen {
        for j in 0..grid_intlen {
            grid[i][j] = grid_chars[i+j] as u32;
        }
    }

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("sudoku-solver", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        clues: grid,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

