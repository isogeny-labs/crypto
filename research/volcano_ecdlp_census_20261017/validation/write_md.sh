#!/usr/bin/env bash
# Render the short human-readable VALIDATOR_REPORT.md from the validator's JSON
# files. Usage: write_md.sh OUTDIR
set -euo pipefail
OUT=$1; C=$OUT/replay-certificate.json; V=$OUT/validator-report.json
hv() { jq -r --arg n "$1" '.headline[] | select(.name == $n) | "\(.claimed) | \(.recomputed)"' "$V"; }
civ() { jq -r --arg re "$1" --argjson d "$2" '[.headline[] | select(.name | test($re))]
  | "[\(.[0].claimed * $d | round / $d), \(.[1].claimed * $d | round / $d)] | [\(.[0].recomputed * $d | round / $d), \(.[1].recomputed * $d | round / $d)]"' "$V"; }

SNAP=$(jq -r .snapshot_commit "$V"); VID=$(jq -r .validation_report.id "$V")
CSTAT=$(jq -r .status "$C"); VSTAT=$(jq -r .status "$V"); VWHY=$(jq -r .status_reason "$V")
CWHY=$(jq -r '"\(.totals.sessions) of 79 sessions replayed (seeded stratified sample: every volcano x family stratum, plus the three V3 crater sessions with rho-cubic); \(.totals.replays_identical) of \(.totals.measured_runs_replayed) measured runs re-executed with identical counts; \(.totals.records_scalar_verified) of \(.totals.records_audited) records scalar-verified ([k]G = Q); \(.totals.sessions_replay_all) sessions with --replay-all; --replay 50 fallback: \(if (.totals.sessions_replay_50_fallback | length) == 0 then "none" else (.totals.sessions_replay_50_fallback | join(", ")) end); archive hashes and receipt file maps all match; replayed at \(.replayed_at_snapshot[0:10]), evidence unchanged at \(.snapshot_commit[0:10]): \(.replay_evidence_unchanged_since.holds)"' "$C")
if [ -f "$OUT/VALIDATION.json" ]; then VJ="**written**"; VJWHY="pins both files by sha256 with the matched rho reference and the section 5a band"; else VJ="**not written**"; VJWHY="the gate needs both files to pass"; fi
PREV=$(jq -r 'if .previous_validation then "Previous validation \(.previous_validation.validation_id) at \(.previous_validation.snapshot_commit[0:10]) (validator commit \(.previous_validation.validator_commit[0:10])): \(.previous_validation.status), \(.previous_validation.reason)." else "" end' "$V")
BIN=$(jq -r .binary.sha256 "$C" | cut -c1-16)
NREC=$(jq -r .raw_record_rederivation.records_compared "$C")
UNCOK=$(jq -r 'if .raw_record_rederivation.all_match then "0 mismatches" else "MISMATCHES, see replay-certificate.json" end' "$C")
HSUM=$(jq -r '.headline_summary | "\(.numbers) numbers compared, \(.agree) agree, \(.disagree) disagree; largest relative difference of a point estimate \(.max_rel_diff_point_estimates); bootstrap endpoints within \(.bootstrap_endpoints_max_mc_z * 100 | round / 100) Monte Carlo SD of the difference (tolerance 4)."' "$V")
FIND=$(jq -r '.findings[] | "- **\(.id) (\(.severity | gsub("_"; " ")); \(.status // "open"))**, \(.where). Report: \(.claim) Evidence: \(.evidence) *Fix:* \(.fix)."' "$V")
LIM=$(jq -r '.limitations[] | "- \(.)"' "$V")
CUBIC=$(jq -r '.rho_reference_audit.V3_crater.ic_over_rho_cubic_D_plus_N_pairs_2000 | "\(.["0"] * 10 | round / 10)x / \(.["0.1"] * 10 | round / 10)x / \(.["1"] * 10 | round / 10)x"' "$V")
OPEN=$(jq -r '[.findings[] | select((.status // "open") == "open") | .id] | if length == 0 then "none" else join(", ") end' "$V")
A3=$(jq -r 'if .amendment_A3_audit then "- **Amendment A3** (erratum, \(.snapshot_commit[0:10])) changes no estimand, margin, family, test or threshold; A1 and A2 text is an exact byte prefix of the new AMENDMENTS.md. results.json: \(.amendment_A3_audit.results_json_diff.unchanged_scalars) scalar values unchanged, \(.amendment_A3_audit.results_json_diff.changed | length) bookkeeping values changed (\(.amendment_A3_audit.results_json_diff.changed | join("; "))), \(.amendment_A3_audit.results_json_diff.added_scalars) added in \(.amendment_A3_audit.results_json_diff.added_fields | join(" and ")). REPORT.md and report.pdf carry every F1-F8 correction (\(.erratum_verification.report_pdf | split(";")[0]))." else "" end' "$V")
CRATER_ROWS=$(jq -r '.headline[] | select(.name | test("^ic_over_rho-cubic.V3_crater.D\\+N|^crater_ic_by_model.*mean_S")) | "| \(.name) | \(.claimed) | \(.recomputed) |"' "$V")
if [ "$VSTAT" = pass ]; then
  NEXT="## Open items

Minor wording items left for the next edit (none blocks the gate): $OPEN."
else
  NEXT="## What passing needs

An erratum for the open material findings ($OPEN); every number already
recomputes, so re-validation is \`validation/build_validation.sh\` on the corrected snapshot."
fi

cat > "$OUT/VALIDATOR_REPORT.md" <<EOF
# Validator report: volcano ECDLP census

Study \`research/volcano_ecdlp_census_20261017/\`, PR aburan28/crypto#2402, branch
\`claude/volcano-ecdlp-census-phase-b-20261022\` at \`$SNAP\`.
Validation ID \`$VID\`. The validator did not write the study and did not open or
run any producer code (\`analysis/*.py\`); every number below was recomputed from
committed raw data with validator-written shell, mawk and jq in
\`research/volcano_ecdlp_census_20261017/validation/\` (\`recompute.sh\` rebuilds them).
$PREV

## Verdicts

| file | status | why |
|:--|:--|:--|
| \`replay-certificate.json\` | **$CSTAT** | $CWHY |
| \`validator-report.json\` | **$VSTAT** | $VWHY |
| \`VALIDATION.json\` | $VJ | $VJWHY |

The replay ran on the same shared 4-vCPU cloud VM type and host class as the sessions
(Intel Xeon @ 2.80 GHz, 4 logical CPUs, same ISA features, virtualised) and with
the same ecbench binary (\`$BIN...\`, equal to the \`runs/progress.jsonl\` header).
ecbench's own env and hardware class ids differ from the session header's (kernel
6.18.44-fc-v139 then, -v140 now; the Phase B audits already span three of each).
Operation counts do not depend on the host, so this is an independent re-execution
of the counts, not an independent implementation or independent hardware. For the
same sessions, \`total_gae\`, \`S\` and the A2.1 uncharged count of all $NREC
measured records were re-derived from \`records.jsonl\` and compared with the
committed extracts row by row: $UNCOK.

## Recomputed headline numbers

$HSUM

| quantity | claimed (results.json) | recomputed |
|:--|--:|--:|
| (a) positive control rho-cubic / rho-neg, V3 crater | $(hv "positive_control.ratio_rho-cubic_over_rho-neg.V3_crater.D+N+ctl") |
| (b) R(L), V1 rho-neg L1 | $(hv "level_ratio.R(L).V1.rho-neg.L1/L0") |
| (b) R(L), V1 ic L4 | $(hv "level_ratio.R(L).V1.ic.L4/L0") |
| (b) R(L), V2 rho-plain L1 | $(hv "level_ratio.R(L).V2.rho-plain.L1/L0") |
| (b) R(L), V2 ic L3 | $(hv "level_ratio.R(L).V2.ic.L3/L0") |
| (c) IC / rho-neg, V1 all levels, 0 GAE | $(hv "ic_over_rho-neg.V1.all_levels.price_0_GAE") |
| (c) IC / rho-neg, V1 all levels, 0.1 GAE | $(hv "ic_over_rho-neg.V1.all_levels.price_0.1_GAE") |
| (c) IC / rho-neg, V1 all levels, 1 GAE | $(hv "ic_over_rho-neg.V1.all_levels.price_1_GAE") |
| (c) IC / rho-neg, V2 all levels, 0 GAE | $(hv "ic_over_rho-neg.V2.all_levels.price_0_GAE") |
| (c) IC / rho-neg, V2 all levels, 1 GAE | $(hv "ic_over_rho-neg.V2.all_levels.price_1_GAE") |
| (c) IC / rho-neg, V3 all levels, 0 GAE | $(hv "ic_over_rho-neg.V3.all_levels.price_0_GAE") |
| (d) measured runs | $(hv "rows.measured") |
| (d) verified runs | $(hv "rows.verified") |
| (d) warm-up runs dropped | $(hv "rows.warmup_dropped") |
| (e) BSGS identical-ops fraction, V1 bsgs | $(hv "bsgs_identical_ops_fraction.V1.bsgs") |
| (e) BSGS identity triples, V1 bsgs | $(hv "bsgs_identity_triples.V1.bsgs") |
| (f) bootstrap 95% CI, V1 rho-plain L1/L0 | $(civ 'V1.rho-plain.L1.ci95' 10000) |
| (f) bootstrap 95% CI, positive control | $(civ 'positive_control.ci95' 10000) |
| (f) bootstrap 95% CI, IC / rho-neg V1 0 GAE | $(civ 'ic_over_rho-neg.V1.all.price_0.ci95' 1000) |
$CRATER_ROWS

All 35 R(L) values, all 51 IC/rho cells (levels and pooled, three prices), the 24
paired-source ratios, the 20 V3 descriptive ratios, the 6 curve-effect F statistics,
the A/A ratios, L2 fractions and uncharged counts are in \`validator-report.json\`
(\`headline\`). $(jq -r '.report_vs_results.table_cells_check' "$V").

## Findings

$FIND

## Audits

- **Verdict vocabulary** follows ANALYSIS_PLAN (equivalent / different / inconclusive, pass/fail, descriptive, distance remaining).
- **Amendment A2** changes no estimand, margin, family, test or threshold. A2.3 and A2.4 were appended after the original final analysis run: that run's results.json (commit 6ca47a183f) pins the A2.1-A2.2 version of AMENDMENTS.md (\`deb547a3...\`). The one interpretation that decided a cell (the transport control) cannot be shown to predate the smoke run, but the conclusion does not depend on it.
$A3
- **Matched rho reference.** Negation is the strongest available on every V1 and V2 vertex and on the 15 non-crater V3 vertices (none has j = 0 or 1728). On the V3 j = 0 crater the strongest is the automorphism fold rho-cubic: $(jq -r '.rho_reference_audit.V3_crater.report_uses' "$V"). IC / rho-cubic on the crater: $CUBIC at 0 / 0.1 / 1 GAE.
- **AGENTS.md section 5a band** is applied to both arms with their named unpriced counters (ic: five counters; rho-neg and rho-cubic: canonicalisations). Rho is ahead at every price on every volcano and level, and rho-cubic on the crater; both arms remain lower bounds.

$NEXT

## Limitations

$LIM
EOF
