#!/usr/bin/env node
// Deterministic spec generator for research/koblitz_frob_ksum_sib_20261118.
// Held-out target seeds 2026111801..2026111809, disjoint from pilot seeds 999001..999005.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const SPECS_DIR = join(HERE, "specs");
mkdirSync(SPECS_DIR, { recursive: true });

const LEDGER_PATH = join(HERE, "../../docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json");
const ledgerInstances = JSON.parse(readFileSync(LEDGER_PATH, "utf8")).ledger.instances;

function binaryExplicit(name) {
  const inst = ledgerInstances.find((x) => x.curve.name === name);
  if (!inst) throw new Error(`missing ledger instance ${name}`);
  const c = inst.curve;
  const n = c.field.degree;
  const modulus = c.field.polynomial_low_terms.reduce((acc, t) => acc + 2 ** t, 2 ** n);
  const hex = (s) => parseInt(s, 16);
  return {
    kind: "binary_explicit",
    n,
    modulus,
    a: Number(c.a),
    b: hex(c.b),
    group_order: c.group_order,
    r: inst.r,
    gx: hex(c.generator.x),
    gy: hex(c.generator.y),
  };
}

const MAX_TRIALS = "2000000000";

function genLayoutArm(cols) {
  return {
    name: `inc-gen-layout-c${cols}`,
    role: "baseline",
    method: {
      id: "ic.pipeline",
      params: {
        factor_base: `compact-orbit-scan:columns=${cols},raw_x_cap=100000000,generator_orbit=1`,
        oracle: "mitm-frobenius-layout-counted:m=2",
        max_trials: MAX_TRIALS,
        targets: "stride-generator",
      },
    },
  };
}

const kf3BaselineArm = {
  name: "inc-kf3",
  role: "baseline",
  method: {
    id: "ic.pipeline",
    params: {
      factor_base: "compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1",
      oracle: "ksum-frobenius:m=3",
      max_trials: MAX_TRIALS,
      targets: "stride-generator",
    },
  },
};

const kf4PrimaryArm = {
  name: "cand-kf4-c2-free",
  role: "candidate",
  method: {
    id: "ic.pipeline",
    params: {
      factor_base: "compact-orbit-scan:columns=2,raw_x_cap=100000000,generator_orbit=1",
      oracle: "ksum-frobenius:m=4,layout=1,prune=1,sib=1,free=1,scale=1.35",
      max_trials: MAX_TRIALS,
      targets: "stride-generator",
    },
  },
};

const kf5SecondaryArm = {
  name: "cand-kf5-c1",
  role: "candidate",
  method: {
    id: "ic.pipeline",
    params: {
      factor_base: "compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1",
      oracle: "ksum-frobenius:m=5,layout=1,prune=1,sib=1,scale=1.0",
      max_trials: MAX_TRIALS,
      targets: "stride-generator",
    },
  },
};

const koblitzRows = [
  { id: "K1-n29", a: 1, n: 29, targets: 32, targetSeed: 2026111801, seed: 8101, cols: [2, 4], includeKf3: true, includeKf5: true, timeout: 120 },
  { id: "K0-n23", a: 0, n: 23, targets: 32, targetSeed: 2026111802, seed: 8102, cols: [6], includeKf3: true, includeKf5: true, timeout: 120 },
  { id: "K1-n23", a: 1, n: 23, targets: 32, targetSeed: 2026111803, seed: 8103, cols: [8], includeKf3: true, includeKf5: true, timeout: 120 },
  { id: "K0-n37", a: 0, n: 37, targets: 32, targetSeed: 2026111804, seed: 8104, cols: [13, 16], includeKf3: true, includeKf5: true, timeout: 120 },
  { id: "K0-n41", a: 0, n: 41, targets: 24, targetSeed: 2026111805, seed: 8105, cols: [90, 158], includeKf3: false, includeKf5: false, timeout: 180 },
  { id: "K0-n53", a: 0, n: 53, targets: 16, targetSeed: 2026111806, seed: 8106, cols: [220], includeKf3: false, includeKf5: false, timeout: 300 },
];

for (const row of koblitzRows) {
  const arms = [
    { name: "rho-strong", role: "reference", method: { id: "rho.signed_frobenius_strong" } },
    { name: "rho-strong-aa", role: "control", method: { id: "rho.signed_frobenius_strong" } },
    ...row.cols.map(genLayoutArm),
  ];
  if (row.includeKf3) arms.push(kf3BaselineArm);
  arms.push(kf4PrimaryArm);
  if (row.includeKf5) arms.push(kf5SecondaryArm);

  const spec = {
    schema: "ecbench.spec/v1",
    label: `Cohort K (${row.id}): cold single-target Koblitz IC speedup`,
    description: "Preregistered paired comparison of cand-kf4-c2-free (and cand-kf5-c1) against gen-layout, kf3, and rho.signed_frobenius_strong.",
    workloads: {
      curves: [{ kind: "koblitz", a: row.a, n: row.n }],
      targets_per_curve: row.targets,
      target_seed: row.targetSeed,
      target_kind: "public",
    },
    arms,
    measurement: {
      rounds: 2,
      warmup: 1,
      order: "alternate",
      seed: row.seed,
      isolation_required: "L0",
      timeout_seconds: row.timeout,
    },
  };
  writeFileSync(join(SPECS_DIR, `${row.id}.json`), JSON.stringify(spec, null, 2) + "\n");
}

const binaryRows = [
  { id: "B21", ledgerName: "random-binary-n21-b1b6f3b", targetSeed: 2026111807, seed: 8107, n4: 10, n5: 9, nCand: 10 },
  { id: "B24", ledgerName: "random-binary-n24-b5fc9da", targetSeed: 2026111808, seed: 8108, n4: 12, n5: 9, nCand: 10 },
  { id: "B27", ledgerName: "random-binary-n27-b845462", targetSeed: 2026111809, seed: 8109, n4: 16, n5: 12, nCand: 12 },
];

for (const row of binaryRows) {
  const spec = {
    schema: "ecbench.spec/v1",
    label: `Cohort B (${row.id}): BinaryGroup::sibling_pair ksum speedup on Table B primary curve`,
    description: "Preregistered paired comparison of ksum:m=5,prune=1,sib=1,free=1 against 2026-11-15 ksum:m=4|5,prune=1 incumbents and rho.negation.",
    workloads: {
      curves: [binaryExplicit(row.ledgerName)],
      targets_per_curve: 32,
      target_seed: row.targetSeed,
      target_kind: "public",
    },
    arms: [
      { name: "rho-neg", role: "reference", method: { id: "rho.negation" } },
      { name: "rho-neg-aa", role: "control", method: { id: "rho.negation" } },
      {
        name: "inc-ksum4-prune",
        role: "baseline",
        method: {
          id: "ic.pipeline",
          params: {
            factor_base: `binary-multiplier:size=${row.n4},mult=1`,
            oracle: "ksum:m=4,prune=1",
            max_trials: "13528399621",
          },
        },
      },
      {
        name: "inc-ksum5-prune",
        role: "baseline",
        method: {
          id: "ic.pipeline",
          params: {
            factor_base: `binary-multiplier:size=${row.n5},mult=1`,
            oracle: "ksum:m=5,prune=1",
            max_trials: "13528399621",
          },
        },
      },
      {
        name: "cand-ksum5-sib-free",
        role: "candidate",
        method: {
          id: "ic.pipeline",
          params: {
            factor_base: `binary-multiplier:size=${row.nCand},mult=1`,
            oracle: "ksum:m=5,prune=1,sib=1,free=1",
            max_trials: "13528399621",
          },
        },
      },
    ],
    measurement: {
      rounds: 2,
      warmup: 1,
      order: "alternate",
      seed: row.seed,
      isolation_required: "L0",
      timeout_seconds: 120,
    },
  };
  writeFileSync(join(SPECS_DIR, `${row.id}.json`), JSON.stringify(spec, null, 2) + "\n");
}
console.log("Generated 9 preregistered specs in", SPECS_DIR);
