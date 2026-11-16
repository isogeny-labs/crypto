#!/bin/sh
# The accepted prime-run candidates of the 2026-10-16 census, published and
# skipped: models (Rust), recount and checks (GP). Thin orchestration only.
#   research/nist_binary_skipped_20261020/prime_skipped.sh
set -eu
here=$(cd "$(dirname "$0")" && pwd)
census="$here/../nist_seed_provenance_20261016/census"
out="$here/census/prime_models.txt"
: > "$out"
# run: accepted candidates, from ../nist_seed_provenance_20261016/census/summary.json
for spec in 863711343:36,1562,2185,2320,2491,2682,2863,3048 863966734:139,2198,5228,5405,6013 \
            871002724:1136 871696354:876,1842 872952608:2688 891607927:777,1485,5736,7026; do
  run=${spec%%:*}; list=${spec#*:}
  models=$(mktemp)
  cargo run --quiet --release --example nist_prime_candidate_models -- --run "$run" --candidates "$list" > "$models"
  printf 'MODELS="%s";\nORDERS="%s";\nRUN="%s";\n' "$models" "$census/orders_$run.txt" "$run" \
    | cat - "$here/prime_skipped.gp" | gp -q 2>/dev/null | grep '^model ' >> "$out"
done
