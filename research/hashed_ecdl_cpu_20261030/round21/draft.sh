#!/usr/bin/env bash
# Materialize metadata only, after prepare.sh; neither freeze nor execute.
set -euo pipefail
repo=$(git rev-parse --show-toplevel)
study="$repo/research/hashed_ecdl_cpu_20261030/round21"
old="$repo/research/hashed_ecdl_cpu_20261030/round20"
test ! -e "$study/protocol.draft.json"
cp "$old/analyze.rs" "$old/run.sh" "$study/"
ch=$(sha256sum "$study/candidate-materialization.json" | cut -d ' ' -f1)
ph=$(sha256sum "$study/previous-materialization.json" | cut -d ' ' -f1)
zh=$(sha256sum "$study/inverse-materialization.json" | cut -d ' ' -f1)
candidate=$(cat "$study/candidate-materialization.json")
previous=$(cat "$study/previous-materialization.json")
inverse=$(cat "$study/inverse-materialization.json")
jq --argjson candidate "$candidate" --argjson previous "$previous" --argjson inverse "$inverse" \
  --arg ch "$ch" --arg ph "$ph" --arg zh "$zh" \
  -f "$study/protocol-draft.jq" "$old/protocol.json" > "$study/protocol.draft.json"
jq --argjson candidate "$candidate" --argjson previous "$previous" --argjson inverse "$inverse" \
  '.protocol_sha256=null | .candidate_source=$candidate.repository_commit | .previous_source=$previous.repository_commit | .inverse_source=$inverse.repository_commit | .candidate_materialization=$candidate | .previous_materialization=$previous | .inverse_materialization=$inverse | .binaries.candidate=$candidate.binary_sha256 | .binaries.previous=$previous.binary_sha256 | .binaries.inverse=$inverse.binary_sha256' \
  "$old/build.json" > "$study/build.json"
git diff 2f08282ec3f5ee9b16f61795421e4871a1282800 5ad00c65dc4f06d66c8f09abb9ca4a265783c20d -- src/cryptanalysis/hashed_ecdl.rs crates/hashed-ecdl-cpu crates/hashed-ecdl-tensor > "$study/pivot-source-delta.txt"
git diff 5b37f957e00d2347210f9bcadaeeac09a895ce8f 2f08282ec3f5ee9b16f61795421e4871a1282800 -- src/cryptanalysis/hashed_ecdl.rs crates/hashed-ecdl-cpu crates/hashed-ecdl-tensor > "$study/replay-source-delta.txt"
