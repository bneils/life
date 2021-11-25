use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::rect::Rect;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Hash for Point<isize> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // cantor's set
        let n = ((self.x + self.y)*(self.x + self.y + 1)/2) + self.y;
        n.hash(state);
    }
}

pub struct Cells {
    pub pts: HashSet<Point<isize>>,
}

impl Cells {
    pub fn new() -> Cells {
        Cells {
            pts: HashSet::new(),
        }
    }

    pub fn display(&self, center: Point<isize>, fov: f64, canvas: &mut Canvas<Window>) {
        let (total_x, total_y) = canvas.window().size();
        let len = (total_x as f64 / fov) as isize;
        let offset = Point {
            x: total_x as isize / 2 - center.x,
            y: total_y as isize / 2 - center.y
        };

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for pt in &self.pts {
            let x = (pt.x * (len + 1) + offset.x) as i32;
            let y = (pt.y * (len + 1) + offset.y) as i32;
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(Rect::new(x, y, len as u32, len as u32)).unwrap();
        }
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        
        let mut i = (offset.x - 1) % (len + 1);
        while i < total_x as isize {
            canvas.fill_rect(rect::Rect::new(i as i32, 0, 1, total_y)).unwrap();
            i += len + 1;
        }
        let mut i = (offset.y - 1) % (len + 1);
        while i < total_y as isize {
            canvas.fill_rect(rect::Rect::new(0, i as i32, total_x, 1)).unwrap();
            i += len + 1;
        }
    }

    pub fn update(&mut self) {
        let mut neighbor_counts: HashMap<Point<isize>, u8> = HashMap::new();
        // Increment neighbor count of surrounding cells
        for live_cell in &self.pts {
            for x in live_cell.x - 1..live_cell.x + 2 {
                for y in live_cell.y - 1..live_cell.y + 2 {
                    if y != live_cell.y || x != live_cell.x {
                        let adj = Point { x, y };
                        if self.pts.contains(&adj) {
                            *neighbor_counts.entry(*live_cell).or_insert(0u8) += 1;
                        }
                        else {
                            *neighbor_counts.entry(adj).or_insert(0u8) += 1;
                        }
                    }
                }
            }
        }

        for (pt, count) in &neighbor_counts {
            if self.pts.contains(pt) {
                if *count < 2 || *count > 3 {
                    self.pts.remove(pt);
                }
            } else {
                if *count == 3 {
                    self.pts.insert(*pt);
                }
            }
        }
    }
}
