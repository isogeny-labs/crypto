// Derived graphs for README.md from the native outputs (analysis/compare-*.json,
// analysis/band.json).  Writes ratio.svg and rho_multiple.svg.
// Thin rendering only: every measured number plotted is read from ecbench's own
// output; the dashed predictions are PROTOCOL.md's registered table.
import { readFileSync, writeFileSync } from "node:fs";

const SIZES = [20, 24, 28, 32];
const W3 = ["0", "0.1", "1"];
const cmp = (b, a, c, w) => JSON.parse(readFileSync(`analysis/compare-P${b}-${a}__${c}-w${w}.json`, "utf8")).ops;
const band = JSON.parse(readFileSync("analysis/band.json", "utf8"));
const S = (arm, bits) => band.rows.find((r) => r.arm === arm && Math.round(r.log2_r) === bits).mean_s;
const PRED = { 20: 0.7, 24: 0.45, 28: 0.33, 32: 0.22 };

{
  const W = 840, H = 460, L = 70, R = 290, T = 40, B = 60;
  const x = (bits) => L + ((bits - 18) / 16) * (W - L - R);
  const lo = Math.log10(0.15), hi = Math.log10(1.25);
  const y = (v) => T + (1 - (Math.log10(v) - lo) / (hi - lo)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">k-sum table against the mitm incumbent, same session and targets</text>\n`;
  for (const v of [0.2, 0.3, 0.5, 0.7, 1]) svg += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#999" : "#ddd"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const b of SIZES) svg += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">bits (incumbent holdout curves); 48 targets x 2 rounds = 96 pairs per cell</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">paired ratio S(candidate) / S(inc) (log scale)</text>\n`;
  svg += `<polyline fill="none" stroke="#555" stroke-dasharray="5 4" points="${SIZES.map((b) => `${x(b)},${y(PRED[b])}`).join(" ")}"/>\n`;
  const arms = [["ksum5", "#7d3c98", -9, "k = 5 sum table"], ["ksum4", "#1e8449", 9, "k = 4 sum table"]];
  arms.forEach(([arm, color, dx, label], i) => {
    const pts = SIZES.map((b) => [b, W3.map((w) => cmp(b, "inc", arm, w))]);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" points="${pts.map(([b, o]) => `${x(b) + dx},${y(o[0].ratio_b_over_a)}`).join(" ")}"/>\n`;
    for (const [b, o] of pts) {
      W3.forEach((_, k) => {
        const c = o[k], xx = x(b) + dx + (k - 1) * 4, op = [1, 0.7, 0.4][k];
        svg += `<line x1="${xx}" x2="${xx}" y1="${y(c.ci95_bootstrap[0])}" y2="${y(c.ci95_bootstrap[1])}" stroke="${color}" stroke-opacity="${op}" stroke-width="${k ? 1.5 : 3}"/>`;
        svg += `<circle cx="${xx}" cy="${y(c.ratio_b_over_a)}" r="${k ? 2.2 : 3.5}" fill="${color}" fill-opacity="${op}"/>`;
      });
      svg += "\n";
    }
    const ly = T + 20 + i * 22;
    svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2"/><text x="${W - R + 52}" y="${ly + 4}">${label} / inc</text>\n`;
  });
  svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 64}" y2="${T + 64}" stroke="#555" stroke-dasharray="5 4"/><text x="${W - R + 52}" y="${T + 68}">registered prediction (k = 5)</text>
<text x="${W - R + 15}" y="${T + 104}" font-size="11" fill="#444">bars: cluster-bootstrap 95% CI,</text>
<text x="${W - R + 15}" y="${T + 120}" font-size="11" fill="#444">left to right w = 0, 0.1, 1 (fading)</text>
<text x="${W - R + 15}" y="${T + 144}" font-size="11" fill="#444">below 1: cheaper than the incumbent</text>
<text x="${W - R + 15}" y="${T + 168}" font-size="11" fill="#444">inc: mitm negation-folded, N 45 / 162 /</text>
<text x="${W - R + 15}" y="${T + 184}" font-size="11" fill="#444">407 / 1024 (prime_base_oracle_tuning)</text>
<text x="${W - R + 15}" y="${T + 208}" font-size="11" fill="#444">source: analysis/compare-P*-inc__*.json</text>
</svg>\n`;
  writeFileSync("ratio.svg", svg);
}

{
  const W = 840, H = 460, L = 70, R = 290, T = 40, B = 60;
  const x = (bits) => L + ((bits - 18) / 16) * (W - L - R);
  const lo = Math.log10(0.8), hi = Math.log10(40);
  const y = (v) => T + (1 - (Math.log10(v) - lo) / (hi - lo)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">Distance remaining: mean S over rho-neg, same session, w = 0</text>\n`;
  for (const v of [1, 2, 5, 10, 20]) svg += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#999" : "#ddd"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const b of SIZES) svg += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">bits (incumbent holdout curves)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">S(arm) / S(rho-neg) (log scale)</text>\n`;
  const arms = [["inc", "#c0392b", "", "mitm incumbent (inc)"], ["ksum4", "#1e8449", "", "k = 4 sum table"], ["ksum5", "#7d3c98", "", "k = 5 sum table"], ["bsgs-neg", "#566573", "2 3", "BSGS (negation)"]];
  arms.forEach(([arm, color, dash, label], i) => {
    const pts = SIZES.map((b) => [b, S(arm, b)[0] / S("rho-neg", b)[0]]);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""} points="${pts.map(([b, v]) => `${x(b)},${y(v)}`).join(" ")}"/>`;
    for (const [b, v] of pts) svg += `<circle cx="${x(b)}" cy="${y(v)}" r="3.5" fill="${color}"/>`;
    const ly = T + 20 + i * 22;
    svg += `\n<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""}/><text x="${W - R + 52}" y="${ly + 4}">${label}</text>\n`;
  });
  svg += `<text x="${W - R + 15}" y="${T + 124}" font-size="11" fill="#444">1 = rho with negation map, same</text>
<text x="${W - R + 15}" y="${T + 140}" font-size="11" fill="#444">session, curves and targets</text>
<text x="${W - R + 15}" y="${T + 164}" font-size="11" fill="#444">generic track: limit is BSGS</text>
<text x="${W - R + 15}" y="${T + 188}" font-size="11" fill="#444">source: analysis/band.json</text>
</svg>\n`;
  writeFileSync("rho_multiple.svg", svg);
}
