#!/bin/bash
set -euo pipefail
source_root=${1:?pass the original round1 research directory}
for fixture_order in 359 479 683; do
 jq -nr --argjson r "$fixture_order" --slurpfile f "$source_root/larger-primefield-fixtures/recovery-inputs.json" --slurpfile s "$source_root/certified-exclusion-recovery/runs/r${fixture_order}-candidate-setup.json" '$f[0].fixtures[]|select(.curve.r==$r)|.curve as $c|[$c.p,$c.a,$c.b,$c.r,$c.P[0],$c.P[1],$s[0].source.certificate.progression[0].value,($s[0].source.certificate.division_values|map("\(.index):\(.value)")|join(","))]|@tsv'
done
