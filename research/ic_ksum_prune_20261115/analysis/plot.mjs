// Plot recorded values from summary.json as SVG (layout only): ratio cost(ksum-p)/cost(ksum) per row, three weights, k = 5 and k = 4.
import { readFileSync, writeFileSync } from "node:fs";
const d = JSON.parse(readFileSync("summary.json", "utf8"));
const W = ["0", "0.1", "1"], COL = { 0: "#1b6ec2", 0.1: "#d9822b", 1: "#7a3e9d" };
const lo = 0.80, hi = 1.02, wd = 1100, ht = 380, T = 50, B = 70, ph = ht - T - B, y = (v) => T + ph * (1 - (v - lo) / (hi - lo));
let s = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${wd} ${ht}" font-family="Helvetica,Arial,sans-serif" font-size="11"><rect width="${wd}" height="${ht}" fill="white"/>`;
const panel = (k, x0, pw) => {
  s += `<text x="${x0}" y="22" font-size="13" font-weight="bold">${k === "ksum5" ? "k = 5" : "k = 4"}: cost(prune=1) / cost(prune=0), paired</text>`;
  for (let v = lo; v <= hi + 1e-9; v += 0.02) s += `<line x1="${x0}" x2="${x0 + pw}" y1="${y(v)}" y2="${y(v)}" stroke="${Math.abs(v - 1) < 1e-6 ? "#222" : "#ddd"}"/><text x="${x0 - 6}" y="${y(v) + 4}" text-anchor="end">${v.toFixed(2)}</text>`;
  const gw = pw / d.rows.length;
  d.rows.forEach((r, i) => { const gx = x0 + gw * i + gw / 2;
    s += `<text x="${gx}" y="${ht - B + 18}" text-anchor="middle">${r.id}</text>`;
    W.forEach((w, j) => { const x = gx + (j - 1) * 16, q = r.weights[w][k], [a, b] = q.ci95;
      s += `<line x1="${x}" x2="${x}" y1="${y(a)}" y2="${y(b)}" stroke="${COL[w]}" stroke-width="2"/>`;
      for (const p of q.per_replicate) s += `<circle cx="${x + 4}" cy="${y(p)}" r="2" fill="none" stroke="${COL[w]}"/>`;
      s += `<rect x="${x - 3}" y="${y(q.ratio) - 3}" width="6" height="6" fill="${COL[w]}"/>`; }); });
};
panel("ksum5", 60, 480); panel("ksum4", 590, 480);
W.forEach((w, k) => { s += `<rect x="${60 + k * 75}" y="${ht - 20}" width="9" height="9" fill="${COL[w]}"/><text x="${60 + k * 75 + 14}" y="${ht - 12}">w = ${w}</text>`; });
s += `<text x="300" y="${ht - 12}" fill="#444">square: pooled ratio; bar: 95% interval; circle: replicate session; [pcc:unverified]</text></svg>\n`;
writeFileSync("ratios.svg", s);
