// Director's predictions, fixed before any data of this study, from the prune study's committed sessions (analysis/phases.json)
// and the closed-form build counts. usage: node analysis/predict.mjs > analysis/predictions.json
import { readFileSync } from "node:fs";
const ph = JSON.parse(readFileSync("analysis/phases.json", "utf8"));
const sum = JSON.parse(readFileSync("../ic_ksum_prune_20261115/summary.json", "utf8"));
const C = (n, j) => { let c = 1; for (let i = 0; i < j; i++) c = (c * (n - i)) / (i + 1); return n < j ? 0 : Math.round(c); };
const SIZE = { P20: [10, 9], P22: [12, 10], P24: [14, 11], B15: [7, 7], B18: [7, 7], B21: [10, 9], B24: [12, 9], B27: [16, 12] };
const out = {};
for (const row of sum.rows) {
  const id = row.id, sq = Math.sqrt(row.r ?? 0);
  out[id] = {};
  for (const [k, a] of [[4, SIZE[id][0]], [5, SIZE[id][1]]]) {
    const pruned = [...Array(k - 1)].map((_, i) => i + 2).reduce((s, j) => s + 2 ** (j - 1) * C(a - (k - j), j), 0);
    const recorded = row.build_adds[`ksum${k}`].build_adds_pruned / 576; // 3 sessions x 4 rounds x 48 targets
    const saved = 2 ** (k - 2) * C(a - 1, k - 1) - 2 * C(a - k + 2, 2);
    const m = ph.rows[id][`ksum${k}-p`];
    const sqrtR = m["oracle_setup.adds"] > 0 ? recorded / m["oracle_setup.adds"] : NaN; // build adds / (build adds per sqrt r)
    const split = saved / sqrtR, walkSetup = m.split_of_S.walk_setup_and_restarts;
    out[id][`ksum${k}`] = {
      abscissae: a, build_adds_pruned_formula: pruned, build_adds_pruned_recorded: recorded, split_saved_adds: saved,
      S_incumbent: m.S, pred_R0_split: 1 - split / m.S,
      // stride-renew: three draws of [a]G + [b]Q (S, T, R0) against the walk's 16 jumps, its start and pooled restarts:
      // the set-up and restarts fall by about 85 % (6 of about 48 scalar multiplications remain).
      pred_R0_stride: 1 - (0.85 * walkSetup) / m.S,
      pred_R0_split_stride: 1 - (split + 0.85 * walkSetup) / m.S,
    };
  }
}
console.log(JSON.stringify(out, null, 1));
