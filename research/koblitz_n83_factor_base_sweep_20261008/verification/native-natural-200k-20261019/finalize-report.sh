#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-200k-20261019
study=research/koblitz_n83_factor_base_sweep_20261008
proof="$study/verification/native-natural-200k-20261019"
report="$study/reports/run-20261019T1922Z"
"$work/bin/runner-analysis-v9" summarize-xor "$work/manifest-completed.json" "$work/summary-analysis-v9.json"
mkdir -p "$report" "$proof/cards" "$proof/records"
cp "$work/summary-analysis-v9.json" "$proof/SUMMARY.json"
cp "$work/cards/"* "$proof/cards/"
cp "$work/report/"* "$report/"
cp "$work/summary-analysis-v9.json" "$report/figure-data.json"
jq -r '"Recorded new outcomes (native audit; n=1 each):",(.rows[]|select(.id=="k2048-200k" or .id=="k4096-200k")|"K\(.columns): \(.status); terminal conflicts \(.terminal_counters.conflicts), models \(.terminal_counters.models), relations \(.terminal_counters.relations), rank \(.terminal_counters.rank); solver \(.natural_solver_ms) ms, outer \(.outer_wall_ms) ms.")' "$work/summary-analysis-v9.json" > "$report/outcomes.typ"
cat >> "$report/REPORT.md" <<'TEXT'

## Audited recorded outcomes

| run | native status | terminal conflicts | models | relations | rank | native solver ms | supervised outer ms |
|:--|:--|--:|--:|--:|--:|--:|--:|
TEXT
jq -r '.rows[]|select(.id=="k2048-200k" or .id=="k4096-200k")|"| \(.id) | \(.status) | \(.terminal_counters.conflicts) | \(.terminal_counters.models) | \(.terminal_counters.relations) | \(.terminal_counters.rank) | \(.natural_solver_ms) | \(.outer_wall_ms) |"' "$work/summary-analysis-v9.json" >> "$report/REPORT.md"
cat > "$proof/RESULT.md" <<'TEXT'
# Native N83 200000-conflict continuation

[Full report](../../reports/run-20261019T1922Z/REPORT.md),
[inspected PDF](../../reports/run-20261019T1922Z/report.pdf),
[native summary](SUMMARY.json), [cards and assessments](cards/),
[compact records](records/), [complete raw S3 publication](PUBLICATION.md).

New caps200000conflicts/1200seconds each, identical retained worker and query.
Full core14/30 is separately frozen; earlier observations stay immutable.
Both actual setup failures and their assessments/correction are preserved.
Unknowns stay bounded, with no UNSAT, best-base, fastest-tool or recovery claim.
TEXT
cat "$report/outcomes.typ" >> "$proof/RESULT.md"
for id in k2048-v8b k4096-v8; do
 mkdir -p "$proof/records/$id"
 for name in audit.json outer.json config.json spec.json isolation-before.json isolation-after.json inspect-live.json inspect-final.json custody.json; do
  cp "$proof/exec/$id/$name" "$proof/records/$id/"
 done
done
/tmp/report-venv/bin/python "$report/render.py"
mkdir -p benchmarks/20261019-n83-natural-200k
cp "$report/report.pdf" "$report/"*.svg "$report/"*.png benchmarks/20261019-n83-natural-200k/
