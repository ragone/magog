use std::collections::HashMap;
use calx::{color, V2, Anchor};
use calx::backend::{Canvas, CanvasUtil, Event, Key, Fonter, Align};
use world;
use world::action;
use world::action::Input::*;
use world::query;
use world::query::ControlState::*;
use world::{Msg, FovStatus, World};
use calx::Dir6;
use calx::Dir6::*;
use calx_ecs::Entity;
use content::Brush;
use world::item::Slot;
use worldview;
use sprite::{WorldSprites, GibSprite, BeamSprite, ExplosionSprite};
use msg_queue::MsgQueue;
use {Screen, ScreenAction};
use titlescreen::TitleScreen;

/// Type of effect signaled by making a visible entity blink for a moment.
#[derive(Copy, Clone)]
pub enum Blink {
    /// The entity was damaged.
    Damaged,
    /// The entity is a threat that halted an automated activity.
    Threat,
}

pub struct GameScreen {
    /// Transient effect sprites drawn in game world view.
    world_spr: WorldSprites,
    /// Counters for entities with flashing damage animation.
    damage_timers: HashMap<Entity, (Blink, u32)>,

    /// Flag for autoexploration.
    // TODO: Probably going to need a general "ongoing activity" system at
    // some point.
    exploring: bool,

    msg: MsgQueue,
    ui_state: UiState,

    world: World,
}

enum UiState {
    Gameplay,
    Inventory,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {
            world_spr: WorldSprites::new(),
            damage_timers: HashMap::new(),
            exploring: false,
            msg: MsgQueue::new(),
            ui_state: UiState::Gameplay,
            world: World::new(::with_config(|c| c.rng_seed)),
        }
    }

    fn draw_player_ui(&mut self, ctx: &mut Canvas, player: Entity) {
        let hp = query::hp(&self.world, player);
        let max_hp = query::max_hp(&self.world, player);

        // Draw heart containers.
        for i in 0..((max_hp + 1) / 2) {
            let pos = V2(i as f32 * 8.0, 8.0);
            let heart_phase = if hp >= (i + 1) * 2 {
                0
            } else if hp == i * 2 + 1 {
                1
            } else {
                2
            };
            ctx.draw_image(Brush::IconHeart.get(heart_phase),
                           pos,
                           0.0,
                           color::FIREBRICK,
                           color::BLACK);
        }
    }

    fn base_paint(&mut self, ctx: &mut Canvas) {
        ctx.clear_color = color::GRAY1;
        let camera = self.world.flags.camera;
        worldview::draw_world(&self.world, &camera, ctx, &self.damage_timers);

        self.world_spr
            .draw(|x| query::fov_status(&self.world, (camera + x)) == Some(FovStatus::Seen),
                  &camera,
                  ctx);
        self.world_spr.update();

        Fonter::new(ctx)
            .color(color::LIGHTGRAY)
            .border(color::BLACK)
            .anchor(Anchor::TopRight)
            .align(Align::Right)
            .text(query::area_name(&self.world, camera))
            .draw(V2(638.0, 0.0));

        self.msg.draw(ctx);
        if let Some(player) = query::player(&self.world) {
            self.draw_player_ui(ctx, player);
        }

        if ::with_config(|c| c.show_fps) {
            let fps = 1.0 / ctx.window.frame_duration();
            Fonter::new(ctx)
                .color(color::LIGHTGRAY)
                .border(color::BLACK)
                .text(format!("FPS {:.0}", fps))
                .draw(V2(0.0, 8.0));
        }
    }

    fn base_update(&mut self, ctx: &mut Canvas) {
        // Process events
        loop {
            match world::pop_msg() {
                Some(Msg::Gib(loc)) => {
                    self.world_spr.add(Box::new(GibSprite::new(loc)));
                }
                Some(Msg::Damage(entity)) => {
                    self.damage_timers.insert(entity, (Blink::Damaged, 2));
                }
                Some(Msg::Text(txt)) => {
                    self.msg.msg(txt)
                }
                Some(Msg::Caption(txt)) => {
                    self.msg.caption(txt)
                }
                Some(Msg::Beam(loc1, loc2)) => {
                    self.world_spr.add(Box::new(BeamSprite::new(loc1, loc2, 10)));
                }
                Some(Msg::Sparks(_loc)) => {
                    // TODO
                }
                Some(Msg::Explosion(loc)) => {
                    self.world_spr.add(Box::new(ExplosionSprite::new(loc)));
                }
                None => break,
            }
        }

        self.base_paint(ctx);

        if query::control_state(&self.world) == ReadyToUpdate {
            action::update(&mut self.world);
        }

        if self.exploring {
            if query::control_state(&self.world) == AwaitingInput {
                self.exploring = self.autoexplore();
            }
        }

        // Decrement damage timers.
        // XXX: Can we do mutable contents iter without the cloning?
        self.damage_timers = self.damage_timers
                                 .clone()
                                 .into_iter()
                                 .filter(|&(_, (_, t))| t > 0)
                                 .map(|(e, (b, t))| (e, (b, t - 1)))
                                 .collect();

        self.msg.update();
    }

    fn inventory_update(&mut self, ctx: &mut Canvas) {
        unimplemented!();
        /*
        let player = query::player(&self.world).unwrap();
        for (i, slot_data) in SLOT_DATA.iter().enumerate() {
            let y = 8.0 * (i as f32);
            Fonter::new(ctx).color(color::LIGHTGRAY)
                .align(Align::Center).anchor(Anchor::Top)
                .text(format!("{}", slot_data.key))
                .draw(V2(4.0, y));
            Fonter::new(ctx).color(color::LIGHTGRAY)
                .text("]".to_string())
                .draw(V2(8.0, y));
            Fonter::new(ctx).color(color::LIGHTGRAY)
                .align(Align::Right).anchor(Anchor::TopRight)
                .text(format!("{}:", slot_data.name))
                .draw(V2(76.0, 8.0 * (i as f32)));

            Fonter::new(ctx).color(color::LIGHTGRAY)
                .text(match player.equipped(slot_data.slot) {
                    Some(item) => item.name(),
                    None => "".to_string()
                })
                .draw(V2(80.0, 8.0 * (i as f32)));
        }

        Fonter::new(ctx).color(color::LIGHTGRAY)
            .anchor(Anchor::BottomLeft)
            .text("Press letter to equip/unequip item. Press shift+letter to drop item.".to_string())
            .draw(V2(0.0, 360.0));
        */
    }

    pub fn inventory_process(&mut self, ctx: &mut Canvas, event: Event) -> bool {
        unimplemented!();
        /*
        let player = query::player(&self.world).unwrap();
        match event {
            Event::RenderFrame => { self.update(ctx); }
            Event::KeyPress(Key::Escape) | Event::KeyPress(Key::Tab) => {
                self.ui_state = UiState::Gameplay
            }
            Event::KeyPress(Key::F12) => { ctx.save_screenshot(&"magog"); }
            Event::KeyPress(_) => {}

            Event::Char(ch) => {
                for slot_data in SLOT_DATA.iter() {
                    if ch == slot_data.key {
                        if slot_data.slot.is_gear_slot() {
                            // Unequip gear
                            match player.free_bag_slot() {
                                None => {
                                    // No room in bag, can't unequip until
                                    // drop something.
                                    // TODO: Message about full bag.
                                }
                                Some(swap_slot) => {
                                    player.swap_equipped(slot_data.slot, swap_slot);
                                }
                            }
                        }
                        if slot_data.slot.is_bag_slot() {
                            // Bag items get equipped if they have are gear
                            // with a preferred slot.
                            if let Some(item) = player.equipped(slot_data.slot) {
                                let equip_slots = item.equip_slots();
                                for &swap_slot in equip_slots.iter() {
                                    if player.equipped(swap_slot).is_none() {
                                        player.swap_equipped(slot_data.slot, swap_slot);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if ch == slot_data.key.to_uppercase().next().unwrap() {
                        // Drop item in slot.
                        if let Some(item) = player.equipped(slot_data.slot) {
                            let loc = query::location(&self.world, player);
                            self.world.spatial.insert_at(item, loc);
                        }
                        break;
                    }
                }
            }

            _ => ()
        }
        true
        */
    }

    fn smart_move(&mut self, dir: Dir6) {
        let player = query::player(&self.world).unwrap();
        let loc = query::location(&self.world, player).unwrap();

        if !query::has_mobs(&self.world, loc + dir.to_v2()) {
            // Shoot instead of moving if you'd hit an enemy and there's no
            // melee target.
            let shoot_range = query::stats(&self.world, player).ranged_range as usize;
            if let Some(e) = query::find_target(&self.world, player, dir, shoot_range) {
                if query::is_hostile_to(&self.world, player, e) {
                    action::input(&mut self.world, Shoot(dir));
                    return;
                }
            }
        }

        let dirset = if ::with_config(|c| c.wall_sliding) {
            vec![dir, dir + 1, dir - 1]
        } else {
            vec![dir]
        };

        for &d in dirset.iter() {
            let target_loc = loc + d.to_v2();
            if query::has_mobs(&self.world, target_loc) {
                action::input(&mut self.world, Melee(d));
                return;
            } else if query::can_step(&self.world, player, d) {
                action::input(&mut self.world, Step(d));
                return;
            }
        }
    }

    fn autoexplore(&mut self) -> bool {
        let player = query::player(&self.world).unwrap();

        unimplemented!();
        /*
        let threats = player.is_threatened(6);
        if !threats.is_empty() {
            for &e in threats.iter() {
                // Blink the threatening enemies so that the player sees
                // what's blocking the explore.
                self.damage_timers.insert(e, (Blink::Threat, 2));
            }
            return false;
        }
        if let Some(pathing) = action::autoexplore_map(32) {
            let loc = player.location().unwrap();
            let steps = pathing.sorted_neighbors(&loc);
            if steps.len() == 0 {
                return false;
            }

            action::input(Step(loc.dir6_towards(steps[0]).unwrap()));
            return true;
        }
        */

        false
    }

    /// Context-specific interaction with the current cell.
    fn interact(&mut self) {
        let player = query::player(&self.world).unwrap();
        let loc = query::location(&self.world, player).unwrap();
        if let Some(item) = query::top_item(&self.world, loc) {
            action::pick_up(&mut self.world, player, item);
            return;
        }
    }

    /// Process a player control keypress.
    pub fn gameplay_process_key(&mut self, key: Key) -> bool {
        if query::control_state(&self.world) != AwaitingInput {
            return false;
        }

        if self.exploring {
            self.exploring = false;
        }

        match key {
            Key::Q | Key::Pad7 => {
                self.smart_move(NorthWest);
            }
            Key::W | Key::Pad8 | Key::Up => {
                self.smart_move(North);
            }
            Key::E | Key::Pad9 => {
                self.smart_move(NorthEast);
            }
            Key::A | Key::Pad1 => {
                self.smart_move(SouthWest);
            }
            Key::S | Key::Pad2 | Key::Down => {
                self.smart_move(South);
            }
            Key::D | Key::Pad3 => {
                self.smart_move(SouthEast);
            }

            Key::Enter => {
                self.interact();
            }
            Key::Space => {
                action::input(&mut self.world, Pass);
            }
            Key::X => {
                self.exploring = true;
            }

            // Open inventory
            Key::Tab => {
                self.ui_state = UiState::Inventory;
            }

            Key::F5 if cfg!(debug_assertions) => {
                action::save_game(&self.world);
            }
            Key::F9 if cfg!(debug_assertions) => {
                match action::load_game() {
                    Ok(world) => self.world = world,
                    _ => {}
                };
            }
            _ => {
                return false;
            }
        }
        return true;
    }

    pub fn gameplay_process(&mut self, ctx: &mut Canvas, event: Event) -> bool {
        match event {
            Event::RenderFrame => {
                self.update(ctx);
            }
            // TODO: Better quit confirmation than just pressing esc.
            Event::KeyPress(Key::Escape) => {
                return false;
            }
            Event::KeyPress(Key::F12) => {
                ctx.save_screenshot(&"magog");
            }
            Event::KeyPress(k) => {
                self.gameplay_process_key(k);
            }

            Event::Char(ch) => {
                // TODO: Chars and keypresses in same lookup (use variants?)
                match ch {
                    // Debug
                    '>' if cfg!(debug_assertions) => {
                        unimplemented!();
                        //action::next_level();
                    }
                    _ => (),
                }
            }

            _ => (),
        }
        true
    }
}

impl Screen for GameScreen {
    fn update(&mut self, ctx: &mut Canvas) -> Option<ScreenAction> {
        match self.ui_state {
            UiState::Gameplay => self.base_update(ctx),
            UiState::Inventory => self.inventory_update(ctx),
        }

        // TODO
        let mut running = true;

        for event in ctx.events().into_iter() {
            if event == Event::Quit {
                return Some(ScreenAction::Quit);
            }
            running = running &&
                      match self.ui_state {
                UiState::Gameplay => self.gameplay_process(ctx, event),
                UiState::Inventory => self.inventory_process(ctx, event),
            };
        }

        if !running {
            ctx.clear_color = color::BLACK;
            Some(ScreenAction::Change(Box::new(TitleScreen::new())))
        } else {
            None
        }
    }
}

struct SlotData {
    key: char,
    slot: Slot,
    name: &'static str,
}

static SLOT_DATA: [SlotData; 34] = [
    SlotData { key: '1', slot: Slot::Spell1,     name: "Ability" },
    SlotData { key: '2', slot: Slot::Spell2,     name: "Ability" },
    SlotData { key: '3', slot: Slot::Spell3,     name: "Ability" },
    SlotData { key: '4', slot: Slot::Spell4,     name: "Ability" },
    SlotData { key: '5', slot: Slot::Spell5,     name: "Ability" },
    SlotData { key: '6', slot: Slot::Spell6,     name: "Ability" },
    SlotData { key: '7', slot: Slot::Spell7,     name: "Ability" },
    SlotData { key: '8', slot: Slot::Spell8,     name: "Ability" },
    SlotData { key: 'a', slot: Slot::Melee,      name: "Weapon" },
    SlotData { key: 'b', slot: Slot::Ranged,     name: "Ranged" },
    SlotData { key: 'c', slot: Slot::Head,       name: "Head" },
    SlotData { key: 'd', slot: Slot::Body,       name: "Body" },
    SlotData { key: 'e', slot: Slot::Feet,       name: "Feet" },
    SlotData { key: 'f', slot: Slot::TrinketF,   name: "Trinket" },
    SlotData { key: 'g', slot: Slot::TrinketG,   name: "Trinket" },
    SlotData { key: 'h', slot: Slot::TrinketH,   name: "Trinket" },
    SlotData { key: 'i', slot: Slot::TrinketI,   name: "Trinket" },
    SlotData { key: 'j', slot: Slot::InventoryJ, name: "" },
    SlotData { key: 'k', slot: Slot::InventoryK, name: "" },
    SlotData { key: 'l', slot: Slot::InventoryL, name: "" },
    SlotData { key: 'm', slot: Slot::InventoryM, name: "" },
    SlotData { key: 'n', slot: Slot::InventoryN, name: "" },
    SlotData { key: 'o', slot: Slot::InventoryO, name: "" },
    SlotData { key: 'p', slot: Slot::InventoryP, name: "" },
    SlotData { key: 'q', slot: Slot::InventoryQ, name: "" },
    SlotData { key: 'r', slot: Slot::InventoryR, name: "" },
    SlotData { key: 's', slot: Slot::InventoryS, name: "" },
    SlotData { key: 't', slot: Slot::InventoryT, name: "" },
    SlotData { key: 'u', slot: Slot::InventoryU, name: "" },
    SlotData { key: 'v', slot: Slot::InventoryV, name: "" },
    SlotData { key: 'w', slot: Slot::InventoryW, name: "" },
    SlotData { key: 'x', slot: Slot::InventoryX, name: "" },
    SlotData { key: 'y', slot: Slot::InventoryY, name: "" },
    SlotData { key: 'z', slot: Slot::InventoryZ, name: "" },
];
