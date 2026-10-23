set +e
cargo build --release --example quartic_twist_s4_structure > "$EPHEMERAL_OUT/build.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/build.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
prlimit --as=8589934592 -- timeout 1200s target/release/examples/quartic_twist_s4_structure "$EPHEMERAL_OUT/derive-v1" > "$EPHEMERAL_OUT/derive.stdout" 2> "$EPHEMERAL_OUT/derive.stderr"
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/derive.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
cargo test --release --example quartic_twist_s4_structure > "$EPHEMERAL_OUT/tests.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/tests.exit"
exit "$probe_status"
