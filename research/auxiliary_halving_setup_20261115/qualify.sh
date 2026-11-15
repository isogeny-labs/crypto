#!/usr/bin/env bash
# Execute a separately copied frozen version of this qualifier.
set -euo pipefail
if [ "$#" -ne 4 ]; then echo "usage: qualify.sh BUILD PREFIX LEGACY_COLD_ROOT NEW_ROOT" >&2; exit 2; fi
halving_build=$(realpath "$1")
halving_prefix=$(realpath "$2")
halving_legacy=$(realpath "$3")
halving_root=$(realpath -m "$4")
halving_repo=$(git rev-parse --show-toplevel)
halving_study="$halving_repo/research/auxiliary_halving_setup_20261115"
halving_msolve="$halving_prefix/usr/bin/msolve"
mkdir "$halving_root"
export LD_LIBRARY_PATH="$halving_prefix/usr/lib/x86_64-linux-gnu"
ldd "$halving_msolve" > "$halving_root/native-linker.txt"
if rg -q 'not found' "$halving_root/native-linker.txt"; then exit 125; fi
cp "$halving_build/build-source-binary.sha256" "$halving_root/build-source-binary.sha256"
cp "$halving_build/compiler.txt" "$halving_root/compiler.txt"
cp "$halving_build/openssl.txt" "$halving_root/openssl.txt"
cp "$halving_build/identity-build.log" "$halving_root/identity-build.log"
git rev-parse HEAD > "$halving_root/source-commit.txt"
sha256sum "$0" "$halving_study/qualify.sh" "$halving_study/build.sh" \
 "$halving_build/"{halving,cold,replay,legacy-cold,legacy-replay,identity} \
 "$halving_msolve" > "$halving_root/pre-call-binary-script.sha256"
printf '%s\n' "BUILD=$halving_build" "PREFIX=$halving_prefix" "LEGACY=$halving_legacy" \
 "ROOT=$halving_root" "setup-fields=19,23,31,43,59,83,127,251" \
 "setup-seed=1234; collection-seed=5678; max-curve=4096; max-coset=64; max-proposal=128" \
 > "$halving_root/arguments.txt"
run_arm() {
 local halving_name=$1
 shift
 if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$@") \
  > "$halving_root/$halving_name.stdout" 2> "$halving_root/$halving_name.stderr"; then
  halving_exit=0
 else halving_exit=$?
 fi
 printf '%s\n' "$halving_exit" > "$halving_root/$halving_name.exit"
}
run_arm selftest "$halving_build/halving" --selftest
test "$halving_exit" = 0
run_arm identity-issue "$halving_build/identity" --issue "$halving_root/halving-identity.json"
test "$halving_exit" = 0
run_arm identity-check "$halving_build/identity" --check "$halving_root/halving-identity.json"
test "$halving_exit" = 0
run_arm preimages "$halving_build/halving" --census "$halving_root/census"
if [ "$halving_exit" = 0 ]; then
 run_arm preimages-openssl "$halving_build/replay" --census "$halving_root/census/preimages.tsv"
 test "$halving_exit" = 0
fi
for halving_prime in 19 23 31 43 59 83 127 251; do
 run_arm "setup-p$halving_prime" "$halving_build/halving" --setup "$halving_prime" 1234 unfiltered "$halving_root/setup-p$halving_prime"
 halving_setup_exit=$halving_exit
 if [ -s "$halving_root/setup-p$halving_prime/events.txt" ]; then
  run_arm "setup-p$halving_prime-openssl" "$halving_build/replay" --setup "$halving_root/setup-p$halving_prime" "$halving_setup_exit"
  test "$halving_exit" = 0
 fi
done
run_arm legacy-cold "$halving_build/legacy-cold" "$halving_msolve" "$halving_root/legacy-cold"
test "$halving_exit" = 0
run_arm legacy-openssl "$halving_build/legacy-replay" --cold "$halving_root/legacy-cold" "$halving_root/legacy-cold/summary.tsv"
test "$halving_exit" = 0
cmp "$halving_legacy/summary.tsv" "$halving_root/legacy-cold/summary.tsv"
for halving_trace in cold-attempts.tsv cold-relations.tsv cold-kernels.tsv; do
 cmp "$halving_legacy/target-x18-y1/$halving_trace" "$halving_root/legacy-cold/target-x18-y1/$halving_trace"
done
printf '%s\n' 'PASS legacy F19 summary and all3 traces byte-identical to cold006' > "$halving_root/legacy-trace-identity.txt"
run_arm generated-cold "$halving_build/cold" "$halving_msolve" "$halving_root/generated-cold"
if [ "$halving_exit" = 0 ]; then
 run_arm generated-openssl "$halving_build/replay" --cold "$halving_root/generated-cold" "$halving_root/generated-cold/summary.tsv"
 test "$halving_exit" = 0
fi
cat "$halving_root/identity-check.stdout"
if [ -f "$halving_root/preimages-openssl.stdout" ]; then cat "$halving_root/preimages-openssl.stdout"; fi
cat "$halving_root/legacy-trace-identity.txt" "$halving_root/generated-cold.stdout"
if [ -f "$halving_root/generated-openssl.stdout" ]; then cat "$halving_root/generated-openssl.stdout"; fi
