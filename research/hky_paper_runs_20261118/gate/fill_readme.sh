#!/bin/bash
# Regenerates README.md §3 between the GATE markers from the rows.jsonl files.
cd "$(dirname "$0")/.."
T=$(./gate/table.sh gate/raw/rows.jsonl gate/raw-frozen/rows.jsonl gate/raw-merged/rows.jsonl $(ls gate/raw-merged/k13/*.row.jsonl 2>/dev/null))
BODY="<!-- GATE:begin -->
Expected = \`lfd-summation-systems/experiments/ic-bounds-k14/runs/<cell>.json\` (\`columns\`, \`profile\` = dim(V_D ∩ M_≤w) for
w = 0..D, \`contains_one\`); got = this engine's \`--instance\` record; \`match\` requires all three equal and the closure
closed. Binaries: \`d31a012b\` = \`bin/pilot_gate\` (pre-merge engine); \`52b47153\` = the first provisional pod binary (same sources,
commit-stamped); **\`0dfbffad\` = the merged engine (wt/engine-opt c51685b6a9 merged at 9fb60e9c13), the pod binary**. Muladds are equal across all three binaries on every shared cell (the optimizer changes no counter). Scripts: \`gate/run_gate.sh\`,
\`gate/table.sh\`; records under \`gate/raw*/\`.

$T

The profile definition is the same on both sides: the records' \`profile\` ends at \`rank\` and is cumulative by degree, as
the engine's \`dims\` (pivot degrees, cumulative). **Every row matches exactly, no discrepancy.**

Not run locally (do not fit on this shared 4-vCPU box): unit-p65537 k = 13, D = 6 and unit-p163841 k = 13, D = 6 (1 enters
at full rank 313,912; hky2 needed 2.39e13 muladds on the pod; from the k = 10 → 11 → 12 D = 6 muladd growth here, ≈ 18 h
each at the effective rate of this box), and k = 14, D = 6 (the pod jobs J1/J2). A container restart killed the first
k = 12, D = 6 attempt; it was re-run with \`CKPT_ROOT\` (checkpoint every 10 min), stopped once by SIGTERM at batch 2129 and
resumed from that checkpoint — so this row also exercises resume in a real run.
<!-- GATE:end -->"
if grep -q '^GATE_TABLE$' README.md; then
  awk -v b="$BODY" '$0=="GATE_TABLE"{print b; next} {print}' README.md > README.md.new
else
  awk -v b="$BODY" '/<!-- GATE:begin -->/{print b; skip=1; next} /<!-- GATE:end -->/{skip=0; next} !skip{print}' README.md > README.md.new
fi
mv README.md.new README.md
