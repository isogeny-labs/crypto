# validator-report.json (validator code).  Inputs: $m META.json, $head
# headline.json, $rec recompute.json, $band band.json, $inst instance-check.json,
# $find findings.json, $raw, $inputs, $scripts, $outputs, $cert_sha.
$m[0] as $M | $head[0] as $H | $band[0] as $B
| ([ $H.headline[] | .agrees ] | all) as $ok
| {
    schema: "validator-report/v1",
    study: "research/fes_then_batched_20261112",
    snapshot: $M.snapshot,
    status: (if $ok then "pass" else "fail" end),
    status_scope: "pass = every number of the README table (the headline the dispatch names) recomputes from the committed raw reports within half a unit of its printed precision. It does not rank fes-then-batched first among IC rows or against inherited-f4 under AGENTS.md 5a (findings F2, F3), and it does not cover the README's distance figures (F5).",
    independent: true,
    independence: $M.report_independence,
    raw_data_paths_relative_to: "research/fes_then_batched_20261112/ (the evidence directory; each entry also carries repo_path). For pcc, a recompute proof over this report needs evidence_dir = research/fes_then_batched_20261112.",
    raw_data: $raw,
    inputs: $inputs,
    scripts: $scripts,
    outputs: $outputs,
    rule: $B.rule,
    headline: [ $H.headline[] | {name, claimed, recomputed, rel_tol, printed, agrees, recomputed_rounds_to_printed} ],
    headline_summary: $H.summary,
    unknowns: $H.unknowns,
    recomputed_only: [ $rec[0][] as $i | {
        instance: $i.name,
        mean_S: (($i.arms | map_values(.mean_s_recomputed)) + {rho: $i.rho.mean_s_recomputed}),
        S_over_rho: ($i.arms | map_values(.mean_s_over_rho_recomputed)),
        ftb_over_if4: $i.ratios.ftb_over_if4,
        ftb_over_pair: $i.ratios.ftb_over_pair,
        ftb_over_fti: $i.ratios.ftb_over_fti,
        rho_method: $i.rho.method,
        rho_runs: $i.rho.runs,
        arms_all_verified: ([ $i.arms[] | .all_verified ] | all),
        max_total_gae_rebuild_rel_err: ([ $i.arms[] | .max_G_rel_err ] | max)
      } ],
    secondary_claims: $H.secondary,
    band: {
      path: "validation/band.json",
      gae_band: $B.gae_band, gae_band_basis: $B.gae_band_basis, gae_band_detail: $B.gae_band_detail,
      per_instance: [ $B.instances[] | {name, S, ratio_ftb_over_if4, ratio_ftb_over_rho,
                        orders: (.orders | map_values({order, verdict}))} ],
      summary: $B.summary
    },
    instance_checks: [ $inst[0][] | del(.known_answer) ],
    certificate_discipline: "No IC report or rho record emits the base point or the target, so no [k]P = Q check with code independent of the solver is possible from the committed artifacts. What was checked independently: every recovered logarithm equals the planted value the same process recorded for that repeat and lies in [1, r); r is prime, the cofactor <= 8 and the group order equals 2^n + 1 - t from the slug; the Koblitz traces are recomputed from the Frobenius recurrence (validation/instance-check.json). The verified flags themselves are the binary's.",
    replay_certificate: {path: "validation/replay-certificate.json", sha256: $cert_sha},
    findings: $find[0].findings,
    controls: $find[0].controls,
    limitations: $M.limitations,
    validation_report: $M.validation_report
  }
