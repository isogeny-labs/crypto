// Plot recorded values from summary.json as SVG (no computation beyond layout): left, ksum5 / strictest incumbent per row and
// weight with the 95% interval and the three replicate ratios; right, distance to matched rho (S over rho-neg S) at w = 0.
import { readFileSync, writeFileSync } from "node:fs";
const d = JSON.parse(readFileSync("summary.json", "utf8"));
const W = ["0", "0.1", "1"], COL = { 0: "#1b6ec2", 0.1: "#d9822b", 1: "#7a3e9d" };
const wd = 760, ht = 380, L = 70, T = 40, B = 70, R = 20, pw = 420, ph = ht - T - B;
const y = (v) => T + ph * (1 - (v - 0.3) / 0.8);        // 0.3 .. 1.1
let s = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${wd} ${ht}" font-family="Helvetica,Arial,sans-serif" font-size="11">`;
s += `<rect width="${wd}" height="${ht}" fill="white"/><text x="${L}" y="20" font-size="13" font-weight="bold">k = 5 sum table / strictest pair-table incumbent (lower is better)</text>`;
for (const v of [0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]) s += `<line x1="${L}" x2="${L + pw}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#222" : "#ddd"}"/><text x="${L - 6}" y="${y(v) + 4}" text-anchor="end">${v.toFixed(1)}</text>`;
const gw = pw / d.rows.length;
d.rows.forEach((r, i) => {
  const gx = L + gw * i + gw / 2;
  s += `<text x="${gx}" y="${ht - B + 18}" text-anchor="middle">${r.slug.split("-")[1]}${r.role === "secondary" ? " (secondary)" : ""}</text><text x="${gx}" y="${ht - B + 32}" text-anchor="middle" fill="#555">row ${r.slug.split("-").slice(2).join("-")}</text>`;
  W.forEach((w, k) => {
    const x = gx + (k - 1) * 40, q = r.weights[w], [lo, hi] = q.ci95;
    s += `<line x1="${x}" x2="${x}" y1="${y(lo)}" y2="${y(hi)}" stroke="${COL[w]}" stroke-width="2"/>`;
    for (const p of q.per_replicate) s += `<circle cx="${x + 7}" cy="${y(p)}" r="2.6" fill="none" stroke="${COL[w]}"/>`;
    s += `<rect x="${x - 4}" y="${y(q.ksum5_over_incumbent) - 4}" width="8" height="8" fill="${COL[w]}"/>`;
  });
});
s += `<text x="${L - 48}" y="${T + ph / 2}" transform="rotate(-90 ${L - 48} ${T + ph / 2})" text-anchor="middle">ksum5 / strictest incumbent (paired total cost)</text>`;
[["0", 0], ["0.1", 1], ["1", 2]].forEach(([w, k]) => { s += `<rect x="${L + k * 75}" y="${ht - 20}" width="9" height="9" fill="${COL[w]}"/><text x="${L + k * 75 + 14}" y="${ht - 12}">w = ${w}</text>`; });
s += `<text x="${L + 235}" y="${ht - 12}" fill="#444">square: pooled; bar: 95%; circle: replicate</text>`;
// right panel: distance to matched rho at w = 0
const rx = L + pw + 50, rw = wd - rx - R, mx = Math.max(...d.rows.map((r) => r.weights["0"].s[r.weights["0"].strictest_incumbent] / r.weights["0"].s["rho-neg"]));
s += `<text x="${rx}" y="${T - 8}" font-weight="bold">S over matched rho, w = 0</text>`;
d.rows.forEach((r, i) => {
  const x = r.weights["0"], yy = T + 10 + i * 78;
  for (const [j, [lab, v, c]] of [["incumbent", x.s[x.strictest_incumbent] / x.s["rho-neg"], "#999"], ["ksum5", x.s.ksum5 / x.s["rho-neg"], "#1b6ec2"], ["rho", 1, "#222"]].entries())
    s += `<rect x="${rx}" y="${yy + j * 18}" width="${(rw - 120) * v / mx}" height="14" fill="${c}"/><text x="${rx + (rw - 120) * v / mx + 4}" y="${yy + j * 18 + 11}">${v.toFixed(1)} ${lab}</text>`;
  s += `<text x="${rx}" y="${yy + 62}" fill="#555">${r.slug.split("-")[1]}</text>`;
});
s += `</svg>\n`;
writeFileSync("l1-ratios.svg", s);
