use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Frame, Drawable};
use crate::shot::Shot;
use crate::invaders::Invaders;
use crate::explosives::MiniBombs;


pub struct Player {
    x: usize, 
    y: usize,
    number_of_shots: usize,
    shots: Vec<Shot>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            number_of_shots: 3,
            shots: Vec::new(),
        }
    }
}

impl Player {
    pub fn new(number_of_shots: usize) -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            number_of_shots,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS -1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < self.number_of_shots {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool{
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding && invaders.kill_invader_at(shot.x, shot.y){
                hit_something = true;
                shot.explode();
            }
        }
        hit_something
    }

    pub fn detect_hit_minibombs(&mut self, minibombs: &mut MiniBombs, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding && minibombs.trigger_bomb_at(shot.x, shot.y, invaders) {
                hit_something = true;
            }
        }

        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}

