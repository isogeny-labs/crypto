#!/usr/bin/env bash
# A3: one fixed panel on the new graph-substituted presentation.
set -euo pipefail
aux_build=$(realpath "${1:?usage: qualify-graph.sh NEW_AUX_BUILD PC_BUILD PREFIX NEW_OUTDIR}")
aux_pc=$(realpath "${2:?early-completion PC build}")
aux_prefix=$(realpath "${3:?native prefix}")
aux_root=$(realpath -m "${4:?fresh output root}")
aux_study="$(git rev-parse --show-toplevel)/research/projective_aux_norm_20261115"
mkdir "$aux_root"
export LD_LIBRARY_PATH="$aux_prefix/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export PC_VERIFIER="$aux_pc/verify" PC_DEGREE_CAP=6
aux_msolve="$aux_prefix/usr/bin/msolve"
ldd "$aux_msolve" > "$aux_root/native-linker.txt"
if rg -q 'not found' "$aux_root/native-linker.txt"; then exit 125; fi
sha256sum "$0" "$aux_study/qualify-graph.sh" "$aux_build/"{auxiliary,replay} \
 "$aux_pc/"{produce,verify} "$aux_msolve" "$aux_study/"*.cpp > "$aux_root/pre-call-source-binary.sha256"
git -C "$aux_study" rev-parse HEAD > "$aux_root/source-commit.txt"
"$aux_build/auxiliary" --selftest > "$aux_root/structural-selftest.txt" 2>&1
for aux_mode in msolve pc; do
 if [ "$aux_mode" = msolve ]; then aux_backend=$aux_msolve; else aux_backend="$aux_pc/produce"; fi
 if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/auxiliary" --qualify-eliminated "$aux_backend" "$aux_root/$aux_mode" "$aux_root/$aux_mode.tsv") \
  > "$aux_root/$aux_mode.stdout" 2> "$aux_root/$aux_mode.stderr"; then aux_exit=0; else aux_exit=$?; fi
 printf '%s\n' "$aux_exit" > "$aux_root/$aux_mode.exit"
 test "$aux_exit" = 0
 "$aux_build/replay" "$aux_root/$aux_mode" "$aux_root/$aux_mode.tsv" > "$aux_root/$aux_mode.openssl.txt" 2>&1
done
cat "$aux_root/"*.openssl.txt
