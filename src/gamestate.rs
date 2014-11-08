use std::io::File;
use std::io::fs::PathExtensions;

use calx::color;
use calx::Context;
use calx::event;
use calx::key;
use calx::{Fonter, V2};
use world;
use world::action;
use world::action::{Step};
use world::dir6::*;
use worldview;
use sprite::{WorldSprites};

pub struct GameState {
    world_spr: WorldSprites,
}

impl GameState {
    pub fn new(seed: Option<u32>) -> GameState {
        world::init_world(seed);
        GameState {
            world_spr: WorldSprites::new(),
        }
    }

    /// Repaint view, update game world if needed.
    pub fn update(&mut self, ctx: &mut Context) {
        ctx.clear(&color::BLACK);
        let camera = world::camera();
        worldview::draw_world(&camera, ctx);

        // TODO use FOV for sprite draw.
        self.world_spr.draw(|_| true, &camera, ctx);
        self.world_spr.update();

        let fps = 1.0 / ctx.render_duration;
        let _ = write!(&mut ctx.text_writer(V2(0, 8), 0.1, color::LIGHTGREEN)
                       .set_border(color::BLACK),
                       "FPS {}", fps);

        if action::control_state() == action::ReadyToUpdate {
            action::update();
        }
    }

    pub fn save_game(&self) {
        let save_data = world::save();
        let mut file = File::create(&Path::new("/tmp/magog_save.json"));
        file.write_str(save_data.as_slice());
    }

    pub fn load_game(&mut self) {
        let path = Path::new("/tmp/magog_save.json");
        if !path.exists() { return; }
        let save_data = File::open(&path).read_to_string().unwrap();
        world::load(save_data.as_slice());
    }

    /// Process a player control keypress.
    pub fn process_key(&mut self, key: key::Key) -> bool {
        if action::control_state() != action::AwaitingInput {
            return false;
        }

        match key {
            key::KeyQ | key::KeyPad7 => { action::input(Step(NorthWest)); }
            key::KeyW | key::KeyPad8 | key::KeyUp => { action::input(Step(North)); }
            key::KeyE | key::KeyPad9 => { action::input(Step(NorthEast)); }
            key::KeyA | key::KeyPad1 => { action::input(Step(SouthWest)); }
            key::KeyS | key::KeyPad2 | key::KeyDown => { action::input(Step(South)); }
            key::KeyD | key::KeyPad3 => { action::input(Step(SouthEast)); }
            key::KeyF5 => { self.save_game(); }
            key::KeyF9 => { self.load_game(); }
            _ => { return false; }
        }
        return true;
    }

    pub fn process(&mut self, event: event::Event) -> bool {
        match event {
            event::Render(ctx) => {
                self.update(ctx);
            }
            event::KeyPressed(key::KeyEscape) => {
                return false;
            }
            event::KeyPressed(k) => {
                self.process_key(k);
            }
            _ => ()
        }
        true
    }
}
