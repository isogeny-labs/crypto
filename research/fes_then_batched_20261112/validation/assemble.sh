#!/usr/bin/env bash
# Assemble replay-certificate.json and validator-report.json from the files
# validate.sh generated (thin orchestration: hashes in bash, structure in jq).
#
#   research/fes_then_batched_20261112/validation/assemble.sh META.json
#
# META.json (written by the validator at replay time) carries the snapshot,
# the replay binary and build facts, the replay command and the host.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
study="$(cd "$here/.." && pwd)"
repo="$(cd "$study/../.." && pwd)"
meta="$1"
names="koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23"
h() { sha256sum "$1" | cut -d' ' -f1; }
raw="$(for n in $names; do
  printf '{"path":"results/%s.json","repo_path":"research/fes_then_batched_20261112/results/%s.json","sha256":"%s"}\n' \
    "$n" "$n" "$(h "$study/results/$n.json")"; done | jq -s '.')"
scripts="$(for f in recompute.jq headline.jq band.jq counters.jq instance_check.jq compare_replay.jq assemble_certificate.jq assemble_report.jq \
                    make_validation_json.jq recompute.sh validate.sh assemble.sh claimed-readme-table.json findings.json replay-meta.json; do
  printf '{"path":"validation/%s","sha256":"%s"}\n' "$f" "$(h "$here/$f")"; done | jq -s '.')"
outputs="$(for f in recompute.json headline.json band.json counters.json instance-check.json; do
  printf '{"path":"validation/%s","sha256":"%s"}\n' "$f" "$(h "$here/$f")"; done | jq -s '.')"
receipts="$(for n in $names; do
  printf '{"name":"%s","replay_report":"validation/replay/%s.json","replay_report_sha256":"%s","compare":"validation/replay/compare-%s.json","compare_sha256":"%s"}\n' \
    "$n" "$n" "$(h "$here/replay/$n.json")" "$n" "$(h "$here/replay/compare-$n.json")"; done | jq -s '.')"
inputs="$(jq -n --arg readme "$(h "$study/README.md")" --arg results_md "$(h "$study/RESULTS.md")" \
  --arg calib "$(h "$repo/docs/ic/calibration.json")" \
  --arg rec "$(h "$repo/benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json")" \
  --arg sha "$(h "$study/results/ic-sha256.txt")" --arg summ "$(h "$study/results.json")" '[
  {path: "README.md", repo_path: "research/fes_then_batched_20261112/README.md", sha256: $readme, role: "claimed side: the table at lines 53-58 (transcribed in validation/claimed-readme-table.json) and the distance bullet"},
  {path: "RESULTS.md", sha256: $results_md, role: "read only for its S/rho column (F5)"},
  {path: "../../docs/ic/calibration.json", repo_path: "docs/ic/calibration.json", sha256: $calib, role: "pinned ratios, cross-checked against the Koblitz reports"},
  {path: "../../benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json", repo_path: "benchmarks/records/20261112T2009Z-fes-then-batched-e2e.json", sha256: $rec, role: "provenance (commit, dirty flag, command, raw-data digests)"},
  {path: "results/ic-sha256.txt", sha256: $sha, role: "the producer binary digest"},
  {path: "results.json", sha256: $summ, role: "producer summary: NOT read by the recompute"}]')"
# Collect the six comparisons into one array.
cmp="$(jq -s '.' $(for n in $names; do echo "$here/replay/compare-$n.json"; done))"
jq -n --slurpfile m "$meta" --slurpfile rr "$here/replay/recompute-replay.json" \
  --argjson cmp "$cmp" --argjson receipts "$receipts" --argjson scripts "$scripts" \
  -f "$here/assemble_certificate.jq" >"$here/replay-certificate.json"
jq -n --slurpfile m "$meta" --slurpfile head "$here/headline.json" --slurpfile rec "$here/recompute.json" \
  --slurpfile band "$here/band.json" --slurpfile inst "$here/instance-check.json" \
  --slurpfile find "$here/findings.json" --argjson raw "$raw" --argjson inputs "$inputs" \
  --argjson scripts "$scripts" --argjson outputs "$outputs" \
  --arg cert_sha "$(h "$here/replay-certificate.json")" \
  -f "$here/assemble_report.jq" >"$here/validator-report.json"
echo "replay-certificate.json $(h "$here/replay-certificate.json")"
echo "validator-report.json   $(h "$here/validator-report.json")"
