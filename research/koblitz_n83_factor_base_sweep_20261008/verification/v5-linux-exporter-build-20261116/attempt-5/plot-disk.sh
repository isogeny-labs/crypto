#!/usr/bin/env bash
set -euo pipefail
input=${1:?disk.tsv required}
output=${2:?output.svg required}
awk -F '\t' '
  BEGIN { n=0 }
  NR > 1 && NF == 3 { t[n]=$1; v[n]=$2 + 0; n++ }
  END {
    if (n < 2) exit 2
    print "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"920\" height=\"420\" viewBox=\"0 0 920 420\">"
    print "<rect width=\"920\" height=\"420\" fill=\"white\"/>"
    print "<text x=\"64\" y=\"32\" font-family=\"sans-serif\" font-size=\"18\">Boot-volume available disk — N83 V5 attempt 5</text>"
    for (g=5; g<=20; g+=5) {
      y=350 - 14*g
      printf "<line x1=\"70\" y1=\"%d\" x2=\"890\" y2=\"%d\" stroke=\"#e5e7eb\"/>\n", y, y
      printf "<text x=\"10\" y=\"%d\" font-family=\"sans-serif\" font-size=\"12\">%d GiB</text>\n", y+4, g
    }
    print "<line x1=\"70\" y1=\"350\" x2=\"890\" y2=\"350\" stroke=\"#344054\"/>"
    print "<line x1=\"70\" y1=\"70\" x2=\"70\" y2=\"350\" stroke=\"#344054\"/>"
    print "<line x1=\"70\" y1=\"322\" x2=\"890\" y2=\"322\" stroke=\"#b42318\" stroke-dasharray=\"8 5\"/>"
    print "<text x=\"700\" y=\"315\" font-family=\"sans-serif\" font-size=\"12\" fill=\"#b42318\">2 GiB stop floor</text>"
    printf "<polyline fill=\"none\" stroke=\"#2563eb\" stroke-width=\"2\" points=\""
    for (i=0; i<n; i++) {
      x=70 + 820*i/(n-1)
      y=350 - 280*v[i]/20971520
      printf "%.1f,%.1f ", x, y
    }
    print "\"/>"
    printf "<circle cx=\"890\" cy=\"%.1f\" r=\"5\" fill=\"#16a34a\"/>\n", 350-280*v[n-1]/20971520
    printf "<text x=\"70\" y=\"378\" font-family=\"sans-serif\" font-size=\"12\">%s UTC</text>\n", t[0]
    printf "<text x=\"730\" y=\"378\" font-family=\"sans-serif\" font-size=\"12\">%s UTC</text>\n", t[n-1]
    printf "<text x=\"70\" y=\"403\" font-family=\"sans-serif\" font-size=\"12\">All %d ten-second samples; source: disk.tsv; values are KiB.</text>\n", n
    print "</svg>"
  }
' "$input" > "$output"
