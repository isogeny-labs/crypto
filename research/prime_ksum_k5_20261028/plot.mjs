// Derived graphs for README.md from the native outputs (analysis/band-prime.json,
// test/sessions/*/records.jsonl).  Writes s_vs_bits.svg and ratio.svg.
// Thin rendering only: every measured number plotted is read from ecbench's own
// output; the dashed predictions are PROTOCOL.md's registered table.
import { readFileSync, writeFileSync } from "node:fs";

const SIZES = [20, 24, 28, 32, 36, 40, 44, 48];
const band = JSON.parse(readFileSync("analysis/band-prime.json", "utf8"));
const S = (arm, bits) => band.rows.find((r) => r.arm === arm && Math.round(r.log2_r) === bits)?.mean_s;
const PRED5 = { 20: 4.3, 24: 4.9, 28: 5.5, 32: 6.2, 36: 7.1, 40: 8.2, 44: 9.5, 48: 11.0 };
const PRED4 = { 44: 13.9, 48: 16.9 };
const arms = [
  ["ic-ksum5", "k = 5 sum table (this study)", "#7d3c98", ""],
  ["ic-ksum4", "k = 4 sum table (#2808)", "#1e8449", ""],
  ["bsgs-neg", "BSGS (negation)", "#566573", "2 3"],
  ["rho-neg", "rho (negation), reference", "#1f4e79", ""],
];
const runs = {};
for (const b of SIZES) {
  for (const line of readFileSync(`test/sessions/P${b}/records.jsonl`, "utf8").split("\n")) {
    if (!line.trim()) continue;
    const r = JSON.parse(line);
    if (r.warmup) continue;
    (runs[`${b}:${r.arm}`] ??= []).push(r.cost.s);
  }
}
const ci = (xs) => {
  const m = xs.reduce((a, b) => a + b, 0) / xs.length;
  const v = xs.reduce((a, b) => a + (b - m) ** 2, 0) / (xs.length - 1);
  return 1.96 * Math.sqrt(v / xs.length);
};

{
  const W = 840, H = 500, L = 70, R = 290, T = 40, B = 60;
  const x = (bits) => L + ((bits - 20) / 28) * (W - L - R);
  const lo = Math.log10(0.5), hi = Math.log10(40);
  const y = (s) => T + (1 - (Math.log10(s) - lo) / (hi - lo)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">Prime-field ECDLP: mean S per arm, w = 0 (bars: w = 0.1 and w = 1)</text>\n`;
  for (const s of [0.5, 1, 2, 5, 10, 20]) svg += `<line x1="${L}" x2="${W - R}" y1="${y(s)}" y2="${y(s)}" stroke="#ddd"/><text x="${L - 8}" y="${y(s) + 4}" text-anchor="end">${s}</text>\n`;
  for (const b of SIZES) svg += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">log2 r (bits); 16 targets x 2 rounds per arm and size, every run verified</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">S = group-op-equivalents / sqrt(r) (log scale)</text>\n`;
  const pred = (P, color) => `<polyline fill="none" stroke="${color}" stroke-width="1.2" stroke-dasharray="5 4" stroke-opacity="0.8" points="${Object.entries(P).map(([b, s]) => `${x(+b)},${y(s)}`).join(" ")}"/>\n`;
  svg += pred(PRED5, "#7d3c98") + pred(PRED4, "#1e8449");
  arms.forEach(([arm, label, color, dash], i) => {
    const pts = SIZES.map((b) => [b, S(arm, b)]).filter(([, s]) => s);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""} points="${pts.map(([b, s]) => `${x(b)},${y(s[0])}`).join(" ")}"/>\n`;
    for (const [b, s] of pts) {
      const e = ci(runs[`${b}:${arm}`]);
      svg += `<line x1="${x(b)}" x2="${x(b)}" y1="${y(Math.max(s[0] - e, 0.5))}" y2="${y(s[0] + e)}" stroke="${color}" stroke-width="3"/>`;
      svg += `<line x1="${x(b) + 4}" x2="${x(b) + 4}" y1="${y(s[0])}" y2="${y(s[2])}" stroke="${color}" stroke-opacity="0.45"/>`;
      svg += `<circle cx="${x(b)}" cy="${y(s[0])}" r="3.5" fill="${color}"/>`;
      svg += `<circle cx="${x(b) + 4}" cy="${y(s[1])}" r="2" fill="${color}" fill-opacity="0.6"/>\n`;
    }
    const ly = T + 20 + i * 22;
    svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""}/><text x="${W - R + 52}" y="${ly + 4}">${label}</text>\n`;
  });
  svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 108}" y2="${T + 108}" stroke="#555" stroke-dasharray="5 4"/><text x="${W - R + 52}" y="${T + 112}">registered prediction</text>
<text x="${W - R + 15}" y="${T + 150}" font-size="11" fill="#444">thick bar: 95% CI of the w = 0 mean</text>
<text x="${W - R + 15}" y="${T + 166}" font-size="11" fill="#444">thin bar: w = 0 to w = 1, dot at w = 0.1</text>
<text x="${W - R + 15}" y="${T + 190}" font-size="11" fill="#444">sessions: test/sessions/P20 ... P48</text>
<text x="${W - R + 15}" y="${T + 206}" font-size="11" fill="#444">44 and 48 bits: new curves (PROTOCOL.md)</text>
</svg>\n`;
  writeFileSync("s_vs_bits.svg", svg);
}

{
  const W = 840, H = 420, L = 70, R = 290, T = 40, B = 60;
  const x = (bits) => L + ((bits - 20) / 28) * (W - L - R);
  const y0 = 0.5, y1 = 1.2;
  const y = (v) => T + (1 - (v - y0) / (y1 - y0)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">k = 5 against k = 4: ratio of mean S at each band weight</text>\n`;
  for (let v = 0.5; v <= 1.2001; v += 0.1) svg += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="${Math.abs(v - 1) < 1e-9 ? "#999" : "#ddd"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v.toFixed(1)}</text>\n`;
  for (const b of SIZES) svg += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">log2 r (bits)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">S(ic-ksum5) / S(ic-ksum4)</text>\n`;
  const P4 = { 20: 6.74, 24: 6.71, 28: 7.18, 32: 7.93, 36: 9.83, 40: 11.56, ...PRED4 }; // #2808 measured below 44, registered law above
  svg += `<polyline fill="none" stroke="#555" stroke-dasharray="5 4" points="${SIZES.map((b) => `${x(b)},${y(PRED5[b] / P4[b])}`).join(" ")}"/>\n`;
  [[0, "#7d3c98", "w = 0"], [1, "#a569bd", "w = 0.1"], [2, "#d2b4de", "w = 1"]].forEach(([k, color, lab], i) => {
    const pts = SIZES.map((b) => [b, S("ic-ksum5", b)[k] / S("ic-ksum4", b)[k]]);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" points="${pts.map(([b, v]) => `${x(b)},${y(v)}`).join(" ")}"/>`;
    for (const [b, v] of pts) svg += `<circle cx="${x(b)}" cy="${y(v)}" r="3.5" fill="${color}"/>`;
    const ly = T + 20 + i * 22;
    svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2"/><text x="${W - R + 52}" y="${ly + 4}">measured, ${lab}</text>\n`;
  });
  svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 86}" y2="${T + 86}" stroke="#555" stroke-dasharray="5 4"/><text x="${W - R + 52}" y="${T + 90}">registered model, w = 0</text>
<text x="${W - R + 15}" y="${T + 124}" font-size="11" fill="#444">below 1: k = 5 cheaper</text>
<text x="${W - R + 15}" y="${T + 140}" font-size="11" fill="#444">model: k = 5 law over k = 4 measured</text>
<text x="${W - R + 15}" y="${T + 156}" font-size="11" fill="#444">(20-40 bits) or k = 4 law (44, 48)</text>
<text x="${W - R + 15}" y="${T + 180}" font-size="11" fill="#444">source: analysis/band-prime.json</text>
</svg>\n`;
  writeFileSync("ratio.svg", svg);
}
