#!/usr/bin/env bash
# PDF version of the figure: pgfplots from fig_data.tsv (lualatex). No Python.
set -eu
cd "$(dirname "$0")"
co() { awk -F'\t' -v p="$1" -v s="$2" '$1==p && $2==s {print $3, $4}' fig_data.tsv | sort -g | awk '{printf "(%s,%s)", ($1==0&&p~/dose/)?0.5:$1, $2}' p="$1"; }
plot() { echo "\\addplot+[mark=*,thick,${4:-}] coordinates {$(co "$1" "$2")}; \\addlegendentry{$3}"; }
{
cat <<'T'
\documentclass{standalone}
\usepackage{pgfplots}\pgfplotsset{compat=1.17}
\begin{document}
\begin{tikzpicture}
\begin{axis}[name=a,title={(a) F4 solving degree $D$ vs $k$},xlabel={$k$ ($|F|=2^k$)},ylabel={$D$},legend pos=north west,legend style={font=\tiny},width=7cm,height=6cm]
T
plot a_k tower-H tower-H "blue,mark size=3.5pt"; plot a_k coset "coset (dashed)" "cyan,dashed"; plot a_k tower-rand tower-rand "red"; plot a_k bits-H bits-H "green!60!black,mark size=3.5pt"; plot a_k bits-rand "bits-rand (dashed)" "orange,dashed"
cat <<'T'
\end{axis}
\begin{axis}[name=b,at={(a.east)},anchor=west,xshift=1.5cm,xmode=log,log basis x=2,title={(b) dose: ops / tower-H ops ($r{=}0.5$ means $H$)},xlabel={$r$ points of $H$ replaced},ylabel={ratio},legend pos=south east,legend style={font=\tiny},width=7cm,height=6cm]
T
for s in k=6 k=7 k=8; do plot c_opsdose $s $s; done
cat <<'T'
\end{axis}
\begin{axis}[name=c,at={(b.east)},anchor=west,xshift=1.5cm,title={(c) coset union: $D$ vs closing degree $j$},xlabel={$j$},ylabel={$D$},legend pos=north west,legend style={font=\tiny},width=7cm,height=6cm]
T
for s in k=6 k=7 k=8; do plot d_cu $s $s; done
cat <<'T'
\addplot[dashed,domain=0:8] {max(5,x)}; \addlegendentry{$\max(5,j)$}
\end{axis}
\end{tikzpicture}
\end{document}
T
} > fig_enc_control.tex
pdflatex -interaction=nonstopmode fig_enc_control.tex > /dev/null
rm -f fig_enc_control.aux fig_enc_control.log
