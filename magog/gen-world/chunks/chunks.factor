! Copyright (C) 2011 Risto Saarelma

USING: dust.asciimap ;

IN: magog.gen-world.chunks

CONSTANT: chunk-dim { 5 5 }

! XXX There needs to be an empty line after each ASCIIMAP: definition due to a
! bug in the multiline parsing code. It should be fixed in trunk.

CONSTANT: water-chunks {
ASCIIMAP:
??????
?~~~~~?
?~~~~~?
?~~~~~?
?~~~~~?
?~~~~~?
 ??????
;

}

CONSTANT: water-ground-chunks {
ASCIIMAP:
111111
~.....1
~~....1
~~~...1
~~~~..1
~~~~~.1
 ~~~~~1
;

ASCIIMAP:
1~~~~~
1~~~~~~
1.~~~~~
1..~~~~
1...~~~
1....~~
 111111
;

ASCIIMAP:
??????
?.....?
?.....?
?.....?
?.....?
?.....?
 ??????
;

}

CONSTANT: natural-chunks {
ASCIIMAP:
111111
1.....1
1.....1
1.....1
1.....1
1.....1
 111111
;

ASCIIMAP:
111111
1.....1
1..##.1
1.#...1
1...#.1
1.....1
 111111
;

ASCIIMAP:
111111
1.....1
1.##..1
1.###.1
1..##.1
1.....1
 111111
;

ASCIIMAP:
011111
0##...1
0#....1
0#....1
0##...1
0#....1
 111111
;

ASCIIMAP:
000000
1#####1
1..###1
1.....1
1.....1
1.....1
 111111
;

ASCIIMAP:
111111
1....#0
1...##0
1...##0
1....#0
1....#0
 111110
;

ASCIIMAP:
111111
1.....1
1.....1
1.....1
1#..#.1
1#####1
 000000
;

}

CONSTANT: stairwell-chunks {
ASCIIMAP:
111111
1.###.1
1.#z#.1
1.#A#.1
1.....1
1.....1
 111111
;

ASCIIMAP:
111111
1.....1
1.#.#.1
1.#y#.1
1.###.1
1.....1
 111111
;

}

CONSTANT: fallback-chunk
ASCIIMAP:
000000
0#####0
0#####0
0#####0
0#####0
0#####0
 000000
;


CONSTANT: edge-chunk
ASCIIMAP:
******
*******
*******
*******
*******
*******
 ******
;
