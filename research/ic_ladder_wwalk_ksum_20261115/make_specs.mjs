// Spec generator for PROTOCOL.md.  Thin orchestration: it lays out frozen parameters as ecbench.spec/v1 files.
//   node make_specs.mjs train            specs/train/T<bits>.json   (k = 6, 7 base-size grids; selection stage)
//   [SIZES="48"] node make_specs.mjs test   specs/test/P<bits>.json  (needs params/params-<bits>.json from select.mjs)
//   node make_specs.mjs control          specs/control/PC32.json    (k = 32 positive control)
//   TOY=1 SIZES="16 20" node make_specs.mjs train|test   toy sizes (prime_search curves, model-optimum bases, NON-FROZEN seeds,
//                                         3-4 targets) into specs-toy/: the local pipeline smoke test only, never evidence
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";

const mode = process.argv[2];
const here = new URL(".", import.meta.url).pathname;
const curves = JSON.parse(readFileSync(here + "curves.json", "utf8"));
const TOY = process.env.TOY === "1";
const SIZES = TOY ? (process.env.SIZES ?? "16 20").split(" ").map(Number) : [36, 40, 44, 48];
const OUT = TOY ? "specs-toy" : "specs";
const KMAX = "13528399621";

// Frozen before any data (PROTOCOL.md section 4).
const A5 = { 36: 25, 40: 33, 44: 44, 48: 59 };          // research/prime_ksum_k5_20261028/params.json (train picks, = model optimum)
const A8 = { 36: 15, 40: 17, 44: 19, 48: 23 };          // model optimum, analysis/model.mjs (ksum8 is a control, not trained)
const ASTAR = { 6: { 36: 19, 40: 23, 44: 29, 48: 36 }, 7: { 36: 16, 40: 19, 44: 23, 48: 27 } }; // model optimum (analysis/model.mjs)
const GRID = [0.7, 0.85, 1, 1.1, 1.2];                  // train grid factors on A*
const CAP = 140e6;                                       // grid points with more table entries are dropped (memory, pod 32 GiB)
const SEEDS = TOY ? { train: 20261115900, test: 20261115950, control: 20261115990 } : {   // target_seed = BASE + bits*100 + 1 ; session seed +500
  train: 20261115000, test: 20261115200, control: 20261115300,
};
const choose = (a, k) => { let c = 1; for (let i = 0; i < k; i++) c = (c * (a - i)) / (i + 1); return c; };
const model = (k, r) => { const M = (a) => 2 ** (k - 1) * choose(a, k), S = (a) => M(a) + (a * r) / (2 * M(a)); let best = k + 1; for (let a = k + 1; a < 400; a++) if (S(a) < S(best)) best = a; return best; };
const curveOf = (b) => (TOY ? { kind: "prime_search", bits: b, seed: 5 } : curves.ladder[b]);
const rOf = (b) => 2 ** b;
if (TOY) for (const b of SIZES) { A5[b] = model(5, rOf(b)); A8[b] = model(8, rOf(b)); ASTAR[6][b] = model(6, rOf(b)); ASTAR[7][b] = model(7, rOf(b)); }

const entries = (a, k) => { let c = 1; for (let i = 0; i < k; i++) c = (c * (a - i)) / (i + 1); return Math.round(c * 2 ** (k - 1)); };
const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
const ks = (name, role, k, a) => arm(name, role, "ic.pipeline", { factor_base: `prime-abscissa:size=${a}`, oracle: `ksum:m=${k}`, max_trials: KMAX });
const WFB = (n) => `prime-projected-abscissa:size=${n},scan_cap=32`;
const wwalkC2 = (name, role, x) => arm(name, role, "ic.pipeline", { factor_base: WFB(2), oracle: `wwalk:x=${x},g=1,grow=2,seed=run` });
// The C = 1 recipe of research/curve25519_autolab_20261104/c1 (c1-x07-j16-g): a labelled BSGS-boundary arm, not an IC row.
const wwalkC1 = () => arm("wwalk-c1", "control", "ic.pipeline", { factor_base: WFB(1), oracle: "wwalk:x=0.7,jumps=16,g=1,grow=1,seed=run", targets: "stride-generator" });
const generic = () => [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"), arm("bsgs-neg", "reference", "bsgs.negation")];

const write = (dir, file, spec) => { mkdirSync(`${here}${OUT}/${dir}`, { recursive: true }); writeFileSync(`${here}${OUT}/${dir}/${file}`, JSON.stringify(spec, null, 1) + "\n"); };
const base = (label, curve, targets, tseed, mseed, arms, rounds, warmup) => ({
  schema: "ecbench.spec/v1",
  label: `ic_ladder_wwalk_ksum_20261115 ${label}`,
  description: "Generic decomposition track (bounded by sqrt(r); cannot beat rho or BSGS). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
  workloads: { curves: [curve], targets_per_curve: targets, target_seed: tseed, target_kind: "public" },
  arms,
  measurement: { rounds, warmup, order: "alternate", seed: mseed, isolation_required: "L0", timeout_seconds: 86400 },
});

if (mode === "train") {
  for (const b of (process.env.SIZES ?? SIZES.join(" ")).split(" ").map(Number)) {
    const arms = [arm("rho-neg", "reference", "rho.negation")];
    for (const k of [6, 7]) for (const f of GRID) {
      const a = Math.max(k + 1, Math.round(ASTAR[k][b] * f));
      if (entries(a, k) > CAP) continue;
      if (!arms.some((x) => x.name === `ksum${k}-A${a}`)) arms.push(ks(`ksum${k}-A${a}`, "candidate", k, a));
    }
    write("train", `T${b}.json`, base(`train ${b}-bit`, curveOf(b), TOY ? 3 : 4, SEEDS.train + b * 100 + 1, SEEDS.train + b * 100 + 501, arms, 1, 0));
  }
} else if (mode === "test") {
  const only = (process.env.SIZES ?? SIZES.join(" ")).split(" ").map(Number);
  for (const b of only) {
    const pf = `${here}${TOY ? "params-toy" : "params"}/params-${b}.json`;
    if (!existsSync(pf)) throw new Error(`${pf} missing: run select.mjs after the train stage`);
    const P = { [b]: JSON.parse(readFileSync(pf, "utf8")).selected };
    const arms = [...generic(),
      ks("ksum5-sel", "baseline", 5, A5[b]), ks("ksum6-sel", "candidate", 6, P[b][6]), ks("ksum7-sel", "candidate", 7, P[b][7]),
      ks("ksum8-model", "control", 8, A8[b]),
      wwalkC2("wwalk-c2", "candidate", 0.9), wwalkC2("wwalk-c2-aa", "control", 0.9), wwalkC2("wwalk-c2-x06", "candidate", 0.6), wwalkC1()];
    write("test", `P${b}.json`, base(`${b}-bit ladder curve, test panel`, curveOf(b), TOY ? 4 : 24, SEEDS.test + b * 100 + 1, SEEDS.test + b * 100 + 501, arms, 2, 1));
  }
} else if (mode === "control") {
  const c = { ...curves.positive_control_k32 }; delete c.note;
  const arms = [...generic().slice(0, 2), wwalkC2("wwalk-c2-x06", "baseline", 0.6), wwalkC2("wwalk-c2", "candidate", 0.9), wwalkC1()];
  write("control", "PC32.json", base("positive control, autolab k = 32 curve", c, 24, SEEDS.control + 1, SEEDS.control + 501, arms, 2, 1));
} else throw new Error("mode: train | test | control");
console.log(`wrote specs for mode ${mode}`);
