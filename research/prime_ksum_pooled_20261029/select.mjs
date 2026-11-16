// PROTOCOL.md "Selection" (tooling): reads analysis/band-train.json (ecbench band
// over the four train sessions) and writes select/selection.json. Nothing but the
// registered rule: per family, the cell with the smallest run-weighted mean S at
// w = 1 over cells verified on every run with no uncounted counter; ties go to the
// smaller table; k* is the lower selected mean among ksum4 and ksum5 (ties: k = 4).
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const band = JSON.parse(readFileSync("analysis/band-train.json", "utf8"));
const bits = (r) => Math.round(r.log2_r);
const W = 2; // index of w = 1 in mean_s
const cells = {};
for (const r of band.rows) {
  const m = /^(inc|ksum4|ksum5)-[na](\d+)$/.exec(r.arm);
  if (!m) continue;
  const key = `${bits(r)}:${r.arm}`;
  const c = (cells[key] ??= { b: bits(r), fam: m[1], size: +m[2], runs: 0, ok: true, sum: 0, sum0: 0, arm: r.arm });
  c.runs += r.runs;
  c.sum += r.mean_s[W] * r.runs;
  c.sum0 += r.mean_s[0] * r.runs;
  if (r.verified !== r.runs || (r.uncounted ?? []).length) c.ok = false;
}
const out = {};
for (const b of [20, 24, 28, 32]) {
  out[b] = {};
  for (const fam of ["inc", "ksum4", "ksum5"]) {
    const cs = Object.values(cells).filter((c) => c.b === b && c.fam === fam && c.ok).map((c) => ({ ...c, s1: c.sum / c.runs }));
    cs.sort((x, y) => x.s1 - y.s1 || x.size - y.size);
    const pick = cs[0];
    const sizes = cs.map((c) => c.size).sort((x, y) => x - y);
    out[b][fam] = { [fam === "inc" ? "n" : "a"]: pick.size, s1: pick.s1, arm: pick.arm, edge: pick.size === sizes[0] || pick.size === sizes[sizes.length - 1], cells: cs.map((c) => [c.size, +c.s1.toFixed(4)]) };
  }
  { // the incumbent cell with the smallest mean S at w = 0 (a second baseline arm, PROTOCOL.md)
    const cs0 = Object.values(cells).filter((c) => c.b === b && c.fam === "inc" && c.ok).map((c) => ({ ...c, s0: c.sum0 / c.runs }));
    cs0.sort((x, y) => x.s0 - y.s0 || x.size - y.size);
    out[b].inc0 = { n: cs0[0].size, s0: cs0[0].s0 };
  }
  out[b].kstar = out[b].ksum5.s1 < out[b].ksum4.s1 ? 5 : 4;
}
mkdirSync("select", { recursive: true });
writeFileSync("select/selection.json", JSON.stringify(out, null, 1) + "\n");
for (const b of [20, 24, 28, 32]) console.log(b, `inc N=${out[b].inc.n}${out[b].inc.edge ? "*" : ""} (${out[b].inc.s1.toFixed(2)})`, `k4 A=${out[b].ksum4.a}${out[b].ksum4.edge ? "*" : ""} (${out[b].ksum4.s1.toFixed(2)})`, `k5 A=${out[b].ksum5.a}${out[b].ksum5.edge ? "*" : ""} (${out[b].ksum5.s1.toFixed(2)})`, `inc0 N=${out[b].inc0.n}`, `k*=${out[b].kstar}`);
