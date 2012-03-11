/* effects.hpp

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
#ifndef WORLD_EFFECTS_HPP
#define WORLD_EFFECTS_HPP

#include <world/location.hpp>

// These are implemented on the UI module.

void beam_fx(const Location& location, const Vec2i& dir, int length, const Color& color);

void raw_msg(std::string string);

#endif
