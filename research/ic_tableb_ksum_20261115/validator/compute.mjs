// Stage 1: independent recomputation. Writes own-numbers.json. Reads NO producer output
// (results.json/results.md/summary.json/band/analysis are not opened here).
import { writeFileSync } from "node:fs";
import path from "node:path";
import { loadAll, HERE, BITS, REPS, WEIGHTS, EXPECTED } from "./load.mjs";
import { analyzeCurve, l1Rule, BOOT } from "./stats.mjs";
import { sha256File } from "./lib.mjs";

const C = loadAll((m) => console.error(m));
const own = { schema: "ic_tableb_ksum.validator.own-numbers/v1", bootstrap: { ...BOOT, note: "stratified by session; draws: (workload, round) pairs (pair bootstrap) or workloads (cluster bootstrap); percentile interval, type-7 quantiles; PRNG splitmix32 stream" } };
own.provenance = C.provenance; own.registration = C.registration; own.binary_sha256_expected = C.binary_sha256_expected;
own.errors = C.errors;
own.cross = C.cross;
own.sessions = Object.fromEntries(Object.entries(C.sessions).map(([n, S]) => [n, S.checks]));
own.curves = {};
own.analysis = {};
for (const b of BITS) {
  const { checks, ledgerSpec } = C.curves[b];
  own.curves[b] = { ...checks, expected: EXPECTED[b] };
  const { cv, out } = analyzeCurve(C, b);
  out.l1_rule = l1Rule(out);
  // marker coverage (table-insert pricing) per IC arm and per-arm uncharged counters (section 5a listing)
  out.pricing_audit = {};
  for (const a of cv.armNames) {
    let n = 0, withMarker = 0, nonInt = 0, calibrated = 0; const unp = {}, tw = {}; let maxfrac = 0; const mean = (x) => x;
    for (const rep of REPS) { const S = C.sessions[`P${b}-r${rep}`];
      S.recs.forEach((r, i) => { if (r.arm !== a || r.warmup) return; const d = S.dec[i]; n++;
        if (d.tableWork.table_insert_attempts !== undefined) withMarker++;
        if (d.nC > 0) calibrated++;
        const fr = Math.abs((d.G - d.C) - Math.round(d.G - d.C)); maxfrac = Math.max(maxfrac, fr); if (fr > 1e-6) nonInt++;
        for (const nm of d.unpricedNames) unp[nm] = (unp[nm] || 0) + 1;
        for (const [nm, v] of Object.entries(d.tableWork)) tw[nm] = (tw[nm] || 0) + v; }); }
    out.pricing_audit[a] = { measured_runs: n, runs_with_table_insert_marker: withMarker, runs_with_calibrated_native_work: calibrated, runs_with_nonintegral_G_minus_C: nonInt, max_fractional_part_of_G_minus_C: maxfrac, unpriced_counter_run_counts: unp, mean_table_work: Object.fromEntries(Object.entries(tw).map(([k, v]) => [k, v / n])) };
  }
  own.analysis[b] = out;
}
// code hashes for the report
own.code_sha256 = Object.fromEntries(["lib.mjs", "load.mjs", "stats.mjs", "compute.mjs"].map((f) => [f, sha256File(path.join(HERE, f))]));
writeFileSync(path.join(HERE, "own-numbers.json"), JSON.stringify(own, null, 1) + "\n");
console.error("wrote own-numbers.json; errors:", C.errors.length);
