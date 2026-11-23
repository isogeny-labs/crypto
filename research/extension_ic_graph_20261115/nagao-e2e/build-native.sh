#!/usr/bin/env bash
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: build-native.sh FROZEN_LIBRARY_ROOT TARGET_DIR NEW_OUTPUT_DIR' >&2; exit 2; }
root=$(cd "$1" && pwd)
target=$(cd "$2" && pwd)
[[ $(git -C "$root" rev-parse HEAD) == fcda1e22cb0c5424ca3244f783e69be8b99792f4 ]] || exit 2
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
mkdir "$3"
out=$(cd "$3" && pwd)
source /home/agent/.cargo/env
for name in jv_cover jv_extension_remainder_dlp jv_sieve_two_large_prime; do cmp "$here/library-overlay/$name.rs" "$root/src/cryptanalysis/$name.rs"; done
args=(--edition=2021 -C opt-level=3)
# Cargo's artifact JSON, not a newest-file guess, chooses the linked library.
lib=$(python3 -c 'import json,sys; rows=[json.loads(l) for l in open(sys.argv[1])]; a=[f for r in rows if r.get("reason")=="compiler-artifact" and r.get("target",{}).get("name")=="crypto_lib" for f in r["filenames"] if f.endswith(".rlib")]; assert len(a)==1,a; print(a[0])' /tmp/nagao-e2e-library-build-2.jsonl)
args+=(--extern "crypto_lib=$lib")
for dep in rand rayon serde serde_json sha2 clap blake3; do
 mapfile -t matches < <(find "$target/release/build/$dep" -path "*/out/lib${dep}-*.rlib")
 if [[ $dep == sha2 ]]; then
   mapfile -t matches < <(python3 -c 'import json,sys; rows=[json.loads(l) for l in open(sys.argv[1])]; print("\n".join(f for r in rows if r.get("reason")=="compiler-artifact" and r.get("package_id","").endswith("#sha2@0.10.9") for f in r["filenames"] if f.endswith(".rlib")))' /tmp/nagao-e2e-library-build-2.jsonl)
 fi
 [[ ${#matches[@]} == 1 ]] || { echo "Ambiguous dependency $dep" >&2; exit 2; }
 args+=(--extern "$dep=${matches[0]}")
done
while IFS= read -r p; do args+=(-L "dependency=$p" -L "native=$p"); done < <(find "$target/release/build" -type d -name out)
rustc -Vv > "$out/compiler.txt"
printf '%s\n' "${args[@]}" > "$out/rustc-args.txt"
sha256sum "$lib" > "$out/library.sha256"
rustc "${args[@]}" "$here/native/worker.rs" -o "$out/worker"
rustc "${args[@]}" --test "$here/native/worker.rs" -o "$out/worker-tests"
rustc "${args[@]}" "$here/native/run.rs" -o "$out/run"
rustc "${args[@]}" --test "$here/native/run.rs" -o "$out/qualifier-tests"
rustc "${args[@]}" "$here/native/analyze.rs" -o "$out/analyze"
rustc "${args[@]}" "$here/native/prepare.rs" -o "$out/prepare"
sha256sum "$out/worker" "$out/worker-tests" "$out/run" "$out/qualifier-tests" "$out/analyze" "$out/prepare" > "$out/binaries.sha256"
