// Evaluate the registered tests of PROTOCOL.md section 6 from the recorded sessions.
//   node analysis/evaluate.mjs <ecbench binary> <sessions dir (with test/P<bits>, control/PC32)> <out dir>
// It computes nothing the harness does not: S, ratios and intervals come from `ecbench band` and `ecbench compare`;
// this file applies the registered thresholds and lays the numbers out.  Output: <out>/results.json, <out>/results.md.
// SIZES / WEIGHTS env override for the toy smoke test (the registered thresholds are only meaningful at 36-48 bits).
import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";

const [bin, sess, out] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const W = [0, 0.1, 1];
const SIZES = (process.env.SIZES ?? "36 40 44 48").split(" ").map(Number);
const TEST = process.env.TESTDIR ?? "test";
const run = (args) => execFileSync(bin, args, { encoding: "utf8", maxBuffer: 1 << 28 });
const bandOf = (d) => {
  const f = `${out}/.band.json`;
  run(["band", "--dir", d, "--json", f]);
  const rows = JSON.parse(readFileSync(f, "utf8")).rows;
  return Object.fromEntries(rows.map((r) => [r.arm, r]));
};
const cmp = (d, a, b, w) => {
  const j = JSON.parse(run(["compare", "--dir", d, "--a", a, "--b", b, "--weight", String(w), "--resamples", "4000", "--seed", "20261115", "--json"]));
  const o = j.ops ?? j;
  const ci = o.ci95, cb = o.ci95_bootstrap;
  return { ratio: o.ratio_b_over_a, ci95: ci, ci95_bootstrap: cb, lower: Math.min(ci?.[0] ?? NaN, cb?.[0] ?? NaN), upper: Math.max(ci?.[1] ?? NaN, cb?.[1] ?? NaN), pairs: o.workloads, status: o.status };
};
const f = (x, d = 3) => (typeof x === "number" ? x.toFixed(d) : String(x));
const inRange = (x, lo, hi) => x >= lo && x <= hi;

const res = { schema: "ic_ladder_wwalk_ksum.evaluation/v1", sizes: {}, kills: [], notes: [] };
for (const b of SIZES) {
  const d = `${sess}/${TEST}/P${b}`;
  if (!existsSync(d)) { res.notes.push(`P${b}: session missing`); continue; }
  const band = bandOf(d);
  const arms = Object.keys(band);
  const unverified = arms.filter((a) => band[a].verified !== band[a].runs);
  const S = (a, wi) => band[a].mean_s[wi];
  const r = { bits: b, arms, unverified, S: {}, ratios: {} };
  for (const a of arms) r.S[a] = { w0: S(a, 0), w01: S(a, 1), w1: S(a, 2), runs: band[a].runs };
  W.forEach((w, wi) => {
    const x = {};
    x.c2_over_rho = cmp(d, "rho-neg", "wwalk-c2", w);
    x.c2x06_over_rho = cmp(d, "rho-neg", "wwalk-c2-x06", w);
    x.c1_over_rho = cmp(d, "rho-neg", "wwalk-c1", w);
    x.c2_over_ksum5 = cmp(d, "ksum5-sel", "wwalk-c2", w);
    x.ksum6_over_ksum5 = cmp(d, "ksum5-sel", "ksum6-sel", w);
    x.ksum7_over_ksum5 = cmp(d, "ksum5-sel", "ksum7-sel", w);
    x.ksum7_over_ksum6 = cmp(d, "ksum6-sel", "ksum7-sel", w);
    x.ksum8_over_ksum7 = cmp(d, "ksum7-sel", "ksum8-model", w);
    x.ksum5_over_rho = cmp(d, "rho-neg", "ksum5-sel", w);
    x.rho_aa_over_rho = cmp(d, "rho-neg", "rho-neg-aa", w);
    x.c2_aa_over_c2 = cmp(d, "wwalk-c2", "wwalk-c2-aa", w);
    x.bsgs_over_rho = cmp(d, "rho-neg", "bsgs-neg", w);
    r.ratios[w] = x;
  });
  // Registered checks (PROTOCOL.md section 6)
  const q = r.ratios;
  r.checks = {
    c2_over_rho_w0_in_band: inRange(q[0].c2_over_rho.ratio, 1.75, 2.15),
    c2_over_rho_w01_in_band: inRange(q[0.1].c2_over_rho.ratio, 1.75, 2.15),
    c2_over_rho_w1_le_2_45: q[1].c2_over_rho.ratio <= 2.45,
    c2_over_ksum5_le_0_25: q[0].c2_over_ksum5.ratio <= 0.25,
    ksum6_below_ksum5_all_weights: W.every((w) => q[w].ksum6_over_ksum5.upper < 1),
    ksum7_below_ksum5_all_weights: W.every((w) => q[w].ksum7_over_ksum5.upper < 1),
    ksum7_below_ksum6_all_weights: W.every((w) => q[w].ksum7_over_ksum6.upper < 1),
    ksum8_not_below_ksum7_by_3pct_registered_at_36_only: b === 36 ? q[0].ksum8_over_ksum7.ratio >= 0.97 : null,
    floor_audit_wwalk_S_ge_1_50: ["wwalk-c2", "wwalk-c2-x06", "wwalk-c2-aa"].every((a) => S(a, 0) >= 1.5),
    floor_audit_ksum_S_ge_4_0: ["ksum5-sel", "ksum6-sel", "ksum7-sel", "ksum8-model"].every((a) => S(a, 0) >= 4.0),
    rho_aa_ratio_is_1: Math.abs(q[0].rho_aa_over_rho.ratio - 1) < 1e-9,
    wwalk_aa_ratio_is_1: Math.abs(q[0].c2_aa_over_c2.ratio - 1) < 1e-9,
    all_verified: unverified.length === 0,
  };
  if (b >= 40 && q[0].c2_over_rho.ratio > 2.5) res.kills.push(`${b}: wwalk-c2 / rho-neg ${f(q[0].c2_over_rho.ratio)} > 2.5 at w = 0`);
  if (b === 40 && q[0].c2_over_ksum5.ratio > 0.4) res.kills.push(`40: wwalk-c2 / ksum5-sel ${f(q[0].c2_over_ksum5.ratio)} > 0.40`);
  if (unverified.length) res.kills.push(`${b}: unverified arms ${unverified.join(",")}`);
  res.sizes[b] = r;
}
const bs = SIZES.filter((b) => res.sizes[b]);
if (res.sizes[36] && res.sizes[40] && res.sizes[36].ratios[0.1].ksum6_over_ksum5.upper >= 1 && res.sizes[40].ratios[0.1].ksum6_over_ksum5.upper >= 1)
  res.kills.push("L2: ksum6 / ksum5 upper end >= 1 at both 36 and 40 bits at w = 0.1");
for (let i = 1; i < bs.length; i++) {
  const [a, c] = [res.sizes[bs[i - 1]], res.sizes[bs[i]]];
  const step = (c.ratios[0].c2_over_rho.ratio - a.ratios[0].c2_over_rho.ratio) / ((c.bits - a.bits) / 4);
  c.c2_rise_per_4bits = step;
  if (step > 0.15) res.kills.push(`${c.bits}: wwalk-c2 / rho-neg rises ${f(step)} per 4 bits (> 0.15)`);
}
// Positive control
const pc = `${sess}/control/PC32`;
if (existsSync(pc)) {
  const band = bandOf(pc);
  const c = { S: Object.fromEntries(Object.keys(band).map((a) => [a, band[a].mean_s])), unverified: Object.keys(band).filter((a) => band[a].verified !== band[a].runs) };
  c.x06_over_rho_w0 = cmp(pc, "rho-neg", "wwalk-c2-x06", 0);
  c.x06_S_w0 = band["wwalk-c2-x06"].mean_s[0];
  c.reproduces_1_842_pm10pct = inRange(c.x06_S_w0, 1.842 * 0.9, 1.842 * 1.1);
  c.c1_over_rho_w0 = cmp(pc, "rho-neg", "wwalk-c1", 0);
  res.positive_control = c;
  if (!c.reproduces_1_842_pm10pct) res.kills.push("positive control: wwalk-c2-x06 S at w = 0 outside 1.842 +/- 10%: the instrument is not the autolab's; stop");
}
res.negative_control_c0 = "not run: ecbench has no C = 0 wwalk oracle (PROTOCOL.md section 5); rho-neg-aa and wwalk-c2-aa are the only null arms";
res.killed = res.kills.length > 0;
writeFileSync(`${out}/results.json`, JSON.stringify(res, null, 1) + "\n");

let md = `# Ladder panel evaluation (generated by evaluate.mjs from the recorded sessions)\n\nS = counted GAE / sqrt(r) per run at w = 0 / 0.1 / 1; ratios paired by target and round, 4,000 resamples.\n\n`;
md += `| bits | arm | S w=0 | S w=0.1 | S w=1 | runs |\n|--:|:--|--:|--:|--:|--:|\n`;
for (const b of bs) for (const a of res.sizes[b].arms) { const s = res.sizes[b].S[a]; md += `| ${b} | ${a} | ${f(s.w0, 2)} | ${f(s.w01, 2)} | ${f(s.w1, 2)} | ${s.runs} |\n`; }
md += `\n| bits | w | wwalk-c2 / rho-neg | wwalk-c2 / ksum5-sel | ksum6 / ksum5 | ksum7 / ksum5 | ksum7 / ksum6 | ksum8 / ksum7 | ksum5 / rho | wwalk-c1 / rho |\n|--:|--:|:--|:--|:--|:--|:--|:--|--:|--:|\n`;
const cell = (x) => `${f(x.ratio)} [${f(x.lower)}, ${f(x.upper)}]`;
for (const b of bs) for (const w of W) {
  const x = res.sizes[b].ratios[w];
  md += `| ${b} | ${w} | ${cell(x.c2_over_rho)} | ${cell(x.c2_over_ksum5)} | ${cell(x.ksum6_over_ksum5)} | ${cell(x.ksum7_over_ksum5)} | ${cell(x.ksum7_over_ksum6)} | ${cell(x.ksum8_over_ksum7)} | ${f(x.ksum5_over_rho.ratio, 2)} | ${f(x.c1_over_rho.ratio, 2)} |\n`;
}
md += `\n| bits | check | value |\n|--:|:--|:--|\n`;
for (const b of bs) for (const [k, v] of Object.entries(res.sizes[b].checks)) md += `| ${b} | ${k} | ${v} |\n`;
if (res.positive_control) md += `\nPositive control (autolab k = 32 curve): wwalk-c2-x06 S(w=0) = ${f(res.positive_control.x06_S_w0)} (registered 1.842 +/- 10%): ${res.positive_control.reproduces_1_842_pm10pct}\n`;
md += `\nNegative control C = 0: ${res.negative_control_c0}\n\nKill criteria hit: ${res.kills.length ? res.kills.map((k) => `\n- ${k}`).join("") : "none"}\n`;
writeFileSync(`${out}/results.md`, md);
console.log(md);
