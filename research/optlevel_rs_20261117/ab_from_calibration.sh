#!/usr/bin/env bash
# C-versus-Rust per class, from one `optlevel calibrate --c-kernels` document: the variants of a class run
# interleaved inside each repeat, single-thread, pinned.  Best C build against best Rust isa, by maximum
# (the ceiling rule) and by median, with each one's spread (max - min)/median.
# usage: ab_from_calibration.sh CAL.json
set -euo pipefail
jq -r '
  def best(f): map(select(.key | split("@")[1] | f)) | max_by(.value.max);
  def row(x): [x.key, x.value.max, x.value.median, ((x.value.max - x.value.min) / x.value.median), x.value.n];
  (["class", "best C variant", "C max", "C median", "C spread", "n", "best Rust variant", "Rust max", "Rust median", "Rust spread", "n", "Rust/C max", "Rust/C median", "kept"] | @tsv),
  (.compute | to_entries[] | .key as $cls | (.value.variants | to_entries) as $v
   | ($v | best(startswith("c-"))) as $c | ($v | best(startswith("c-") | not)) as $r
   | ([$cls] + row($c) + row($r) + [($r.value.max / $c.value.max), ($r.value.median / $c.value.median), (if $r.value.max >= $c.value.max then "Rust" else "C" end)]) | @tsv),
  (.memory.bandwidth_variants | to_entries[] | .key as $k | (.value | to_entries) as $v
   | ($v | map(select(.key | endswith("@c-native"))) | max_by(.value.max)) as $c | ($v | map(select(.key | endswith("@c-native") | not)) | max_by(.value.max)) as $r
   | (["bandwidth B/s, " + $k + " core(s)"] + row($c) + row($r) + [($r.value.max / $c.value.max), ($r.value.median / $c.value.median), (if $r.value.max >= $c.value.max then "Rust" else "C" end)]) | @tsv)
' "$1"
