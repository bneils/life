mod life;
use life::{Cells, Point};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::thread;
use std::time::{Instant, Duration};
use std::collections::HashSet;

fn toad(cells: &mut Cells) {
    cells.pts.insert(Point { x: 2, y: 2 });
    cells.pts.insert(Point { x: 3, y: 2 });
    cells.pts.insert(Point { x: 4, y: 2 });
    cells.pts.insert(Point { x: 1, y: 3 });
    cells.pts.insert(Point { x: 2, y: 3 });
    cells.pts.insert(Point { x: 3, y: 3 });
}

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let video = video_subsystem
        .window("game of life (draft)", 700, 700)
        .build()
        .unwrap();

    let mut canvas = video.into_canvas().build().unwrap();
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();

    let mut event_pump = _sdl.event_pump().unwrap();

    let mut life = Cells::new();
    let mut running = false;
    let mut has_deployed = false;
    //toad(&mut life);

    let hz = 5;
    let mut fov = 60.0;
    let mut center = Point { x: 0, y: 0 };
    let mut last_game_tick = Instant::now();

    life.display(center, fov, &mut canvas);
    canvas.present();

    let mut mouse_down = false;
    let mut mouse_pos: Option<Point<isize>> = None;

    let mut draft = None;

    loop {
        canvas.clear();
        let lctrl_pressed = event_pump.keyboard_state().is_scancode_pressed(Scancode::LCtrl);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("Event::Quit reached"),
                Event::MouseButtonDown { .. } => {
                    mouse_down = true;
                    if lctrl_pressed {
                        match mouse_pos {
                            Some(mouse_pos) => {
                                let pt = Cells::pixel_to_cell_coord(mouse_pos, center, fov, &mut canvas);
                                if life.pts.contains(&pt) {
                                    life.pts.remove(&pt);
                                } else {
                                    life.pts.insert(pt);
                                }
                            },
                            None => {},
                        }
                    }
                },
                Event::MouseButtonUp { .. } => {
                    mouse_down = false;
                }
                Event::MouseMotion { xrel, yrel, x, y, .. } => {
                    mouse_pos = Some(Point { x: x as isize, y: y as isize });
                    if mouse_down && !lctrl_pressed {
                        center.x -= xrel as isize;
                        center.y -= yrel as isize;
                    }
                },
                Event::KeyDown { scancode, .. } => {
                    match scancode {
                        Some(scancode) => {
                            match scancode {
                                Scancode::S => {
                                    if !has_deployed {
                                        has_deployed = true;
                                        running = true;
                                        draft = Some(life.pts.clone());
                                        canvas.window_mut().set_title("game of life (running)").unwrap();
                                    }
                                },
                                Scancode::P => {
                                    if has_deployed {
                                        running = !running;
                                        canvas.window_mut().set_title(
                                            format!("game of life ({})", if running { "running" } else { "paused" }).as_str()
                                        ).unwrap();
                                    }
                                },
                                Scancode::R => {
                                    if has_deployed {
                                        has_deployed = false;
                                        running = false;
                                        canvas.window_mut().set_title("game of life (draft)").unwrap();
                                        match &draft {
                                            Some(draft) => {
                                                life.pts = draft.clone();
                                            },
                                            None => {},
                                        }
                                    } else {
                                        life.pts = HashSet::new();
                                    }
                                },
                                Scancode::Equals => {
                                    if lctrl_pressed && fov > 1.0 {
                                        fov -= 1.0;
                                    }
                                },
                                Scancode::Minus => {
                                    if lctrl_pressed && fov < 100.0 {
                                        fov += 1.0;
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

        if running && last_game_tick.elapsed().as_millis() >= 1000 / hz {
            life.update();
            last_game_tick = Instant::now();
        }

        life.display(center, fov, &mut canvas);
        
        canvas.present();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
