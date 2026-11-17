set +e
mkdir -p "$EPHEMERAL_OUT/source"
printf "%s" "$S4_PROFILE_BUNDLE_B64" | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
probe_manifest="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020/s4_twist_elimination/native/Cargo.toml"
export CARGO_TARGET_DIR=/tmp/twist-s4-profile-target
cargo build --release --manifest-path "$probe_manifest" > "$EPHEMERAL_OUT/build.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/build.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
prlimit --as=8589934592 -- timeout 1200s /tmp/twist-s4-profile-target/release/twist-s4-structure-probe "$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020" "$EPHEMERAL_OUT/profile-v1" > "$EPHEMERAL_OUT/profile.stdout" 2> "$EPHEMERAL_OUT/profile.stderr"
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/profile.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
cargo test --release --manifest-path "$probe_manifest" > "$EPHEMERAL_OUT/tests.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/tests.exit"
exit "$probe_status"
