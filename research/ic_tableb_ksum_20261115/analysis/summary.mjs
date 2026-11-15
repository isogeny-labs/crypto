// Lay out results.json (evaluate.mjs) and Table B's recorded before-marks (docs/ic/leaderboard.json) as summary.json,
// the file scripts/build_ic_leaderboard.py reads.  Computes nothing: every number is copied.
import { readFileSync, writeFileSync } from "node:fs";
const res = JSON.parse(readFileSync("results.json", "utf8"));
const lb = JSON.parse(readFileSync("../../docs/ic/leaderboard.json", "utf8"));
const tb = Object.fromEntries(lb.board.filter((r) => r.regime === "prime" || r.regime === "char2").map((r) => [r.slug, r]));
const rep = JSON.parse(readFileSync("replay-certificate.json", "utf8"));
const W = ["0", "0.1", "1"];
const rows = res.rows.map((r) => {
  const t = tb[r.slug];
  return {
    slug: r.slug, role: r.role, incumbents: r.incumbents,
    table_b: t ? { variant: t.best.variant, recipe: t.best.label, columns: t.best.columns, signed_points: t.best.base_points, s_ic: t.best.s, s_rho: t.reference_s, ic_over_rho: t.best.ratio_rho, source: t.source } : null,
    weights: Object.fromEntries(W.map((w) => {
      const x = r.weights[w], p = x.ksum5_over_incumbent;
      return [w, { strictest_incumbent: x.incumbent, s: x.S, ksum5_over_incumbent: p.ratio, ci95: p.ci95, ci95_bootstrap: p.ci95_bootstrap, upper: p.upper,
        per_replicate: x.per_replicate.map((q) => q.ratio), ksum4_over_incumbent: x.ksum4_over_incumbent.ratio, ksum5_over_bsgs: x.ksum5_over_bsgs.ratio, gap_to_rho: x.gap_to_rho }];
    })),
    replicate_spread: r.replicate_spread, delta: r.delta, max_upper_pooled: r.max_upper_pooled, max_upper_replicate: r.max_upper_replicate, holds: r.holds,
  };
});
const out = {
  schema: "crypto.ic_tableb_ksum_summary/v1", unit: "ecbench counted GAE per sqrt(r), native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md section 5a)",
  isolation: "L0 (counted cost; wall time not claimed)", track: "generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS)",
  protocol: "research/ic_tableb_ksum_20261115/PROTOCOL.md", registered_at: readFileSync("registered-at.txt", "utf8").trim(),
  binary_sha256: readFileSync("BINARY.sha256", "utf8").split(/\s+/)[0], commit: "7b3c057e1cef83ccf5712950473ef91e10153499",
  sessions: 9, targets_per_curve: 48, rounds: 3, replicates: 3,
  replay: { records: rep.runs.reduce((a, x) => a + x.records, 0), replays: rep.runs.reduce((a, x) => a + x.replays, 0), host: "second host class (Intel Xeon 2.10 GHz container); recorded on a RunPod pod (AMD EPYC)" },
  l1_holds_on_a_primary_row: res.l1_holds_on_a_primary_row, l1_falsified: res.l1_falsified, rows,
};
writeFileSync("summary.json", JSON.stringify(out, null, 1) + "\n");
console.log(rows.map((r) => `${r.slug} tableB=${r.table_b?.s_ic?.toFixed(2)} w0 ratio=${r.weights["0"].ksum5_over_incumbent.toFixed(3)}`).join("\n"));
