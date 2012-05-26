/* fonter_system.hpp

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
#ifndef UTIL_FONTER_SYSTEM_HPP
#define UTIL_FONTER_SYSTEM_HPP

#include <util/gl_texture.hpp>
#include <util/file_system.hpp>
#include <util/format.hpp>
#include <util/vec.hpp>
#include <vector>
#include <string>

class Fonter_System {
public:
  struct Font_Data {
    int x0, y0, x1, y1; ///< The rectangle points on the font texture
    float x_off, y_off; ///< Rendering offsets
    float char_width;
  };

  enum Align {
    LEFT,
    CENTER,
    RIGHT
  };

  Fonter_System(
    File_System& file,
    const char* ttf_file,
    int font_height,
    int first_char = 32,
    int num_chars = 96);

  int width(const char* text);
  int height() { return font_height; }

  template<typename... Args>
  int draw(const Vec2f& pos, const char* fmt, Args... args) {
    auto str = format(fmt, args...);
    return raw_draw(pos, LEFT, str.c_str());
  }

  template<typename... Args>
  int draw(const Vec2f& pos, Align align, const char* fmt, Args... args) {
    auto str = format(fmt, args...);
    return raw_draw(pos, align, str.c_str());
  }

private:
  Fonter_System(const Fonter_System&);
  Fonter_System& operator=(const Fonter_System&);

  int raw_draw(Vec2f pos, char ch);
  int raw_draw(const Vec2f& pos, Align align, const char* text);

  void load_font(const char* filename, int height, int first, int num);

  File_System& file;

  Gl_Texture font_texture;
  std::vector<Font_Data> font_data;
  int font_height;
  int first_char;
};

#endif