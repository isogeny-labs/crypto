# Lay out ranks.json (recorded frontier values) as an SVG: one row per ranked
# bound, ops (x floor) on a log axis with its 95 % interval. No computation
# beyond placement.
def x: (log10 - (0.3|log10)) / ((30|log10) - (0.3|log10)) * 520 + 300;
def r(v): v * 10 | round / 10;
def col: if .method | startswith("rho") then "#2b6cb0" elif .method | test("naive") then "#a0a0a0" else "#c05621" end;
[.[] | . as $d | .entries[] | . + {oracle: $d.oracle, unit: $d.unit}] as $rows
| ($rows | length) as $n
| (24 * $n + 90) as $h
| [ "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 870 \($h)' font-family='sans-serif' font-size='11'>",
    "<rect width='870' height='\($h)' fill='white'/>",
    "<text x='435' y='18' text-anchor='middle' font-size='13'>Cheon and rho on the medium-tier sessions: ops (× floor), 95 % interval</text>",
    ( [0.3, 1, 3, 10, 30][] as $t | "<line x1='\($t|x)' y1='30' x2='\($t|x)' y2='\($h-40)' stroke='#ddd'/><text x='\($t|x)' y='\($h-26)' text-anchor='middle'>\($t)</text>" ),
    "<text x='560' y='\($h-10)' text-anchor='middle'>ops = S / S_floor (log scale); lower is cheaper</text>",
    ( range(0; $n) as $i | $rows[$i] as $e | (40 + 24 * $i) as $y
      | "<text x='8' y='\($y+4)'>\($e.oracle[10:13]) · \($e.unit | sub("ecbench.gae\\+band"; "band") | sub("ecbench.gae"; "w=0"))</text>",
        "<text x='295' y='\($y+4)' text-anchor='end'>\($e.method)\(if $e.is_frontier then "" else " (dominated)" end)</text>",
        "<line x1='\($e.lo|x)' y1='\($y)' x2='\($e.hi|x)' y2='\($y)' stroke='\($e|col)' stroke-width='2'/>",
        "<circle cx='\($e.ops|x)' cy='\($y)' r='4' fill='\(if $e.is_frontier then ($e|col) else "white" end)' stroke='\($e|col)' stroke-width='1.5'/>",
        "<text x='\($e.hi|x + 6)' y='\($y+4)' fill='#444'>\(r($e.ops))</text>" ),
    "</svg>" ] | join("\n")
