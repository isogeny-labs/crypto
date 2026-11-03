// PROTOCOL.md "Readings": per (shape, m), each frontier point of
// data/m*.jsonl against the Jochemsz–May (dim_ℓ, δ_ℓ) curve, interpolated
// linearly in log dimension, and against the asymptote.  Writes
// analysis/p3.json, analysis/p3.md and analysis/p3.svg.  Run from here.
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";

const rows = ["data/m3.jsonl", "data/m4.jsonl"].filter(existsSync).flatMap((f) =>
  readFileSync(f, "utf8").trim().split("\n")
    .map((l) => JSON.parse(l.replace(/\((\d+), ([0-9.e-]+)\)/g, "[$1,$2]"))));
const groups = {};
for (const r of rows) (groups[`${r.shape}|${r.m}`] ??= []).push(r);
const out = [];
for (const [key, rs] of Object.entries(groups)) {
  const [shape, m] = key.split("|");
  const jm = rs.map((r) => [r.jm_dim, r.jm_set_delta]).sort((a, b) => a[0] - b[0]);
  const interp = (d) => {
    if (d <= jm[0][0]) return d === jm[0][0] ? jm[0][1] : null;
    for (let i = 1; i < jm.length; i++)
      if (d <= jm[i][0]) {
        const t = Math.log(d / jm[i - 1][0]) / Math.log(jm[i][0] / jm[i - 1][0]);
        return jm[i - 1][1] + t * (jm[i][1] - jm[i - 1][1]);
      }
    return null; // beyond the last JM level: not compared
  };
  const pts = rs.flatMap((r) => r.frontier.map(([d, v]) => ({ level: r.level, dim: d, delta: v })));
  // Global frontier across levels.
  pts.sort((a, b) => a.dim - b.dim || b.delta - a.delta);
  const front = [];
  for (const p of pts) if (!front.length || p.delta > front[front.length - 1].delta + 1e-12) front.push(p);
  let maxGain = null;
  for (const p of front) {
    const j = interp(p.dim);
    p.jm_at_dim = j;
    p.ratio = j ? p.delta / j : null;
    if (p.ratio && (!maxGain || p.ratio > maxGain.ratio)) maxGain = p;
  }
  const asym = rs[0].asymptote;
  out.push({ shape, m: +m, asymptote: asym, jm, frontier: front,
    max_frontier_delta: Math.max(...front.map((p) => p.delta)),
    exceeds_asymptote: front.some((p) => p.delta > asym),
    max_ratio_vs_jm_at_equal_dim: maxGain });
}
mkdirSync("analysis", { recursive: true });
writeFileSync("analysis/p3.json", JSON.stringify(out, null, 1) + "\n");
let md = "| shape | m | JM curve (dim, δ) | best frontier δ (dim) | max δ / δ_JM at equal dim (dim) | asymptote | exceeded |\n|:--|--:|:--|:--|:--|--:|:--|\n";
for (const g of out) {
  const best = g.frontier.reduce((a, b) => (b.delta > a.delta ? b : a));
  const mg = g.max_ratio_vs_jm_at_equal_dim;
  md += `| ${g.shape} | ${g.m} | ${g.jm.map(([d, v]) => `(${d}, ${v.toFixed(4)})`).join(" ")} | ${best.delta.toFixed(4)} (${best.dim}) | ${mg ? `${mg.ratio.toFixed(3)} (${mg.dim})` : "—"} | ${g.asymptote.toFixed(4)} | ${g.exceeds_asymptote ? "**yes**" : "no"} |\n`;
}
writeFileSync("analysis/p3.md", md);

// Figure: δ against log10(dim), JM points and frontier, per (shape, m).
const W = 760, H = 380, L = 70, R = 170, T = 40, B = 50;
const all = out.flatMap((g) => g.frontier.map((p) => p.dim).concat(g.jm.map((j) => j[0])));
const lx0 = Math.log10(Math.min(...all)) - 0.05, lx1 = Math.log10(Math.max(...all)) + 0.05;
const ymax = 0.045;
const X = (d) => L + (W - L - R) * (Math.log10(d) - lx0) / (lx1 - lx0);
const Y = (v) => T + (H - T - B) * (1 - v / ymax);
const col = { "simplex|3": "#c2410c", "box|3": "#1d4ed8", "simplex|4": "#a16207", "box|4": "#047857" };
let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" font-family="Helvetica,Arial,sans-serif" font-size="11">
<rect width="${W}" height="${H}" fill="#fff"/>
<text x="${(W - R + L) / 2}" y="20" text-anchor="middle" font-size="13">P3 fallback: triangular shift lattices, determinant reach δ vs dimension</text>
<line x1="${L}" y1="${H - B}" x2="${W - R}" y2="${H - B}" stroke="#333"/><line x1="${L}" y1="${T}" x2="${L}" y2="${H - B}" stroke="#333"/>`;
for (let e = Math.ceil(lx0); e <= lx1; e++) svg += `<text x="${X(10 ** e)}" y="${H - B + 16}" text-anchor="middle">10^${e}</text>`;
for (const v of [0, 0.01, 0.02, 0.03, 0.04]) svg += `<text x="${L - 6}" y="${Y(v) + 4}" text-anchor="end">${v}</text><line x1="${L}" x2="${W - R}" y1="${Y(v)}" y2="${Y(v)}" stroke="#eee"/>`;
svg += `<text x="${(W - R + L) / 2}" y="${H - 12}" text-anchor="middle">lattice dimension |M| (log scale)</text>`;
svg += `<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">δ = Σk / Σ⟨w,β⟩ (determinant bound)</text>`;
let ly = T;
for (const g of out) {
  const c = col[`${g.shape}|${g.m}`] ?? "#555";
  svg += `<line x1="${L}" x2="${W - R}" y1="${Y(g.asymptote)}" y2="${Y(g.asymptote)}" stroke="${c}" stroke-dasharray="2 4"/>`;
  svg += `<polyline fill="none" stroke="${c}" stroke-width="1.2" points="${g.frontier.map((p) => `${X(p.dim)},${Y(p.delta)}`).join(" ")}"/>`;
  for (const [d, v] of g.jm) svg += `<rect x="${X(d) - 3.5}" y="${Y(v) - 3.5}" width="7" height="7" fill="none" stroke="${c}" stroke-width="1.5"/>`;
  svg += `<line x1="${W - R + 10}" x2="${W - R + 30}" y1="${ly + 6}" y2="${ly + 6}" stroke="${c}"/><text x="${W - R + 34}" y="${ly + 10}">${g.shape}, m = ${g.m}</text>`;
  ly += 18;
}
svg += `<rect x="${W - R + 16}" y="${ly + 2}" width="7" height="7" fill="none" stroke="#333"/><text x="${W - R + 34}" y="${ly + 10}">JM level ℓ</text>
<line x1="${W - R + 10}" x2="${W - R + 30}" y1="${ly + 24}" y2="${ly + 24}" stroke="#333" stroke-dasharray="2 4"/><text x="${W - R + 34}" y="${ly + 28}">asymptote</text>
<text x="${W - R + 10}" y="${ly + 50}" font-size="10">line: search frontier</text>
<text x="${W - R + 10}" y="${ly + 64}" font-size="10">(our family, not the</text>
<text x="${W - R + 10}" y="${ly + 78}" font-size="10">ePrint 2026/1027 LP)</text></svg>\n`;
writeFileSync("analysis/p3.svg", svg);
console.log(md);
