#!/usr/bin/env bash
# Wait until no other rustc is compiling crypto_lib (shared 16 GB host; two
# concurrent lib builds OOM), then build each tree in turn.
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator
for c in "$@"; do
  while pgrep -f "rustc --crate-name crypto_lib" >/dev/null; do sleep 15; done
  $V/scripts/build_all.sh $c
done
