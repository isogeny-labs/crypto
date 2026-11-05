#!/bin/bash
set -euo pipefail
source_root=${1:?pass original round1 directory}
mode=${2:?pass points or slots}
case_file="$source_root/certified-exclusion-recovery/runs/r479-candidate-cases.jsonl"
if [ "$mode" = points ]; then
 jq -r 'select(.k==0 or .k==1 or .k==2 or .k==478)|.k as $id|.geometry.snapshots[]|[$id,.offset,(if .point==null then -1 else .point[0] end),(if .point==null then -1 else .point[1] end)]|@tsv' "$case_file"
elif [ "$mode" = slots ]; then
 jq -r 'select(.k==0 or .k==1 or .k==2 or .k==478)|.k as $id|.geometry.slots[]|[$id,.offset,.slot,.known_multiplier,.projective[],.affine[],.inverse_z]|@tsv' "$case_file"
else exit 2
fi
