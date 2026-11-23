// Spec generator for PROTOCOL.md: `node make_specs.mjs` writes specs/B<bits>-r<rep>.json (cwd = this directory).
// Sizes frozen from the two disclosed sizing pilots (pilot/); they are not re-tuned on study data.
import { writeFileSync, mkdirSync } from "node:fs";
import { ROWS, explicit, arm, sub, mult, ks } from "./curves.mjs";
const INC = { 15: [15, 18, 21], 18: [21, 26, 30], 21: [43, 62, 87], 24: [56, 67, 80], 27: [127, 150, 178] };
const K4 = { 15: 7, 18: 7, 21: 10, 24: 12, 27: 16 }, K5 = { 15: 7, 18: 7, 21: 9, 24: 9, 27: 12 };
mkdirSync("specs", { recursive: true });
for (const r of ROWS) for (const rep of [1, 2, 3]) {
  const arms = [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"), arm("bsgs-neg", "reference", "bsgs.negation"),
    sub("inc-tb", "baseline", r.dim), ...INC[r.bits].map((n) => mult(`inc-n${n}`, "baseline", n)), ks("ksum4", 4, K4[r.bits]), ks("ksum5", 5, K5[r.bits])];
  const spec = { schema: "ecbench.spec/v1", label: `ic_tableb_binary_ksum_20261115 ${r.name} replicate ${rep}`,
    description: "Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
    workloads: { curves: [explicit(r.name)], targets_per_curve: 48, target_seed: 20261115000 + r.bits * 10 + rep, target_kind: "public" },
    arms, measurement: { rounds: 3, warmup: 1, order: "alternate", seed: 20261115100 + r.bits * 10 + rep, isolation_required: "L0", timeout_seconds: 1800 } };
  writeFileSync(`specs/B${r.bits}-r${rep}.json`, JSON.stringify(spec, null, 1) + "\n");
}
