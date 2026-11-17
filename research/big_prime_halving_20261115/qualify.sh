#!/usr/bin/env bash
# Copy this wrapper to an immutable separate path before invoking it.
set -euo pipefail
if [ "$#" -ne 3 ]; then echo 'usage: qualify.sh BUILD NEW_ROUND PRESERVED_OLD_ROUND' >&2; exit 2; fi
big_repo=$(git rev-parse --show-toplevel)
big_study="$big_repo/research/big_prime_halving_20261115"
big_build=$(realpath "$1")
big_round=$(realpath -m "$2")
big_old=$(realpath "$3")
mkdir "$big_round"
cp "$big_build/"{build-source-binary.sha256,compiler.txt,openssl.txt,candidate-linker.txt,reference-linker.txt} "$big_round/"
git rev-parse HEAD > "$big_round/source-commit.txt"
sha256sum "$0" "$big_study/qualify.sh" "$big_study/"*.cpp "$big_study/big.hpp" \
 "$big_build/"{setup,replay,mutate} > "$big_round/pre-call-source-wrapper-binary.sha256"
sha256sum -c "$big_build/build-source-binary.sha256" > "$big_round/build-binding.txt"
sha256sum /lib/x86_64-linux-gnu/lib{gmpxx.so.4,gmp.so.10,crypto.so.3} \
 /usr/include/gmpxx.h /usr/include/x86_64-linux-gnu/gmp.h \
 /usr/include/openssl/{bn.h,ec.h,opensslv.h} > "$big_round/system-library-header.sha256"
dpkg-query -W libgmp10 libgmpxx4ldbl libssl3 openssl > "$big_round/system-packages.txt"
printf '%s\n' 'qualification001; one thread; 2GiB; census120s; other30s; outer1500s' \
 "BUILD=$big_build" "OLD=$big_old" 'all seeds1234; fixed18fields/depths; frozen caps' > "$big_round/arguments.txt"
run_arm(){
 local name=$1 seconds=$2; shift 2
 if (ulimit -v 2097152; ulimit -t "$seconds"; export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1; exec timeout --signal=KILL "$seconds" "$@") \
  > "$big_round/$name.stdout" 2> "$big_round/$name.stderr"; then big_exit=0; else big_exit=$?; fi
 printf '%s\n' "$big_exit" > "$big_round/$name.exit"
}
must_pass(){ if [ "$big_exit" -ne 0 ]; then echo "STOP unexpected failure: $1 exit=$big_exit" >&2; exit 2; fi; }
run_arm admissions 30 "$big_build/setup" --selftest
must_pass admissions
run_arm census 120 "$big_build/setup" --census "$big_round/preimages.txt"
must_pass census
run_arm census-reference 120 "$big_build/replay" --census "$big_round/preimages.txt"
must_pass census-reference
run_arm fixtures 30 "$big_build/setup" --fixtures
must_pass fixtures
big_case=0
while read -r big_prime big_depth big_seed; do
 big_case=$((big_case+1))
 big_name="setup-$(printf '%02d' "$big_case")"
 run_arm "$big_name" 30 "$big_build/setup" --setup "$big_prime" "$big_depth" "$big_seed" "$big_round/$big_name"
 if [ "$big_exit" -eq 0 ]; then
  run_arm "$big_name-reference" 30 "$big_build/replay" --setup "$big_round/$big_name"
  must_pass "$big_name-reference"
  if [ "$big_case" -le 8 ]; then
   run_arm "$big_name-legacy" 30 "$big_build/replay" --legacy "$big_round/$big_name" "$big_old/setup-p$big_prime"
   must_pass "$big_name-legacy"
  fi
 elif [ "$big_exit" -ne 2 ] && [ "$big_exit" -ne 124 ] && [ "$big_exit" -ne 137 ] && [ "$big_exit" -ne 152 ]; then
  echo "STOP unexpected producer status $big_exit" >&2; exit 2
 fi
done < "$big_round/fixtures.stdout"
if [ "$big_case" -ne 18 ]; then echo 'STOP fixture count' >&2; exit 2; fi
if [ "$(cat "$big_round/setup-01.exit")" -ne 0 ]; then echo 'STOP missing positive adverse source' >&2; exit 2; fi
for big_kind in root curve numerator rng; do
 run_arm "mutate-$big_kind" 30 "$big_build/mutate" "$big_round/setup-01" "$big_round/adverse-$big_kind" "$big_kind"
 must_pass "mutate-$big_kind"
 run_arm "adverse-$big_kind" 30 "$big_build/replay" --setup "$big_round/adverse-$big_kind"
 if [ "$big_exit" -ne 2 ]; then echo "STOP adverse $big_kind status $big_exit" >&2; exit 2; fi
 case "$big_kind" in root) big_reason='root completeness';; curve) big_reason='global curve identity';; numerator) big_reason='composed numerator binding';; rng) big_reason='RNG mismatch';; esac
 rg -F "$big_reason" "$big_round/adverse-$big_kind.stderr" > "$big_round/adverse-$big_kind-reason.txt"
done
sha256sum -c "$big_round/pre-call-source-wrapper-binary.sha256" > "$big_round/post-call-source-wrapper-binary.txt"
sha256sum -c "$big_build/build-source-binary.sha256" > "$big_round/post-call-build-binding.txt"
echo 'COMPLETE qualification001; inspect all retained statuses before interpretation'
