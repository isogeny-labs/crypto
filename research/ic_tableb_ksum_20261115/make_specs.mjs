// Spec generator (layout of frozen parameters from PROTOCOL.md): `node make_specs.mjs` writes specs/P<bits>-r<rep>.json.
// Curves are the Round-5 ledger's explicit prime curves (Table B), resolved to Table B's ICV1 slugs by `ecbench plan`.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const LEDGER = "../../docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json";
const NAME = { 20: "bench-20bit", 22: "generated-22bit-3290411", 24: "generated-24bit-10935329" };
const TB = { 20: 51, 22: 74, 24: 111 };                  // Table B |F|/2 (signed 102, 148, 222)
const REC = { 20: 45, 24: 162 };                          // leaderboard recorded baseline
const OTHER = { 20: [["inc-sel", 64]], 24: [["inc-w0", 115]], 22: [["inc-lo", 53], ["inc-hi", 104]] };
const A4 = { 20: 10, 22: 12, 24: 14 }, A5 = { 20: 9, 22: 10, 24: 11 };
const MAXT = { 20: "1410236", 22: "4243777", 24: "4243777" };
const KMAX = "13528399621";
const REPS = [1, 2, 3];

const ledger = JSON.parse(readFileSync(LEDGER, "utf8")).ledger.instances;
const explicit = (bits) => {
  const i = ledger.find((x) => x.curve.name === NAME[bits]);
  const c = i.curve;
  return { kind: "prime_explicit", p: c.p, a: c.a, b: c.b, group_order: c.group_order, r: i.r, gx: c.generator.x, gy: c.generator.y };
};
const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
const mitm = (name, role, n, b) => arm(name, role, "ic.pipeline", { factor_base: `prime-abscissa:size=${n}`, oracle: "mitm:negation_folded=1", max_trials: MAXT[b] });
const ks = (name, k, a) => arm(name, "candidate", "ic.pipeline", { factor_base: `prime-abscissa:size=${a}`, oracle: `ksum:m=${k}`, max_trials: KMAX });

mkdirSync("specs", { recursive: true });
for (const b of [20, 22, 24]) for (const rep of REPS) {
  const arms = [arm("rho-neg", "reference", "rho.negation"), arm("rho-neg-aa", "control", "rho.negation"), arm("bsgs-neg", "reference", "bsgs.negation"),
    mitm("inc-tb", "baseline", TB[b], b)];
  if (REC[b] !== undefined) arms.push(mitm("inc-rec", "candidate", REC[b], b));
  for (const [nm, n] of OTHER[b]) arms.push(mitm(nm, "candidate", n, b));
  arms.push(ks("ksum4", 4, A4[b]), ks("ksum5", 5, A5[b]));
  const spec = {
    schema: "ecbench.spec/v1",
    label: `ic_tableb_ksum_20261115 ${b}-bit Table B curve, replicate ${rep}`,
    description: "Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
    workloads: { curves: [explicit(b)], targets_per_curve: 48, target_seed: 20261115000 + b * 10 + rep, target_kind: "public" },
    arms,
    measurement: { rounds: 3, warmup: 1, order: "alternate", seed: 20261115100 + b * 10 + rep, isolation_required: "L0", timeout_seconds: 1800 },
  };
  writeFileSync(`specs/P${b}-r${rep}.json`, JSON.stringify(spec, null, 1) + "\n");
}
