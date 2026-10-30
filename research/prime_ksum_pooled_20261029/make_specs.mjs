// Spec generator (tooling) for PROTOCOL.md: `node make_specs.mjs train` writes
// specs/train-P<b>.json; `node make_specs.mjs holdout` reads select/selection.json
// (written by select.mjs) and writes specs/holdout-P<b>.json.
import { readFileSync, writeFileSync } from "node:fs";

const SIZES = [20, 24, 28, 32];
const N0 = { 20: 32, 24: 162, 28: 407, 32: 1024 }; // incumbent recipe's base (prime_base_oracle_tuning_20261029 `inc`)
const REC = { 20: 45, 24: 162, 28: 407, 32: 1024 }; // the recorded incumbent of #2866
const A4 = { 20: 10, 24: 14, 28: 21, 32: 30 }; // frozen k = 4 sizes (#2808)
const A5 = { 20: 9, 24: 11, 28: 15, 32: 19 }; // frozen k = 5 sizes (#2852)
const MAXT = { 20: "1410236", 24: "4243777", 28: "26479085", 32: "167871873" };
const F = [0.5, 0.71, 1, 1.41, 2];
const grid = (a0) => [...new Set(F.map((f) => Math.max(2, Math.round(a0 * f))))];
const base = JSON.parse(readFileSync("../prime_ksum_vs_incumbent_20261029/specs/P24.json", "utf8"));
const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
const mitm = (n, b) => arm(`inc-n${n}`, "candidate", "ic.pipeline", { factor_base: `prime-abscissa:size=${n}`, oracle: "mitm:negation_folded=1", max_trials: MAXT[b] });
const ks = (k, a) => arm(`ksum${k}-a${a}`, "candidate", "ic.pipeline", { factor_base: `prime-abscissa:size=${a}`, oracle: `ksum:m=${k}`, max_trials: "13528399621" });
const mode = process.argv[2];
for (const b of SIZES) {
  const t = mode === "train";
  const seeds = [0, 1, 2, 3].slice(0, t ? 2 : 4).map((i) => (t ? 20261029100 : 20261029200) + b * 10 + i);
  const spec = {
    schema: base.schema,
    label: `prime_ksum_pooled_20261029 ${mode}, ${b}-bit prime-order curves`,
    description: "Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curves and targets; no security claim.",
    workloads: { curves: seeds.map((s) => ({ kind: "prime_search", bits: b, seed: s })), targets_per_curve: t ? 12 : 24, target_seed: (t ? 20261029111 : 20261029211) + b, target_kind: "public" },
    measurement: { ...base.measurement, seed: (t ? 20261029121 : 20261029221) + b },
  };
  if (t) {
    spec.arms = [arm("rho-neg", "reference", "rho.negation"), arm("bsgs-neg", "reference", "bsgs.negation"),
      ...grid(N0[b]).concat(b === 20 ? [45] : []).filter((v, i, a) => a.indexOf(v) === i).map((n) => mitm(n, b)),
      ...grid(A4[b]).map((a) => ks(4, a)), ...grid(A5[b]).map((a) => ks(5, a))];
  } else {
    const sel = JSON.parse(readFileSync("select/selection.json", "utf8"))[b];
    const named = (nm, a) => ({ ...a, name: nm, role: "candidate" });
    spec.arms = [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"), arm("bsgs-neg", "reference", "bsgs.negation"),
      named("inc-sel", mitm(sel.inc.n, b)), ...(sel.inc0.n !== sel.inc.n ? [named("inc-w0", mitm(sel.inc0.n, b))] : []), named("inc-rec", mitm(REC[b], b)),
      named("ksum4-sel", ks(4, sel.ksum4.a)), named("ksum5-sel", ks(5, sel.ksum5.a))];
    spec.arms[3].role = "baseline";
  }
  writeFileSync(`specs/${mode}-P${b}.json`, JSON.stringify(spec, null, 2) + "\n");
}
