use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl Hash for Point<isize> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // cantor's set
        let n = ((self.x + self.y)*(self.x + self.y + 1)/2) + self.y;
        n.hash(state);
    }
}

pub struct GameMatrix {
    pub pts: HashSet<Point<isize>>,
}

impl GameMatrix {
    pub fn new() -> GameMatrix {
        GameMatrix {
            pts: HashSet::new(),
        }
    }

    pub fn pixel_to_cell_coord(scr: Point<isize>, center: Point<isize>, fov: f32, canvas: &mut Canvas<Window>) -> Point<isize> {
        // x = pt.x * (len+1) + offset.x
        // pt.x = (x - offset.x) / (len + 1)
        let (total_x, total_y) = canvas.window().size();
        let len = (total_x as f32 / fov) as isize;
        let offset = Point {
            x: total_x as isize / 2 - center.x,
            y: total_y as isize / 2 - center.y
        };

        let x = ((scr.x - offset.x) as f64 / (len + 1) as f64).floor() as isize;
        let y = ((scr.y - offset.y) as f64 / (len + 1) as f64).floor() as isize;

        Point { x, y }
    }

    pub fn display(&self, center: Point<isize>, fov: f32, canvas: &mut Canvas<Window>) {
        let (total_x, total_y) = canvas.window().size();
        let len = (total_x as f32 / fov) as isize;
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
            canvas.fill_rect(rect::Rect::new(x, y, len as u32, len as u32)).unwrap();
        }
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        
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
            neighbor_counts.insert(*live_cell, 0u8);
            for x in live_cell.x - 1..live_cell.x + 2 {
                for y in live_cell.y - 1..live_cell.y + 2 {
                    // We are checking a 3x3 with the current cell as a center.
                    // To only check the surrounding cells, we need to exclude the current one.
                    if y != live_cell.y || x != live_cell.x {
                        let adj = Point { x, y };
                        // If this cell is alive, we can forget about incrementing our count,
                        // because they will do that for "us" (current cell).
                        if self.pts.contains(&adj) {
                            *neighbor_counts.entry(*live_cell).or_insert(0u8) += 1;
                        }
                        // It is only when this point is not iterated over, that we must increment
                        // their count, and not ours, because they don't affect ours, but our
                        // presence affects their count.
                        else {
                            *neighbor_counts.entry(adj).or_insert(0u8) += 1;
                        }
                    }
                }
            }
        }
        
        // These steps do have to be done sequentially, because if state is changed mid-way, it WILL
        // affect how other cells change, and if done arbitrarily (as is the case with hash sets,
        // iirc), the game of Life is no longer deterministic.

        for (pt, count) in &neighbor_counts {
            if self.pts.contains(pt) {
                // Alive and contains less than 2 (underpopulated) or more than 3 (overpopulated)
                // Then it dies.
                if *count < 2 || *count > 3 {
                    self.pts.remove(pt);
                }
            } else {
                // If it is already dead, then if it is rejuvenated if it has 3 neighbors.
                if *count == 3 {
                    self.pts.insert(*pt);
                }
            }
        }
    }
}
