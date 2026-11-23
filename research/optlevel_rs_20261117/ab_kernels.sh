#!/usr/bin/env bash
# C-versus-Rust A/B of the optlevel calibration kernels on this host.
# Every variant of a kernel runs once per round, in a fixed interleaved order,
# pinned to one CPU; the 1-minute load average is recorded at each round.
# env: NO_TEP=1 skips the timer-every-pass variants; STREAM_KERNELS and NO_CHASE select the above-cache set.
# usage: ab_kernels.sh OPTLEVEL_BIN C_BUILD_DIR OUT.tsv [ROUNDS [SECONDS [BIG_N]]]
#   C_BUILD_DIR holds optk-native, optk-w512, optk-scalar built from kernels/optk.c with
#   cc -O3 -march=native -ffp-contract=fast [-mprefer-vector-width=512 | -fno-tree-vectorize -fno-tree-slp-vectorize]
set -euo pipefail
BIN=$1; C=$2; OUT=$3; ROUNDS=${4:-7}; SEC=${5:-0.3}; BIG=${6:-0}
CPU=$(taskset -cp $$ | sed 's/.*[:, -]//')
rate() { jq -r '[.ops/.kernel_seconds, (.bytes_declared/.kernel_seconds)] | @tsv'; }
echo -e "kernel\tn\tround\tvariant\tops_per_s\tbytes_per_s\tloadavg1" > "$OUT"
run_set() { # kernel n
  local k=$1 n=$2 extra=(); [ "$n" != 0 ] && extra=(--n "$n")
  for r in $(seq 1 "$ROUNDS"); do
    la=$(cut -d' ' -f1 /proc/loadavg)
    for b in native w512 scalar; do
      [ "$k" = fma_f64_reg ] && [ "$b" = scalar ] && continue
      echo -e "$k\t$n\t$r\tc-$b\t$(taskset -c "$CPU" "$C/optk-$b" "$k" "${extra[@]}" --seconds "$SEC" | rate)\t$la" >> "$OUT"
    done
    for i in $("$BIN" kernel 2>&1 | sed -n 's/.*--isa \([a-z0-9|]*\).*/\1/p' | tr '|' ' '); do
      [ "$k" = fma_f64_reg ] && [ "$i" = scalar ] && continue
      echo -e "$k\t$n\t$r\trs-$i\t$(taskset -c "$CPU" "$BIN" kernel "$k" --isa "$i" "${extra[@]}" --seconds "$SEC" | rate)\t$la" >> "$OUT"
      [ -n "${NO_TEP:-}" ] && continue   # arrays above the cache: one pass outlasts the timer interval, the two modes are the same code path
      echo -e "$k\t$n\t$r\trs-$i-timer-every-pass\t$(taskset -c "$CPU" "$BIN" kernel "$k" --isa "$i" "${extra[@]}" --seconds "$SEC" --timer-every-pass | rate)\t$la" >> "$OUT"
    done
  done
}
if [ "$BIG" = 0 ]; then
  for k in muladd_u32 muladd_u64 mulwide_u64 modmuladd_p31 modmuladd_p62 horner_p31 xor_u64 popcnt_u64 fma_f64 fma_f64_reg chase clock; do run_set $k 0; done
else
  for k in ${STREAM_KERNELS:-triad xor_u64 popcnt_u64 fma_f64}; do run_set $k "$BIG"; done
  [ -n "${NO_CHASE:-}" ] || run_set chase $((BIG / 8))
fi
