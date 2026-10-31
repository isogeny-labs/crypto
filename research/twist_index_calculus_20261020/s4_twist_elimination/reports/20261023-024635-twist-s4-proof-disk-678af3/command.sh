#!/usr/bin/env bash
# Run by the repository ephemeral CLI with the two public source inputs.
set -euo pipefail
proof_work=/work/twist-s4-proof
mkdir -p "$proof_work"
printf '%s' "$S4_INTEGER_SOURCE_B64" | base64 -d > "$EPHEMERAL_OUT/exact.rs"
printf '%s' "$S4_AUDIT_SOURCE_B64" | base64 -d > "$EPHEMERAL_OUT/AuditAxioms.lean"
sha256sum "$EPHEMERAL_OUT/exact.rs" "$EPHEMERAL_OUT/AuditAxioms.lean" > "$EPHEMERAL_OUT/source.sha256"
rustc --edition 2021 -O "$EPHEMERAL_OUT/exact.rs" -o "$proof_work/exact" > "$EPHEMERAL_OUT/build.log" 2>&1
"$proof_work/exact" --lean "$EPHEMERAL_OUT/TwistS4Identity.lean" > "$EPHEMERAL_OUT/structure.json" 2> "$EPHEMERAL_OUT/derive.stderr"
curl --fail --location --retry 2 https://github.com/leanprover/lean4/releases/download/v4.34.1/lean-4.34.1-linux.tar.zst -o "$proof_work/lean.tar.zst" > "$EPHEMERAL_OUT/lean-download.log" 2>&1
tar --zstd -xf "$proof_work/lean.tar.zst" -C "$proof_work" > "$EPHEMERAL_OUT/lean-extract.log" 2>&1
proof_lean="$proof_work/lean-4.34.1-linux/bin/lean"
test -x "$proof_lean"
"$proof_lean" --version > "$EPHEMERAL_OUT/lean-version.txt"
test -s "$EPHEMERAL_OUT/lean-version.txt"
grep -q 'version 4.34.1' "$EPHEMERAL_OUT/lean-version.txt"
cd "$EPHEMERAL_OUT"
prlimit --as=8589934592 -- timeout 1200s "$proof_lean" -o TwistS4Identity.olean TwistS4Identity.lean > lean-build.log 2>&1
test -s TwistS4Identity.olean
test -s lean-build.log
LEAN_PATH="$EPHEMERAL_OUT" timeout 1200s "$proof_lean" --run AuditAxioms.lean -- TwistS4Identity > axiom-audit.log 2>&1
test -s axiom-audit.log
grep -q '^ok    TwistS4Identity.symmetrised_resultant ' axiom-audit.log
grep -q '^ok    TwistS4Identity.fully_symmetrised_resultant ' axiom-audit.log
grep -q '^audited 2 theorems .*; 0 failing, 0 trusted ' axiom-audit.log
sha256sum TwistS4Identity.lean TwistS4Identity.olean > theorem.sha256
printf 'checked two theorem artifacts and the axiom audit\n' > proof-complete.txt
