#!/usr/bin/env bash
# Change a generated copy of production caddq, then require a counterexample.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
# shellcheck source=toolchain.env
source "$here/toolchain.env"
cache=${ASSAY_CACHE:-"$here/.cache"}
saw_home=${ASSAY_SAW_HOME:-"$cache/saw-$ASSAY_SAW_VERSION"}
export PATH="$saw_home/bin:$PATH"
cd "$here"
test -f build/native.bc
cp build/ml_dsa.rs build/ml_dsa.saved.rs
trap 'cp build/ml_dsa.saved.rs build/ml_dsa.rs' EXIT
sed 's/a + ((a >> 31) \& Q)/a - ((a >> 31) \& Q)/' \
    "$root/src/pqc/fast/ml_dsa.rs" > build/ml_dsa.rs
cat exports.rs >> build/ml_dsa.rs
if cmp -s build/ml_dsa.rs build/ml_dsa.saved.rs; then
    echo 'Mutation did not change the input' >&2; exit 1
fi
rustc --edition=2021 --crate-type=lib --emit=llvm-bc \
    -C opt-level=0 -C overflow-checks=on -C panic=abort \
    harness.rs -o build/mutant.bc
sed "s@build/native.bc@$here/build/mutant.bc@" reduce.saw > build/mutant.saw
if CRYPTOLPATH="$cache/upstream/model/cryptol" saw -v 3 build/mutant.saw > build/mutation-caddq.txt 2>&1; then
    echo 'Wrong production caddq passed' >&2; exit 1
fi
# First two are the preceding functions' spec controls; the third is caddq.
test "$(grep -c '^----------Counterexample----------$' build/mutation-caddq.txt)" -eq 3
printf '%s\n' 'SOURCE MUTANT REJECTED: caddq subtracting q instead of adding it'
