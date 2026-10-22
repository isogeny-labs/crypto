#!/usr/bin/env bash
# checkpoint-hook.sh: run one experiment command under the application
# checkpoint contract of taskq/deploy/spot-checkpoint/README.md, inside a
# pod or on a spot host.
#
#   checkpoint-hook.sh --checkpoint-dir DIR --store s3://bucket/prefix \
#       [--grace SECONDS] [--interval SECONDS] [--keep N] -- CMD ARGS...
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
# Exit status: the command's own when it exits by itself; 75 when the run
# was interrupted after a checkpoint; 70 when restore validation failed
# (never silently start from zero); 64 on usage errors.
set -euo pipefail

usage() { sed -n '2,32p' "$0" >&2; exit 64; }

ckdir='' store='' grace=90 interval=300 keep=2
while [ $# -gt 0 ]; do
  case "$1" in
    --checkpoint-dir) ckdir=$2; shift 2 ;;
    --store) store=$2; shift 2 ;;
    --grace) grace=$2; shift 2 ;;
    --interval) interval=$2; shift 2 ;;
    --keep) keep=$2; shift 2 ;;
    --) shift; break ;;
    *) usage ;;
  esac
done
[ -n "$ckdir" ] && [ -n "$store" ] && [ $# -gt 0 ] || usage
case "$grace$interval$keep" in *[!0-9]*) usage ;; esac
mkdir -p "$ckdir"

log() { printf '%s checkpoint-hook: %s\n' "$(date -u +%FT%TZ)" "$*" >&2; }

# A generation is valid when manifest.json lists files whose sha256 match.
valid_generation() {
  local gen=$1 manifest=$1/manifest.json path sum
  [ -f "$manifest" ] || return 1
  python3 -c 'import json,sys; m=json.load(open(sys.argv[1])); assert m.get("committed") is True and m["files"]; [print(f["path"], f["sha256"]) for f in m["files"]]' "$manifest" 2>/dev/null |
  while read -r path sum; do
    case "$path" in /*|*..*) return 1 ;; esac
    [ -f "$gen/$path" ] || return 1
    [ "$(sha256sum "$gen/$path" | cut -d' ' -f1)" = "$sum" ] || return 1
  done
}

# Generation directories, newest number first, as absolute paths.
list_generations() {
  (cd "$ckdir" && ls -d gen-[0-9]* 2>/dev/null) | sort -t- -k2,2nr | sed "s|^|$ckdir/|"
}

newest_valid() {
  local gen
  for gen in $(list_generations); do
    if valid_generation "$gen"; then printf '%s\n' "$gen"; return 0; fi
    log "rejecting $gen (incomplete or hash mismatch)"
  done
  return 1
}

sync_up() {
  local gen
  for gen in "$ckdir"/gen-[0-9]*; do
    [ -d "$gen" ] && valid_generation "$gen" || continue
    aws s3 sync --only-show-errors "$gen" "$store/$(basename "$gen")" || log "sync of $gen failed"
  done
}

prune() {
  local n=0 gen
  for gen in $(list_generations); do
    valid_generation "$gen" || continue
    n=$((n + 1))
    [ "$n" -gt "$keep" ] && rm -rf "$gen"
  done
}

# 1. restore
resumed=0 manifest=''
if [ -z "$(ls -A "$ckdir" 2>/dev/null)" ]; then
  aws s3 sync --only-show-errors "$store" "$ckdir" || true
fi
if gen=$(newest_valid); then
  resumed=1; manifest=$gen/manifest.json
  log "resuming from $gen"
elif [ -n "$(list_generations)" ]; then
  log "generations exist but none is valid: refusing to start from zero"
  exit 70
fi

# 2. start the command in its own process group
export CHECKPOINT_DIR=$ckdir CHECKPOINT_RESUMED=$resumed CHECKPOINT_MANIFEST=$manifest
setsid "$@" &
child=$!
log "started pid $child: $*"

interrupted=0
on_term() {
  interrupted=1
  log "SIGTERM: forwarding to the command's process group, grace ${grace}s"
  kill -TERM -- "-$child" 2>/dev/null || true
}
trap on_term TERM INT

# 3. periodic sync while the command runs
while kill -0 "$child" 2>/dev/null; do
  elapsed=0
  while [ "$elapsed" -lt "$interval" ] && kill -0 "$child" 2>/dev/null && [ "$interrupted" -eq 0 ]; do
    sleep 1; elapsed=$((elapsed + 1))
  done
  if [ "$interrupted" -eq 1 ]; then
    waited=0
    while kill -0 "$child" 2>/dev/null && [ "$waited" -lt "$grace" ]; do sleep 1; waited=$((waited + 1)); done
    sync_up
    if kill -0 "$child" 2>/dev/null; then
      log "command did not exit within ${grace}s; killing its process group"
      kill -KILL -- "-$child" 2>/dev/null || true
    fi
    log "interrupted after sync; exit 75 so the queue resumes this job"
    exit 75
  fi
  sync_up; prune
done

# 4. the command exited by itself
wait "$child" && status=0 || status=$?
sync_up; prune
log "command exited $status"
exit "$status"
