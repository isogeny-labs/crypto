#!/bin/bash
# Local-directory stand-in for `aws s3 cp|sync|ls`: s3://<bucket>/<key> maps to $FAKE_S3/<bucket>/<key>.
m() { case "$1" in s3://*) echo "$FAKE_S3/${1#s3://}";; *) echo "$1";; esac; }
op=$1; shift; args=(); for a in "$@"; do case "$a" in --only-show-errors) ;; *) args+=("$a");; esac; done
case $op in
  cp) src=$(m "${args[0]}"); dst=$(m "${args[1]}"); [ -f "$src" ] || exit 1; mkdir -p "$(dirname "$dst")"; cp "$src" "$dst.tmp.$$" && mv "$dst.tmp.$$" "$dst" ;;
  sync) src=$(m "${args[0]}"); dst=$(m "${args[1]}"); mkdir -p "$dst"; [ -d "$src" ] && cp -a "$src/." "$dst/" ;;
  ls) d=$(m "${args[0]}"); d=${d%/}; [ -d "$d" ] || exit 1; for f in "$d"/*; do [ -f "$f" ] && echo "2026-11-10 00:00:00 $(stat -c %s "$f") $(basename "$f")"; done ;;
esac
