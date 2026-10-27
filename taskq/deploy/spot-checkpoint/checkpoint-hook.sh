#!/usr/bin/env bash
# checkpoint-hook.sh: run one experiment command under the application
# checkpoint contract of taskq/deploy/spot-checkpoint/README.md, inside a
# pod or on a spot host.
#
#   checkpoint-hook.sh --checkpoint-dir DIR --store s3://bucket/prefix \
#       --job-id ID --spec-sha256 SHA256 \
#       [--grace SECONDS] [--interval SECONDS] [--keep N] \
#       [--sync-timeout SECONDS] -- CMD ARGS...
#
# It does five things and nothing else:
#   1. restores the newest valid generation from STORE into DIR when DIR is
#      empty, and exports CHECKPOINT_RESUMED=1 and CHECKPOINT_MANIFEST for the
#      command (a generation is valid when its manifest.json lists every
#      file with a sha256 that matches);
#   2. starts CMD in its own process group with CHECKPOINT_DIR=DIR;
#   3. every INTERVAL seconds, and on SIGTERM (the provider's interruption
#      notice, forwarded by the kubelet or the supervisor), syncs every
#      committed generation to STORE;
#   4. on SIGTERM forwards SIGTERM to the command's process group, waits up
#      to GRACE seconds for it to commit a generation, syncs, then SIGKILLs
#      whatever is left and exits 75 (EX_TEMPFAIL: resume me);
#   5. prunes generations older than the newest KEEP valid ones in DIR
#      (never in STORE).
#
# The command owns the contract: it writes a complete generation under
# $CHECKPOINT_DIR/gen-<n>/, fsyncs every file, writes manifest.json LAST
# (temp file, fsync, rename), restores only from $CHECKPOINT_MANIFEST when
# CHECKPOINT_RESUMED=1, and exits within GRACE seconds of SIGTERM. The hook
# validates; it never edits a generation. Shell is used here as thin run
# orchestration only (AGENTS.md "Implementation language"); no measurement
# or research result is produced by this script.
#
# Exit status: the command's own when it exits by itself; 74 when a successful
# command's final upload fails; 75 when the run was interrupted; 70 when
# restore or validation failed (never silently start from zero); 64 on usage
# errors. An interruption does not guarantee a checkpoint was committed.
set -euo pipefail

usage() { sed -n '2,32p' "$0" >&2; exit 64; }

ckdir='' store='' job_id='' spec_sha256='' grace=90 interval=300 keep=2 sync_timeout=15
while [ $# -gt 0 ]; do
  [ "$1" = -- ] || [ $# -ge 2 ] || usage
  case "$1" in
    --checkpoint-dir) ckdir=$2; shift 2 ;;
    --store) store=$2; shift 2 ;;
    --grace) grace=$2; shift 2 ;;
    --interval) interval=$2; shift 2 ;;
    --keep) keep=$2; shift 2 ;;
    --job-id) job_id=$2; shift 2 ;;
    --spec-sha256) spec_sha256=$2; shift 2 ;;
    --sync-timeout) sync_timeout=$2; shift 2 ;;
    --) shift; break ;;
    *) usage ;;
  esac
done
[ -n "$ckdir" ] && [ -n "$job_id" ] && [ $# -gt 0 ] || usage
[[ "$store" == s3://?* && "$spec_sha256" =~ ^[0-9a-f]{64}$ ]] || usage
for n in "$grace" "$interval" "$keep" "$sync_timeout"; do
  [[ "$n" =~ ^[1-9][0-9]{0,5}$ ]] || usage
done
[[ "$ckdir" != *$'\n'* ]] || usage
for dependency in jq sha256sum aws timeout setsid; do
  command -v "$dependency" >/dev/null || { printf 'missing dependency: %s\n' "$dependency" >&2; exit 69; }
done
mkdir -p "$ckdir"
ckdir=$(cd "$ckdir" && pwd -P)
store=${store%/}

log() { printf '%s checkpoint-hook: %s\n' "$(date -u +%FT%TZ)" "$*" >&2; }

# Identity, complete file coverage, sizes and hashes must all match. The
# restricted relative path alphabet makes TSV iteration unambiguous.
valid_generation() {
  local gen=$1 manifest=$1/manifest.json path sum bytes count expected
  [ -d "$gen" ] && [ ! -L "$gen" ] && [ -f "$manifest" ] && [ ! -L "$manifest" ] || return 1
  [ -z "$(find "$gen" ! -type d ! -type f -print)" ] || return 1
  jq -e --arg job "$job_id" --arg spec "$spec_sha256" --arg dir "${gen##*/}" '
    .schema == "experiment.checkpoint/v1" and .committed == true and
    .job_id == $job and .spec_sha256 == $spec and
    (.generation | type == "number" and . >= 0 and . <= 9007199254740991 and floor == .) and
    ("gen-" + (.generation | tostring)) == $dir and
    (.files | type == "array" and length > 0) and
    ([.files[].path] | length == (unique | length)) and
    all(.files[];
      (.path | type == "string" and test("^[A-Za-z0-9][A-Za-z0-9._/-]*$") and
        (split("/") | all(. != "" and . != "." and . != ".."))) and
      .path != "manifest.json" and
      (.sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
      (.bytes | type == "number" and . >= 0 and floor == .))
  ' "$manifest" >/dev/null 2>&1 || return 1
  count=$(find "$gen" -type f | wc -l)
  expected=$(jq '.files | length + 1' "$manifest")
  [ "$count" -eq "$expected" ] || return 1
  while IFS=$'\t' read -r path sum bytes; do
    [ -f "$gen/$path" ] || return 1
    [ "$(wc -c < "$gen/$path" | tr -d ' ')" = "$bytes" ] || return 1
    [ "$(sha256sum "$gen/$path" | cut -d' ' -f1)" = "$sum" ] || return 1
  done < <(jq -r '.files[] | [.path, .sha256, .bytes] | @tsv' "$manifest")
  return 0
}

# Generation directories, newest number first, as absolute paths.
list_generations() {
  local gen
  for gen in "$ckdir"/gen-*; do
    [ -e "$gen" ] || [ -L "$gen" ] || continue
    printf '%s\n' "${gen##*/}"
  done | LC_ALL=C sort -t- -k2,2nr
}

newest_valid() {
  local gen
  while IFS= read -r gen; do
    gen=$ckdir/$gen
    if valid_generation "$gen"; then printf '%s\n' "$gen"; return 0; fi
    log "rejecting $gen (incomplete or hash mismatch)"
  done < <(list_generations)
  return 1
}

# Invoked by the bounded subshell in bounded_sync.
# shellcheck disable=SC2329
sync_up() {
  local gen failed=0
  for gen in "$ckdir"/gen-[0-9]*; do
    if [ ! -d "$gen" ] || ! valid_generation "$gen"; then continue; fi
    # Publish the commit marker last; a partial upload is never admitted.
    if aws s3 sync --only-show-errors --exclude manifest.json "$gen" "$store/${gen##*/}" &&
       aws s3 cp --only-show-errors "$gen/manifest.json" "$store/${gen##*/}/manifest.json"; then
      :
    else
      log "sync of $gen failed"; failed=1
    fi
  done
  return "$failed"
}

bounded_sync() {
  # Bound the entire batch, including every AWS retry and every generation.
  export ckdir store job_id spec_sha256
  export -f log valid_generation sync_up
  local status=0
  timeout --kill-after=2 "$sync_timeout" bash -c sync_up &
  sync_pid=$!
  # wait is interruptible by the parent's trap, unlike a foreground utility.
  wait "$sync_pid" || status=$?
  if [ "$status" -ne 0 ]; then
    kill -TERM -- "-$sync_pid" 2>/dev/null || true
    wait "$sync_pid" 2>/dev/null || true
  fi
  sync_pid=''
  return "$status"
}

prune() {
  local n=0 gen
  while IFS= read -r gen; do
    gen=$ckdir/$gen
    valid_generation "$gen" || continue
    n=$((n + 1))
    if [ "$n" -gt "$keep" ]; then rm -rf -- "$gen"; fi
  done < <(list_generations)
  return 0
}

# 1. restore
resumed=0 manifest=''
if [ -z "$(ls -A "$ckdir" 2>/dev/null)" ]; then
  if ! timeout --kill-after=2 "$sync_timeout" aws s3 sync --only-show-errors "$store" "$ckdir"; then
    log "checkpoint download failed: refusing to start from zero"
    exit 70
  fi
fi
if gen=$(newest_valid); then
  resumed=1; manifest=$gen/manifest.json
  log "resuming from $gen"
elif [ -n "$(ls -A "$ckdir")" ]; then
  log "checkpoint state exists but none is valid: refusing to start from zero"
  exit 70
fi

# 2. start the command in its own process group
export CHECKPOINT_DIR=$ckdir CHECKPOINT_RESUMED=$resumed CHECKPOINT_MANIFEST=$manifest
child='' sync_pid='' interrupted=0
on_term() {
  interrupted=1
  log "SIGTERM: forwarding to the command's process group, grace ${grace}s"
  if [ -n "$child" ]; then kill -TERM -- "-$child" 2>/dev/null || true; fi
  if [ -n "$sync_pid" ]; then kill -TERM -- "-$sync_pid" 2>/dev/null || true; fi
}
trap on_term TERM INT
# Invoked by the EXIT trap.
# shellcheck disable=SC2329
cleanup() {
  if [ -n "$child" ]; then kill -KILL -- "-$child" 2>/dev/null || true; fi
  if [ -n "$sync_pid" ]; then kill -KILL -- "-$sync_pid" 2>/dev/null || true; fi
}
trap cleanup EXIT
setsid "$@" &
child=$!
log "started pid $child"
if [ "$interrupted" -eq 1 ]; then on_term; fi

# 3. periodic sync while the command runs
while kill -0 "$child" 2>/dev/null; do
  elapsed=0
  while [ "$elapsed" -lt "$interval" ] && kill -0 "$child" 2>/dev/null && [ "$interrupted" -eq 0 ]; do
    sleep 1; elapsed=$((elapsed + 1))
  done
  if [ "$interrupted" -eq 1 ]; then
    waited=0
    while kill -0 "$child" 2>/dev/null && [ "$waited" -lt "$grace" ]; do sleep 1; waited=$((waited + 1)); done
    if kill -0 "$child" 2>/dev/null; then
      log "command did not exit within ${grace}s; killing its process group"
      kill -KILL -- "-$child" 2>/dev/null || true
    fi
    wait "$child" 2>/dev/null || true
    if bounded_sync; then prune; else log "sync failed; retaining every local generation"; fi
    log "interrupted; exit 75 so the queue resumes this job"
    exit 75
  fi
  if bounded_sync; then prune; else log "sync failed; retaining every local generation"; fi
done

# 4. the command exited by itself
wait "$child" && status=0 || status=$?
if bounded_sync; then prune; else
  log "final sync failed; retaining every local generation"
  [ "$status" -ne 0 ] || status=74
fi
if [ "$interrupted" -eq 1 ]; then status=75; fi
log "command exited $status"
exit "$status"
