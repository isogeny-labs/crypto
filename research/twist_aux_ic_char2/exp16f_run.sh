# EXP16f: D doubled-point count vs Q (Q=64 more targets, Q=128), classify off. Sage cache/temp on SSD990 (SSD990-2 full).
cd "$(dirname "$0")"
R=/Volumes/SSD990/cryptanalysis/third_party/sage-binary
A="$(cd .. && pwd)/.sage_alt"
export DOT_SAGE="$A/dotsage" MPLCONFIGDIR="$A/mpl" TMPDIR="$A/tmp"
"$R/sage" exp16a_doubled2.sage 6 D,P1 2 2 2611 0 > results/exp16f_q64.log 2>&1 &
"$R/sage" exp16a_doubled2.sage 7 D,P1 2 2 2711 0 > results/exp16f_q128.log 2>&1 &
wait
