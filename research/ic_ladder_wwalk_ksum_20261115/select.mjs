// Selection stage (PROTOCOL.md section 4): per size and per k, the base size with the smallest mean S at w = 0.1
// among arms verified on every train run.  Reads `ecbench band` output only; writes params.json.
//   SIZES="48" node select.mjs <ecbench binary> <train sessions dir> [out dir, default params/]  -> params/params-<bits>.json per size
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";

const [bin, dir, out = new URL(process.env.TOY === "1" ? "params-toy" : "params", import.meta.url).pathname] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const sizes = (process.env.SIZES ?? "36 40 44 48").split(" ").map(Number);
const tmp = mkdtempSync(`${tmpdir()}/sel-`);
const selected = {}, detail = {};
for (const b of sizes) {
  const f = `${tmp}/band-${b}.json`;
  execFileSync(bin, ["band", "--dir", `${dir}/T${b}`, "--json", f], { encoding: "utf8", maxBuffer: 1 << 28 });
  const rows = JSON.parse(readFileSync(f, "utf8")).rows;
  selected[b] = {}; detail[b] = {};
  for (const k of [6, 7]) {
    const cand = rows.filter((r) => r.arm.startsWith(`ksum${k}-A`) && r.verified === r.runs && r.runs > 0)
      .map((r) => ({ arm: r.arm, A: Number(r.arm.split("-A")[1]), s_w0: r.mean_s[0], s_w01: r.mean_s[1], s_w1: r.mean_s[2] }))
      .sort((x, y) => x.A - y.A);
    if (!cand.length) throw new Error(`no verified ksum${k} candidate at ${b} bits`);
    const best = cand.reduce((x, y) => (y.s_w01 < x.s_w01 ? y : x));
    selected[b][k] = best.A;
    detail[b][k] = { candidates: cand, interior: best.A !== cand[0].A && best.A !== cand[cand.length - 1].A };
  }
}
for (const b of sizes) writeFileSync(`${out}/params-${b}.json`, JSON.stringify({
  schema: "ic_ladder_wwalk_ksum.params/v1", bits: b,
  rule: "smallest mean S at w = 0.1 among candidates verified on every train run (4 targets, target seed 20261115000 + 100*bits + 1); ksum5 is frozen from research/prime_ksum_k5_20261028/params.json; edge optima are reported, the grid is not extended",
  selected: selected[b], detail: detail[b],
}, null, 1) + "\n");
console.log(JSON.stringify(selected), Object.entries(detail).map(([b, d]) => `${b}: interior k6=${d[6].interior} k7=${d[7].interior}`).join("; "));
