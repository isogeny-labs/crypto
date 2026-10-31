#!/usr/bin/env bash
# Supervisor integration checks with a local object-store stand-in; no cloud I/O.
# Subprocess scripts deliberately expand their own environment.
# shellcheck disable=SC2016
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
hook=${1:-$here/checkpoint-hook.sh}
tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
mkdir -p "$tmp/bin"
export MOCK_STORE=$tmp/store MOCK_LOG=$tmp/aws.log
export PATH="$tmp/bin:$PATH"
if ! command -v setsid >/dev/null; then
  cat > "$tmp/setsid.c" <<'C'
#include <unistd.h>
int main(int argc, char **argv) {
    if (argc < 2 || setsid() == -1) return 69;
    execvp(argv[1], argv + 1);
    return 69;
}
C
  cc "$tmp/setsid.c" -o "$tmp/bin/setsid"
fi
cat > "$tmp/bin/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
[ "$1" = s3 ]; op=$2; shift 2
exclude=''
while [[ ${1:-} == --* ]]; do
  case "$1" in
    --only-show-errors) shift ;;
    --exclude) exclude=$2; shift 2 ;;
    *) exit 64 ;;
  esac
done
src=$1 dst=$2
printf '%s %s %s\n' "$op" "$src" "$dst" >> "$MOCK_LOG"
if [[ "$src" == s3://* ]]; then
  [ "${MOCK_DOWNLOAD_FAIL:-0}" = 0 ] || exit 1
  src=$MOCK_STORE/${src#s3://}
fi
if [[ "$dst" == s3://* ]]; then
  if [ "${MOCK_UPLOAD_SLEEP:-0}" != 0 ]; then sleep "$MOCK_UPLOAD_SLEEP"; fi
  [ "${MOCK_UPLOAD_FAIL:-0}" = 0 ] || exit 1
  dst=$MOCK_STORE/${dst#s3://}
fi
if [ "$op" = cp ]; then
  mkdir -p "$(dirname "$dst")"; cp "$src" "$dst"; exit
fi
mkdir -p "$dst"
[ -d "$src" ] || exit 0
while IFS= read -r file; do
  rel=${file#"$src"/}
  [ "$rel" != "$exclude" ] || continue
  mkdir -p "$(dirname "$dst/$rel")"; cp "$file" "$dst/$rel"
done < <(find "$src" -type f)
SH
chmod +x "$tmp/bin/aws"
spec=$(printf spec | sha256sum | cut -d' ' -f1)
cases=0
fresh() {
  rm -rf -- "$tmp/local" "$MOCK_STORE"
  mkdir -p "$tmp/local" "$MOCK_STORE"
  unset MOCK_UPLOAD_FAIL MOCK_DOWNLOAD_FAIL MOCK_UPLOAD_SLEEP
  : > "$MOCK_LOG"
}
generation() {
  local n=$1 dir=$tmp/local/gen-$1
  mkdir -p "$dir"
  printf 'generation %s' "$n" > "$dir/state.bin"
  jq -n --arg job job-1 --arg spec "$spec" --argjson n "$n" \
    --arg hash "$(sha256sum "$dir/state.bin" | cut -d' ' -f1)" \
    --argjson size "$(wc -c < "$dir/state.bin")" \
    '{schema:"experiment.checkpoint/v1", job_id:$job, spec_sha256:$spec,
      generation:$n, committed:true, files:[{path:"state.bin",sha256:$hash,bytes:$size}]}' \
    > "$dir/manifest.json"
}
run() {
  local want=$1; shift
  local got=0
  timeout --kill-after=2 15 bash "$hook" --checkpoint-dir "$tmp/local" \
    --store s3://bucket/job --job-id job-1 --spec-sha256 "$spec" \
    --interval 1 --grace 1 --sync-timeout 2 --keep 1 -- "$@" \
    > "$tmp/output" 2>&1 || got=$?
  if [ "$got" -ne "$want" ]; then
    cat "$tmp/output" >&2
    printf 'case %s: expected exit %s, got %s\n' "$((cases + 1))" "$want" "$got" >&2
    exit 1
  fi
  cases=$((cases + 1))
}

fresh; generation 1
run 0 bash -c 'test "$CHECKPOINT_RESUMED" = 1; test -f "$CHECKPOINT_MANIFEST"'
test -f "$MOCK_STORE/bucket/job/gen-1/manifest.json"

fresh; generation 1; generation 2
printf corrupt > "$tmp/local/gen-2/state.bin"
run 0 bash -c 'test "${CHECKPOINT_MANIFEST%/manifest.json}" = "$CHECKPOINT_DIR/gen-1"'

fresh; generation 1
jq '.job_id="wrong-job"' "$tmp/local/gen-1/manifest.json" > "$tmp/m"
mv "$tmp/m" "$tmp/local/gen-1/manifest.json"
run 70 true

fresh; generation 1
jq '.spec_sha256=("0" * 64)' "$tmp/local/gen-1/manifest.json" > "$tmp/m"
mv "$tmp/m" "$tmp/local/gen-1/manifest.json"
run 70 true

fresh; generation 1
printf extra > "$tmp/local/gen-1/extra.bin"
run 70 true

fresh; generation 1
jq '.files[0].bytes += 1' "$tmp/local/gen-1/manifest.json" > "$tmp/m"
mv "$tmp/m" "$tmp/local/gen-1/manifest.json"
run 70 true

fresh; generation 1
jq '.files += .files' "$tmp/local/gen-1/manifest.json" > "$tmp/m"
mv "$tmp/m" "$tmp/local/gen-1/manifest.json"
run 70 true

fresh; generation 1
mv "$tmp/local/gen-1/state.bin" "$tmp/state.bin"
ln -s "$tmp/state.bin" "$tmp/local/gen-1/state.bin"
run 70 true

fresh; export MOCK_DOWNLOAD_FAIL=1
run 70 touch "$tmp/should-not-start"
test ! -e "$tmp/should-not-start"

fresh; generation 1; generation 2; generation 3
export MOCK_UPLOAD_FAIL=1
run 74 true
test -f "$tmp/local/gen-1/manifest.json"
test -f "$tmp/local/gen-2/manifest.json"
test -f "$tmp/local/gen-3/manifest.json"

fresh; generation 1
export MOCK_UPLOAD_SLEEP=10
run 74 true
test -f "$tmp/local/gen-1/manifest.json"

fresh; generation 1; generation 2
run 0 true
test ! -d "$tmp/local/gen-1"
test -f "$MOCK_STORE/bucket/job/gen-1/manifest.json"
test -f "$tmp/local/gen-2/manifest.json"

fresh
run 7 bash -c 'exit 7'

for busy in 0 10; do
fresh; generation 1
export MOCK_UPLOAD_SLEEP=$busy
rm -f "$tmp/ready"
bash "$hook" --checkpoint-dir "$tmp/local" --store s3://bucket/job \
  --job-id job-1 --spec-sha256 "$spec" --interval 1 --grace 1 --sync-timeout 2 \
  -- bash -c 'trap "exit 0" TERM; touch "$1"; while :; do sleep 0.1; done' bash "$tmp/ready" \
  > "$tmp/output" 2>&1 &
supervisor=$!
for _ in {1..100}; do
  [ ! -f "$tmp/ready" ] || break
  sleep 0.05
done
test -f "$tmp/ready"
if [ "$busy" != 0 ]; then
  for _ in {1..100}; do
    [ ! -s "$MOCK_LOG" ] || break
    sleep 0.05
  done
  test -s "$MOCK_LOG"
fi
started=$SECONDS
kill -TERM "$supervisor"
got=0; wait "$supervisor" || got=$?
if [ "$got" -ne 75 ]; then cat "$tmp/output" >&2; exit 1; fi
test "$((SECONDS - started))" -le 7
cases=$((cases + 1))
done
printf '%s checkpoint supervisor integration cases passed\n' "$cases"
