// PROTOCOL.md "Measurements": per arm and size, the target-blind cost P, the
// online cost T and the table entries E, read from the native ecbench records
// (test/sessions/P*/records.jsonl), and the quantities derived from them at
// w = 0, 0.1, 1 (each unpriced lookup priced at w GAE).  Thin derivation only:
// every input is a record field.  Writes analysis/regime.json and
// analysis/regime.md.  Run from this directory.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const SIZES = (process.env.SIZES ?? "32,36,40").split(",").map(Number);
const WEIGHTS = [0, 0.1, 1];
const BLIND = {
  "kl-ksum4": ["factor_base", "oracle_setup"],
  "pre-eqS": ["setup", "precompute"],
  "pre-eqP": ["setup", "precompute"],
};
// The record's unpriced counters (cost.unpriced, the ones ecbench band prices
// at w), split into target-blind and online work by what produced them: the
// known-log base and the table build, or the precomputation's walks and
// inserts, are target-blind; everything else is online.  The k-sum build reads
// `x_index` once per base abscissa, inside `lookups_uncharged`.
const unpriced = (rec) => {
  const c = rec.counters;
  let blind = 0;
  let online = 0;
  for (const name of rec.cost.unpriced ?? []) {
    let v = c[name] ?? 0;
    if (rec.arm === "kl-ksum4" && name === "lookups_uncharged") {
      const size = Number(rec.method?.params?.size ?? 0);
      blind += size;
      v -= size;
    }
    const isBlind =
      (rec.arm === "kl-ksum4" && (name.startsWith("base_") || name === "table_insert_attempts_uncharged")) ||
      (rec.arm.startsWith("pre-") && (name.startsWith("precompute_") || name === "table_inserts_uncharged"));
    if (isBlind) blind += v;
    else online += v;
  }
  return { blind, online };
};
const entriesOf = (rec) =>
  rec.counters.ksum_distinct_signed_classes ?? rec.counters.table_entries ?? null;

const mean = (xs) => xs.reduce((a, b) => a + b, 0) / xs.length;
const ci95 = (xs) => {
  const m = mean(xs);
  return 1.96 * Math.sqrt(xs.reduce((a, b) => a + (b - m) ** 2, 0) / (xs.length - 1) / xs.length);
};

const rows = [];
for (const bits of SIZES) {
  const recs = readFileSync(`test/sessions/P${bits}/records.jsonl`, "utf8")
    .trim()
    .split("\n")
    .map((l) => JSON.parse(l))
    .filter((r) => !r.warmup);
  const r = recs[0].workload.curve.r;
  const n = r / 2;
  const sr = Math.sqrt(r);
  const byArm = {};
  for (const rec of recs) (byArm[rec.arm] ??= []).push(rec);
  const rho = byArm["rho-neg"].map((x) => x.cost.total_gae);
  for (const [arm, list] of Object.entries(byArm)) {
    for (const w of WEIGHTS) {
      const blindNames = BLIND[arm];
      const P = blindNames
        ? list.map((x) => x.phases.filter((p) => blindNames.includes(p.name)).reduce((s, p) => s + p.gae, 0) + w * unpriced(x).blind)
        : null;
      const T = list.map((x) => {
        const tot = x.cost.total_gae + w * (unpriced(x).blind + unpriced(x).online);
        return blindNames ? tot - P[list.indexOf(x)] : tot;
      });
      const Trho = mean(rho.map((g, i) => g + w * unpriced(byArm["rho-neg"][i]).online));
      const E = entriesOf(list[0]);
      const row = {
        bits, arm, w, runs: list.length,
        verified: list.filter((x) => x.outcome?.status === "verified").length,
        online_s: mean(T) / sr,
        online_s_ci95: ci95(T) / sr,
        setup_s: P ? mean(P) / sr : null,
        entries: E,
      };
      if (P) {
        row.break_even_targets = mean(P) / (Trho - mean(T));
        row.pt_over_n = (mean(P) * mean(T)) / n;
        row.et2_over_n = E ? (E * mean(T) ** 2) / n : null;
      }
      rows.push(row);
    }
  }
}
// Crossover between kl-ksum4 and each preprocessing arm: L where
// P_a/L + T_a = P_b/L + T_b.
const cross = [];
for (const bits of SIZES)
  for (const w of WEIGHTS) {
    const kl = rows.find((x) => x.bits === bits && x.arm === "kl-ksum4" && x.w === w);
    for (const b of ["pre-eqS", "pre-eqP"]) {
      const o = rows.find((x) => x.bits === bits && x.arm === b && x.w === w);
      if (!kl || !o) continue;
      const L = (o.setup_s - kl.setup_s) / (kl.online_s - o.online_s);
      cross.push({ bits, w, a: "kl-ksum4", b, crossover_targets: L });
    }
  }
mkdirSync("analysis", { recursive: true });
writeFileSync("analysis/regime.json", JSON.stringify({ rows, crossovers: cross }, null, 1) + "\n");
const f = (x, d = 3) => (x == null || !isFinite(x) ? "—" : Math.abs(x) >= 1000 ? x.toExponential(2) : x.toFixed(d));
let md = "| bits | arm | w | runs | online S (±95%) | set-up S | entries | L* vs rho | P·T/N | E·T²/N |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|\n";
for (const x of rows)
  md += `| ${x.bits} | ${x.arm} | ${x.w} | ${x.runs} | ${f(x.online_s, 4)} ± ${f(x.online_s_ci95, 4)} | ${f(x.setup_s, 2)} | ${x.entries ?? "—"} | ${f(x.break_even_targets, 1)} | ${f(x.pt_over_n, 2)} | ${f(x.et2_over_n, 2)} |\n`;
md += "\n| bits | w | arms | crossover targets |\n|--:|--:|:--|--:|\n";
for (const c of cross) md += `| ${c.bits} | ${c.w} | ${c.a} vs ${c.b} | ${f(c.crossover_targets, 0)} |\n`;
writeFileSync("analysis/regime.md", md);
console.log(md);
