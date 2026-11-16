set +e
mkdir -p "$EPHEMERAL_OUT/source"
{ printf "%s" "$S4_BUNDLE_A"; printf "%s" "$S4_BUNDLE_B"; } | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
probe_manifest="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020/s4_twist_elimination/native/Cargo.toml"
probe_root="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020"
probe_fixture="$probe_root/s4_twist_elimination/fixtures/profile-v1.json"
probe_sha=756f5b72404c2c208f477f8cc2eeb20e8b9094a02250a1b5571776bb9d853f8a
export CARGO_TARGET_DIR=/tmp/twist-s4-fixtures-target
cargo build --release --manifest-path "$probe_manifest" > "$EPHEMERAL_OUT/build.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/build.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
prlimit --as=8589934592 -- timeout 1200s /tmp/twist-s4-fixtures-target/release/twist-s4-structure-probe --fixtures "$probe_fixture" "$probe_sha" "$probe_root" "$EPHEMERAL_OUT/fixtures-v1" > "$EPHEMERAL_OUT/fixtures.stdout" 2> "$EPHEMERAL_OUT/fixtures.stderr"
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/fixtures.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
/tmp/twist-s4-fixtures-target/release/twist-s4-structure-probe --corrupt-fixture "$probe_fixture" "$EPHEMERAL_OUT/corrupt-fixture.json"
/tmp/twist-s4-fixtures-target/release/twist-s4-structure-probe --fixtures "$EPHEMERAL_OUT/corrupt-fixture.json" "$probe_sha" "$probe_root" "$EPHEMERAL_OUT/corrupt-result" > "$EPHEMERAL_OUT/mutation.stdout" 2> "$EPHEMERAL_OUT/mutation.stderr"
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/mutation.exit"
if test "$probe_status" -eq 0; then exit 1; fi
cargo test --release --manifest-path "$probe_manifest" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/tests.exit"
exit "$probe_status"
