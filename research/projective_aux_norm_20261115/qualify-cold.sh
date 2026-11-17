#!/usr/bin/env bash
# A5: frozen-copy wrapper for the sole auxiliary cold control and default regression.
set -euo pipefail
aux_build=$(realpath "${1:?usage: qualify-cold.sh NEW_BUILD PREFIX SAVED_PARENT_ROOT SAVED_PARENT_TSV NEW_OUTDIR}")
aux_prefix=$(realpath "${2:?native prefix}")
aux_saved=$(realpath "${3:?saved parent cold root}")
aux_summary=$(realpath "${4:?saved parent cold TSV}")
aux_root=$(realpath -m "${5:?fresh output root}")
aux_study="$(git rev-parse --show-toplevel)/research/projective_aux_norm_20261115"
mkdir "$aux_root"
export LD_LIBRARY_PATH="$aux_prefix/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
aux_msolve="$aux_prefix/usr/bin/msolve"
ldd "$aux_msolve" > "$aux_root/native-linker.txt"
if rg -q 'not found' "$aux_root/native-linker.txt"; then exit 125; fi
cp "$aux_build/build-source-binary.sha256" "$aux_root/build-source-binary.sha256"
cp "$aux_build/compiler.txt" "$aux_root/compiler.txt"
cp "$aux_build/openssl.txt" "$aux_root/openssl.txt"
sha256sum "$0" "$aux_study/qualify-cold.sh" "$aux_build/"{cold,core,auxiliary,parent-replay,replay} \
 "$aux_msolve" > "$aux_root/pre-call-binary-script.sha256"
git -C "$aux_study" rev-parse HEAD > "$aux_root/source-commit.txt"
"$aux_build/core" --selftest > "$aux_root/core-selftest.txt" 2>&1
"$aux_build/auxiliary" --selftest > "$aux_root/auxiliary-selftest.txt" 2>&1
"$aux_build/parent-replay" --cold "$aux_saved" "$aux_summary" > "$aux_root/saved-parent-replay.txt" 2>&1
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/core" --qualify-cold "$aux_msolve" "$aux_root/default-regression" "$aux_root/default-regression.tsv") \
 > "$aux_root/default.stdout" 2> "$aux_root/default.stderr"; then aux_exit=0; else aux_exit=$?; fi
printf '%s\n' "$aux_exit" > "$aux_root/default.exit"
test "$aux_exit" = 0
"$aux_build/parent-replay" --cold "$aux_root/default-regression" "$aux_root/default-regression.tsv" > "$aux_root/default.openssl.txt" 2>&1
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$aux_build/cold" "$aux_msolve" "$aux_root/recovery") \
 > "$aux_root/cold.stdout" 2> "$aux_root/cold.stderr"; then aux_exit=0; else aux_exit=$?; fi
printf '%s\n' "$aux_exit" > "$aux_root/cold.exit"
test "$aux_exit" = 0
"$aux_build/replay" --cold "$aux_root/recovery" "$aux_root/recovery/summary.tsv" > "$aux_root/cold.openssl.txt" 2>&1
cat "$aux_root/default.openssl.txt" "$aux_root/cold.stdout" "$aux_root/cold.openssl.txt"
