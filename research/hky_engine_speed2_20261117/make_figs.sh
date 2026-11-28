#!/bin/bash
# Emit two SVGs from the measured data: v2/v1 muladd ratio per cell, and the rank-at-exit law.
# Pure shell, no plotting library (none available offline); data is inline from the README tables.
set -u
O="$(dirname "$0")/figures"; mkdir -p "$O"
# Fig 1: v2/v1 muladd ratio (bars; 1.0 = v1 baseline line)
cat > "$O/v2_v1_ratio.svg" <<'EOF'
<svg xmlns="http://www.w3.org/2000/svg" width="640" height="360" font-family="sans-serif" font-size="13">
<rect width="640" height="360" fill="white"/>
<text x="320" y="24" text-anchor="middle" font-size="16">hky3 (v2) / hky2 (v1) multiply-adds, closure where 1 enters</text>
<line x1="70" y1="300" x2="610" y2="300" stroke="#333"/>
<line x1="70" y1="60" x2="70" y2="300" stroke="#333"/>
<!-- y: 0..1.2 mapped to 300..60 (240px = 1.2) -->
<line x1="70" y1="100" x2="610" y2="100" stroke="#c00" stroke-dasharray="4 3"/>
<text x="614" y="104" fill="#c00" font-size="11">v1 = 1.0</text>
<text x="40" y="304" text-anchor="end">0</text>
<text x="40" y="104" text-anchor="end">1.0</text>
EOF
i=0
for d in "(2,9):0.80" "(2,10):0.47" "(2,11):0.60" "(6,2):0.24" "(7,2):0.86" "(5,2):0.55" "(5,3):0.81" "(4,3):0.94"; do
  lab=${d%%:*}; r=${d##*:}
  x=$((90 + i*64)); h=$(awk "BEGIN{print int($r*200)}"); y=$((300 - h))
  col=$(awk "BEGIN{print ($r<0.6)?\"#2a7\":($r<0.9?\"#5080c0\":\"#c84\")}")
  printf '<rect x="%d" y="%d" width="40" height="%d" fill="%s"/><text x="%d" y="316" text-anchor="middle" font-size="11">%s</text><text x="%d" y="%d" text-anchor="middle" font-size="10">%s</text>\n' \
    $x $y $h "$col" $((x+20)) "$lab" $((x+20)) $((y-4)) "$r" >> "$O/v2_v1_ratio.svg"
  i=$((i+1))
done
echo '</svg>' >> "$O/v2_v1_ratio.svg"
# Fig 2: rank-at-exit vs muladds (log y), pod v1 data
cat > "$O/rank_at_exit.svg" <<'EOF'
<svg xmlns="http://www.w3.org/2000/svg" width="640" height="380" font-family="sans-serif" font-size="13">
<rect width="640" height="380" fill="white"/>
<text x="320" y="24" text-anchor="middle" font-size="16">Work is set by rank-at-exit, not column count (v1, 32 vCPU pod)</text>
<line x1="70" y1="320" x2="610" y2="320" stroke="#333"/>
<line x1="70" y1="60" x2="70" y2="320" stroke="#333"/>
<text x="340" y="360" text-anchor="middle">rank fraction at the closure where 1 enters</text>
<text x="20" y="190" text-anchor="middle" transform="rotate(-90 20 190)">muladds (log)</text>
EOF
# points: label frac muladds  -- x: frac 0..1 -> 90..590; y: log10(muladds) 8..13 -> 320..60
for p in "(8,2)c8:0.38:3.76e11:#2a7" "(7,2)c8:0.91:4.86e12:#c00" "(6,2)c7:0.55:5.68e9:#5080c0*v2"; do
  lab=${p%%:*}; rest=${p#*:}; frac=${rest%%:*}; rest=${rest#*:}; mul=${rest%%:*}; col=${rest##*:}
  x=$(awk "BEGIN{print int(90+$frac*500)}"); y=$(awk "BEGIN{l=log($mul)/log(10); print int(320-(l-8)/5*260)}")
  printf '<circle cx="%d" cy="%d" r="6" fill="%s"/><text x="%d" y="%d" font-size="11">%s</text>\n' $x $y "${col%\**}" $((x+9)) $((y+4)) "$lab" >> "$O/rank_at_exit.svg"
done
cat >> "$O/rank_at_exit.svg" <<'EOF'
<text x="90" y="324" font-size="11">0</text><text x="590" y="324" font-size="11">1.0</text>
<text x="300" y="300" font-size="11" fill="#555">(7,2) has 1/3 the columns of (8,2) but 13x the work</text>
</svg>
EOF
echo "wrote $O/v2_v1_ratio.svg $O/rank_at_exit.svg"
