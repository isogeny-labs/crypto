# EXP14 runs: d=5 and d=6 (brute force on), then d=7 (brute force off)
cd "$(dirname "$0")"
R=/Volumes/SSD990/cryptanalysis/third_party/sage-binary
export DOT_SAGE="/Volumes/SSD990-2/claude-501/dotsage" MPLCONFIGDIR="/Volumes/SSD990-2/claude-501/mpl"
"$R/sage" exp14_char2_solve.sage 5 3 3 1402 1 > results/exp14_d5.log 2>&1
"$R/sage" exp14_char2_solve.sage 6 3 3 1602 1 > results/exp14_d6.log 2>&1
"$R/sage" exp14_char2_solve.sage 7 2 3 1702 0 > results/exp14_d7.log 2>&1
