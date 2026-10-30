// Tables and the registered verdicts (tooling), from ecbench's own outputs only:
// analysis/compare-*.json (pooled paired ratios), analysis/band.json (per-curve means),
// analysis/bounds/*.json (exponent fits), select/selection.json.  Writes analysis/ratios.md,
// analysis/per_curve.md, analysis/verdicts.json and analysis/exponents.md.
import { readFileSync, writeFileSync, existsSync } from "node:fs";

const SIZES = [20, 24, 28, 32];
const WS = ["0", "0.1", "1"];
const f3 = (x) => (Math.round(x * 1000) / 1000).toString();
const cmp = (b, a, c, w) => {
  const p = `analysis/compare-P${b}-${a}__${c}-w${w}.json`;
  return existsSync(p) ? JSON.parse(readFileSync(p, "utf8")).ops : null;
};
const band = JSON.parse(readFileSync("analysis/band.json", "utf8"));
const sel = JSON.parse(readFileSync("select/selection.json", "utf8"));
const cell = (o, i = 0) => (o ? `${f3(o.band.ratio_b_over_a[i])} [${f3(o.ci95_bootstrap[0])}, ${f3(o.ci95_bootstrap[1])}]` : "n/a");

// --- pooled paired ratios
const PAIRS = [["inc-sel", "ksum5-sel"], ["inc-sel", "ksum4-sel"], ["inc-w0", "ksum5-sel"], ["inc-w0", "ksum4-sel"], ["inc-rec", "ksum5-sel"], ["inc-rec", "ksum4-sel"], ["inc-rec", "inc-sel"], ["ksum4-sel", "ksum5-sel"], ["rho-neg", "ksum5-sel"], ["rho-neg", "ksum4-sel"], ["rho-neg", "inc-sel"], ["bsgs-neg", "ksum5-sel"], ["bsgs-neg", "ksum4-sel"]];
let md = "| bits | pair (B / A) | w = 0 | w = 0.1 | w = 1 | band order |\n|--:|:--|:--|:--|:--|:--|\n";
for (const b of SIZES) for (const [a, c] of PAIRS) {
  const o = WS.map((w) => cmp(b, a, c, w));
  if (!o[0]) continue;
  md += `| ${b} | ${c} / ${a} | ${o.map((x, i) => cell(x, i)).join(" | ")} | ${o[0].band.order} |\n`;
}
writeFileSync("analysis/ratios.md", md);

// --- per-curve ratios (test a), w = 0, from band rows
const rows = {};
const sizeOf = (r) => +/^icv1-fp(\d+)-/.exec(r.curve)[1];
for (const r of band.rows) ((rows[sizeOf(r)] ??= {})[r.curve] ??= {})[r.arm] = r.mean_s;
let pc = "| bits | curve | inc-sel | inc-w0 | ksum4-sel | ksum5-sel | k5 / inc-sel | k4 / inc-sel | k5 / rho | k5 / bsgs |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|\n";
const perCurve = {};
for (const b of SIZES) {
  perCurve[b] = [];
  for (const [curve, arms] of Object.entries(rows[b])) {
    const inc = arms["inc-sel"][0], inc0 = arms["inc-w0"]?.[0], k4 = arms["ksum4-sel"][0], k5 = arms["ksum5-sel"][0];
    perCurve[b].push({ curve, k5inc: k5 / inc, k4inc: k4 / inc, k5inc0: inc0 ? k5 / inc0 : null, k4inc0: inc0 ? k4 / inc0 : null, k5: k5, k4: k4, k5rho: k5 / arms["rho-neg"][0], k4rho: k4 / arms["rho-neg"][0], k5bsgs: k5 / arms["bsgs-neg"][0] });
    pc += `| ${b} | \`${curve}\` | ${f3(inc)} | ${inc0 ? f3(inc0) : "-"} | ${f3(k4)} | ${f3(k5)} | ${f3(k5 / inc)} | ${f3(k4 / inc)} | ${f3(k5 / arms["rho-neg"][0])} | ${f3(k5 / arms["bsgs-neg"][0])} |\n`;
  }
}
writeFileSync("analysis/per_curve.md", pc);

// --- registered verdicts (PROTOCOL.md section 3)
const V = { primary: {}, a: {}, b20: null, c: {}, d: {}, f: {} };
const below1 = (b, a, c, w, upper) => { const o = cmp(b, a, c, w); return !!o && o.band.ratio_b_over_a[WS.indexOf(w)] < 1 && (!upper || o.ci95_bootstrap[1] < 1); };
for (const b of SIZES) {
  const ok = (k) => WS.every((w) => below1(b, "inc-sel", k, w, w === "0")) && (b === 20 || WS.every((w) => below1(b, "inc-w0", k, w, false)));
  const key = { ksum4: ok("ksum4-sel"), ksum5: ok("ksum5-sel") };
  V.primary[b] = { ksum5_sel_below_inc_sel: ok("ksum5-sel"), ksum4_sel_below_inc_sel: ok("ksum4-sel"), holds: ok("ksum5-sel") && ok("ksum4-sel"), vs_inc_w0_checked: b !== 20 };
  // (a): the k-sum family with lower pooled S is below inc-sel on at least 3 of the 4 curves at w = 0
  const lower = (cmp(b, "ksum4-sel", "ksum5-sel", "0").band.ratio_b_over_a[0] < 1) ? "ksum5" : "ksum4";
  const n = perCurve[b].filter((r) => (lower === "ksum5" ? r.k5inc : r.k4inc) < 1).length;
  const n0 = perCurve[b].filter((r) => (lower === "ksum5" ? r.k5inc0 : r.k4inc0) < 1 || r.k5inc0 === null).length;
  V.a[b] = { family: lower, curves_below_inc_sel: n, curves_below_inc_w0: n0, of: perCurve[b].length, holds: n >= 3 && n0 >= 3 };
  V.c[b] = { inc_sel_over_inc_rec: cmp(b, "inc-rec", "inc-sel", "0").band.ratio_b_over_a, k5_over_inc_rec: cmp(b, "inc-rec", "ksum5-sel", "0").band.ratio_b_over_a };
  // (d): the rule's choice (kstar) is not above the other family at w = 1 outside the pooled interval
  const o = cmp(b, "ksum4-sel", "ksum5-sel", "1");
  const r = o.band.ratio_b_over_a[2], lo = o.ci95_bootstrap[0], hi = o.ci95_bootstrap[1];
  V.d[b] = { kstar: sel[b].kstar, k5_over_k4_w1: r, ci: [lo, hi], rule_ok: sel[b].kstar === 5 ? !(lo > 1) : !(hi < 1) };
  V.f[b] = { k5_over_rho: cmp(b, "rho-neg", "ksum5-sel", "0").band.ratio_b_over_a[0], k4_over_rho: cmp(b, "rho-neg", "ksum4-sel", "0").band.ratio_b_over_a[0], k5_over_bsgs: cmp(b, "bsgs-neg", "ksum5-sel", "0").band.ratio_b_over_a[0], k4_over_bsgs: cmp(b, "bsgs-neg", "ksum4-sel", "0").band.ratio_b_over_a[0] };
}
V.b20 = V.primary[20];
V.primary_overall = [24, 28, 32].every((b) => V.primary[b].holds) && [24, 28, 32].every((b) => V.a[b].holds);
writeFileSync("analysis/verdicts.json", JSON.stringify(V, null, 1) + "\n");

// --- exponents (e)
let ex = "| arm | w | exponent alpha | 95% interval | sizes | admissible |\n|:--|:--|--:|:--|--:|:--|\n";
for (const arm of ["rho-neg", "bsgs-neg", "inc-sel", "ksum4-sel", "ksum5-sel"]) for (const w of WS) {
  const p = `analysis/bounds/P-${arm}-toy-w${w}.json`;
  if (!existsSync(p)) { ex += `| ${arm} | ${w} | refused | | | no |\n`; continue; }
  const j = JSON.parse(readFileSync(p, "utf8"));
  const fit = j.fit ?? {};
  ex += `| ${arm} | ${w} | ${fit.alpha !== undefined ? f3(fit.alpha) : "?"} | ${fit.alpha_ci95 ? `[${f3(fit.alpha_ci95[0])}, ${f3(fit.alpha_ci95[1])}]` : "?"} | ${fit.sizes ?? "?"} | ${j.admissibility?.status ?? "?"} |\n`;
}
writeFileSync("analysis/exponents.md", ex);
console.log(JSON.stringify({ primary: V.primary, a: V.a, d: V.d, overall: V.primary_overall }, null, 0));
