set +e
printf "%s" "$S4_INTEGER_SOURCE_B64" | base64 -d > "$EPHEMERAL_OUT/exact.rs"
rustc --edition 2021 -O "$EPHEMERAL_OUT/exact.rs" -o "$EPHEMERAL_OUT/exact" > "$EPHEMERAL_OUT/build.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/build.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
prlimit --as=8589934592 -- timeout 1200s "$EPHEMERAL_OUT/exact" > "$EPHEMERAL_OUT/structure.json" 2> "$EPHEMERAL_OUT/derive.stderr"
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/derive.exit"
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
rustc --edition 2021 --test "$EPHEMERAL_OUT/exact.rs" -o "$EPHEMERAL_OUT/exact-tests" > "$EPHEMERAL_OUT/test-build.log" 2>&1
probe_status=$?
if test "$probe_status" -ne 0; then exit "$probe_status"; fi
"$EPHEMERAL_OUT/exact-tests" > "$EPHEMERAL_OUT/tests.log" 2>&1
probe_status=$?
printf "%s\n" "$probe_status" > "$EPHEMERAL_OUT/tests.exit"
exit "$probe_status"
