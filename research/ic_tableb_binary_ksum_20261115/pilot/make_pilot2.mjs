// Second sizing pilot (disclosed): refines the grids around pilot 1's optima. 48 targets, 2 rounds, seeds not used elsewhere.
import { writeFileSync, mkdirSync } from "node:fs";
import { ROWS, explicit, arm, sub, mult, ks } from "../curves.mjs";
const out = process.argv[2]; mkdirSync(out, { recursive: true });
const G = { 15: [[15, 18, 21, 26], [6, 7, 8], [6, 7, 8]], 18: [[15, 21, 26, 30], [6, 7, 8, 9], [6, 7, 8]],
  21: [[62, 87, 124, 174], [9, 10, 11, 12], [8, 9, 10, 11]], 24: [[56, 67, 80, 94], [10, 11, 12, 13], [8, 9, 10, 11]],
  27: [[89, 127, 150, 178], [14, 16, 18, 20], [11, 12, 13, 14]] };
for (const r of ROWS) {
  const [I, K4, K5] = G[r.bits], arms = [arm("rho-neg", "reference", "rho.negation"), sub("inc-sub", "baseline", r.dim)];
  for (const n of I) arms.push(mult(`inc-n${n}`, "baseline", n));
  for (const n of K4) arms.push(ks(`ksum4-n${n}`, 4, n));
  for (const n of K5) arms.push(ks(`ksum5-n${n}`, 5, n));
  const spec = { schema: "ecbench.spec/v1", label: `pilot2 sizing ${r.name}`, description: "sizing pilot 2, not data. Public synthetic curve.",
    workloads: { curves: [explicit(r.name)], targets_per_curve: 48, target_seed: 920000 + r.bits, target_kind: "public" }, arms,
    measurement: { rounds: 2, warmup: 0, order: "alternate", seed: 930000 + r.bits, isolation_required: "L0", timeout_seconds: 1800 } };
  writeFileSync(`${out}/pilot2-${r.bits}.json`, JSON.stringify(spec, null, 1) + "\n");
}
