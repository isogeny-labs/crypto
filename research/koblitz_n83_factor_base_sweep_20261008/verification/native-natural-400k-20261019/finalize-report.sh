#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-400k-20261019
study=research/koblitz_n83_factor_base_sweep_20261008
proof="$study/verification/native-natural-400k-20261019"
report="$study/reports/run-20261019T2015Z"
"$work/bin/runner-v11" summarize-xor "$work/manifest-corrected.json" "$work/summary.json"
mkdir -p "$report" "$proof/cards" "$proof/records"
cp "$work/summary.json" "$proof/SUMMARY.json"
cp "$work/cards/"* "$proof/cards/"
cp "$work/report/"* "$report/"
cp "$work/summary.json" "$report/figure-data.json"
jq -r '"Recorded new outcomes (native audit; n=1 each):",(.rows[]|select(.id=="k2048-400k" or .id=="k4096-400k")|"K\(.columns): \(.status); terminal conflicts \(.terminal_counters.conflicts), models \(.terminal_counters.models), relations \(.terminal_counters.relations), rank \(.terminal_counters.rank); solver \(.natural_solver_ms) ms, outer \(.outer_wall_ms) ms.")' "$work/summary.json" > "$report/outcomes.typ"
cat >> "$report/REPORT.md" <<'TEXT'

## Recorded new outcomes

| run | native status | terminal conflicts | models | relations | rank | native solver ms | supervised outer ms |
|:--|:--|--:|--:|--:|--:|--:|--:|
TEXT
jq -r '.rows[]|select(.id=="k2048-400k" or .id=="k4096-400k")|"| \(.id) | \(.status) | \(.terminal_counters.conflicts) | \(.terminal_counters.models) | \(.terminal_counters.relations) | \(.terminal_counters.rank) | \(.natural_solver_ms) | \(.outer_wall_ms) |"' "$work/summary.json" >> "$report/REPORT.md"
cat > "$proof/RESULT.md" <<'TEXT'
# Native N83 400000-conflict continuation

[Full report](../../reports/run-20261019T2015Z/REPORT.md),
[inspected PDF](../../reports/run-20261019T2015Z/report.pdf),
[native summary](SUMMARY.json), [cards and assessments](cards/),
[compact records](records/), [complete raw S3 publication](PUBLICATION.md).

New caps400000conflicts/2400seconds each; identical retained worker and query,
strict full core14/30 reservation. Historical observations remain immutable.
Every actual attempt/failure/censor is retained. One K4096 orchestration failure is preserved; its separately assessed repeat
raises maximum cumulative worker allocation to7200seconds across3attempts.
Unknown stays bounded, with
no UNSAT, best-base, fastest-tool or unknown-scalar recovery inference.
TEXT
cat "$report/outcomes.typ" >> "$proof/RESULT.md"
for id in k2048-v10 k4096-v11; do
 mkdir -p "$proof/records/$id"
 for name in audit.json outer.json config.json spec.json isolation-before.json isolation-after.json inspect-live.json inspect-final.json custody.json; do
  cp "$proof/exec/$id/$name" "$proof/records/$id/"
 done
done
/tmp/report-venv/bin/python "$report/render.py"
mkdir -p benchmarks/20261019-n83-natural-400k
cp "$report/report.pdf" "$report/"*.svg "$report/"*.png benchmarks/20261019-n83-natural-400k/
