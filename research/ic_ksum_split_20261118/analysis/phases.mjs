// Step 1 (before any data of this study): break the charged cost of the incumbent pruned k-sum arms into phases,
// from the committed, validated prune study's sessions (research/ic_ksum_prune_20261115/sessions, VAL-20261115-276156).
// Measured rounds only (warm-up excluded). Every quantity is per sqrt(r), mean over the 432 runs of a row and arm.
// usage (cwd = this study directory): node analysis/phases.mjs > analysis/phases.json
import { readFileSync, readdirSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import { ROWS } from "../rows.mjs";
const SRC = "../ic_ksum_prune_20261115/sessions";
const ARMS = ["ksum4-p", "ksum5-p", "rho-neg"];
const out = { source: SRC, unit: "counted GAE per sqrt(r) (w = 0) and native operations per sqrt(r)", rows: {} };
for (const row of ROWS) {
  const acc = {};
  for (const rep of [1, 2, 3]) {
    const lines = gunzipSync(readFileSync(`${SRC}/${row.id}-r${rep}/records.jsonl.gz`)).toString().trim().split("\n");
    for (const l of lines) {
      const r = JSON.parse(l);
      if (r.warmup || !ARMS.includes(r.arm)) continue;
      const sq = Math.sqrt(Number(r.workload.curve.r));
      const a = (acc[r.arm] ??= { n: 0 });
      const add = (k, v) => { a[k] = (a[k] ?? 0) + v / sq; };
      a.n++; a.S = (a.S ?? 0) + r.cost.s;
      for (const p of r.phases) {
        add(`${p.name}.gae`, p.gae); add(`${p.name}.adds`, p.adds); add(`${p.name}.doubles`, p.doubles ?? 0);
        add(`${p.name}.scalar_mults`, p.scalar_mults ?? 0);
        for (const [k, v] of Object.entries(p.native ?? {})) if (typeof v === "number") add(`${p.name}.native.${k}`, v);
        for (const [k, v] of Object.entries(p.work_native ?? {})) if (typeof v === "number" && !k.startsWith("counted:")) add(`${p.name}.work.${k}`, v);
      }
    }
  }
  const res = {};
  for (const [arm, a] of Object.entries(acc)) {
    const m = {}; for (const [k, v] of Object.entries(a)) if (k !== "n") m[k] = v / a.n;
    m.runs = a.n;
    if (arm.startsWith("ksum")) {
      const rel = (k) => m[`relations.${k}`] ?? 0;
      // Phase split of S (w = 0): table build, the walk's own steps (one add each), the set-up of the walk
      // (jumps, pooled restarts, start: scalar multiplications, their doubles and adds), the probe confirmations, the rest.
      const steps = rel("native.walk_steps"), relAdds = rel("adds");
      m.split_of_S = {
        oracle_setup_build: m["oracle_setup.gae"],
        walk_steps: steps,
        walk_setup_and_restarts: rel("gae") - steps - 0, // includes the probe confirmations (k-1 adds per hit), see below
        probe_confirm_adds: rel("native.relations") * (Number(arm[4]) - 1),
        linear_algebra: m["linear_algebra.gae"] ?? 0,
        factor_base: m["factor_base.gae"] ?? 0,
        verify: m["verify.gae"] ?? 0,
      };
      m.split_of_S.walk_setup_and_restarts -= m.split_of_S.probe_confirm_adds;
      m.split_of_S.relations_adds_not_steps = relAdds - steps;
    }
    res[arm] = m;
  }
  out.rows[row.id] = res;
}
console.log(JSON.stringify(out, null, 1));
