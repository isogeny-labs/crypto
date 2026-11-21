# EXP14b: per-type change-of-order (Singular stdfglm) timing at d = 5; 4 processes, 1 curve x 2 targets each
cd "$(dirname "$0")"
R=/Volumes/SSD990/cryptanalysis/third_party/sage-binary
export DOT_SAGE="/Volumes/SSD990-2/claude-501/dotsage" MPLCONFIGDIR="/Volumes/SSD990-2/claude-501/mpl"
for s in 2501 2502 2503 2504; do
  "$R/sage" exp14_char2_solve.sage 5 1 2 $s 0 1 0 > results/exp14b_fglm_s$s.log 2>&1 &
done
wait
