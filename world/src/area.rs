use std::rand::StdRng;
use std::rand::SeedableRng;
use serialize::{Decodable, Decoder, Encodable, Encoder};
use std::collections::hashmap::HashMap;
use terrain::TerrainType;
use terrain;
use location::Location;
use mapgen;
use egg::Egg;
use mob::{Player};
use {AreaSpec};

// Note to maintainer: Due to the way serialization works, Area *must* be
// generated to have exactly the same contents every time given the same seed
// value. That means absolutely no outside sources of randomness. Only use an
// internal generator initialized with the given seed value. And watch out for
// surprise randomness. A map generator in a previous project used memory
// addresses of temporary structures as indexing keys, and ended up with a
// nondeterminism bug that depended on the numerical order of the arbitrary
// address values.

/// Immutable procedurally generated terrain initialized on random seed.
pub struct Area {
    seed: u32,
    terrain: HashMap<Location, TerrainType>,
}

impl<E, D:Decoder<E>> Decodable<D, E> for Area {
    fn decode(d: &mut D) -> Result<Area, E> {
        Ok(Area::new(try!(d.read_u32())))
    }
}

impl<E, S:Encoder<E>> Encodable<S, E> for Area {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_u32(self.seed)
    }
}

impl Area {
    pub fn new(seed: u32) -> Area {
        let mut terrain = HashMap::new();
        let mut rng: StdRng = SeedableRng::from_seed([seed as uint].as_slice());
        // TODO: Underground areas.
        mapgen::gen_herringbone(
            &mut rng,
            &AreaSpec::new(::Overland, 1),
            |p, t| {terrain.insert(Location::new(0, 0) + p, t);});

        Area {
            seed: seed,
            terrain: terrain,
        }
    }

    /// Return terrain at given location.
    pub fn terrain(&self, loc: Location) -> TerrainType {
        match self.terrain.find(&loc) {
            Some(&t) => t,
            None => self.default_terrain(loc)
        }
    }

    /// List the objects that should be hatched in the world during init. This
    /// is tied to map generation, so it goes in the area module.
   pub fn get_eggs(&self) -> Vec<(Egg, Location)> {
        // TODO: Properly.
        vec![
            (Egg::new(::MobKind(Player)), Location::new(0, 0)),
        ]
    }

    fn default_terrain(&self, _loc: Location) -> TerrainType {
        // TODO: Different default terrains in different biomes.
        terrain::Rock
    }
}
