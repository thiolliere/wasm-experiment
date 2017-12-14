// TODO: synchronize, not synchronize.
//
// synchronize animation have a loop duration that is multiple of tempo (eg 0.5 tempo or 4 tempo)
// then its image is computed from percentage of tempo and number of frame
//
// not sync have a framerate

use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/animations.rs"));
const FRAME_DURATION: f64 = 0.2;

lazy_static! {
    static ref ANIMATION_MAP: HashMap<(Entity, State), Vec<u32>> = {
        use self::Entity::*;
        use self::State::*;

        let mut map = HashMap::new();
        macro_rules! animation {($e:ident, $s:ident => $i:ident) => { map.insert(($e, $s), $i.iter().cloned().collect()); }}
        macro_rules! transition {($e:ident, $s:ident => $i:ident) => { map.insert(($e, $s), $i.iter().cloned().rev().collect()); }}

        animation!(Character, Walking => ESSAI);
        transition!(Character, Running => TRUC);

        map
    };
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Entity {
    Character,
    Monster,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum State {
    Walking,
    Running,
}

pub struct Animated {
    entity: Entity,
    tile_index: usize,
    tiles: Vec<u32>,
    transition_tiles: Vec<u32>,
    timer: f64,
}

impl Animated {
    pub fn new(entity: Entity, state: State) -> Self {
        let mut animated = Animated {
            entity,
            tile_index: 0,
            tiles: vec!(),
            transition_tiles: vec!(),
            timer: 0.0,
        };
        animated.set_state(state);
        animated
    }

    pub fn update(&mut self, dt: f64) {
        self.timer += dt;
        while self.timer > FRAME_DURATION {
            self.timer -= FRAME_DURATION;

            if self.transition_tiles.pop().is_some() {
                continue;
            }
            self.tile_index += 1;
            self.tile_index %= self.tiles.len();
        }
    }

    pub fn tile(&self) -> u32 {
        self.transition_tiles.last().cloned().unwrap_or_else(|| {
            self.tiles[self.tile_index].clone()
        })
    }

    pub fn set_state(&mut self, state: State) {
        let desc = ANIMATION_MAP.get(&(self.entity, state)).unwrap();

        self.tiles.clear();
        self.tiles.extend_from_slice(&desc);

        self.tile_index = 0;
    }

    pub fn set_transition(&mut self, state: State) {
        let desc = ANIMATION_MAP.get(&(self.entity, state)).unwrap();

        self.transition_tiles.clear();
        self.transition_tiles.extend_from_slice(&desc);
    }
}
