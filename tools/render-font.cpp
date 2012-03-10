/* render-font.cpp

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

#include "load_fonts.cpp"
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include <contrib/stb/stb_image_write.h>

int main(int argc, char* argv[]) {
  Font_Data data = load_fonts(argc, argv);

  int result = stbi_write_png(argv[5], data.width, data.height, 1, data.pixels.data(), 0);
  if (!result)
    return 1;
  return 0;
}
