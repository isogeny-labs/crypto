// Spec generator: `node make_specs.mjs` (cwd = this directory) writes specs/<row>-r<rep>.json (24 specs).
// Curves, targets_per_curve, rounds, warm-up, order, isolation and the k-sum arms' factor bases are copied from the
// prune study's spec of the same name (research/ic_ksum_prune_20261115/specs); the arm list is new and the target
// and measurement seeds are fresh (registered in PROTOCOL.md section 2).
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { ROWS } from "./rows.mjs";
mkdirSync("specs", { recursive: true });
const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
let n = 0;
ROWS.forEach((row, ri) => {
  for (const rep of [1, 2, 3]) {
    const name = `${row.id}-r${rep}`;
    const s = JSON.parse(readFileSync(`../ic_ksum_prune_20261115/specs/${name}.json`, "utf8"));
    const fb = (k) => s.arms.find((a) => a.name === `ksum${k}-p`).method.params;
    const mk = (k, suffix, split, stride) => {
      const p = fb(k);
      const params = { factor_base: p.factor_base, oracle: `ksum:m=${k},prune=1${split ? ",split=1" : ""}`, max_trials: p.max_trials };
      if (stride) params.targets = "stride-renew";
      return arm(`ksum${k}-${suffix}`, "candidate", "ic.pipeline", params);
    };
    const idx = ri * 10 + rep;
    const spec = {
      schema: "ecbench.spec/v1",
      label: `ic_ksum_split_20261118 ${name} (curves of ic_ksum_prune_20261115)`,
      description: "Paired study of ksum split=1 (pair-sum build) and targets=stride-renew against the pruned k-sum incumbent (prune=1, walk). Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
      workloads: { ...s.workloads, target_seed: 20261118000 + idx },
      arms: [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"),
        mk(4, "p", false, false), mk(5, "p", false, false), mk(4, "ps", true, false), mk(5, "ps", true, false),
        mk(4, "pr", false, true), mk(5, "pr", false, true), mk(4, "psr", true, true), mk(5, "psr", true, true)],
      measurement: { ...s.measurement, seed: 20261118500 + idx },
    };
    writeFileSync(`specs/${name}.json`, JSON.stringify(spec, null, 1) + "\n"); n++;
  }
});
console.log(n, "specs");
