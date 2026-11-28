// summary.json from results.json, data/correctness.json, data/verify.txt and analysis/predictions.json (selection only).
//   node analysis/summary.mjs   (cwd = study directory)
import { readFileSync, writeFileSync } from "node:fs";
const res = JSON.parse(readFileSync("results.json", "utf8"));
const cor = JSON.parse(readFileSync("data/correctness.json", "utf8"));
const pred = JSON.parse(readFileSync("analysis/predictions.json", "utf8"));
const ver = readFileSync("data/verify.txt", "utf8").trim().split("\n");
const W = ["0", "0.1", "1"];
const rows = res.rows.map((r) => {
  const o = { id: r.id, slug: r.slug, field: r.field, bits: r.bits, weights: {}, holds: {} };
  for (const w of W) {
    const x = r.weights[w], arms = {};
    for (const k of [4, 5]) for (const c of ["p", "ps", "pr", "psr"]) {
      const a = `ksum${k}-${c}`;
      arms[a] = { s: x.S[a], gap_to_rho: x.gap_to_rho[a] };
      if (c !== "p") Object.assign(arms[a], { ratio_vs_p: x[a].ratio.ratio, ci95: x[a].ratio.ci95, upper: x[a].ratio.upper,
        per_replicate: x[a].per_replicate.map((q) => q.ratio), per_replicate_upper: x[a].per_replicate.map((q) => q.upper) });
    }
    o.weights[w] = { s_rho_neg: x.S["rho-neg"], s_rho_neg_aa: x.S["rho-neg-aa"], aa_ratio: x.aa.ratio, arms,
      best: { incumbent: x.best.incumbent, candidate: x.best.candidate, ratio: x.best.ratio.ratio, ci95: x.best.ratio.ci95,
        s_incumbent: x.S[x.best.incumbent], s_candidate: x.S[x.best.candidate], gap_incumbent: x.best.gap_incumbent, gap_candidate: x.best.gap_candidate } };
  }
  for (const a of ["ksum5-ps", "ksum4-ps", "ksum5-psr", "ksum4-psr"]) o.holds[a] = r[a];
  o.prediction = { ksum5_ps_r0: pred[r.id].ksum5.pred_R0_split, ksum4_ps_r0: pred[r.id].ksum4.pred_R0_split,
    ksum5_psr_r0: pred[r.id].ksum5.pred_R0_split_stride, ksum4_psr_r0: pred[r.id].ksum4.pred_R0_split_stride };
  o.prediction.ksum5_ps_held = Math.abs(o.weights["0"].arms["ksum5-ps"].ratio_vs_p - o.prediction.ksum5_ps_r0) <= 0.03;
  o.prediction.ksum5_psr_held = Math.abs(o.weights["0"].arms["ksum5-psr"].ratio_vs_p - o.prediction.ksum5_psr_r0) <= 0.06;
  o.split_setup_adds_saved = { k5: cor.setup_adds_saved[`${r.id} k5`], k4: cor.setup_adds_saved[`${r.id} k4`] };
  return o;
});
const summary = {
  schema: "crypto.ic_ksum_split_summary/v1",
  unit: "ecbench counted GAE per sqrt(r), native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a)",
  isolation: "L0 (counted cost; wall time not claimed)",
  track: "generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS)",
  standing: "[pcc:unverified]",
  protocol: "research/ic_ksum_split_20261118/PROTOCOL.md (amendments A1-A4)",
  binary_sha256: readFileSync("BINARY.sha256", "utf8").split(" ")[0],
  sessions: 24, targets_per_curve: 48, rounds: 3, replicates: 3,
  verdict: {
    split5_holds_all_rows: res.split5_holds_all_rows, split4_holds_all_rows: res.split4_holds_all_rows,
    combined5_holds_all_rows: res.combined5_holds_all_rows, combined4_holds_all_rows: res.combined4_holds_all_rows,
    split5_holds_rows: res.split5_holds_rows, split4_holds_rows: res.split4_holds_rows,
    combined5_holds_rows: res.combined5_holds_rows, combined4_holds_rows: res.combined4_holds_rows,
    correctness_ok: cor.correctness_ok, correctness_pairs_checked: cor.pairs_checked, correctness_mismatches: cor.mismatches.length,
    all_verified: cor.all_verified, runs_checked: cor.runs,
    all_sessions_verified: ver.length === 24 && ver.every((l) => l.endsWith("rc=0")), sessions_verified: ver.filter((l) => l.endsWith("rc=0")).length,
    prediction_ksum5_ps_held_rows: rows.filter((r) => r.prediction.ksum5_ps_held).map((r) => r.id),
    prediction_ksum5_psr_held_rows: rows.filter((r) => r.prediction.ksum5_psr_held).map((r) => r.id),
  },
  rows,
};
writeFileSync("summary.json", JSON.stringify(summary, null, 1) + "\n");
console.log(JSON.stringify(summary.verdict, null, 1));
