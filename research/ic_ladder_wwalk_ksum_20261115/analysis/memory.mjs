// Peak RSS and wall time per arm and session, from the recorded runs (time.max_rss_kib, time.process_wall_ns).
//   node analysis/memory.mjs <out dir>      (reads <out>/{train,test,control}/*/records.jsonl)
import { readdirSync, readFileSync, existsSync } from "node:fs";
const out = process.argv[2];
const rows = [];
for (const stage of ["train", "test", "control"]) {
  const d = `${out}/${stage}`;
  if (!existsSync(d)) continue;
  for (const s of readdirSync(d)) {
    const f = `${d}/${s}/records.jsonl`;
    if (!existsSync(f)) continue;
    const per = {};
    for (const line of readFileSync(f, "utf8").split("\n")) {
      if (!line.trim()) continue;
      const r = JSON.parse(line);
      const a = (per[r.arm] ??= { runs: 0, rss: 0, wall: 0, s: 0 });
      a.runs++; a.rss = Math.max(a.rss, r.time?.max_rss_kib ?? 0); a.wall += (r.time?.process_wall_ns ?? 0) / 1e9; a.s += r.cost?.s ?? 0;
    }
    for (const [arm, a] of Object.entries(per)) rows.push({ stage, session: s, arm, runs: a.runs, peak_rss_gib: +(a.rss / 1048576).toFixed(2), wall_s_per_run: +(a.wall / a.runs).toFixed(2), mean_S_w0: +(a.s / a.runs).toFixed(3) });
  }
}
console.log("stage\tsession\tarm\truns\tpeak_rss_GiB\twall_s_per_run\tmean_S_w0(unpriced)");
for (const r of rows) console.log([r.stage, r.session, r.arm, r.runs, r.peak_rss_gib, r.wall_s_per_run, r.mean_s_w0 ?? r.mean_S_w0].join("\t"));
const bySession = {};
for (const r of rows) bySession[`${r.stage}/${r.session}`] = (bySession[`${r.stage}/${r.session}`] ?? 0) + r.wall_s_per_run * r.runs;
console.log("\nsession\ttotal_process_wall_h");
for (const [k, v] of Object.entries(bySession)) console.log(`${k}\t${(v / 3600).toFixed(3)}`);
