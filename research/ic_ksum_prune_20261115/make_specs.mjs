// Spec generator: `node make_specs.mjs` (cwd = this directory) writes specs/<row>-r<rep>.json (24 specs).
// Each spec copies curve, workloads (target_seed, targets_per_curve), measurement (seed, rounds, warmup, order, isolation)
// and the ksum arms' factor bases from the sibling study's spec of the same name; only the arm list differs.
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs";
const SIB = ["../ic_tableb_ksum_20261115/specs", "../ic_tableb_binary_ksum_20261115/specs"];
mkdirSync("specs", { recursive: true });
const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
let n = 0;
for (const dir of SIB) for (const f of readdirSync(dir).filter((x) => x.endsWith(".json")).sort()) {
  const s = JSON.parse(readFileSync(`${dir}/${f}`, "utf8"));
  const ks = (nm) => s.arms.find((a) => a.name === nm).method.params;
  const mk = (name, k, prune) => { const p = ks(`ksum${k}`);
    return arm(name, "candidate", "ic.pipeline", { factor_base: p.factor_base, oracle: `ksum:m=${k}${prune ? ",prune=1" : ""}`, max_trials: p.max_trials }); };
  const row = f.replace(".json", "");
  const spec = {
    schema: "ecbench.spec/v1",
    label: `ic_ksum_prune_20261115 ${row} (twin of ${s.label.split(" ")[0]})`,
    description: "Paired study of ksum prune=1 against prune=0 (build adds only). Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
    workloads: s.workloads,
    arms: [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"),
      mk("ksum4", 4, false), mk("ksum4-p", 4, true), mk("ksum5", 5, false), mk("ksum5-p", 5, true)],
    measurement: s.measurement,
  };
  writeFileSync(`specs/${f}`, JSON.stringify(spec, null, 1) + "\n"); n++;
}
console.log(n, "specs");
