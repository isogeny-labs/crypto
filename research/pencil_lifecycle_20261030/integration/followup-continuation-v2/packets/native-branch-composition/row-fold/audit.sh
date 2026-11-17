#!/usr/bin/env bash
set -euo pipefail
cd /var/tmp/pencil-kummer-row-fold-20261101-v1
jq -Rn '[inputs | capture("^\u0027(?<declaration>[^\u0027]+)\u0027 depends on axioms: \\[(?<names>.*)\\]$") | {declaration,axioms:(.names|split(", ")|map(select(length>0)))}]' Readback.stdout > axioms.json
jq -e 'length==22 and ([.[].declaration]|unique|length)==22 and all(.[]; all(.axioms[]; .=="propext" or .=="Quot.sound" or .=="Classical.choice"))' axioms.json > axiom-check.txt
for module in Comparator WordComparator Repair RowFold Controls Readback; do jq -e '.native_exit==0' "$module.receipt.json" > /dev/null; done
expected=$(jq -r '.files[]|select(.path=="walk-source.rs")|.sha256' fixed133-binding.json)
printf '%s  snapshot/walk-source.rs.txt\n' "$expected" | sha256sum -c - > source-binding-check.txt
