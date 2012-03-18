/* terrain_system.hpp

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
#ifndef WORLD_TERRAIN_SYSTEM_HPP
#define WORLD_TERRAIN_SYSTEM_HPP

#include <world/location.hpp>
#include <world/terrain.hpp>
#include <map>

class Terrain_System {
public:
  Terrain_System() {}

  bool contains(Location loc) const;

  /// Create a Location handle object that refers back to this system.
  Location loc(uint16_t area, const Vec2i& pos);

  Terrain get(Location loc) const;
  void set(Location loc, Terrain terrain);
  void clear(Location loc);

  Portal get_portal(Location loc) const;
  void set_portal(Location loc, Portal portal);
  void clear_portal(Location loc);

  bool blocks_movement(Location loc);
  bool blocks_shot(Location loc);
  bool blocks_sight(Location loc);
private:
  Terrain_System(const Terrain_System&);
  Terrain_System& operator=(const Terrain_System&);

  std::map<Location, Terrain> terrain;
  std::map<Location, Portal> portals;
};

#endif