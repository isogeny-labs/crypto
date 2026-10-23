set +e
printf "%s" "$S4_INTEGER_SOURCE_B64" | base64 -d > "$EPHEMERAL_OUT/exact.rs"
printf "%s" "$S4_AUDIT_SOURCE_B64" | base64 -d > "$EPHEMERAL_OUT/AuditAxioms.lean"
rustc --edition 2021 -O "$EPHEMERAL_OUT/exact.rs" -o /tmp/twist-s4-exact > "$EPHEMERAL_OUT/build.log" 2>&1
proof_status=$?
printf "%s\n" "$proof_status" > "$EPHEMERAL_OUT/build.exit"
if test "$proof_status" -ne 0; then exit "$proof_status"; fi
/tmp/twist-s4-exact --lean "$EPHEMERAL_OUT/TwistS4Identity.lean" > "$EPHEMERAL_OUT/structure.json" 2> "$EPHEMERAL_OUT/derive.stderr"
proof_status=$?
printf "%s\n" "$proof_status" > "$EPHEMERAL_OUT/derive.exit"
if test "$proof_status" -ne 0; then exit "$proof_status"; fi
curl --fail --location --retry 2 https://github.com/leanprover/lean4/releases/download/v4.34.1/lean-4.34.1-linux.tar.zst -o /tmp/twist-s4-lean.tar.zst > "$EPHEMERAL_OUT/lean-download.log" 2>&1
proof_status=$?
if test "$proof_status" -ne 0; then exit "$proof_status"; fi
tar --zstd -xf /tmp/twist-s4-lean.tar.zst -C /tmp
export PATH=/tmp/lean-4.34.1-linux/bin:$PATH
lean --version > "$EPHEMERAL_OUT/lean-version.txt"
cd "$EPHEMERAL_OUT"
prlimit --as=8589934592 -- timeout 1200s lean -o TwistS4Identity.olean TwistS4Identity.lean > lean-build.log 2>&1
proof_status=$?
printf "%s\n" "$proof_status" > lean-build.exit
if test "$proof_status" -ne 0; then exit "$proof_status"; fi
LEAN_PATH="$EPHEMERAL_OUT" timeout 1200s lean --run AuditAxioms.lean -- TwistS4Identity > axiom-audit.log 2>&1
proof_status=$?
printf "%s\n" "$proof_status" > axiom-audit.exit
exit "$proof_status"
