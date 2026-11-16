#!/usr/bin/env bash
# Every tracked file of the study directories (and the inputs read across
# studies) in the working tree equals its blob at the validated commit.
C=${1:-79c91dd9136662ed4ee007852ca5bc6abe415c6d}
cd /home/user/crypto
n=0; bad=0
for f in $(git ls-tree -r --name-only $C -- research/ic_positive_control_lucas_20261116 research/ic_positive_control_la_20261116 research/ic_positive_control_unit_t_20261116 research/ic_positive_control_capped_20261116 research/ic_positive_control_20261112 benchmarks/records/20261116T0122Z-ic-positive-control-lucas.json benchmarks/records/20261116T0636Z-ic-positive-control-la.json benchmarks/records/20261116T0751Z-ic-positive-control-unit-t.json benchmarks/records/20261116T1001Z-ic-positive-control-capped.json); do
  n=$((n+1))
  a=$(git show $C:$f | sha256sum | cut -d' ' -f1); b=$(sha256sum < $f | cut -d' ' -f1)
  if [ "$a" != "$b" ]; then echo "DIFF $f"; bad=$((bad+1)); fi
  echo "$a  $f"
done > /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator/out/pinned_sha256_$(echo $C | cut -c1-10).txt
grep -c . /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator/out/pinned_sha256_$(echo $C | cut -c1-10).txt
grep DIFF /tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator/out/pinned_sha256_$(echo $C | cut -c1-10).txt || echo "all files equal to $C"
