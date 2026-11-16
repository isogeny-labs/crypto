#!/bin/bash
set -euo pipefail
source_root=${1:?pass original round1 directory}
mode=${2:?pass fixtures or slots}
for fixture_order in 359 479 683; do
 if [ "$mode" = fixtures ]; then
  jq -nr --argjson r "$fixture_order" --slurpfile f "$source_root/larger-primefield-fixtures/recovery-inputs.json" --slurpfile s "$source_root/certified-exclusion-recovery/runs/r${fixture_order}-candidate-setup.json" '$f[0].fixtures[]|select(.curve.r==$r)|.curve as $c|$s[0].source.certificate as $g|[$c.p,$c.a,$c.b,$c.r,$c.P[0],$c.P[1],$g.width,$g.blocks,($g.tail/2|floor),($g.tail%2)]|@tsv'
 elif [ "$mode" = slots ]; then
  jq -r --argjson r "$fixture_order" '.source.certificate.geometry.slots|to_entries[]|[$r,.key,.value.known_multiplier,.value.projective[],.value.affine[],.value.inverse_z]|@tsv' "$source_root/certified-exclusion-recovery/runs/r${fixture_order}-candidate-setup.json"
 else
  exit 2
 fi
done
