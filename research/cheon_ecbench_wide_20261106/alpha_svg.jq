# Lay out alpha.json as an SVG: one row per (family, weight, arm), the fitted
# exponent of S with its 95 % interval, on a linear axis 0.38 to 0.56. No
# computation beyond placement.
def x: (. - 0.38) / (0.56 - 0.38) * 520 + 250;
def col: if .arm | startswith("rho") then "#2b6cb0" else "#c05621" end;
. as $rows | ($rows|length) as $n | (26 * $n + 100) as $h
| [ "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 820 \($h)' font-family='sans-serif' font-size='11'>",
    "<rect width='820' height='\($h)' fill='white'/>",
    "<text x='410' y='18' text-anchor='middle' font-size='13'>Fitted exponent α of S against r, 95 % interval (11 sizes, 36 to 56 bits, 24 targets)</text>",
    ( [0.40, 0.44, 0.48, 0.50, 0.52, 0.56][] as $t | "<line x1='\($t|x)' y1='32' x2='\($t|x)' y2='\($h-46)' stroke='\(if $t == 0.5 then "#999" else "#ddd" end)'/><text x='\($t|x)' y='\($h-32)' text-anchor='middle'>\($t)</text>" ),
    "<text x='510' y='\($h-14)' text-anchor='middle'>α (rho's generic exponent is 0.5)</text>",
    ( range(0; $n) as $i | $rows[$i] as $e | (44 + 26 * $i + (if $i >= 6 then 12 else 0 end)) as $y
      | "<text x='8' y='\($y+4)'>\($e.family) · w=\($e.w)</text>",
        "<text x='240' y='\($y+4)' text-anchor='end'>\($e.arm)</text>",
        "<line x1='\($e.lo|x)' y1='\($y)' x2='\($e.hi|x)' y2='\($y)' stroke='\($e|col)' stroke-width='2'/>",
        "<circle cx='\($e.alpha|x)' cy='\($y)' r='4' fill='\($e|col)'/>",
        "<text x='\($e.hi|x + 6)' y='\($y+4)' fill='#444'>\($e.alpha*1000|round/1000) [\($e.lo*1000|round/1000), \($e.hi*1000|round/1000)]</text>" ),
    "</svg>" ] | join("\n")
