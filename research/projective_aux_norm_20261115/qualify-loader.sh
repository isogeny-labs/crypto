#!/usr/bin/env bash
# A1: exactly one corrected-loader arm; no new PC invocation.
set -euo pipefail
aux_build=$(realpath "${1:?usage: qualify-loader.sh BUILD PREFIX NEW_OUTDIR}")
aux_prefix=$(realpath "${2:?native prefix}")
aux_root=$(realpath -m "${3:?fresh output root}")
aux_study="$(git rev-parse --show-toplevel)/research/projective_aux_norm_20261115"
mkdir "$aux_root"
export LD_LIBRARY_PATH="$aux_prefix/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
aux_msolve="$aux_prefix/usr/bin/msolve"
ldd "$aux_msolve" > "$aux_root/native-linker.txt"
if rg -q 'not found' "$aux_root/native-linker.txt"; then exit 125; fi
sha256sum "$0" "$aux_study/qualify-loader.sh" "$aux_build/"{auxiliary,replay} "$aux_msolve" "$aux_study/"*.cpp \
 "$aux_prefix/usr/lib/x86_64-linux-gnu/libneogb.so.0" > "$aux_root/pre-call-source-binary.sha256"
git -C "$aux_study" rev-parse HEAD > "$aux_root/source-commit.txt"
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/auxiliary" --qualify "$aux_msolve" "$aux_root/models" "$aux_root/summary.tsv") \
 > "$aux_root/stdout.log" 2> "$aux_root/stderr.log"; then aux_exit=0; else aux_exit=$?; fi
printf '%s\n' "$aux_exit" > "$aux_root/exit.txt"
test "$aux_exit" = 0
"$aux_build/replay" "$aux_root/models" "$aux_root/summary.tsv" > "$aux_root/openssl-replay.txt" 2>&1
cat "$aux_root/openssl-replay.txt"
