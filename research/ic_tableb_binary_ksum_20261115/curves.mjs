// Table B random-binary curves from the Round-5 ledger (binary_explicit); shared by make_specs.mjs and pilot/make_pilot.mjs.
import { readFileSync } from "node:fs";
export const LEDGER = new URL("../../docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json", import.meta.url).pathname;
export const ROWS = [
  { bits: 15, name: "random-binary-n15-b524b", role: "secondary", slug: "icv1-f2m15-t131-ed7256f0", dim: 5 },
  { bits: 18, name: "random-binary-n18-b6507", role: "secondary", slug: "icv1-f2m18-t393-d2c04152", dim: 5 },
  { bits: 21, name: "random-binary-n21-b1b6f3b", role: "primary", slug: "icv1-f2m21-t2591-de54d72e", dim: 6 },
  { bits: 24, name: "random-binary-n24-b5fc9da", role: "primary", slug: "icv1-f2m24-tm3047-2d0c0e8c", dim: 7 },
  { bits: 27, name: "random-binary-n27-b845462", role: "primary", slug: "icv1-f2m27-t12543-569dca8b", dim: 8 },
];
const inst = JSON.parse(readFileSync(LEDGER, "utf8")).ledger.instances;
export const explicit = (name) => {
  const i = inst.find((x) => x.curve.name === name), c = i.curve;
  const n = c.field.degree;
  const modulus = c.field.polynomial_low_terms.reduce((a, t) => a + 2 ** t, 2 ** n);
  const h = (s) => parseInt(s, 16);
  return { kind: "binary_explicit", n, modulus, a: Number(c.a), b: h(c.b), group_order: c.group_order, r: i.r, gx: h(c.generator.x), gy: h(c.generator.y) };
};
export const arm = (name, role, id, params) => ({ name, role, method: params ? { id, params } : { id } });
export const MAXT = "13528399621";
export const sub = (name, role, d) => arm(name, role, "ic.pipeline", { factor_base: `binary-subspace:dimension=${d}`, oracle: "mitm:m=2,negation_folded=1", max_trials: MAXT });
export const mult = (name, role, n) => arm(name, role, "ic.pipeline", { factor_base: `binary-multiplier:size=${n},mult=1`, oracle: "mitm:m=2,negation_folded=1", max_trials: MAXT });
export const ks = (name, k, n) => arm(name, "candidate", "ic.pipeline", { factor_base: `binary-multiplier:size=${n},mult=1`, oracle: `ksum:m=${k}`, max_trials: MAXT });
