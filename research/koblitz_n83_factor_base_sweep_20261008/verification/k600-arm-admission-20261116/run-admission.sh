#!/usr/bin/env bash
set -euo pipefail
full=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
h2=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-k600-preflight-c-20261116-attempt-2
root=/Volumes/SSD990-2/tmp/n83-colima-output/k600-arm-admission-20261116-attempt-1
stage=/Volumes/SSD990-2/tmp/n83-colima-output/k600-arm-stage-20261116-attempt-1
index=/private/tmp/n83-k600-s3-adoption-20261115-1/inputs/store-index.json
lock="$h2/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json"
source_c=ba620214684e41def8bc6caddf2e782f43920218
source_h2=a30e67492f821bc6f58ff97c508be9cfe144d7ab
object_sha=3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
image=sha256:b84bd73914a2c76a7c8635ee4d40564d974f0fc7188b9e71f315726050bae2b3
socket=unix:///Volumes/SSD990-2/tmp/n83-colima-home/n83-arm64/docker.sock

test ! -e "$root"
test ! -e "$stage"
test "$(git -C "$h2" rev-parse HEAD)" = "$source_h2"
test "$(git -C "$h2" rev-parse HEAD^)" = "$source_c"
test -z "$(git -C "$h2" status --porcelain)"
test "$(git -C "$h2" diff --name-only HEAD^ HEAD)" = research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json
test "$(shasum -a 256 "$lock" | awk '{print $1}')" = b97fd192c5ab2af87bd368062014b2c4bef643e791cf6a0ab335a1197c986cae
test "$(shasum -a 256 "$index" | awk '{print $1}')" = 298b5c1396b58a1b4025813be092934be6f65e34544856d92f1edadabeabd28b
test "$(DOCKER_HOST="$socket" docker image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}} {{index .Config.Labels "org.opencontainers.image.revision"}}')" = "$image linux/arm64 $source_c"
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR==2 {print $4}')" -ge $((20*1024*1024))
test "$(df -Pk / | awk 'NR==2 {print $4}')" -ge $((5*1024*1024))

mkdir -p "$root"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$root/start.utc"
rustc_path=$(rustup which rustc --toolchain 1.99.1)
"$rustc_path" --version > "$root/rustc.version"
cargo --version > "$root/cargo.version"

# The H2 checkout is sparse with custom rules; apply the repository minimal
# profile to materialize tracked paths. This changes no committed source.
(
  cd "$h2"
  bash "$full/scripts/sparse-checkout.sh" apply --profile minimal \
    --path examples/n83_k600_m6_native_arms.rs \
    --path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/manifest.json \
    --path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/replay.json \
    --path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/upload-receipt.json \
    --path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json \
    --path research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-validation.json \
    --path research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json
) > "$root/sparse.stdout" 2> "$root/sparse.stderr"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$root/sparse.end.utc"
test -z "$(git -C "$h2" status --porcelain)"
test -f "$h2/examples/n83_k600_m6_native_arms.rs"
for name in manifest.json replay.json upload-receipt.json probe-corpus.json probe-validation.json; do
  test -f "$h2/research/koblitz_n83_factor_base_sweep_20261008/pilot-01/$name"
done

run_bounded() {
  local name=$1 cap=$2; shift 2
  local started_epoch
  mkdir -p "$root/$name"
  started_epoch=$(date -u +%s)
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$root/$name/start.utc"
  gtimeout -s TERM -k 15s "${cap}s" "$@" > "$root/$name/stdout.log" 2> "$root/$name/stderr.log" &
  local child=$!
  (
    while kill -0 "$child" 2>/dev/null; do
      local ssd boot
      ssd=$(df -Pk /Volumes/SSD990-2 | awk 'NR==2 {print $4}')
      boot=$(df -Pk / | awk 'NR==2 {print $4}')
      printf '%s\t%s\t%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" "$ssd" "$boot" >> "$root/$name/disk-watch.tsv"
      if (( ssd < 20*1024*1024 || boot < 5*1024*1024 )); then
        printf 'disk floor reached: ssd=%s boot=%s KiB\n' "$ssd" "$boot" > "$root/$name/disk-stop.txt"
        kill -TERM "$child" 2>/dev/null || true
        break
      fi
      sleep 15
    done
  ) &
  local watcher=$!
  set +e
  wait "$child"
  local code=$?
  set -e
  kill "$watcher" 2>/dev/null || true
  wait "$watcher" 2>/dev/null || true
  printf '%s\n' "$code" > "$root/$name/exit.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$root/$name/end.utc"
  local outer_seconds=$(( $(date -u +%s) - started_epoch ))
  printf '%s\n' "$outer_seconds" > "$root/$name/outer-seconds.txt"
  if (( outer_seconds > cap + 30 )); then
    printf 'outer admission ceiling exceeded: %s > %s seconds\n' "$outer_seconds" "$((cap+30))" > "$root/$name/outer-cap.txt"
  fi
  test "$code" -eq 0
  test ! -e "$root/$name/disk-stop.txt"
  test ! -e "$root/$name/outer-cap.txt"
  test -z "$(git -C "$h2" status --porcelain)"
}

run_bounded build 2700 env RUSTC="$rustc_path" CARGO_TARGET_DIR="$root/target" \
  cargo build --release --locked --offline -j 1 --example n83_k600_m6_native_arms --manifest-path "$h2/Cargo.toml"
binary="$root/target/release/examples/n83_k600_m6_native_arms"
test -f "$binary"
file "$binary" > "$root/binary.file"
rg -q 'Mach-O 64-bit executable arm64' "$root/binary.file"
strings "$binary" | rg -F "$h2" > /dev/null
shasum -a 256 "$binary" > "$root/binary.sha256"
wc -c < "$binary" > "$root/binary.bytes"

run_bounded stage 300 env AWS_PROFILE=ecc-challenges-new \
  CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
  CRYPTO_STORE_INDEX="$index" CRYPTO_STORE_CACHE="$root/unused-cache" \
  "$binary" stage "$h2/research/koblitz_n83_factor_base_sweep_20261008/pilot-01" "$stage"
test "$(jq -r .status "$stage/staging.json")" = PASS_READ_ONLY_STAGE
test "$(jq -r .source_commit "$stage/staging.json")" = "$source_h2"
test "$(jq -r .object_sha256 "$stage/staging.json")" = "$object_sha"
test "$(jq -r .object_bytes "$stage/staging.json")" = 3025798
test "$(jq -r .fresh_s3_get "$stage/staging.json")" = true
test "$(jq -r .uploaded "$stage/staging.json")" = false
test "$(jq -r .solver_executed "$stage/staging.json")" = false
test "$(jq -r .total_index_calculus_runtime_ms "$stage/staging.json")" = null
test "$(jq -r .selected_best_total_runtime "$stage/staging.json")" = null
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$root/end.utc"
printf 'PASS_READ_ONLY_STAGE\n' > "$root/status.txt"
