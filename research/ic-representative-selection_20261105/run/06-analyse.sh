#!/usr/bin/env bash
# Stage 3: the frozen analysis (PROTOCOL.md §7), masked first, then unmasked.
# The nonce was drawn and recorded in INPUTS.md after the inputs freeze; the key stays
# in $D/analysis/masked/key.json, unopened, until analysis-masked.json exists.
source "$(dirname "$0")/common.sh"
out=$D/analysis
mkdir -p $out
ls $D/sessions/*/session/records.jsonl > $out/records.list
find $D/inputs/specs -name '*.labels.jsonl' | LC_ALL=C sort > $out/labels.list
nonce=$(grep -oE 'mask nonce: [0-9a-f]{16}' $D/INPUTS.md | awk '{print $3}')
$REPSEL mask --labels @$out/labels.list --comparisons $D/comparisons.json --nonce "$nonce" --out-dir $out/masked
ls $out/masked/*.labels.jsonl > $out/masked-labels.list
seed=$(jq -er .analysis.seed $DESIGN); boot=$(jq -er .analysis.boot $DESIGN); perm=$(jq -er .analysis.perm $DESIGN)
$REPSEL analyse --records @$out/records.list --labels @$out/masked-labels.list \
  --comparisons $out/masked/comparisons.json --boot $boot --perm $perm --seed $seed --out $out/analysis-masked.json
sha256sum $out/analysis-masked.json > $out/analysis-masked.sha256
$REPSEL unmask --key $out/masked/key.json --in $out/analysis-masked.json --out $out/analysis.json
# Cross-check against the harness's own band, session by session.
for s in $D/sessions/*/session; do
  $ECBENCH band --dir "$s" --weights 0,0.1,1 --json "$out/band-$(basename "$(dirname "$s")").json" > /dev/null
done
