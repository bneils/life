mod life;
use life::{Cells, Point};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::keyboard::Scancode;
use std::thread;
use std::time::{Instant, Duration};

fn beacon(cells: &mut Cells) {
    cells.pts.insert(Point { x: 1, y: 1 });
    cells.pts.insert(Point { x: 2, y: 1 });
    cells.pts.insert(Point { x: 1, y: 2 });
    cells.pts.insert(Point { x: 4, y: 3 });
    cells.pts.insert(Point { x: 3, y: 4 });
    cells.pts.insert(Point { x: 4, y: 4 });
}

fn toad(cells: &mut Cells) {
    cells.pts.insert(Point { x: 2, y: 2 });
    cells.pts.insert(Point { x: 3, y: 2 });
    cells.pts.insert(Point { x: 4, y: 2 });
    cells.pts.insert(Point { x: 1, y: 3 });
    cells.pts.insert(Point { x: 2, y: 3 });
    cells.pts.insert(Point { x: 3, y: 3 });
}

fn glidergun(cells: &mut Cells) {
    cells.pts.insert(Point { x: 25, y: 1 });
    cells.pts.insert(Point { x: 23, y: 2 });
    cells.pts.insert(Point { x: 25, y: 2 });
    cells.pts.insert(Point { x: 13, y: 3 });
    cells.pts.insert(Point { x: 14, y: 3 });
    cells.pts.insert(Point { x: 21, y: 3 });
    cells.pts.insert(Point { x: 22, y: 3 });
    cells.pts.insert(Point { x: 35, y: 3 });
    cells.pts.insert(Point { x: 36, y: 3 });
    cells.pts.insert(Point { x: 12, y: 4 });
    cells.pts.insert(Point { x: 16, y: 4 });
    cells.pts.insert(Point { x: 21, y: 4 });
    cells.pts.insert(Point { x: 22, y: 4 });
    cells.pts.insert(Point { x: 35, y: 4 });
    cells.pts.insert(Point { x: 36, y: 4 });
    cells.pts.insert(Point { x: 1, y: 5 });
    cells.pts.insert(Point { x: 2, y: 5 });
    cells.pts.insert(Point { x: 11, y: 5 });
    cells.pts.insert(Point { x: 17, y: 5 });
    cells.pts.insert(Point { x: 21, y: 5 });
    cells.pts.insert(Point { x: 22, y: 5 });
    cells.pts.insert(Point { x: 1, y: 6 });
    cells.pts.insert(Point { x: 2, y: 6 });
    cells.pts.insert(Point { x: 11, y: 6 });
    cells.pts.insert(Point { x: 15, y: 6 });
    cells.pts.insert(Point { x: 17, y: 6 });
    cells.pts.insert(Point { x: 18, y: 6 });
    cells.pts.insert(Point { x: 23, y: 6 });
    cells.pts.insert(Point { x: 25, y: 6 });
    cells.pts.insert(Point { x: 11, y: 7 });
    cells.pts.insert(Point { x: 17, y: 7 });
    cells.pts.insert(Point { x: 25, y: 7 });
    cells.pts.insert(Point { x: 12, y: 8 });
    cells.pts.insert(Point { x: 16, y: 8 });
    cells.pts.insert(Point { x: 13, y: 9 });
    cells.pts.insert(Point { x: 14, y: 9 });
}

fn pulsar(cells: &mut Cells) {
    cells.pts.insert(Point { x: 10, y: 1 });
    cells.pts.insert(Point { x: 10, y: 2 });
    cells.pts.insert(Point { x: 5, y: 3 });
    cells.pts.insert(Point { x: 9, y: 3 });
    cells.pts.insert(Point { x: 10, y: 3 });
    cells.pts.insert(Point { x: 3, y: 5 });
    cells.pts.insert(Point { x: 6, y: 5 });
    cells.pts.insert(Point { x: 8, y: 5 });
    cells.pts.insert(Point { x: 10, y: 5 });
    cells.pts.insert(Point { x: 5, y: 6 });
    cells.pts.insert(Point { x: 9, y: 6 });
    cells.pts.insert(Point { x: 10, y: 6 });
    cells.pts.insert(Point { x: 5, y: 8 });
    cells.pts.insert(Point { x: 9, y: 8 });
    cells.pts.insert(Point { x: 10, y: 8 });
    cells.pts.insert(Point { x: 3, y: 9 });
    cells.pts.insert(Point { x: 6, y: 9 });
    cells.pts.insert(Point { x: 8, y: 9 });
    cells.pts.insert(Point { x: 10, y: 9 });
    cells.pts.insert(Point { x: 1, y: 10 });
    cells.pts.insert(Point { x: 2, y: 10 });
    cells.pts.insert(Point { x: 3, y: 10 });
    cells.pts.insert(Point { x: 5, y: 10 });
    cells.pts.insert(Point { x: 6, y: 10 });
    cells.pts.insert(Point { x: 8, y: 10 });
    cells.pts.insert(Point { x: 9, y: 10 });
    cells.pts.insert(Point { x: 13, y: 10 });
    cells.pts.insert(Point { x: 10, y: 13 });
}

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let video = video_subsystem
        .window("game of life", 700, 700)
        .build()
        .unwrap();

    let mut canvas = video.into_canvas().build().unwrap();
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();

    let mut event_pump = _sdl.event_pump().unwrap();

    let mut life = Cells::new();
    pulsar(&mut life);

    let hz = 7;
    let fov = 60.0;
    let mut center = Point { x: 0.0, y: 0.0 };
    let mut last_game_tick = Instant::now();

    life.display(center, fov, &mut canvas);
    canvas.present();

    let mut mouse_down = false;
    
    loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => panic!("Event::Quit reached"),
                Event::MouseButtonDown { .. } => mouse_down = true,
                Event::MouseButtonUp { .. } => mouse_down = false,
                Event::MouseMotion { xrel, yrel, .. } => {
                    if mouse_down {
                        center.x -= xrel as f64;
                        center.y -= yrel as f64;
                    }
                },
                _ => {},
            }
        }

        let lctrl_pressed = event_pump.keyboard_state().is_scancode_pressed(Scancode::LCtrl);

        if last_game_tick.elapsed().as_millis() >= 1000 / hz {
            life.update();
            last_game_tick = Instant::now();
        }

        life.display(center, fov, &mut canvas);
        canvas.present();
        thread::sleep(Duration::from_millis(1000 / 100));
    }
}
