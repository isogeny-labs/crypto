#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
awk -F, '
BEGIN {
 print "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"760\" height=\"350\" viewBox=\"0 0 760 350\">"
 print "<rect width=\"760\" height=\"350\" fill=\"#fff\"/>"
 print "<text x=\"24\" y=\"30\" font-family=\"sans-serif\" font-size=\"20\">R91 pole-clearing powers, 32 generators per direction</text>"
 print "<rect x=\"520\" y=\"48\" width=\"12\" height=\"8\" fill=\"#3575ad\"/><text x=\"540\" y=\"56\" font-family=\"sans-serif\" font-size=\"12\">forward</text>"
 print "<rect x=\"620\" y=\"48\" width=\"12\" height=\"8\" fill=\"#d68034\"/><text x=\"640\" y=\"56\" font-family=\"sans-serif\" font-size=\"12\">reverse</text>"
}
NR>1 {
 y=65+(NR-2)*23
 printf "<text x=\"30\" y=\"%d\" font-family=\"sans-serif\" font-size=\"13\">power %s</text>\n",y+14,$1
 printf "<rect x=\"130\" y=\"%d\" width=\"%d\" height=\"8\" fill=\"#3575ad\"/>\n",y,$2*30
 printf "<rect x=\"130\" y=\"%d\" width=\"%d\" height=\"8\" fill=\"#d68034\"/>\n",y+9,$3*30
 printf "<text x=\"%d\" y=\"%d\" font-family=\"sans-serif\" font-size=\"11\">%s / %s</text>\n",140+$2*30,y+15,$2,$3
 nf+=$2;nr+=$3;rows++
}
END {
 if(rows!=11 || nf!=32 || nr!=32)exit 1
 print "<text x=\"24\" y=\"336\" font-family=\"sans-serif\" font-size=\"12\">Exact R91 column counts; one run, no timing metric</text>"
 print "</svg>"
}' "$here/power-histogram.csv" > "$here/power-histogram.svg"
rsvg-convert -f pdf -o "$here/power-histogram.pdf" "$here/power-histogram.svg"
