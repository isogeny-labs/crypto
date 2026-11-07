import fs from "fs";
const [newRoot, oldRoot] = process.argv.slice(2);
const load = (f, arm) => fs.readFileSync(f, "utf8").trim().split("\n").map(JSON.parse).filter(r => r.arm === arm);
let bad = 0;
for (const b of [20, 24, 28, 32, 36, 40]) {
  const N = load(`${newRoot}/P${b}/records.jsonl`, "ic-ksum4"), O = load(`${oldRoot}/P${b}/records.jsonl`, "ic-ksum4");
  const key = r => `${r.workload.target_index}|${r.round}|${r.warmup}`;
  const om = new Map(O.map(r => [key(r), r]));
  let match = 0, miss = 0, diff = 0;
  for (const r of N) {
    const o = om.get(key(r));
    if (!o) { miss++; continue; }
    const same = r.cost.total_gae === o.cost.total_gae && JSON.stringify(r.counters) === JSON.stringify(o.counters)
      && r.algorithm_seed === o.algorithm_seed && r.workload.workload_sha256 === o.workload.workload_sha256
      && r.method.method_sha256 === o.method.method_sha256 && r.outcome.recovered === o.outcome.recovered;
    if (same) match++; else { diff++; if (diff <= 2) console.log("DIFF", b, key(r), r.cost.total_gae, o.cost.total_gae, r.method.method_sha256 === o.method.method_sha256, r.algorithm_seed === o.algorithm_seed); }
  }
  console.log(`P${b}: new ${N.length} old ${O.length} match ${match} diff ${diff} missing ${miss}`);
  bad += diff + miss + (N.length !== O.length);
}
console.log(bad ? "REPLICATION FAILED" : "REPLICATION OK");
process.exit(bad ? 1 : 0);
