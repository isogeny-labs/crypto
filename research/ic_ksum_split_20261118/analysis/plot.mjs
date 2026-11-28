// Plot recorded values from analysis output (layout only; no computation beyond reading results.json).
//   node analysis/plot.mjs   (cwd = study directory) -> ratios.svg, gap.svg
import { readFileSync, writeFileSync } from "node:fs";
const d = JSON.parse(readFileSync("results.json", "utf8"));
const W = ["0", "0.1", "1"], R = d.rows;
const esc = (s) => String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;");
// Figure 1: cost ratio against the incumbent ksum5-p, per row, arms -ps / -pr / -psr, three weights, 95 % CI.
{
  const lo = 0.4, hi = 1.05, wd = 1160, ht = 460, T = 60, B = 80, L = 70, ph = ht - T - B, y = (v) => T + ph * (1 - (v - lo) / (hi - lo));
  const arms = [["ksum5-ps", "#1b6ec2", "split"], ["ksum5-pr", "#d9822b", "stride-renew"], ["ksum5-psr", "#2a9d5c", "split + stride-renew"]];
  const gw = (wd - L - 20) / R.length;
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${wd}" height="${ht}" font-family="DejaVu Sans, Arial" font-size="12"><rect width="100%" height="100%" fill="#fff"/>`;
  s += `<text x="${L}" y="22" font-size="15" font-weight="bold">Counted cost against the incumbent ksum5-p (prune=1, walk), paired by target; 95% CI [pcc:unverified]</text>`;
  s += `<text x="${L}" y="42" fill="#444">Per row: three arms, each at w = 0 / 0.1 / 1 GAE per native op (left to right). 432 paired runs per row (3 sessions x 48 targets x 3 rounds). Below 1 = cheaper.</text>`;
  for (let v = 0.4; v <= 1.0001; v += 0.1) s += `<line x1="${L}" x2="${wd - 20}" y1="${y(v)}" y2="${y(v)}" stroke="${Math.abs(v - 1) < 1e-9 ? "#000" : "#ddd"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v.toFixed(1)}</text>`;
  R.forEach((r, i) => {
    const x0 = L + i * gw;
    s += `<text x="${x0 + gw / 2}" y="${ht - B + 22}" text-anchor="middle" font-weight="bold">${r.id}</text>`;
    arms.forEach(([a, col], ai) => W.forEach((w, wi) => {
      const p = r.weights[w][a].ratio, x = x0 + 10 + ai * (gw - 20) / 3 + (wi + 0.5) * (gw - 20) / 9;
      s += `<line x1="${x}" x2="${x}" y1="${y(p.ci95[1])}" y2="${y(p.ci95[0])}" stroke="${col}" stroke-width="2"/><circle cx="${x}" cy="${y(p.ratio)}" r="${3 + wi}" fill="${col}" fill-opacity="${1 - wi * 0.3}"/>`;
    }));
  });
  arms.forEach(([, col, lab], ai) => s += `<circle cx="${L + 10 + ai * 220}" cy="${ht - 22}" r="5" fill="${col}"/><text x="${L + 20 + ai * 220}" y="${ht - 18}">${esc(lab)}</text>`);
  s += `<text x="${L + 700}" y="${ht - 18}" fill="#444">marker size grows with w (0, 0.1, 1)</text></svg>\n`;
  writeFileSync("ratios.svg", s);
}
// Figure 2: distance remaining to matched rho (S(k-sum)/S(rho-neg)), incumbent best vs new best, per row and weight.
{
  const lo = 0, hi = 8, wd = 1160, ht = 420, T = 60, B = 70, L = 70, ph = ht - T - B, y = (v) => T + ph * (1 - (v - lo) / (hi - lo));
  const gw = (wd - L - 20) / R.length;
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${wd}" height="${ht}" font-family="DejaVu Sans, Arial" font-size="12"><rect width="100%" height="100%" fill="#fff"/>`;
  s += `<text x="${L}" y="22" font-size="15" font-weight="bold">Distance remaining: S(best k-sum arm) / S(matched rho-neg), before and after [pcc:unverified]</text>`;
  s += `<text x="${L}" y="42" fill="#444">Grey: cheaper of ksum4-p / ksum5-p (incumbent). Green: cheaper of ksum4-psr / ksum5-psr. Bars per row at w = 0, 0.1, 1. 1 = parity with rho.</text>`;
  for (let v = 0; v <= 8; v += 1) s += `<line x1="${L}" x2="${wd - 20}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#c00" : "#e3e3e3"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>`;
  R.forEach((r, i) => {
    const x0 = L + i * gw;
    s += `<text x="${x0 + gw / 2}" y="${ht - B + 22}" text-anchor="middle" font-weight="bold">${r.id}</text>`;
    W.forEach((w, wi) => {
      const b = r.weights[w].best, bw = (gw - 30) / 6, x = x0 + 15 + wi * 2 * bw;
      s += `<rect x="${x}" y="${y(b.gap_incumbent)}" width="${bw - 1}" height="${y(0) - y(b.gap_incumbent)}" fill="#9a9a9a"/>`;
      s += `<rect x="${x + bw}" y="${y(b.gap_candidate)}" width="${bw - 1}" height="${y(0) - y(b.gap_candidate)}" fill="#2a9d5c"/>`;
      s += `<text x="${x + bw}" y="${y(Math.max(b.gap_incumbent, b.gap_candidate)) - 4}" text-anchor="middle" font-size="9">${w}</text>`;
    });
  });
  s += `</svg>\n`;
  writeFileSync("gap.svg", s);
}
console.log("ratios.svg gap.svg");
