// T1 correctness (PROTOCOL.md section 3) and T2 verification, over every record of every session (warm-up included).
//   node analysis/correctness.mjs <sessions dir> > data/correctness.json      (cwd = study directory)
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { gunzipSync } from "node:zlib";
const sess = process.argv[2];
const C = (n, j) => { let c = 1; for (let i = 0; i < j; i++) c = (c * (n - i)) / (i + 1); return n < j ? 0 : Math.round(c); };
const eq = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const res = { pairs_checked: 0, mismatches: [], runs: 0, unverified: [], setup_adds_saved: {} };
for (const s of readdirSync(sess).sort()) {
  const f = `${sess}/${s}/records.jsonl`, recs = (existsSync(f) ? readFileSync(f) : gunzipSync(readFileSync(f + ".gz"))).toString().trim().split("\n").map((l) => JSON.parse(l));
  const by = {};
  for (const r of recs) {
    res.runs++;
    if (r.outcome.status !== "verified" || r.outcome.matches_target !== true) res.unverified.push(`${s} ${r.arm} ${r.round} ${r.workload.target_index}`);
    by[`${r.arm}|${r.round}|${r.workload.target_index}`] = r;
  }
  for (const k of [4, 5]) for (const [key, p] of Object.entries(by)) {
    if (!key.startsWith(`ksum${k}-p|`)) continue;
    const q = by[key.replace(`ksum${k}-p|`, `ksum${k}-ps|`)];
    const bad = (why) => res.mismatches.push(`${s} ${key} ${why}`);
    res.pairs_checked++;
    if (!q) { bad("missing -ps twin"); continue; }
    if (p.algorithm_seed !== q.algorithm_seed) bad("seed");
    if (!eq(p.outcome.status, q.outcome.status) || p.outcome.recovered !== q.outcome.recovered) bad("outcome");
    if (!eq(p.counters, q.counters)) bad("counters");
    if (!eq(p.factor_base, q.factor_base)) bad("factor_base");
    if (p.phases.length !== q.phases.length) { bad("phase count"); continue; }
    p.phases.forEach((a, i) => { const b = q.phases[i];
      if (a.name !== b.name) bad(`phase name ${a.name}`);
      if (!eq(a.native, b.native) || !eq(a.work_native, b.work_native)) bad(`native ${a.name}`);
      if (a.doubles !== b.doubles || a.scalar_mults !== b.scalar_mults) bad(`dbl/sm ${a.name}`);
      if (a.name !== "oracle_setup") { if (a.adds !== b.adds || a.gae !== b.gae) bad(`cost ${a.name}`); }
      else {
        const fbsz = Number(/size=(\d+)/.exec(p.method.params.factor_base)[1]);
        const a_ = p.factor_base.abscissae ?? fbsz;
        const bound = 2 ** (k - 2) * C(a_ - 1, k - 1) - 2 * C(a_ - k + 2, 2);
        const saved = a.adds - b.adds;
        if (!(saved > 0 && saved <= bound)) bad(`setup adds saved ${saved} not in (0, ${bound}]`);
        const t = (res.setup_adds_saved[`${s.split("-")[0]} k${k}`] ??= { bound, min: Infinity, max: -Infinity });
        t.min = Math.min(t.min, saved); t.max = Math.max(t.max, saved);
      }
    });
  }
}
res.correctness_ok = res.mismatches.length === 0 && res.pairs_checked > 0;
res.all_verified = res.unverified.length === 0;
console.log(JSON.stringify(res, null, 1));
