#!/usr/bin/env bash
set -euo pipefail
aux_build=$(realpath "${1:?usage: qualify.sh BUILD PC_BUILD MSOLVE NEW_OUTDIR}")
aux_pc=$(realpath "${2:?existing certified PC build}")
aux_msolve=$(realpath "${3:?pinned msolve}")
aux_root=$(realpath -m "${4:?new output root}")
aux_study=$(cd "$(dirname "$0")" && pwd)
aux_native_lib=$(realpath "$(dirname "$aux_msolve")/../lib/x86_64-linux-gnu")
export LD_LIBRARY_PATH="$aux_native_lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
mkdir "$aux_root"
cp "$aux_build/build-source-binary.sha256" "$aux_root/build-source-binary.sha256"
cp "$aux_build/compiler.txt" "$aux_root/compiler.txt"
sha256sum "$aux_build/"{auxiliary,core,replay} "$aux_pc/"{produce,verify} "$aux_msolve" \
 "$aux_study/"*.cpp > "$aux_root/pre-call-source-binary.sha256"
git -C "$aux_study" rev-parse HEAD > "$aux_root/source-commit.txt"
"$aux_build/core" --selftest > "$aux_root/parent-selftest.txt" 2>&1
"$aux_build/auxiliary" --selftest > "$aux_root/auxiliary-selftest.txt" 2>&1
export PC_VERIFIER="$aux_pc/verify" PC_DEGREE_CAP=4
for aux_mode in msolve pc; do
 if [ "$aux_mode" = msolve ]; then aux_backend=$aux_msolve; else aux_backend="$aux_pc/produce"; fi
 if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/auxiliary" --qualify "$aux_backend" "$aux_root/$aux_mode" "$aux_root/$aux_mode.tsv") \
  > "$aux_root/$aux_mode.stdout" 2> "$aux_root/$aux_mode.stderr"; then aux_exit=0; else aux_exit=$?; fi
 printf '%s\n' "$aux_exit" > "$aux_root/$aux_mode.exit"
 test "$aux_exit" = 0
 "$aux_build/replay" "$aux_root/$aux_mode" "$aux_root/$aux_mode.tsv" > "$aux_root/$aux_mode.openssl.txt" 2>&1
done
cat "$aux_root/"*.openssl.txt
