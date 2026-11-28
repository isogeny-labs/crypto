#!/usr/bin/env bash
set -euo pipefail

repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
output=/Volumes/SSD990-2/tmp/n83-colima-output/full-fixture-examples-20261116-attempt-1
target=/Volumes/SSD990-2/tmp/n83-colima-output/full-fixture-lib-20261116-attempt-2/cargo-target
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
log=/private/tmp/n83-full-fixture-examples-20261116-attempt-1.log
child_receipt=/private/tmp/n83-full-fixture-examples-20261116-attempt-1.child.receipt
outer_receipt=/private/tmp/n83-full-fixture-examples-20261116-attempt-1.outer.receipt
inspect_json=/private/tmp/n83-full-fixture-examples-20261116-attempt-1.inspect.json
source_file=/private/tmp/n83-full-fixture-examples-20261116-attempt-1.source.commit
container=n83-full-fixture-examples-20261116-attempt-1
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
fixture=research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl
fixture_sha=8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738
fixture_bytes=2293097

cd "$repo"
test "$(git config --get core.sparseCheckout)" = false || { echo 'source remains sparse' >&2; exit 90; }
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
git diff --quiet a702129de8e5bd72482d9790f3463028a467926f HEAD -- src examples Cargo.toml Cargo.lock crates gpu tools || { echo 'code changed since library validation' >&2; exit 103; }
for example in koblitz_n83_factor_base_export n83_k600_m6_budget n83_k600_m6_linux_build n83_k600_m6_native_arms n83_k600_m6_native_preflight; do
  test -f "examples/$example.rs" || { echo "example absent: $example" >&2; exit 104; }
done
test -f gpu/fes/fes_solve.cpp && test -f research/ic_planner_validation_20261020/sessions/r24/records.jsonl || { echo 'tracked fixtures absent' >&2; exit 92; }
test "$(shasum -a 256 "$fixture" | awk '{print $1}')" = "$fixture_sha" || { echo 'fixture hash mismatch' >&2; exit 93; }
test "$(wc -c < "$fixture" | tr -d ' ')" = "$fixture_bytes" || { echo 'fixture length mismatch' >&2; exit 94; }
test -d "$target/release/deps" || { echo 'completed library Cargo cache absent' >&2; exit 106; }
source_sha=$(git rev-parse HEAD)
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'toolchain image mismatch' >&2; exit 95; }
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper hash mismatch' >&2; exit 96; }
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 20971520 || { echo 'boot disk below 20 GiB' >&2; exit 97; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 52428800 || { echo 'SSD below 50 GiB' >&2; exit 98; }
test "$(COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 99; }
test -z "$(docker --context colima-n83-arm64 ps -q)" || { echo 'concurrent Colima container' >&2; exit 105; }
for candidate in "$output" "$log" "$child_receipt" "$outer_receipt" "$inspect_json" "$source_file"; do
  test ! -e "$candidate" || { echo "output exists: $candidate" >&2; exit 100; }
done
test -z "$(docker --context colima-n83-arm64 ps -aq --filter "name=^/${container}$")" || { echo 'container name exists' >&2; exit 101; }
printf '%s\n' "$source_sha" > "$source_file"

started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
set +e
bash "$wrapper" "$log" "$child_receipt" -- \
  gtimeout 7200 docker --context colima-n83-arm64 run --pull=never \
  --name "$container" --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=bridge --cap-drop=ALL --security-opt=no-new-privileges \
  --env GIT_OPTIONAL_LOCKS=0 --env GIT_CONFIG_COUNT=1 \
  --env GIT_CONFIG_KEY_0=safe.directory --env GIT_CONFIG_VALUE_0="$repo" \
  --env EXPECT_SOURCE="$source_sha" \
  --env EXPECT_FIXTURE_SHA="$fixture_sha" --env EXPECT_FIXTURE_BYTES="$fixture_bytes" \
  --env CARGO_TARGET_DIR="$target" --env CARGO_INCREMENTAL=0 \
  --mount "type=bind,src=$repo,dst=$repo,readonly" \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,dst=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-output,dst=/Volumes/SSD990-2/tmp/n83-colima-output \
  "$image" sh -ec '
    repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
    output=/Volumes/SSD990-2/tmp/n83-colima-output/full-fixture-examples-20261116-attempt-1
    fixture="$repo/research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl"
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test "$(git -C "$repo" rev-parse HEAD)" = "$EXPECT_SOURCE"
    test -z "$(git -C "$repo" status --porcelain)"
    test "$(sha256sum "$fixture" | awk '\''{print $1}'\'')" = "$EXPECT_FIXTURE_SHA"
    test "$(wc -c < "$fixture" | tr -d " ")" = "$EXPECT_FIXTURE_BYTES"
    test -f "$repo/gpu/fes/fes_solve.cpp"
    test -f "$repo/research/ic_planner_validation_20261020/sessions/r24/records.jsonl"
    test -d "$CARGO_TARGET_DIR/release/deps"
    mkdir "$output"
    cd "$repo"
    printf "source=%s fixture_sha=%s target=%s memory.max=%s swap.max=%s cpu.max=%s pids.max=%s\n" \
      "$EXPECT_SOURCE" "$EXPECT_FIXTURE_SHA" "$CARGO_TARGET_DIR" "$(cat /sys/fs/cgroup/memory.max)" \
      "$(cat /sys/fs/cgroup/memory.swap.max)" "$(cat /sys/fs/cgroup/cpu.max)" \
      "$(cat /sys/fs/cgroup/pids.max)"
    cargo test --release --locked -j 1 --example koblitz_n83_factor_base_export --example n83_k600_m6_budget --example n83_k600_m6_linux_build --example n83_k600_m6_native_arms --example n83_k600_m6_native_preflight
    test -z "$(git status --porcelain)"
  '
child_exit=$?
docker --context colima-n83-arm64 inspect "$container" > "$inspect_json"
inspect_exit=$?
docker --context colima-n83-arm64 rm -f "$container" >/dev/null
cleanup_exit=$?
set -e
ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
source_clean=true
test -z "$(git status --porcelain)" || source_clean=false
printf 'schema=n83.colima-full-fixture-examples-outer-receipt/v1\nstarted_utc=%s\nended_utc=%s\nsource_commit=%s\nfixture_sha256=%s\nchild_exit_code=%s\ninspect_exit_code=%s\ncleanup_exit_code=%s\nsource_clean=%s\n' \
  "$started_utc" "$ended_utc" "$source_sha" "$fixture_sha" "$child_exit" "$inspect_exit" "$cleanup_exit" "$source_clean" > "$outer_receipt"
test "$child_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0 && test "$source_clean" = true
