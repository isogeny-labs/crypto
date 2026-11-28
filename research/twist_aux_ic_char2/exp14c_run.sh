# EXP14c: why no system is in shape position (lex GB structure), d = 5, seed 2501 (same systems as exp14b_fglm_s2501 r0)
cd "$(dirname "$0")"
R=/Volumes/SSD990/cryptanalysis/third_party/sage-binary
export DOT_SAGE="/Volumes/SSD990-2/claude-501/dotsage" MPLCONFIGDIR="/Volumes/SSD990-2/claude-501/mpl"
"$R/sage" exp14_char2_solve.sage 5 1 1 2501 0 1 0 > results/exp14c_shape.log 2>&1
