# EXP16e: deciding FGLM run, reduced affine-plane A vs reduced D0, same process
cd "$(dirname "$0")"
R=/Volumes/SSD990/cryptanalysis/third_party/sage-binary
export DOT_SAGE="/Volumes/SSD990-2/claude-501/dotsage" MPLCONFIGDIR="/Volumes/SSD990-2/claude-501/mpl"
for s in 23 31; do "$R/sage" exp16e_fglm_AD.sage $s 0 > results/exp16e_fglm_s$s.log 2>&1 & done
wait
