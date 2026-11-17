// Plot recorded values from summary.json as SVG (layout only). Left: candidate / strictest incumbent per row and weight (square pooled,
// bar 95%, circle replicate), ksum5 in solid colours and ksum4 as the second panel; right: S over matched rho at w = 0.
import { readFileSync, writeFileSync } from "node:fs";
const d = JSON.parse(readFileSync("summary.json", "utf8"));
const W = ["0", "0.1", "1"], COL = { 0: "#1b6ec2", 0.1: "#d9822b", 1: "#7a3e9d" };
const all = d.rows.flatMap((r) => W.flatMap((w) => ["ksum5", "ksum4"].flatMap((k) => [...r.weights[w][k].ci95, ...r.weights[w][k].per_replicate])));
const lo = Math.floor(Math.min(...all, 0.9) * 10) / 10 - 0.1, hi = Math.ceil(Math.max(...all, 1.1) * 10) / 10 + 0.05;
const wd = 1180, ht = 400, T = 50, B = 70, ph = ht - T - B, y = (v) => T + ph * (1 - (v - lo) / (hi - lo));
let s = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${wd} ${ht}" font-family="Helvetica,Arial,sans-serif" font-size="11"><rect width="${wd}" height="${ht}" fill="white"/>`;
const panel = (k, x0, pw) => {
  s += `<text x="${x0}" y="22" font-size="13" font-weight="bold">${k === "ksum5" ? "k = 5" : "k = 4"} sum table / strictest pair-table incumbent</text>`;
  for (let v = Math.ceil(lo * 10) / 10; v <= hi; v += 0.1) s += `<line x1="${x0}" x2="${x0 + pw}" y1="${y(v)}" y2="${y(v)}" stroke="${Math.abs(v - 1) < 1e-6 ? "#222" : "#ddd"}"/><text x="${x0 - 6}" y="${y(v) + 4}" text-anchor="end">${v.toFixed(1)}</text>`;
  const gw = pw / d.rows.length;
  d.rows.forEach((r, i) => {
    const gx = x0 + gw * i + gw / 2;
    s += `<text x="${gx}" y="${ht - B + 18}" text-anchor="middle">n${r.bits}${r.role === "secondary" ? " (sec.)" : ""}</text>`;
    W.forEach((w, j) => { const x = gx + (j - 1) * 22, q = r.weights[w][k], [a, b] = q.ci95;
      s += `<line x1="${x}" x2="${x}" y1="${y(a)}" y2="${y(b)}" stroke="${COL[w]}" stroke-width="2"/>`;
      for (const p of q.per_replicate) s += `<circle cx="${x + 5}" cy="${y(p)}" r="2.2" fill="none" stroke="${COL[w]}"/>`;
      s += `<rect x="${x - 3}" y="${y(q.ratio) - 3}" width="6" height="6" fill="${COL[w]}"/>`; });
  });
};
panel("ksum5", 60, 360); panel("ksum4", 470, 360);
[["0", 0], ["0.1", 1], ["1", 2]].forEach(([w, k]) => { s += `<rect x="${60 + k * 75}" y="${ht - 20}" width="9" height="9" fill="${COL[w]}"/><text x="${60 + k * 75 + 14}" y="${ht - 12}">w = ${w}</text>`; });
s += `<text x="300" y="${ht - 12}" fill="#444">square: pooled; bar: 95%; circle: replicate</text>`;
const rx = 880, rw = wd - rx - 20, mx = Math.max(...d.rows.map((r) => r.weights["0"].s[r.weights["0"].strictest_incumbent] / r.weights["0"].s["rho-neg"]));
s += `<text x="${rx}" y="22" font-size="13" font-weight="bold">S over matched rho, w = 0</text>`;
d.rows.forEach((r, i) => { const x = r.weights["0"], yy = T - 20 + i * 66;
  for (const [j, [lab, v, c]] of [["incumbent", x.s[x.strictest_incumbent] / x.s["rho-neg"], "#999"], ["ksum5", x.s.ksum5 / x.s["rho-neg"], "#1b6ec2"], ["ksum4", x.s.ksum4 / x.s["rho-neg"], "#4aa3a3"]].entries())
    s += `<rect x="${rx}" y="${yy + j * 14}" width="${(rw - 90) * v / mx}" height="11" fill="${c}"/><text x="${rx + (rw - 90) * v / mx + 4}" y="${yy + j * 14 + 9}">${v.toFixed(1)} ${lab}</text>`;
  s += `<text x="${rx - 38}" y="${yy + 22}" fill="#555">n${r.bits}</text>`; });
s += `</svg>\n`; writeFileSync("l1-ratios.svg", s);
