#!/usr/bin/env bash
# Render the short human-readable VALIDATOR_REPORT.md from the two validator
# JSON files. Usage: write_md.sh OUTDIR
set -euo pipefail
OUT=$1; C=$OUT/replay-certificate.json; V=$OUT/validator-report.json
hv() { jq -r --arg n "$1" '.headline[] | select(.name == $n) | "\(.claimed) | \(.recomputed)"' "$V"; }
civ() { jq -r --arg re "$1" --argjson d "$2" '[.headline[] | select(.name | test($re))]
  | "[\(.[0].claimed * $d | round / $d), \(.[1].claimed * $d | round / $d)] | [\(.[0].recomputed * $d | round / $d), \(.[1].recomputed * $d | round / $d)]"' "$V"; }

SNAP=$(jq -r .snapshot_commit "$V"); VID=$(jq -r .validation_report.id "$V")
CSTAT=$(jq -r .status "$C"); VSTAT=$(jq -r .status "$V"); VWHY=$(jq -r .status_reason "$V")
CWHY=$(jq -r '"\(.totals.sessions) of 79 sessions replayed (seeded stratified sample: every volcano x family stratum, plus the three V3 crater sessions with rho-cubic); \(.totals.replays_identical) of \(.totals.measured_runs_replayed) measured runs re-executed with identical counts; \(.totals.records_scalar_verified) of \(.totals.records_audited) records scalar-verified ([k]G = Q); \(.totals.sessions_replay_all) sessions with --replay-all; --replay 50 fallback: \(if (.totals.sessions_replay_50_fallback | length) == 0 then "none" else (.totals.sessions_replay_50_fallback | join(", ")) end); archive hashes and receipt file maps all match"' "$C")
if [ -f "$OUT/VALIDATION.json" ]; then VJ="written"; else VJ="**not written**"; fi
BIN=$(jq -r .binary.sha256 "$C" | cut -c1-16)
NREC=$(jq -r .raw_record_rederivation.records_compared "$C")
UNCOK=$(jq -r 'if .raw_record_rederivation.all_match then "0 mismatches" else "MISMATCHES, see replay-certificate.json" end' "$C")
HSUM=$(jq -r '.headline_summary | "\(.numbers) numbers compared, \(.agree) agree, \(.disagree) disagree; largest relative difference of a point estimate \(.max_rel_diff_point_estimates); bootstrap endpoints within \(.bootstrap_endpoints_max_mc_z * 100 | round / 100) Monte Carlo SD of the difference (tolerance 4)."' "$V")
FIND=$(jq -r '.findings[] | "- **\(.id) (\(.severity | gsub("_"; " ")))**, \(.where). Report: \(.claim) Evidence: \(.evidence) *Fix:* \(.fix)."' "$V")
LIM=$(jq -r '.limitations[] | "- \(.)"' "$V")
CUBIC=$(jq -r '.rho_reference_audit.V3_crater.ic_over_rho_cubic_D_plus_N_pairs_2000 | "\(.["0"] * 10 | round / 10)x / \(.["0.1"] * 10 | round / 10)x / \(.["1"] * 10 | round / 10)x"' "$V")

cat > "$OUT/VALIDATOR_REPORT.md" <<EOF
# Validator report: volcano ECDLP census

Study \`research/volcano_ecdlp_census_20261017/\`, PR aburan28/crypto#2402, branch
\`claude/volcano-ecdlp-census-phase-b-20261022\` at \`$SNAP\`.
Validation ID \`$VID\`. The validator did not write the study and did not open or
run any producer code (\`analysis/*.py\`); every number below was recomputed from
committed raw data with validator-written shell, mawk and jq in
\`research/volcano_ecdlp_census_20261017/validation/\`.

## Verdicts

| file | status | why |
|:--|:--|:--|
| \`replay-certificate.json\` | **$CSTAT** | $CWHY |
| \`validator-report.json\` | **$VSTAT** | $VWHY |
| \`VALIDATION.json\` | $VJ | the gate needs both files to pass |

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

All 35 R(L) values, all 51 IC/rho cells (levels and pooled, three prices), the 24
paired-source ratios, the 20 V3 descriptive ratios, the 6 curve-effect F statistics,
the A/A ratios, L2 fractions and uncharged counts are in \`validator-report.json\`
(\`headline\`). REPORT.md's 110 table cells in sections 4, 6 and 7 equal
results.json as printed.

## Findings

$FIND

## Audits

- **Verdict vocabulary** follows ANALYSIS_PLAN (equivalent / different / inconclusive, pass/fail, descriptive, distance remaining), apart from the F5 label.
- **Amendment A2** changes no estimand, margin, family, test or threshold. A2.3 and A2.4 were appended after the final analysis run: results.json pins the A2.1-A2.2 version of AMENDMENTS.md (\`deb547a3...\`, commit 89e88b1f94); A2.3-A2.4 first appear with results.json in 6ca47a183f. The one interpretation that decided a cell (the transport control) cannot be shown to predate the smoke run, but the conclusion does not depend on it.
- **Matched rho reference.** Negation is the strongest available on every V1 and V2 vertex and on the 15 non-crater V3 vertices (none has j = 0 or 1728). On the V3 j = 0 crater the strongest is the automorphism fold rho-cubic; the report uses rho-neg there (F1). IC / rho-cubic on the crater: $CUBIC at 0 / 0.1 / 1 GAE.
- **AGENTS.md section 5a band** is applied to both arms with their named unpriced counters (ic: five counters; rho-neg and rho-cubic: canonicalisations). Rho is ahead at every price on every volcano and level; both arms remain lower bounds.

## What passing needs

An erratum record for REPORT.md (and an A3 amendment for the AMENDMENTS.md
statements in F6) that reports the V3 crater gap against rho-cubic and withdraws
the j = 0 interpretation (F1, F2), with the minor corrections F3-F8. Every number
already recomputes, so re-validation is \`validation/build_validation.sh\` on the
corrected snapshot.

## Limitations

$LIM
EOF
