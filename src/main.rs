mod life;
use life::{GameMatrix, Point};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::thread;
use std::cmp;
use std::time::{Instant, Duration};
use std::collections::HashSet;

enum State {
    Draft,
    Running,
    RunningPaused,
}

struct App<'a> {
    game: GameMatrix,

    fov: f32,
    pxoffset: Point<isize>,
    canvas: &'a mut Canvas<Window>,
    event_pump: &'a mut EventPump,

    hz: u32,
    last_game_tick: Instant,
    fps: u32,
    last_frame: Instant,

    state: State,
}

impl<'a> App<'a> {
    fn new(hz: u32, fps: u32, fov: f32, canvas: &'a mut Canvas<Window>, event_pump: &'a mut EventPump) -> App<'a> {
        App {
            game: GameMatrix::new(),

            fov,
            pxoffset: Point::new(0, 0),
            canvas,
            event_pump,

            hz,
            last_game_tick: Instant::now(),
            fps,
            last_frame: Instant::now(),

            state: State::Draft,
        }
    }

    fn render(&mut self) {
        self.canvas.window_mut().set_title(
            match self.state {
                State::Draft => "game of life (draft)",
                State::Running => "game of life (running)",
                State::RunningPaused => "game of life (paused)",
            }
        ).unwrap();
        self.event_loop();
        //
        self.game.display(self.pxoffset, self.fov, &mut self.canvas);

        //
        self.canvas.present();
    }

    fn event_loop(&mut self) {
        let lctrl_pressed = self.event_pump
            .keyboard_state()
            .is_scancode_pressed(Scancode::LCtrl);
        
        let left_down = self.event_pump
            .mouse_state()
            .left();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("Event::Quit reached"),
                Event::MouseButtonDown { x, y, .. } => {
                    if matches!(self.state, State::Draft) && lctrl_pressed {
                        let pt = GameMatrix::pixel_to_cell_coord(
                            Point::new(x as isize, y as isize), 
                            self.pxoffset, self.fov, &mut self.canvas);
                        // Toggle cell state
                        if self.game.pts.contains(&pt) {
                            self.game.pts.remove(&pt);
                        } else {
                            self.game.pts.insert(pt);
                        }
                    }
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    if left_down && !lctrl_pressed {
                        self.pxoffset.x -= xrel as isize;
                        self.pxoffset.y -= yrel as isize;
                    }
                },
                Event::KeyDown { scancode, .. } => {
                    match scancode {
                        Some(scancode) => {
                            match scancode {
                                Scancode::S => if matches!(self.state, State::Draft) {
                                    self.state = State::Running;
                                },
                                Scancode::P => match self.state {
                                    State::Running => self.state = State::RunningPaused,
                                    State::RunningPaused => self.state = State::Running,
                                    _ => {},
                                },
                                Scancode::R => match self.state {
                                    State::Running => self.state = State::Draft,
                                    State::RunningPaused => self.state = State::Draft,
                                    State::Draft => {
                                        self.game.pts = HashSet::new();
                                    },
                                },
                                Scancode::Equals => {
                                    if lctrl_pressed && self.fov > 1.0 {
                                        self.fov -= 1.0;
                                    }
                                },
                                Scancode::Minus => {
                                    if lctrl_pressed && self.fov < 100.0 {
                                        self.fov += 1.0;
                                    }
                                }
                                _ => {},
                            }
                        },
                        None => {},
                    }
                }
                _ => {},
            }
        }
    }

    // Send a pulse through this app.
    pub fn pulse(&mut self) {
        if matches!(self.state, State::Running) && 1.0 / self.hz as f64 <= self.last_game_tick.elapsed().as_secs_f64() {
            self.last_game_tick = Instant::now();
            self.game.update();
        }
        if 1.0 / self.fps as f64 <= self.last_frame.elapsed().as_secs_f64() {
            self.last_frame = Instant::now();
            self.render();
        }
    }
}

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let video = video_subsystem
        .window("game of life (draft)", 700, 700)
        .build()
        .unwrap();

    let mut canvas = video.into_canvas().build().unwrap();
    let mut event_pump = _sdl.event_pump().unwrap();

    let mut app = App::new(7, 60, 20.0, &mut canvas, &mut event_pump);

    let shortest_wait_needed = Duration::from_nanos((1_000_000 / cmp::max(app.hz, app.fps)) as u64);

    loop {
        app.pulse();
        thread::sleep(shortest_wait_needed);
    }
}
