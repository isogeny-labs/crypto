#!/usr/bin/env bash
# research/ic_native_ifma_20261115: local run of both specs, each replay-audited; writes $1 (default
# out/ under the scratch dir given). ecbench and rootfind_serve from target/release (built from HEAD).
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"; D=research/ic_native_ifma_20261115; OUT=${1:?out dir}; E=target/release/ecbench
mkdir -p $OUT; git rev-parse --short=10 HEAD > $OUT/REV; nproc > $OUT/nproc.txt; lscpu > $OUT/lscpu.txt
sha256sum $E target/release/rootfind_serve > $OUT/sha256.txt; $E host > $OUT/host.json 2>&1
for s in ifma2 ifma3; do
  $E run --spec $D/specs/$s.json --out $OUT/sessions/$s --cpus none --allow-busy > $OUT/run-$s.log 2>&1; echo "run $s rc=$?"
  $E verify --dir $OUT/sessions/$s --replay-all --exit-code --out $OUT/audit-$s.json > $OUT/audit-$s.log 2>&1; echo "audit $s rc=$? $(tail -1 $OUT/audit-$s.log)"
  $E band --dir $OUT/sessions/$s --json $OUT/band-$s.json > $OUT/band-$s.txt 2>&1
done
(cd $(dirname $OUT) && tar czf $(basename $OUT).tgz $(basename $OUT))
