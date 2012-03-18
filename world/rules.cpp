/* rules.cpp

   Copyright (C) 2012 Risto Saarelma

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "rules.hpp"
#include <world/world.hpp>
#include <world/effects.hpp>
#include <world/parts.hpp>
#include <util/num.hpp>

Actor get_player() {
  // TODO: Assert that the actor is registered in World.

  // XXX: Fixed ID is problematic if we want to switch the player entity
  // around.
  return Actor(1);
}

/// Add results of four dice which can give -1, 0 or 1 with equal
/// probabilities. Result is distributed in a crude approximation of a normal
/// distribution.
int fudge_roll() {
  int result = 0;
  for (int i = 0; i < 4; i++)
    result += rand_int(3) - 1;
  return result;
}

bool blocks_shot(Location location) {
  auto kind = terrain_data[get_terrain(location)].kind;
  return kind == wall_terrain || kind == curtain_terrain;
}

bool blocks_sight(Location location) {
  auto kind = terrain_data[get_terrain(location)].kind;
  return kind == wall_terrain || kind == void_terrain || kind == curtain_terrain;
}

bool can_crush(Actor actor, Actor crushee) {
  return actor.as<Blob_Part>().big && !crushee.as<Blob_Part>().big;
}

bool blocks_movement(Actor actor) {
  return actor.has<Blob_Part>();
}

bool action_walk(Actor actor, const Vec2i& dir) {
  auto loc = actor.location();
  auto new_loc = loc + dir;
  if (actor.can_pop(new_loc)) {
    for (auto a : actors_on(actor.footprint(new_loc))) {
      if (a == actor) continue;
      // Uncrushable actors in the target area, abort movement.
      if (blocks_movement(a) && !can_crush(actor, a))
        return false;
    }

    actor.push();

    // XXX Hacky. Player is tracked by the view space object.
    if (actor == get_player())
      move_view_pos(dir);

    for (auto a : actors_on(actor.footprint(new_loc))) {
      if (blocks_movement(a)) {
        // Crushing damages you.
        damage(actor, a.as<Blob_Part>().armor / 2);
        msg("Crush!");
        delete_actor(a);
      }
    }
    actor.pop(new_loc);
    // Energy cost for movement.
    // TODO: account for terrain differences.
    actor.as<Blob_Part>().energy -= 100;
    return true;
  } else {
    return false;
  }
}

bool action_shoot(Actor actor, const Vec2i& dir) {
  ASSERT(is_hex_dir(dir));
  // TODO: Actors have multiple weapons. (The weapon could be the actor though.)
  const int range = 6; // TODO: Actors have different fire ranges.
  int dist = 0;
  Location loc = actor.location();

  for (loc = loc + dir; dist < range; loc = loc + dir) {
    dist++;

    bool hit_actor = false;
    for (auto& a : actors_at(loc)) {
      if (a != actor) {
        hit_actor = true;
        break;
      }
    }

    if (hit_actor) {
      msg("Zap!");
      damage(loc, actor.as<Blob_Part>().damage);
      break;
    }
    if (blocks_shot(loc))
      break;
  }

  beam_fx(actor.location(), dir, dist, Color("pink"));

  auto& blob = actor.as<Blob_Part>();
  // Energy cost for shooting.
  blob.energy -= 100;
}

void damage(Location location, int amount) {
  for (auto a : actors_at(location))
    damage(a, amount);
}

void damage(Actor actor, int amount) {
  if (actor.has<Blob_Part>()) {
    auto& blob = actor.as<Blob_Part>();
    blob.armor -= amount;
    if (blob.armor <= 0) {
      explosion_fx(actor.location());
      delete_actor(actor);
    }
  }
}

bool has_actors(Location location) {
  for (auto a : actors_at(location))
    return true;
  return false;
}

void start_turn_update(Actor actor) {
  try {
    auto& blob = actor.as<Blob_Part>();
    blob.energy += blob.power;
  } catch (Part_Not_Found& e) {}
}

bool ready_to_act(Actor actor) {
  try {
    return actor.as<Blob_Part>().energy >= 0;
  } catch (Part_Not_Found& e) {
    return false;
  }
}