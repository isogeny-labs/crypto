// PROTOCOL.md "Measurements and readings": per (bits, ℓ, B), planted success
// (hg_holds and ≥ 3 vanishing rows), unplanted hg_holds, ms per reduction
// against (B+1)^2 enumeration; reach = largest B with ≥ half succeeding.
// Writes analysis/${OUT}.json, analysis/${OUT}.md, analysis/${OUT}.svg.  Run here.
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs";

const DIR = process.env.DATA ?? "data";
const OUT = process.env.OUT ?? "l4a";
const all = readdirSync(DIR).filter((f) => f.endsWith(".jsonl"))
  .flatMap((f) => readFileSync(`${DIR}/${f}`, "utf8").trim().split("\n").filter(Boolean).map(JSON.parse));
const skipped = all.filter((r) => r.skipped);
const recs = all.filter((r) => !r.skipped && !r.error);
const bitsOf = (p) => Math.round(Math.log2(p));
const cells = {};
for (const r of recs) {
  const k = `${bitsOf(r.p)}|${r.level}|${r.bound}`;
  const c = (cells[k] ??= { bits: bitsOf(r.p), level: r.level, bound: r.bound, dim: r.dim, ps: 0, pn: 0, us: 0, un: 0, ms: [], enum_cost: r.enum_cost });
  if (r.planted) { c.pn++; if (r.hg_holds && r.vanishing_rows >= 3) c.ps++; }
  else { c.un++; if (r.hg_holds) c.us++; }
  c.ms.push(r.lll_ms);
}
const rows = Object.values(cells).sort((a, b) => a.level - b.level || a.bits - b.bits || a.bound - b.bound);
for (const c of rows) c.ms_median = c.ms.sort((a, b) => a - b)[c.ms.length >> 1];
const reach = {};
for (const c of rows) {
  const k = `${c.bits}|${c.level}`;
  reach[k] ??= { bits: c.bits, level: c.level, reach_B: 0 };
  if (c.ps * 2 >= c.pn && c.pn > 0) reach[k].reach_B = Math.max(reach[k].reach_B, c.bound);
}
for (const r of Object.values(reach)) r.reach_delta = r.reach_B > 1 ? Math.log2(r.reach_B) / r.bits : 0;
const DELTA = { 1: 1 / 210, 2: 1 / 55 };
const fals = rows.filter((c) => c.bits === 40 && c.bound >= 4 && c.ps * 2 >= c.pn && c.pn > 0)
  .map((c) => `success at B=${c.bound}, ℓ=${c.level}`)
  .concat(rows.filter((c) => c.bound >= 2 && c.us > 0).map((c) => `unplanted hg at ${c.bits}b ℓ=${c.level} B=${c.bound}`));
mkdirSync("analysis", { recursive: true });
writeFileSync(`analysis/${OUT}.json`, JSON.stringify({ cells: rows.map(({ ms, ...c }) => c), reach: Object.values(reach), falsifier_hits: fals }, null, 1) + "\n");
let md = "| bits | ℓ | dim | B | planted success | unplanted hg_holds | median LLL ms | enumeration (B+1)² |\n|--:|--:|--:|--:|--:|--:|--:|--:|\n";
for (const c of rows) md += `| ${c.bits} | ${c.level} | ${c.dim} | ${c.bound} | ${c.ps}/${c.pn} | ${c.us}/${c.un} | ${c.ms_median} | ${c.enum_cost} |\n`;
md += "\n| bits | ℓ | reach B | reach δ | §3.2 δ_ℓ | §3.2 B_ℓ |\n|--:|--:|--:|--:|--:|--:|\n";
for (const r of Object.values(reach)) md += `| ${r.bits} | ${r.level} | ${r.reach_B || "<1"} | ${r.reach_delta.toFixed(4)} | ${DELTA[r.level].toFixed(4)} | ${(2 ** (r.bits * DELTA[r.level])).toFixed(2)} |\n`;
md += `\nSkipped planted draws (no non-global root): ${skipped.length ? Object.entries(skipped.reduce((a, r) => ((a[`${bitsOf(r.p)}b ℓ=${r.level} B=${r.bound}`] = (a[`${bitsOf(r.p)}b ℓ=${r.level} B=${r.bound}`] ?? 0) + 1), a), {})).map(([k, v]) => `${k}: ${v}`).join("; ") : "none"}\n`;
md += `\nFalsifier hits: ${fals.length ? fals.join("; ") : "none"}\n`;
writeFileSync(`analysis/${OUT}.md`, md);

// Figure: success fraction vs B, one line per (bits, ℓ), with the §3.2 bound.
const W = 720, H = 360, L = 70, R = 170, T = 40, B = 50;
const bs = [1, 2, 3, 4, 6, 8, 12, 16];
const X = (b) => L + (W - L - R) * Math.log2(b) / 4;
const Y = (f) => T + (H - T - B) * (1 - f);
const col = { "32|1": "#93c5fd", "40|1": "#2563eb", "48|1": "#1e3a8a", "32|2": "#fdba74", "40|2": "#c2410c" };
let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" font-family="Helvetica,Arial,sans-serif" font-size="11">
<rect width="${W}" height="${H}" fill="#fff"/>
<text x="${(W - R + L) / 2}" y="20" text-anchor="middle" font-size="13">${process.env.TITLE ?? "L4-A v2 (global relations excluded): planted-root success vs B"}</text>
<line x1="${L}" y1="${H - B}" x2="${W - R}" y2="${H - B}" stroke="#333"/><line x1="${L}" y1="${T}" x2="${L}" y2="${H - B}" stroke="#333"/>
<line x1="${L}" x2="${W - R}" y1="${Y(0.5)}" y2="${Y(0.5)}" stroke="#999" stroke-dasharray="4 3"/>`;
for (const b of bs) svg += `<text x="${X(b)}" y="${H - B + 16}" text-anchor="middle">${b}</text>`;
for (const f of [0, 0.25, 0.5, 0.75, 1]) svg += `<text x="${L - 6}" y="${Y(f) + 4}" text-anchor="end">${f}</text>`;
svg += `<text x="${(W - R + L) / 2}" y="${H - 12}" text-anchor="middle">root bound B (x_i ∈ [1, B], log scale)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">planted success fraction</text>`;
let ly = T;
for (const [k, c] of Object.entries(col)) {
  const pts = rows.filter((r) => `${r.bits}|${r.level}` === k);
  if (!pts.length) continue;
  svg += `<polyline fill="none" stroke="${c}" stroke-width="1.5" points="${pts.map((r) => `${X(r.bound)},${Y(r.ps / r.pn)}`).join(" ")}"/>`;
  for (const r of pts) svg += `<circle cx="${X(r.bound)}" cy="${Y(r.ps / r.pn)}" r="3" fill="${c}"/>`;
  const [bits, lev] = k.split("|");
  svg += `<line x1="${W - R + 10}" x2="${W - R + 30}" y1="${ly + 6}" y2="${ly + 6}" stroke="${c}" stroke-width="2"/><text x="${W - R + 34}" y="${ly + 10}">${bits} bits, ℓ = ${lev} (dim ${pts[0].dim})</text>`;
  ly += 18;
}
svg += `<text x="${W - R + 10}" y="${ly + 16}" font-size="10">n = ${rows[0]?.pn} planted per cell (ℓ=1);</text>
<text x="${W - R + 10}" y="${ly + 30}" font-size="10">4 per cell at ℓ = 2. dashed: ½</text>
<text x="${W - R + 10}" y="${ly + 44}" font-size="10">§3.2 bound: B ≈ 1.1–1.2 (ℓ=1),</text>
<text x="${W - R + 10}" y="${ly + 58}" font-size="10">1.5–1.7 (ℓ=2)</text></svg>\n`;
writeFileSync(`analysis/${OUT}.svg`, svg);
console.log(md);
