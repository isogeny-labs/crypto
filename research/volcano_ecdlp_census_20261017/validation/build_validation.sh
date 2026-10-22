#!/usr/bin/env bash
# Assemble the validator's replay certificate and validator report for
# benchmarks/20261017-volcano-ecdlp-census/ from the validator's own outputs
# (point_estimates.awk, curve_effect.awk, cluster_boot.awk, replay_sample.sh,
# unc_sample.sh, report_tables_check.sh) and the producer's claims
# (analysis/results.json). Writes VALIDATION.json only when both pass.
#
# Usage: build_validation.sh STUDY WORK SCRATCH OUTDIR NARRATIVE_JSON
#   WORK    : directory holding pe.tsv, curve_effect.tsv, band.tsv,
#             join_stats.txt, boot_q.jsonl, report_tables.txt
#   SCRATCH : directory holding replay_log.jsonl, unc_log.jsonl, sample.txt,
#             receipts/
#   NARRATIVE_JSON : findings, audits and limitations written by the validator
set -euo pipefail
STUDY=$1; W=$2; S=$3; OUT=$4; NARR=$5
REPO=$(git -C "$STUDY" rev-parse --show-toplevel)
R=$STUDY/analysis/results.json
REL=../../research/volcano_ecdlp_census_20261017
mkdir -p "$OUT/replay-receipts"

# ---- validator point estimates as one JSON object -------------------------
awk -F'\t' '{ v = $2; if (v ~ /^-?[0-9.eE+-]+$/) printf "%s\t%s\n", $1, v; else printf "%s\t\"%s\"\n", $1, v }' "$W/pe.tsv" |
  jq -R -s 'split("\n") | map(select(length > 0) | split("\t") | {(.[0]): (.[1] | fromjson)}) | add' > "$W/pe.json"
awk -F'\t' '$2 != "degenerate" && $1 ~ /^F\./ { printf "%s\t%s\n", $1, $2 }' "$W/curve_effect.tsv" |
  jq -R -s 'split("\n") | map(select(length > 0) | split("\t") | {(.[0]): (.[1] | tonumber)}) | add' > "$W/fstat.json"
jq -R -s 'split("\n") | map(select(length > 0) | split(" ") | {(.[0]): (.[1] | tonumber)}) | add' "$W/join_stats.txt" > "$W/join.json"
jq -s 'map({(.check): .}) | add' "$W/boot_q.jsonl" > "$W/boot.json"
jq -R -s 'split("\n") | map(select(length > 0) | split("\t") | . as $f
  | {(if $f[0] == "band" then "\($f[1])|\($f[2])" else "V3crater|\($f[1])" end):
     ($f[3:] | map(split("=") | {(.[0]): (.[1] | tonumber)}) | add)}) | add' "$W/band.tsv" > "$W/band.json"

# ---- replay certificate ------------------------------------------------------
BIN_SHA=$(sha256sum "$REPO/target/release/ecbench" | awk '{print $1}')
HDR_SHA=$(jq -r 'select(.type == "header") | .binary_sha256' "$STUDY/runs/progress.jsonl")
for f in "$S"/receipts/*.json.xz; do cp -f "$f" "$OUT/replay-receipts/"; done
PB=$(jq -s 'map({ok, problems: (.problems | length), rec: .records, ver: .verified_records, rep: (.replays | length), repok: ([.replays[] | select(.reproduced)] | length), bin: .auditor_binary_sha256, env: .auditor_env_class_id, hw: .auditor_hardware_class_id}) | {sessions: length, all_ok: (map(.ok) | all), problems: (map(.problems) | add), records: (map(.rec) | add), scalars_verified_records: (map(.ver) | add), replays: (map(.rep) | add), replays_identical: (map(.repok) | add), auditor_binaries: (map(.bin) | unique), env_classes: (map(.env) | unique), hw_classes: (map(.hw) | unique), note: "committed Phase B receipts sessions/*.audit.json (ecbench verify --replay 12 per session), read, not re-run"}' "$STUDY"/sessions/*.audit.json)
jq -n --slurpfile runs "$S/replay_log.jsonl" --slurpfile unc "$S/unc_log.jsonl" \
  --argjson pb "$PB" --arg hdrenv "$(jq -r 'select(.type == "header") | .ecbench_host | capture("env class +(?<e>[A-Za-z0-9]+)").e' "$STUDY/runs/progress.jsonl")" \
  --arg bin "$BIN_SHA" --arg hdr "$HDR_SHA" --arg nonce "$(cat "$S/nonce.txt")" \
  --arg snap "$(git -C "$REPO" rev-parse HEAD)" --arg started "$(cat "$S/replay_start.txt")" \
  --arg rel "$REL" --argjson receipts "$(cd "$OUT/replay-receipts" && for f in *.json.xz; do printf '{"file":"replay-receipts/%s","sha256":"%s"}\n' "$f" "$(sha256sum "$f" | awk '{print $1}')"; done | jq -s .)" '
  ($runs | map(.archive_sha256_match and .audit_ok and .checks_all_true and (.problems | length == 0)
              and .records == .verified_records and .replays > 0 and .replays == .replays_reproduced
              and .files_match_phase_b_audit and .auditor_binary_sha256 == $hdr) | all) as $all_ok
  | ($unc | map(.archive_sha256_match and .total_gae_mismatch == 0 and .s_mismatch == 0 and .unc_total_mismatch == 0
               and .extract_ops_gae_mismatch == 0 and .extract_S_mismatch == 0
               and .uncharged_rows_without_record == 0 and .records_without_uncharged_row == 0 and .extract_rows_without_record == 0) | all) as $unc_ok
  | {
  schema: "validation-gate/replay-certificate/v1",
  status: (if $all_ok and ($bin == $hdr) then "verified" else "fail" end),
  independent: true,
  scalars_verified: ($all_ok and ($runs | map(.records == .verified_records) | all)),
  study: "research/volcano_ecdlp_census_20261017",
  snapshot_commit: $snap,
  replay_started_utc: $started,
  what_was_replayed: "Each sampled session was extracted from its committed archive sessions/<spec>.tar.xz (SHA-256 checked against runs/progress.jsonl) into a scratch directory and audited with `ecbench verify --dir <extracted> --replay-all`; the fallback `--replay 50` applies only when --replay-all exceeded 600 s (field `mode`). The audit re-derives the session from its own files (file hashes, spec and plan identity, seeds, execution order, record seals and count), re-checks every recovered logarithm [k]G = Q on every record (warm-up included), and re-executes every measured deterministic run, requiring identical answer, total, phases, counters and factor base. The receipt file map was compared with the committed Phase B audit receipt (sessions/<spec>.audit.json). The extracted copy was deleted after each session.",
  independence: {
    replayed_by: "validator process, separate from the Phase B session driver and from the Phase C analysis; started from committed archives only",
    same_vm_type_cpu_and_isa: true,
    same_ecbench_class_ids: false,
    class_ids: {session_header_env_class: $hdrenv, phase_b_audit_env_classes: $pb.env_classes, phase_b_audit_hardware_classes: $pb.hw_classes,
               replay_env_classes: ($runs | map(.auditor_env_class_id) | unique), replay_hardware_classes: ($runs | map(.auditor_hardware_class_id) | unique)},
    same_binary: true,
    honest_scope: "The replay ran on the same shared 4-vCPU cloud VM type as the sessions (Intel Xeon @ 2.80 GHz, 4 logical CPUs, identical ISA feature set, virtualised), on the same host class in that sense, and with the same ecbench binary. ecbench'\''s own env and hardware class ids differ from the session header'\''s (the kernel was 6.18.44-fc-v139 in the session header and is -v140 now; the 79 Phase B audits already span 3 env and 3 hardware class ids). It is an independent re-execution of recorded operation counts, which do not depend on the host; it is not an independent implementation and not independent hardware."
  },
  binary: {path: "target/release/ecbench", sha256: $bin, progress_header_binary_sha256: $hdr, matches_progress_header: ($bin == $hdr)},
  sample: {
    method: "one session per (volcano x family) stratum present (17 strata: V1 and V2 x D, Ua, Ub, S, N, ctl; V3 x D, Ua, Ub, N, ctl), index = SHA-256(nonce|V|family) mod n over the stratum sorted by name (validation/draw_sample.sh), plus every V3 crater session carrying rho-cubic (V3-D-L0-c0, V3-N-L0-c0, V3-ctl-L0-c0) forced",
    nonce: $nonce, nonce_is: "branch HEAD at validation start, fixed before the draw",
    sessions_in_campaign: 79, sessions_replayed: ($runs | length),
    strata_covered: ($runs | map(.spec | split("-") | "\(.[0])|\(.[1])") | unique)
  },
  totals: {
    sessions: ($runs | length),
    records_audited: ($runs | map(.records) | add),
    records_scalar_verified: ($runs | map(.verified_records) | add),
    measured_runs_replayed: ($runs | map(.replays) | add),
    replays_identical: ($runs | map(.replays_reproduced) | add),
    sessions_replay_all: ($runs | map(select(.mode == "replay-all")) | length),
    sessions_replay_50_fallback: ($runs | map(select(.mode == "replay-50") | .spec)),
    all_archive_hashes_match: ($runs | map(.archive_sha256_match) | all),
    all_receipt_file_maps_match_phase_b_audit: ($runs | map(.files_match_phase_b_audit) | all),
    problems: ($runs | map(.problems | length) | add)
  },
  runs: ($runs | map({run: .spec, session_id, archive: "\($rel)/sessions/\(.spec).tar.xz", archive_sha256, archive_sha256_match,
                      mode, fallback_reason, verify_seconds: .verify_s, audit_ok, session_status, problems, checks,
                      records, scalars_verified: (.records == .verified_records), verified_records,
                      replays, replays_identical: .replays_reproduced, replay_details,
                      group_replay: (.replays > 0 and .replays == .replays_reproduced),
                      auditor_binary_sha256, auditor_env_class_id, auditor_hardware_class_id,
                      receipt_sha256, files_match_phase_b_audit})),
  receipts: $receipts,
  phase_b_audits_all_sessions: $pb,
  raw_record_rederivation: {
    what: "For the same 19 sessions, total_gae, s and the A2.1 uncharged count of every measured record were re-derived from records.jsonl with validation/unc_from_records.jq and compared row by row with runs/extract/<spec>.csv.xz and runs/uncharged/<spec>.csv.xz",
    all_match: $unc_ok,
    records_compared: ($unc | map(.records_measured) | add),
    sessions: $unc
  }
}' > "$OUT/replay-certificate.json"

# ---- validator report --------------------------------------------------------
RAW=$( { for d in extract uncharged; do for f in "$STUDY"/runs/$d/*.csv.xz; do echo "runs/$d/$(basename "$f")"; done; done
         for f in "$STUDY"/runs/slugmap/*.json; do echo "runs/slugmap/$(basename "$f")"; done
         echo census/V1/vertices.jsonl; echo census/V2/vertices.jsonl; echo census/V3/vertices.jsonl
         echo runs/progress.jsonl; echo runs/transfer_cost.jsonl; echo analysis/results.json; echo REPORT.md
         echo ANALYSIS_PLAN.md; echo AMENDMENTS.md; echo PLAN.md; } |
       while read -r p; do printf '{"path":"%s/%s","sha256":"%s"}\n' "$REL" "$p" "$(sha256sum "$STUDY/$p" | awk '{print $1}')"; done | jq -s .)
CHECK=$(cat "$W/report_tables.txt")
jq --slurpfile pe "$W/pe.json" --slurpfile fs "$W/fstat.json" --slurpfile js "$W/join.json" --slurpfile bt "$W/boot.json" \
   --slurpfile bd "$W/band.json" --slurpfile nar "$NARR" --argjson raw "$RAW" --arg check "$CHECK" \
   --arg snap "$(git -C "$REPO" rev-parse HEAD)" --slurpfile cert "$OUT/replay-certificate.json" '
  $pe[0] as $p | $fs[0] as $f | $js[0] as $j | $bt[0] as $b | $bd[0] as $band | $nar[0] as $n |
  def rd(a; b): if a == b then 0 else ((a - b) | fabs) / ([(a | fabs), (b | fabs)] | max) end;
  def h(name; claimed; recomputed; tol): {name: name, claimed: claimed, recomputed: recomputed, rel_tol: tol,
       rel_diff: rd(claimed; recomputed), agrees: (rd(claimed; recomputed) <= tol)};
  def pk(k): {"0.0": "0", "0.1": "0.1", "1.0": "1"}[k];
  # Monte Carlo tolerance for a percentile endpoint: 4 x the SD of the difference of two
  # independent B-replicate estimates of the q-quantile (normal approximation).
  def mctol(q; sd; claimed): (4 * 1.4142135623730951 * ((q * (1 - q) / 10000) | sqrt)
       / (if q == 0.025 or q == 0.975 then 0.05844094433345147 else 0.10313564037537128 end) * sd) / (claimed | fabs);
  def ci(name; claimed; mine; q; sd): h(name; claimed; mine; mctol(q; sd; claimed)) + {mc_z: (((claimed - mine) | fabs) / (mctol(q; sd; claimed) * (claimed | fabs) / 4))};
  . as $res |
  ( [ h("positive_control.ratio_rho-cubic_over_rho-neg.V3_crater.D+N+ctl"; .positive_control.row_bootstrap.ratio; $p["pc.ratio"]; 1e-9),
      h("positive_control.n_pairs"; .positive_control.n_pairs; $p["pc.n_rho_cubic"]; 1e-9) ]
    + [ .level_effect | to_entries[] | (.key | split("|")) as [$v, $a] | .value.R | to_entries[]
        | h("level_ratio.R(L).\($v).\($a).L\(.key)/L0"; .value.S.ratio; $p["R.\($v).\($a).\(.key)"]; 1e-9) ]
    + [ .ic_vs_rho | to_entries[] | .key as $v | .value.levels | to_entries[] | .key as $l | .value.prices | to_entries[]
        | h("ic_over_rho-neg.\($v).L\($l).price_\(pk(.key))_GAE"; .value.ratio_ic_over_rho; $p["icrho.\($v).L\($l).p\(pk(.key))"]; 1e-9) ]
    + [ .ic_vs_rho | to_entries[] | .key as $v | .value.pooled | to_entries[]
        | h("ic_over_rho-neg.\($v).all_levels.price_\(pk(.key))_GAE"; .value.ratio_ic_over_rho; $p["icrho.\($v).all.p\(pk(.key))"]; 1e-9) ]
    + [ h("rows.measured"; .data_checks.rows_measured; $j.rows_measured; 1e-9),
        h("rows.all_including_warmup"; .data_checks.rows_all_incl_warmup; $j.rows_all; 1e-9),
        h("rows.warmup_dropped"; (.data_checks.rows_all_incl_warmup - .data_checks.rows_measured); $j.rows_warmup; 1e-9),
        h("rows.verified"; ([.recovery[] | .verified] | add); $p["count.verified"]; 1e-9),
        h("rows.unverified"; .unverified_rows_total; ($j.rows_measured - $p["count.verified"]); 1e-9),
        h("sessions.done"; .sessions; $p["count.sessions"]; 1e-9),
        h("inventory.cells_volcano_family_arm"; (.inventory | length); $p["count.cells"]; 1e-9),
        h("rows.measured.V1"; ([.inventory | to_entries[] | select(.key | startswith("V1|")) | .value.n_rows] | add); $p["count.rows.V1"]; 1e-9),
        h("rows.measured.V2"; ([.inventory | to_entries[] | select(.key | startswith("V2|")) | .value.n_rows] | add); $p["count.rows.V2"]; 1e-9),
        h("rows.measured.V3"; ([.inventory | to_entries[] | select(.key | startswith("V3|")) | .value.n_rows] | add); $p["count.rows.V3"]; 1e-9),
        h("recovery.smallest_cell_cp95_lower"; ([.recovery[] | .cp95[0]] | min); $p["cp95_lower_smallest"]; 1e-6) ]
    + [ .bsgs_identity | to_entries[] | .key as $v | .value | to_entries[] |
        h("bsgs_identical_ops_fraction.\($v).\(.key)"; .value.fraction_identical_ops_gae; $p["bsgs_id.\($v).\(.key)"]; 1e-9),
        h("bsgs_identity_triples.\($v).\(.key)"; .value.n_triples; $p["bsgs_id_n.\($v).\(.key)"]; 1e-9) ]
    + [ .paired_source | to_entries[] | (.key | split("|")) as [$v, $a, $fam]
        | h("paired_source_ratio.\($v).\($a).\($fam)"; .value.ratio_src_over_D; $p["pair.\($v).\($a).\($fam | sub("/D"; ""))_over_D"]; 1e-9) ]
    + [ .v3_descriptive | to_entries[] | .key as $a | .value | to_entries[] | select(.key != "0")
        | h("v3_descriptive_ratio_to_crater.\($a).L\(.key)"; .value.ratio_to_crater; $p["R.V3.\($a).\(.key)"]; 1e-9) ]
    + [ .curve_effect | to_entries[] | select(.value.F != null) | (.key | split("|")) as [$v, $a]
        | h("curve_effect_F.\($v).\($a)"; .value.F; $f["F.\($v).\($a)"]; 1e-9) ]
    + [ .wall_admission | to_entries[] | .key as $v |
        h("aa_wall_ratio.\($v)"; .value.aa_ratio; $p["aa_wall.\($v)"]; 1e-9),
        h("frac_rows_L2.\($v).rho-neg"; .value.arms["rho-neg"].frac_rows_L2; $p["l2frac.\($v).rho-neg"]; 1e-9),
        h("frac_rows_L2.\($v).ic"; .value.arms["ic"].frac_rows_L2; $p["l2frac.\($v).ic"]; 1e-9) ]
    + [ .ic_vs_rho | to_entries[] | .key as $v | .value.uncharged | to_entries[] |
        h("uncharged_per_run.\($v).\(.key)"; ([.value.mean_uncharged_per_run[]] | add); $p["unc_per_run.\($v).\(.key)"]; 1e-9),
        h("uncharged_per_charged_gae.\($v).\(.key)"; .value.mean_unc_total_over_gae; $p["unc_per_gae.\($v).\(.key)"]; 1e-9) ]
    + [ ci("bootstrap.positive_control.ci95_lower"; .positive_control.row_bootstrap.ci95[0]; $b.pc["q0.025"]; 0.025; $b.pc.sd),
        ci("bootstrap.positive_control.ci95_upper"; .positive_control.row_bootstrap.ci95[1]; $b.pc["q0.975"]; 0.975; $b.pc.sd),
        ci("bootstrap.level_ratio.V1.rho-plain.L1.ci95_lower"; .level_effect["V1|rho-plain"].R["1"].S.ci95[0]; $b.v1rp_L1["q0.025"]; 0.025; $b.v1rp_L1.sd),
        ci("bootstrap.level_ratio.V1.rho-plain.L1.ci95_upper"; .level_effect["V1|rho-plain"].R["1"].S.ci95[1]; $b.v1rp_L1["q0.975"]; 0.975; $b.v1rp_L1.sd),
        ci("bootstrap.level_ratio.V1.rho-plain.L1.ci90_lower"; .level_effect["V1|rho-plain"].R["1"].S.ci90[0]; $b.v1rp_L1["q0.05"]; 0.05; $b.v1rp_L1.sd),
        ci("bootstrap.level_ratio.V1.rho-plain.L1.ci90_upper"; .level_effect["V1|rho-plain"].R["1"].S.ci90[1]; $b.v1rp_L1["q0.95"]; 0.95; $b.v1rp_L1.sd),
        ci("bootstrap.ic_over_rho-neg.V1.all.price_0.ci95_lower"; .ic_vs_rho.V1.pooled["0.0"].ci95[0]; $b.v1icrho["q0.025"]; 0.025; $b.v1icrho.sd),
        ci("bootstrap.ic_over_rho-neg.V1.all.price_0.ci95_upper"; .ic_vs_rho.V1.pooled["0.0"].ci95[1]; $b.v1icrho["q0.975"]; 0.975; $b.v1icrho.sd),
        ci("bootstrap.paired_source.V1.rho-plain.S/D.ci95_lower"; .paired_source["V1|rho-plain|S/D"].ci95[0]; $b.pair_vx["q0.025"]; 0.025; $b.pair_vx.sd),
        ci("bootstrap.paired_source.V1.rho-plain.S/D.ci95_upper"; .paired_source["V1|rho-plain|S/D"].ci95[1]; $b.pair_vx["q0.975"]; 0.975; $b.pair_vx.sd) ]
  ) as $head
  | ($head | map(select(.agrees | not))) as $disagree
  | ($n.findings | map(select(.severity == "material"))) as $material
  | {
    schema: "validation-gate/validator-report/v1",
    status: (if ($disagree | length) == 0 and ($material | length) == 0 then "pass" else "fail" end),
    independent: true,
    study: "research/volcano_ecdlp_census_20261017",
    pr: "aburan28/crypto#2402",
    branch: "claude/volcano-ecdlp-census-phase-b-20261022",
    snapshot_commit: $snap,
    status_reason: (if ($disagree | length) > 0 then "recomputed headline numbers disagree: \($disagree | map(.name) | join(", "))"
                    elif ($material | length) > 0 then "every recomputed number matches results.json, but material reporting errors were found: \($material | map(.id) | join(", "))"
                    else "every recomputed headline number matches and no material reporting error was found" end),
    independence: $n.independence,
    raw_data: $raw,
    headline_summary: {numbers: ($head | length), agree: ($head | map(select(.agrees)) | length), disagree: ($disagree | length),
                       max_rel_diff_point_estimates: ($head | map(select(.name | startswith("bootstrap.") | not) | .rel_diff) | max),
                       bootstrap_endpoints_max_mc_z: ($head | map(select(.name | startswith("bootstrap.")) | .mc_z) | max)},
    headline: $head,
    bootstrap_spot_checks: {
      implementation: "validation/cluster_boot.awk (mawk rand(), two-stage stratified cluster bootstrap, 10,000 replicates) and validation/quantiles.sh (percentile, linear interpolation at (n-1)q); seeds 777001-777005 (777006 for the V3 crater IC over rho-cubic CI in rho_reference_audit); shares no code or RNG with analysis/analyze.py",
      tolerance: "rel_tol of each bootstrap endpoint = 4 x sqrt(2) x sqrt(q(1-q)/10000) / phi(z_q) x bootstrap SD / |claimed| (4 SD of the difference of two independent Monte Carlo estimates); mc_z is the difference in units of that SD",
      replicate_summaries: $b,
      "triple_only_sensitivity_V1_rho-plain_S_over_D": {ci95: [$b.pair_tr["q0.025"], $b.pair_tr["q0.975"]], ci90: [$b.pair_tr["q0.05"], $b.pair_tr["q0.95"]], sd: $b.pair_tr.sd,
        note: "resampling (vertex,target,round) triples without the vertex stage; SD is 1/sqrt(2) of the two-stage SD, see finding N1"}
    },
    row_counts: $j,
    report_vs_results: ($n.report_vs_results + {table_cells_check: $check}),
    verdict_vocabulary: $n.verdict_vocabulary,
    amendment_A2_audit: $n.amendment_A2_audit,
    rho_reference_audit: ($n.rho_reference_audit + {gae_band_recomputed: $band}),
    section_5a_band: $n.section_5a_band,
    findings: $n.findings,
    limitations: $n.limitations,
    proposed_validation_json_rho_reference: ($n.validation_json_rho_reference + {note_on_use: "for VALIDATION.json once both reports pass; gae_band is filled from rho_reference_audit.gae_band_recomputed (V1 pooled)"}),
    replay_certificate: {path: "replay-certificate.json", status: $cert[0].status},
    validation_report: ($n.validation_report + {verdict: (if ($disagree | length) == 0 and ($material | length) == 0 then "passed" else "failed" end)})
  }' "$R" > "$OUT/validator-report.json"

# ---- VALIDATION.json only when both pass ------------------------------------
CS=$(jq -r .status "$OUT/replay-certificate.json"); VS=$(jq -r .status "$OUT/validator-report.json")
if [ "$CS" = verified ] && [ "$VS" = pass ]; then
  jq -n --arg c "$(sha256sum "$OUT/replay-certificate.json" | awk '{print $1}')" \
        --arg v "$(sha256sum "$OUT/validator-report.json" | awk '{print $1}')" \
        --slurpfile nar "$NARR" --slurpfile bd "$W/band.json" '
    $bd[0] as $b | {
    schema: "validation-gate/v1",
    replay_certificate: {path: "replay-certificate.json", sha256: $c},
    validator_report: {path: "validator-report.json", sha256: $v},
    rho_reference: ($nar[0].validation_json_rho_reference + {
      gae_band: {"0": {ic: $b["V1|0"].ic, rho: $b["V1|0"].rho_neg}, "0.1": {ic: $b["V1|0.1"].ic, rho: $b["V1|0.1"].rho_neg}, "1": {ic: $b["V1|1"].ic, rho: $b["V1|1"].rho_neg}},
      gae_band_by_volcano: $b })
  }' > "$OUT/VALIDATION.json"
else
  rm -f "$OUT/VALIDATION.json"
fi
echo "replay certificate: $CS; validator report: $VS; VALIDATION.json: $([ -f "$OUT/VALIDATION.json" ] && echo written || echo not written)"
