#!/usr/bin/env bash
set -euo pipefail
proof_work=/work/twist-image-proof
mkdir -p "$proof_work"
printf '%s' "$IMAGE_PROOF_B64" | base64 -d > "$EPHEMERAL_OUT/ImageCoordinate.lean"
printf '%s' "$IMAGE_AUDIT_B64" | base64 -d > "$EPHEMERAL_OUT/AuditAxioms.lean"
printf '%s  %s\n' "$IMAGE_PROOF_SHA" "$EPHEMERAL_OUT/ImageCoordinate.lean" | sha256sum -c - > "$EPHEMERAL_OUT/source-check.log"
curl --fail --location --retry 2 https://github.com/leanprover/lean4/releases/download/v4.34.1/lean-4.34.1-linux.tar.zst -o "$proof_work/lean.tar.zst" > "$EPHEMERAL_OUT/download.log" 2>&1
tar --zstd -xf "$proof_work/lean.tar.zst" -C "$proof_work" > "$EPHEMERAL_OUT/extract.log" 2>&1
export PATH="$proof_work/lean-4.34.1-linux/bin:$PATH"
lean --version > "$EPHEMERAL_OUT/lean-version.txt"
test -s "$EPHEMERAL_OUT/lean-version.txt"
grep -q 'version 4.34.1' "$EPHEMERAL_OUT/lean-version.txt"
cd "$EPHEMERAL_OUT"
prlimit --as=8589934592 -- timeout 1200s lean -o ImageCoordinate.olean ImageCoordinate.lean > proof.log 2>&1
test -s ImageCoordinate.olean
test -s proof.log
LEAN_PATH="$EPHEMERAL_OUT" timeout 1200s lean --run AuditAxioms.lean -- ImageCoordinate > axiom-audit.log 2>&1
test -s axiom-audit.log
grep -q '^ok    ImageCoordinate.coordinate ' axiom-audit.log
grep -q '^audited 1 theorems .*; 0 failing, 0 trusted ' axiom-audit.log
printf 'import ImageCoordinate\nexample : (0 : Int) = 1 := by decide\n' > NegativeControl.lean
set +e
LEAN_PATH="$EPHEMERAL_OUT" timeout 120s lean NegativeControl.lean > negative.log 2>&1
proof_negative=$?
set -e
printf '%s\n' "$proof_negative" > negative.exit
test "$proof_negative" -ne 0
test -s negative.log
sha256sum ImageCoordinate.lean ImageCoordinate.olean > theorem.sha256
printf 'one theorem compiled and audited; false-equality proof rejected\n' > proof-complete.txt
