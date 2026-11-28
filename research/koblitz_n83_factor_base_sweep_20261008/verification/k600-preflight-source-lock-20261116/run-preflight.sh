#!/usr/bin/env bash
set -euo pipefail
wt=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-k600-preflight-c-20261116
lock="$wt/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json"
supervisor=/Volumes/SSD990-2/tmp/n83-colima-output/k600-preflight-mac-supervisor-build-20261116-attempt-2/target/release/examples/n83_k600_m6_native_preflight
worker=/Volumes/SSD990-2/tmp/n83-colima-output/charged-20261116-attempt-4/worker
out=/Volumes/SSD990-2/tmp/n83-colima-output/k600-preflight-20261116-run-attempt-1
outer=/Volumes/SSD990-2/tmp/n83-colima-output/k600-preflight-20261116-run-outer-attempt-1
socket=unix:///Volumes/SSD990-2/tmp/n83-colima-home/n83-arm64/docker.sock
image=sha256:b84bd73914a2c76a7c8635ee4d40564d974f0fc7188b9e71f315726050bae2b3
index=/private/tmp/n83-k600-s3-adoption-20261115-1/inputs/store-index.json
test ! -e "$out"
test ! -e "$outer"
test "$(git -C "$wt" rev-parse HEAD)" = 4905dd76dbf619c6a33ae82c1bdec7713b0a9c29
test "$(git -C "$wt" rev-parse HEAD^)" = ba620214684e41def8bc6caddf2e782f43920218
test -z "$(git -C "$wt" status --porcelain)"
test "$(git -C "$wt" diff --name-only HEAD^ HEAD)" = research/koblitz_n83_factor_base_sweep_20261008/verification/k600-preflight-source-lock-20261116/LOCK.json
test "$(shasum -a 256 "$lock" | cut -d ' ' -f 1)" = deb5508d1b4e78dea92b737eaf2245d1b80c6b5032d532c53bd863cbf779ced8
test "$(shasum -a 256 "$supervisor" | cut -d ' ' -f 1)" = 3ea86decaf897e800aa3f78c5b028c1505b71fe1637cbb858ad717907f0284c0
test "$(shasum -a 256 "$worker" | cut -d ' ' -f 1)" = d6aad2f3737eff25e2d3759e204b6714e83c58212017b39a650414c7b6ac269b
test "$(shasum -a 256 "$index" | cut -d ' ' -f 1)" = 298b5c1396b58a1b4025813be092934be6f65e34544856d92f1edadabeabd28b
test "$(DOCKER_HOST="$socket" docker image inspect "$image" --format '{{index .Config.Labels "org.opencontainers.image.revision"}}')" = ba620214684e41def8bc6caddf2e782f43920218
test -z "$(DOCKER_HOST="$socket" docker ps -aq --filter name='^n83-k600-m6-native-preflight$')"
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR==2 {print $4}')" -ge $((20 * 1024 * 1024))
test "$(df -Pk / | awk 'NR==2 {print $4}')" -ge $((5 * 1024 * 1024))
mkdir -p "$outer"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$outer/start.utc"
(
  cd "$wt"
  env DOCKER_HOST="$socket" AWS_PROFILE=ecc-challenges-new \
    CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
    CRYPTO_STORE_INDEX="$index" \
    CRYPTO_STORE_CACHE=/private/tmp/n83-k600-preflight-20261116-unused-cache \
    gtimeout -s TERM -k 15s 330s "$supervisor" run "$lock" "$out" 300
) > "$outer/stdout.log" 2> "$outer/stderr.log" &
child=$!
(
  while kill -0 "$child" 2>/dev/null; do
    ssd=$(df -Pk /Volumes/SSD990-2 | awk 'NR==2 {print $4}')
    boot=$(df -Pk / | awk 'NR==2 {print $4}')
    printf '%s\t%s\t%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" "$ssd" "$boot" >> "$outer/disk-watch.tsv"
    if (( ssd < 20 * 1024 * 1024 || boot < 5 * 1024 * 1024 )); then
      printf 'disk floor reached: ssd=%s boot=%s KiB\n' "$ssd" "$boot" > "$outer/disk-stop.txt"
      kill -TERM "$child" 2>/dev/null || true
      break
    fi
    sleep 15
  done
) &
watcher=$!
set +e
wait "$child"
code=$?
set -e
kill "$watcher" 2>/dev/null || true
wait "$watcher" 2>/dev/null || true
printf '%s\n' "$code" > "$outer/exit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$outer/end.utc"
DOCKER_HOST="$socket" docker ps -aq --filter name='^n83-k600-m6-native-preflight$' > "$outer/residual-container.txt"
if test -s "$outer/residual-container.txt"; then
  DOCKER_HOST="$socket" docker rm -f n83-k600-m6-native-preflight > "$outer/cleanup.stdout" 2> "$outer/cleanup.stderr" || true
fi
test "$code" -eq 0
test ! -e "$outer/disk-stop.txt"
test ! -s "$outer/residual-container.txt"
test -z "$(git -C "$wt" status --porcelain)"
