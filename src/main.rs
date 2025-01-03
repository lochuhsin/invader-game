use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::explosives::{BigBombs, MiniBombs};
use invaders::frame::{self, Drawable};
use invaders::invaders::Invaders;
use invaders::{player, render};
use rusty_audio::Audio;
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle: thread::JoinHandle<()> = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = player::Player::new(3);
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    let mut minibombs = MiniBombs::new(5);
    let mut bigbombs = BigBombs::new(5);

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::from_nanos(1))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => {
                        player.move_left();
                    }
                    KeyCode::Right => {
                        player.move_right();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        // Updates
        player.update(delta);
        minibombs.update(delta);
        bigbombs.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }
        if player.detect_hit_minibombs(&mut minibombs, &mut invaders) {
            audio.play("explode");
        }
        if player.detect_hit_bigbombs(&mut bigbombs, &mut invaders) {
            audio.play("explode");
        }
        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &minibombs, &bigbombs];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1)); // fps ?

        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
