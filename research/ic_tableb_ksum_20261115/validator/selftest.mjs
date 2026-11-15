// Canary tests: the validator's checks must be able to fail.
import { readFileSync } from "node:fs";
import { Curve, eqPt, decompose, sha256, tQuantile, splitmix32, quantile } from "./lib.mjs";
const base = new URL("../", import.meta.url).pathname;
const cal = JSON.parse(readFileSync(new URL("../../../docs/ic/calibration.json", import.meta.url))).instances;
const lines = readFileSync(base + "sessions/P20-r1/records.jsonl", "utf8").split("\n").filter(Boolean);
const E = new Curve(1048291, 1, 11), G = [1n, 123004n];
const res = {};
// (a) certificate: correct scalar passes, scalar +1 and negated scalar fail
let ok = 0, bumpFail = 0, negFail = 0, n = 0;
for (const l of lines.slice(0, 200)) { const r = JSON.parse(l); const T = [BigInt(r.workload.target[0]), BigInt(r.workload.target[1])]; const d = BigInt(r.outcome.recovered); n++;
  if (eqPt(E.mul(d, G), T)) ok++; if (!eqPt(E.mul(d + 1n, G), T)) bumpFail++; if (!eqPt(E.mul(1046999n - d, G), T)) negFail++; }
res.certificate_canary = { checked: n, true_scalar_passes: ok, plus_one_rejected: bumpFail, negated_scalar_rejected: negFail };
// (b) seal: untouched passes, one-digit edit of a cost fails
const sealOk = (line) => { const r = JSON.parse(line); const f = `"record_id":"${r.record_id}"`; return "ECR1h" + sha256(Buffer.from(line.replace(f, '"record_id":""'))).slice(0, 12) === r.record_id; };
const l0 = lines[10]; const edited = l0.replace(/"total_gae":([0-9.]+)/, (m, x) => `"total_gae":${x}1`);
res.seal_canary = { untouched_ok: sealOk(l0), edited_cost_rejected: !sealOk(edited), edited_differs: edited !== l0 };
// (c) calibration row: right row gives integral G-C, neighbour rows do not
const recs = lines.map((l) => JSON.parse(l)).filter((r) => r.method.id === "ic.pipeline").slice(0, 100);
const nonint = (row) => recs.filter((r) => { const d = decompose(r, row); const g = d.G - d.C; return Math.abs(g - Math.round(g)) > 1e-6; }).length;
res.calibration_canary = { records: recs.length, nonintegral_with_correct_row_bench_20bit: nonint(cal["prime/bench-20bit"]), nonintegral_with_bench_18bit_row: nonint(cal["prime/bench-18bit"]), nonintegral_with_generated_22bit_row: nonint(cal["prime/generated-22bit-3290411"]) };
// (d) statistics: t quantiles, bootstrap coverage of a known ratio
res.t_quantile = { df10: tQuantile(0.975, 10), expected10: 2.228138851986, df143: tQuantile(0.975, 143), expected143_approx: 1.97669 };
const rng = splitmix32(1); const q = []; for (let i = 0; i < 20000; i++) q.push(rng()); q.sort((a, b) => a - b);
res.prng_uniformity = { median: quantile(q, 0.5), q025: quantile(q, 0.025), q975: quantile(q, 0.975) };
console.log(JSON.stringify(res, null, 1));
