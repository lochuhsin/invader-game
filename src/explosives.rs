use std::time::Duration;
use rusty_time::timer::Timer;
use rand::Rng;

use crate::{frame::Drawable, invaders::Invaders, NUM_COLS, NUM_ROWS};

struct MiniBomb {
    x: usize,
    y: usize
}


pub struct MiniBombs {
    bombs: Vec<MiniBomb>,
    max_bombs: usize,
    timer_for_create: Timer,
}

impl MiniBombs {
    pub fn new(max_bomb_count: usize) -> Self {
        MiniBombs {
            bombs: Vec::new(),
            max_bombs: max_bomb_count,
            timer_for_create: Timer::from_millis(1000),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Create a bomb in monitor
        self.timer_for_create.update(duration);
        if self.timer_for_create.ready && self.bombs.len() < self.max_bombs{
            let mut rng = rand::thread_rng();
            self.bombs.push(MiniBomb{
                x: rng.gen_range(0..NUM_COLS),
                y: rng.gen_range(0..NUM_ROWS*2/3),
            });
        }   
    }

    pub fn trigger_bomb_at(&mut self, x: usize, y:usize, invaders: &mut Invaders) -> bool{
        if let Some(idx) = self.bombs.iter().position(|bomb|(bomb.x == x && bomb.y == y)) {
            self.bombs.remove(idx);

            // 9 palace grid
            for ix in x-2..x+2{
                for iy in y-2..y+2{
                    invaders.kill_invader_at(ix, iy);
                }
            }
            true
        }else {
            false
        }
    }
    
}

impl Drawable for MiniBombs {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for bomb in self.bombs.iter() {
            frame[bomb.x][bomb.y] = "G"
        }
    }
}


impl Default for MiniBombs {
    fn default() -> Self {
        Self {
            bombs: Vec::new(),
            max_bombs: 5,
            timer_for_create: Timer::from_millis(1000)
        }
    }
}



pub struct BigBomb {
    x: usize,
    y: usize,
}

pub struct BigBombs {
    bombs: Vec<BigBomb>,
    max_bombs: usize,
    timer_for_create: Timer,
}

impl BigBombs {
    pub fn new(max_bomb_count: usize) -> Self {
        BigBombs {
            bombs: Vec::new(),
            max_bombs: max_bomb_count,
            timer_for_create: Timer::from_millis(1000),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        // Create a bomb in monitor
        self.timer_for_create.update(duration);
        if self.timer_for_create.ready && self.bombs.len() < self.max_bombs{
            let mut rng = rand::thread_rng();
            self.bombs.push(BigBomb{
                x: rng.gen_range(0..NUM_COLS),
                y: rng.gen_range(0..NUM_ROWS*2/3),
            });
        }   
    }

    pub fn trigger_bomb_at(&mut self, x: usize, y:usize, invaders: &mut Invaders) -> bool{
        if let Some(idx) = self.bombs.iter().position(|bomb|(bomb.x == x && bomb.y == y)) {
            self.bombs.remove(idx);
            // 9 palace grid
            for ix in x-4..x+4{
                for iy in y-4..y+4{
                    invaders.kill_invader_at(ix, iy);
                }
            }
            true
        }else {
            false
        }
    }
    
}

impl Drawable for BigBombs {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for bomb in self.bombs.iter() {
            frame[bomb.x][bomb.y] = "B"
        }
    }
}


impl Default for BigBombs {
    fn default() -> Self {
        Self {
            bombs: Vec::new(),
            max_bombs: 5,
            timer_for_create: Timer::from_millis(4000)
        }
    }
}

