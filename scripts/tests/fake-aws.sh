#!/usr/bin/env bash
# A directory-backed stand-in for the few `aws s3` calls ckpt-s3-sync.sh makes.
# s3://BUCKET/KEY lives at $FAKE_S3_ROOT/BUCKET/KEY.
set -euo pipefail
loc() { case $1 in s3://*) echo "$FAKE_S3_ROOT/${1#s3://}" ;; *) echo "$1" ;; esac; }
[ "$1" = configure ] && exit 0
[ "$1" = s3 ] || exit 2
cmd=$2; shift 2
rec=0; excl=""; args=()
while [ $# -gt 0 ]; do
  case $1 in
    --only-show-errors) ;;
    --recursive) rec=1 ;;
    --exclude) excl=$2; shift ;;
    *) args+=("$1") ;;
  esac; shift
done
case $cmd in
  ls)
    p=$(loc "${args[0]}")
    if [[ ${args[0]} == */ ]]; then
      [ -d "$p" ] || exit 1
      for e in "$p"*; do [ -e "$e" ] || continue
        if [ -d "$e" ]; then echo "                           PRE $(basename "$e")/"; else echo "2026-10-24 00:00:00 1 $(basename "$e")"; fi; done
    else [ -e "$p" ] || exit 1; echo "2026-10-24 00:00:00 1 $(basename "$p")"; fi ;;
  cp)
    s=$(loc "${args[0]}"); d=$(loc "${args[1]}")
    if [ "$rec" = 1 ]; then
      mkdir -p "$d"
      (cd "$s" && for f in *; do [ -n "$excl" ] && [ "$f" = "$excl" ] && continue; cp -r "$f" "$d/$f"; done)
    else mkdir -p "$(dirname "$d")"; cp "$s" "$d"; fi ;;
  rm) rm -rf "$(loc "${args[0]}")" ;;
  *) exit 2 ;;
esac
