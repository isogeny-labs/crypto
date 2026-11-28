// Sizing pilot (disclosed in PROTOCOL.md): node make_pilot.mjs <outdir>; 24 targets, 1 round, seeds not used by the study.
import { writeFileSync, mkdirSync } from "node:fs";
import { ROWS, explicit, arm, sub, mult, ks } from "../curves.mjs";
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const C = { 15: 15, 18: 15, 21: 31, 24: 67, 27: 127 };           // Table B recipe columns (signed points / 2)
for (const r of ROWS) {
  const c = C[r.bits], arms = [arm("rho-neg", "reference", "rho.negation"), sub("inc-sub", "baseline", r.dim)];
  for (const f of [0.5, 0.7, 1, 1.4, 2]) { const n = Math.round(c * f); arms.push(mult(`inc-n${n}`, "baseline", n)); }
  for (const n of [5, 6, 7, 8, 10, 12, 14, 16]) arms.push(ks(`ksum4-n${n}`, 4, n));
  for (const n of [5, 6, 7, 8, 9, 10, 12, 14]) arms.push(ks(`ksum5-n${n}`, 5, n));
  const spec = { schema: "ecbench.spec/v1", label: `pilot sizing ${r.name}`, description: "sizing pilot, not data. Public synthetic curve.",
    workloads: { curves: [explicit(r.name)], targets_per_curve: 24, target_seed: 900000 + r.bits, target_kind: "public" }, arms,
    measurement: { rounds: 1, warmup: 0, order: "alternate", seed: 910000 + r.bits, isolation_required: "L0", timeout_seconds: 1800 } };
  writeFileSync(`${out}/pilot-${r.bits}.json`, JSON.stringify(spec, null, 1) + "\n");
}
