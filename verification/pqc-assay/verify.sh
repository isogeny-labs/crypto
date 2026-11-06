#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
# shellcheck source=toolchain.env
source "$here/toolchain.env"
cache=${ASSAY_CACHE:-"$here/.cache"}
saw_home=${ASSAY_SAW_HOME:-"$cache/saw-$ASSAY_SAW_VERSION"}
export PATH="$saw_home/bin:$PATH"
rustc --version | grep -F "rustc $ASSAY_RUST_VERSION ("
saw --version 2>&1 | grep -F "$ASSAY_SAW_VERSION ("
test "$(git -C "$cache/upstream" rev-parse HEAD)" = "$ASSAY_UPSTREAM_REV"
git -C "$cache/upstream" diff --exit-code HEAD -- model/cryptol/MLDSA_NTT.cry
cd "$root"
sha256sum --check "$here/sources.sha256"
mkdir -p "$here/build"
cat src/pqc/fast/ml_dsa.rs "$here/exports.rs" > "$here/build/ml_dsa.rs"
rustc --edition=2021 --crate-type=lib --emit=llvm-bc \
    -C opt-level=0 -C overflow-checks=on -C panic=abort \
    "$here/harness.rs" -o "$here/build/native.bc"
cd "$here"
{
    rustc --version --verbose
    saw --version
    printf 'upstream=%s\n' "$ASSAY_UPSTREAM_REV"
    sha256sum sources.sha256 harness.rs exports.rs reduce.saw build/native.bc
    CRYPTOLPATH="$cache/upstream/model/cryptol" saw -v 3 reduce.saw
} 2>&1 | tee build/verify.txt
# `fails` accepts any exception. Require a solver counterexample per mutant,
# rather than accepting a missing symbol, parse error or unsupported operation.
test "$(grep -c '^----------Counterexample----------$' build/verify.txt)" -eq 4
printf '%s\n' 'ASSAY COMPLETE: four positive proofs and four solver-rejected mutants' | tee -a build/verify.txt
