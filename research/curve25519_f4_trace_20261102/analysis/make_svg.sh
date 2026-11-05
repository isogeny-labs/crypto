#!/usr/bin/env bash
# analysis/trace.svg from f4trace.jsonl (jq + awk): multiply-adds per target,
# log scale, per (t, G): plain F4, trace arm without learning, trace arm
# amortised over the cell's targets (learning included).
set -eu
D=$(cd "$(dirname "$0")" && pwd)
jq -rs '[.[]|select(.trace==true)] as $q | [.[]|select(.trace==false)] as $p
  | $q[] as $c | ($p[]|select(.t==$c.t and .guess==$c.guess)) as $b
  | [$c.t,$c.guess,$b.muladds_per_target,($c.f4_muladds/$c.targets_done),$c.amortised_muladds_per_target]|@tsv' "$D/f4trace.jsonl" |
awk -F'\t' '
BEGIN { W=760; H=420; L=70; R=20; T=40; B=70; ymin=5; ymax=10 }
{ n++; lab[n]="t="$1" G="$2; p[n]=log($3)/log(10); a[n]=log($4)/log(10); m[n]=log($5)/log(10) }
function X(i){ return L + (i-0.5)*(W-L-R)/n }
function Y(v){ return T + (ymax-v)*(H-T-B)/(ymax-ymin) }
END {
  printf "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 %d %d\" font-family=\"sans-serif\" font-size=\"11\">\n", W, H
  printf "<rect width=\"%d\" height=\"%d\" fill=\"white\"/>\n", W, H
  printf "<text x=\"%d\" y=\"20\" font-size=\"13\">F4 multiply-adds per target, isogeny form, k=20, m=2, 100 targets (log scale)</text>\n", L
  for (e=ymin; e<=ymax; e++) {
    printf "<line x1=\"%d\" x2=\"%d\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#ddd\"/><text x=\"%d\" y=\"%.1f\" text-anchor=\"end\">1e%d</text>\n", L, W-R, Y(e), Y(e), L-6, Y(e)+4, e
  }
  for (i=1;i<=n;i++) {
    printf "<line x1=\"%.1f\" x2=\"%.1f\" y1=\"%.1f\" y2=\"%.1f\" stroke=\"#bbb\"/>\n", X(i), X(i), Y(p[i]), Y(m[i])
    printf "<circle cx=\"%.1f\" cy=\"%.1f\" r=\"5\" fill=\"#1b4f9c\"/>\n", X(i), Y(p[i])
    printf "<rect x=\"%.1f\" y=\"%.1f\" width=\"9\" height=\"9\" fill=\"#c2410c\"/>\n", X(i)-4.5, Y(a[i])-4.5
    printf "<path d=\"M%.1f %.1f l5 9 l-10 0 z\" fill=\"#15803d\"/>\n", X(i), Y(m[i])-5
    printf "<text x=\"%.1f\" y=\"%d\" text-anchor=\"middle\">%s</text>\n", X(i), H-B+16, lab[i]
  }
  printf "<circle cx=\"%d\" cy=\"%d\" r=\"5\" fill=\"#1b4f9c\"/><text x=\"%d\" y=\"%d\">plain F4 (trace=0)</text>\n", L+10, H-24, L+20, H-20
  printf "<rect x=\"%d\" y=\"%d\" width=\"9\" height=\"9\" fill=\"#c2410c\"/><text x=\"%d\" y=\"%d\">trace=1, apply + fallback (no learning)</text>\n", L+170, H-28, L+185, H-20
  printf "<path d=\"M%d %d l5 9 l-10 0 z\" fill=\"#15803d\"/><text x=\"%d\" y=\"%d\">trace=1 amortised over the cell (learning included)</text>\n", L+420, H-29, L+432, H-20
  printf "</svg>\n"
}' > "$D/trace.svg"
