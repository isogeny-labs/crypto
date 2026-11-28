// Lay out results.json (evaluate.mjs) and Table B's recorded before-marks (docs/leaderboard.json) as summary.json. Copies numbers; computes nothing.
import { readFileSync, writeFileSync } from "node:fs";
const res = JSON.parse(readFileSync("results.json", "utf8"));
const lb = JSON.parse(readFileSync("../../docs/ic/leaderboard.json", "utf8"));
const tb = Object.fromEntries(lb.board.filter((r) => r.regime === "char2").map((r) => [r.slug, r]));
const W = ["0", "0.1", "1"];
const rows = res.rows.map((r) => {
  const t = tb[r.slug];
  return { slug: r.slug, name: r.name, bits: r.bits, role: r.role, incumbents: r.incumbents,
    table_b: t ? { recipe: t.best.label, columns: t.best.columns, signed_points: t.best.base_points, s_ic: t.best.s, s_rho: t.reference_s, ic_over_rho: t.best.ratio_rho } : null,
    weights: Object.fromEntries(W.map((w) => { const x = r.weights[w]; const c = (k) => ({ ratio: x[k].over_incumbent.ratio, ci95: x[k].over_incumbent.ci95, ci95_bootstrap: x[k].over_incumbent.ci95_bootstrap, upper: x[k].over_incumbent.upper,
        per_replicate: x[k].per_replicate.map((q) => q.ratio), over_bsgs: x[k].over_bsgs.ratio, gap_to_rho: x[k].gap_to_rho });
      return [w, { strictest_incumbent: x.incumbent, s: x.S, inc_tb_over_incumbent: x.inc_tb_over_incumbent, ksum5: c("ksum5"), ksum4: c("ksum4") }]; })),
    replicate_spread: r.replicate_spread, delta: r.delta, ksum5: r.ksum5, ksum4: r.ksum4 };
});
const out = { schema: "crypto.ic_tableb_binary_ksum_summary/v1", unit: "ecbench counted GAE per sqrt(r), native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a)",
  isolation: "L0 (counted cost; wall time not claimed)", track: "generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS)",
  protocol: "research/ic_tableb_binary_ksum_20261115/PROTOCOL.md", binary_sha256: readFileSync("BINARY.sha256", "utf8").split(/\s+/)[0],
  sessions: 15, targets_per_curve: 48, rounds: 3, replicates: 3,
  ksum5_l1_holds_on_a_primary_row: res.ksum5_l1_holds_on_a_primary_row, ksum5_l1_falsified: res.ksum5_l1_falsified,
  ksum4_l1_holds_on_a_primary_row: res.ksum4_l1_holds_on_a_primary_row, ksum4_l1_falsified: res.ksum4_l1_falsified, rows };
writeFileSync("summary.json", JSON.stringify(out, null, 1) + "\n");
console.log(rows.map((r) => `${r.slug} w0 k5=${r.weights["0"].ksum5.ratio.toFixed(3)} k4=${r.weights["0"].ksum4.ratio.toFixed(3)}`).join("\n"));
