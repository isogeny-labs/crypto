// Markdown tables for README.md from the native outputs (analysis/analysis.json,
// analysis/stage1.json, design.json, main/*.tsv for the mean group order only).
// Every measured number is copied from those files; the only arithmetic here is
// the registered distance-remaining formula 1.27 * N^(1/(2h-1)) (PROTOCOL.md section 8).
import { readFileSync, readdirSync, writeFileSync } from "node:fs";

const A = JSON.parse(readFileSync("analysis/analysis.json", "utf8"));
const S1 = JSON.parse(readFileSync("analysis/stage1.json", "utf8"));
const D = JSON.parse(readFileSync("design.json", "utf8"));
const f = (x, d = 3) => (x == null ? "–" : Number(x).toFixed(d));
const ci = (k, c) => `${f(k)} [${f(c[0])}, ${f(c[1])}]`;
const g = (theta, geom) => A.series.find((s) => s.theta === theta && s.geometry === geom);
const bits = A.rungs;
const REAL = [["window", "window (d = 1)"], ["gap2", "label GAP (d = 2)"], ["gap3", "label GAP (d = 3)"]];
let out = "";

// mean group order per rung (display only)
const meanN = {};
for (const b of bits) {
  const lines = readFileSync(`main/main-${b}.tsv`, "utf8").split("\n").filter(Boolean);
  const ns = new Map();
  for (const l of lines) {
    const t = l.split(" ");
    ns.set(t[0], Number(t[3]));
  }
  const v = [...ns.values()];
  meanN[b] = v.reduce((a, c) => a + c, 0) / v.length;
}

out += "### T1. Real geometries: κ per rung, slope and class\n\n";
for (const theta of A.thetas) {
  out += `θ = ${theta}\n\n| geometry | ${bits.map((b) => `κ at ${b} bits`).join(" | ")} | slope [95%] | class |\n|:--|${bits.map(() => ":--").join("|")}|:--|:--|\n`;
  for (const [k, name] of REAL) {
    const s = g(theta, k);
    out += `| ${name} | ${s.cells.map((c) => ci(c.kappa, c.ci95)).join(" | ")} | ${ci(s.slope, s.slope_ci95)} | ${s.class} |\n`;
  }
  out += "\n";
}

out += "### T2. Controls\n\n| θ | control | slope [95%] | expected | κ at 16 bits | κ at 32 bits |\n|--:|:--|:--|:--|:--|:--|\n";
for (const theta of A.thetas) {
  const z = g(theta, "zn_interval");
  out += `| ${theta} | Z/N interval | ${ci(z.slope, z.slope_ci95)} | 1 − θ = ${f(1 - theta, 2)} | ${f(z.cells[0].kappa, 1)} | ${f(z.cells.at(-1).kappa, 1)} |\n`;
  for (const [k, name] of [["plant_g0", "plant γ = 0 (random)"], ["plant_g1_6", "plant γ = 1/6"], ["plant_g1_5", "plant γ = 1/5"]]) {
    const s = g(theta, k);
    out += `| ${theta} | ${name} | ${ci(s.slope, s.slope_ci95)} | ${s.slope_formal_pred == null ? "0" : f(s.slope_formal_pred)} (integer count) | ${f(s.cells[0].kappa, 2)} | ${f(s.cells.at(-1).kappa, 2)} |\n`;
  }
  const n = g(theta, "null_cal");
  out += `| ${theta} | null vs null | ${ci(n.slope, n.slope_ci95)} | 0 | ${f(n.cells[0].kappa, 3)} | ${f(n.cells.at(-1).kappa, 3)} |\n`;
}

out += "\n### T3. Distance remaining (harvest law, c = 1)\n\n| bits | mean N | 1.27·N^(1/3) (h = 2) | 1.27·N^(1/5) (h = 3) | 1.27·N^(1/7) (h = 4) | largest real κ upper bound |\n|--:|--:|--:|--:|--:|--:|\n";
for (const b of bits) {
  const N = meanN[b];
  const top = Math.max(...A.thetas.flatMap((t) => REAL.map(([k]) => g(t, k).cells.find((c) => c.bits === b).ci95[1])));
  out += `| ${b} | ${N.toExponential(2)} | ${f(1.27 * N ** (1 / 3), 0)} | ${f(1.27 * N ** (1 / 5), 0)} | ${f(1.27 * N ** (1 / 7), 0)} | ${f(top)} |\n`;
}

out += "\n### T4. Cells: curves and achieved precision\n\n| bits | θ | b | curves | pilot M1 | pilot V1 | predicted rel. sd | achieved rel. sd of S (window) |\n|--:|--:|--:|--:|--:|--:|--:|--:|\n";
for (const d of D) {
  const s = g(d.theta, "window");
  const c = s.cells.find((x) => x.bits === d.bits);
  const rel = Math.sqrt(c.V * (1 + 1 / 20)) / c.M;
  out += `| ${d.bits} | ${d.theta} | ${d.b} | ${c.curves} | ${f(d.m1, 2)} | ${f(d.v1, 2)} | ${f(d.predicted_rel_sd)} | ${f(rel)} |\n`;
}

out += "\n### T5. Gates and reading\n\n```\n" + JSON.stringify({ gates: A.gates, real_geometry_max_upper: A.real_geometry_max_upper, delta_star: A.delta_star, reading: A.reading }, null, 1) + "\n```\n";

out += "\n### T6. Stage 1: slopes from the recorded EXP-PFDR-ca5e01 cells (bits 16–22)\n\n| θ | d | cells | slope [95%] |\n|--:|--:|--:|:--|\n";
for (const r of S1) out += `| ${r.theta} | ${r.d} | ${r.points} | ${ci(r.slope, r.ci95)} |\n`;

out += "\n### T7. Overlap with EXP-PFDR-ca5e01 at θ = 0.30 (different curve generator)\n\n| bits | d | ca5e01 κ [95%] | this study κ [95%] | intervals overlap |\n|--:|--:|:--|:--|:--|\n";
const ca = readFileSync("stage1/ca5e01-cells.tsv", "utf8").split("\n").filter(Boolean).map((l) => l.split("\t").map(Number));
for (const [theta, d, b, k, lo, hi] of ca) {
  if (theta !== 0.3) continue;
  const mine = g(0.3, ["window", "gap2", "gap3"][d - 1])?.cells.find((c) => c.bits === b);
  if (!mine) continue;
  out += `| ${b} | ${d} | ${ci(k, [lo, hi])} | ${ci(mine.kappa, mine.ci95)} | ${lo <= mine.ci95[1] && mine.ci95[0] <= hi ? "yes" : "no"} |\n`;
}
writeFileSync("analysis/tables.md", out);
console.log(out);
