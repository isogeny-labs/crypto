// Lay out results.json (evaluate.mjs) and data/correctness.json (correctness.py) as summary.json. Copies numbers; computes only
// the verdict booleans and the saved-adds fractions (sums of recorded oracle_setup adds).   cwd = study directory.
import { readFileSync, writeFileSync } from "node:fs";
const res = JSON.parse(readFileSync("results.json", "utf8")), cor = JSON.parse(readFileSync("data/correctness.json", "utf8"));
const ver = readFileSync("data/verify.txt", "utf8").trim().split("\n");
const W = ["0", "0.1", "1"];
const saved = (id, k) => { let u = 0, p = 0; for (const [key, [a, b]] of Object.entries(cor.adds_saved)) if (key.startsWith(`${id}-r`) && key.endsWith(`:ksum${k}`)) { u += a; p += b; } return { build_adds_unpruned: u, build_adds_pruned: p, saved_fraction: 1 - p / u }; };
const rows = res.rows.map((r) => ({ slug: r.slug, id: r.id, name: r.name, bits: r.bits, field: r.field,
  build_adds: { ksum5: saved(r.id, 5), ksum4: saved(r.id, 4) },
  weights: Object.fromEntries(W.map((w) => { const x = r.weights[w], c = (k) => { const q = x[`ksum${k}`], p = q.ratio_p_over_u;
      return { s_unpruned: x.S[`ksum${k}`], s_pruned: x.S[`ksum${k}-p`], ratio: p.ratio, ci95: p.ci95, ci95_bootstrap: p.ci95_bootstrap, upper: p.upper,
        per_replicate: q.per_replicate.map((y) => y.ratio), gap_to_rho_unpruned: q.gap_to_rho_unpruned, gap_to_rho_pruned: q.gap_to_rho_pruned }; };
    return [w, { s_rho_neg: x.S["rho-neg"], s_rho_neg_aa: x.S["rho-neg-aa"], aa_ratio: x.aa.ratio, aa_delta: x.aa.ratio - 1, aa_ci95: x.aa.ci95, ksum5: c(5), ksum4: c(4) }]; })),
  replicate_spread: r.spread, ksum5: r.ksum5, ksum4: r.ksum4 }));
const pred = rows.map((r) => ({ id: r.id, r0: r.weights["0"].ksum5.ratio, held: Math.abs(r.weights["0"].ksum5.ratio - 0.95) <= 0.03 }));
const out = { schema: "crypto.ic_ksum_prune_summary/v1", unit: "ecbench counted GAE per sqrt(r), native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a)",
  isolation: "L0 (counted cost; wall time not claimed)", track: "generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS)", standing: "[pcc:unverified]",
  protocol: "research/ic_ksum_prune_20261115/PROTOCOL.md", binary_sha256: readFileSync("BINARY.sha256", "utf8").split(/\s+/)[0],
  sessions: 24, targets_per_curve: 48, rounds: 3, replicates: 3,
  verdict: { ksum5_p_holds_all_rows: res.ksum5_p_holds_all_rows, ksum4_p_holds_all_rows: res.ksum4_p_holds_all_rows,
    ksum5_p_holds_rows: res.ksum5_p_holds_rows, ksum4_p_holds_rows: res.ksum4_p_holds_rows,
    ksum5_p_holds_sibling_delta_rows: res.ksum5_p_holds_sibling_delta_rows, ksum4_p_holds_sibling_delta_rows: res.ksum4_p_holds_sibling_delta_rows,
    correctness_ok: cor.ok, correctness_pairs_checked: cor.pairs_checked, correctness_mismatches: cor.mismatches.length,
    all_sessions_verified: ver.length === 24 && ver.every((l) => l.endsWith("rc=0")), sessions_verified: ver.filter((l) => l.endsWith("rc=0")).length,
    prediction_ksum5_r0_0p95_pm_0p03_held_rows: pred.filter((p) => p.held).map((p) => p.id) },
  rows };
writeFileSync("summary.json", JSON.stringify(out, null, 1) + "\n");
console.log(JSON.stringify(out.verdict), "\n", rows.map((r) => `${r.id} k5 saved ${(100 * r.build_adds.ksum5.saved_fraction).toFixed(1)}% k4 ${(100 * r.build_adds.ksum4.saved_fraction).toFixed(1)}%`).join("\n"));
