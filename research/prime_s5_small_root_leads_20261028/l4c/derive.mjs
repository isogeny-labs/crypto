// PROTOCOL.md "Registered readings": per (bits, shape, δ), the small-x value
// of each statistic against the five null-1 values, band = [min, max] widened
// by 3σ (σ from the five; where all five are equal, value ± 3√value).
// Writes analysis/l4c.json, analysis/l4c.md and analysis/l4c.svg.  Thin
// derivation over data/census.jsonl; run from this directory.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const rows = readFileSync("data/census.jsonl", "utf8").trim().split("\n")
  .map((l) => JSON.parse(l.replace(/NaN/g, "null")));
const STATS = ["triple_unshared", "pair_collisions", "ge2"];
const cells = {};
for (const r of rows) (cells[`${r.bits}|${r.shape}|${r.delta}`] ??= []).push(r);
const out = [];
for (const [key, rs] of Object.entries(cells)) {
  const [bits, shape, delta] = key.split("|");
  const s = rs.find((r) => r.base === "small_x");
  const nulls = rs.filter((r) => r.base !== "small_x");
  const row = { bits: +bits, shape, delta: +delta, base_size: s.base_size, lambda: s.lambda, stats: {} };
  for (const st of STATS) {
    const v = nulls.map((r) => r[st]);
    const mean = v.reduce((a, b) => a + b, 0) / v.length;
    const sd = Math.sqrt(v.reduce((a, b) => a + (b - mean) ** 2, 0) / (v.length - 1));
    const lo = Math.min(...v), hi = Math.max(...v);
    const w = sd > 0 ? 3 * sd : 3 * Math.sqrt(hi);
    const band = [lo - w, hi + w];
    const x = s[st];
    row.stats[st] = { small_x: x, nulls: v, null_mean: mean, band,
      side: x < band[0] ? "below" : x > band[1] ? "above" : "inside" };
  }
  out.push(row);
}
// Falsifier: same δ and shape, outside on the same side at ≥ 2 sizes.
const flags = [];
for (const shape of ["a-3", "a0"]) for (const delta of [...new Set(out.map((r) => r.delta))])
  for (const st of STATS) for (const side of ["below", "above"]) {
    const n = out.filter((r) => r.shape === shape && r.delta === delta && r.stats[st].side === side).length;
    if (n >= 2) flags.push({ shape, delta, stat: st, side, sizes: n });
  }
mkdirSync("analysis", { recursive: true });
writeFileSync("analysis/l4c.json", JSON.stringify({ cells: out, falsifier_hits: flags }, null, 1) + "\n");
let md = "| bits | shape | δ | |F| | λ | triple_unshared: small-x [nulls] | side | pair_collisions | side | ge2 | side |\n|--:|:--|--:|--:|--:|:--|:--|:--|:--|:--|:--|\n";
for (const r of out.sort((a, b) => a.shape.localeCompare(b.shape) || a.bits - b.bits || a.delta - b.delta)) {
  const c = (st) => `${r.stats[st].small_x} [${r.stats[st].nulls.join(", ")}] | ${r.stats[st].side}`;
  md += `| ${r.bits} | ${r.shape} | ${r.delta} | ${r.base_size} | ${r.lambda.toExponential(1)} | ${c("triple_unshared")} | ${c("pair_collisions")} | ${c("ge2")} |\n`;
}
md += `\nFalsifier hits (outside on the same side at ≥ 2 sizes): ${flags.length ? JSON.stringify(flags) : "none"}\n`;
writeFileSync("analysis/l4c.md", md);

// Figure: triple_unshared / null mean, δ = 1/4 and 0.3, with the band.
const pts = out.filter((r) => r.delta >= 0.25 && r.stats.triple_unshared.null_mean > 0);
const W = 720, H = 360, L = 70, R = 20, T = 40, B = 70;
const xs = pts.map((_, i) => L + (i + 0.5) * (W - L - R) / pts.length);
const y = (v) => T + (H - T - B) * (1 - Math.min(v, 2.5) / 2.5);
let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" font-family="Helvetica,Arial,sans-serif" font-size="11">
<rect width="${W}" height="${H}" fill="#fff"/>
<text x="${W / 2}" y="20" text-anchor="middle" font-size="13">L4-C census: 6-term relations (triple_unshared), small-x factor base vs null 1</text>
<line x1="${L}" y1="${y(1)}" x2="${W - R}" y2="${y(1)}" stroke="#999" stroke-dasharray="4 3"/>
<line x1="${L}" y1="${T}" x2="${L}" y2="${H - B}" stroke="#333"/>`;
for (const t of [0, 0.5, 1, 1.5, 2, 2.5]) svg += `<text x="${L - 6}" y="${y(t) + 4}" text-anchor="end">${t}</text>`;
svg += `<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">count / null-1 mean (exact census, all r targets)</text>`;
pts.forEach((r, i) => {
  const st = r.stats.triple_unshared, m = st.null_mean;
  const [lo, hi] = st.band.map((v) => Math.max(0, v / m));
  svg += `<rect x="${xs[i] - 6}" y="${y(hi)}" width="12" height="${Math.max(1, y(lo) - y(hi))}" fill="#c9d7ea"/>`;
  for (const v of st.nulls) svg += `<circle cx="${xs[i] + 9}" cy="${y(v / m)}" r="2" fill="#6b7f99"/>`;
  svg += `<circle cx="${xs[i]}" cy="${y(st.small_x / m)}" r="4.5" fill="${st.side === "inside" ? "#c2410c" : "#000"}"/>`;
  svg += `<text transform="translate(${xs[i] + 3} ${H - B + 10}) rotate(55)" font-size="10">${r.bits}b ${r.shape} δ=${r.delta}</text>`;
});
svg += `<g transform="translate(${W - 250} ${T + 4})"><rect width="12" height="10" fill="#c9d7ea"/><text x="16" y="9">registered band (null range ± 3σ)</text>
<circle cx="6" cy="22" r="4.5" fill="#c2410c"/><text x="16" y="26">small-x factor base (inside band)</text>
<circle cx="6" cy="38" r="2" fill="#6b7f99"/><text x="16" y="42">five null-1 factor bases</text></g></svg>\n`;
writeFileSync("analysis/l4c.svg", svg);
console.log(md);
