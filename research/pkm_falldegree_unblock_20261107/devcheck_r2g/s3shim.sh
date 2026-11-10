#!/bin/bash
# Local-directory stand-in for `aws s3 cp|sync`: s3://<bucket>/<key> maps to $FAKE_S3/<bucket>/<key> (a "//" in the key
# collapses in the file system; set SHIM_LOG=<file> to log every raw URI instead). Test switches:
#   SHIM_READS_STDIN=1       read all of stdin first (like a command that consumes the caller's stdin)
#   SHIM_FAIL_PROGRESS=<f>   `cp` to a destination named progress.txt exits 1 while file <f> exists
#   SHIM_FAIL_SYNC_RUNS=<f>  `sync` to a destination ending in /runs exits 1 while file <f> exists
[ "${SHIM_READS_STDIN:-0}" = 1 ] && cat > /dev/null
m() { case "$1" in s3://*) echo "$FAKE_S3/${1#s3://}";; *) echo "$1";; esac; }
op=$1; shift; args=(); for a in "$@"; do case "$a" in --only-show-errors) ;; *) args+=("$a");; esac; done
[ -n "${SHIM_LOG:-}" ] && echo "$op ${args[*]}" >> "$SHIM_LOG"
case $op in
  cp) src=$(m "${args[0]}"); dst=$(m "${args[1]}")
      if [ -n "${SHIM_FAIL_PROGRESS:-}" ] && [ -e "$SHIM_FAIL_PROGRESS" ] && [ "$(basename "$dst")" = progress.txt ]; then echo "shim: injected failure for $dst" >&2; exit 1; fi
      [ -f "$src" ] || exit 1; mkdir -p "$(dirname "$dst")"; cp "$src" "$dst.tmp.$$" && mv "$dst.tmp.$$" "$dst" ;;
  sync) src=$(m "${args[0]}"); dst=$(m "${args[1]}")
      if [ -n "${SHIM_FAIL_SYNC_RUNS:-}" ] && [ -e "$SHIM_FAIL_SYNC_RUNS" ] && [ "${dst%/}" != "${dst%/runs}" ]; then echo "shim: injected failure for $dst" >&2; exit 1; fi
      mkdir -p "$dst"; [ -d "$src" ] && cp -a "$src/." "$dst/" ;;
  *) echo "shim: unsupported op $op" >&2; exit 2 ;;
esac
