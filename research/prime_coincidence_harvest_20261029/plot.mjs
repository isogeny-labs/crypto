// Derived graphs for README.md from the native outputs (analysis/analysis.json,
// analysis/stage1.json). Writes kappa_ladder.svg and slopes.svg.
// Thin rendering only: every number plotted is read from coincidence_ladder's
// own output; the dashed break-even envelopes are the registered 2^((bits-16) delta*).
import { readFileSync, writeFileSync } from "node:fs";

const A = JSON.parse(readFileSync(process.argv[2] ?? "analysis/analysis.json", "utf8"));
const STAGE1 = (() => {
  try {
    return JSON.parse(readFileSync(process.argv[3] ?? "analysis/stage1.json", "utf8"));
  } catch {
    return [];
  }
})();
const DELTA = A.delta_star; // 1/3, 1/5, 1/7
const REAL = { window: "#1f4e79", gap2: "#1e8449", gap3: "#b9770e" };
const CTRL = { plant_g1_6: "#7d3c98", plant_g1_5: "#a569bd", zn_interval: "#c0392b" };
const NAMES = {
  window: "window (d = 1)",
  gap2: "label GAP (d = 2)",
  gap3: "label GAP (d = 3)",
  plant_g1_6: "plant γ = 1/6 (control)",
  plant_g1_5: "plant γ = 1/5 (control)",
  zn_interval: "Z/N interval (control)",
};
const get = (theta, geom) => A.series.find((s) => s.theta === theta && s.geometry === geom);
const bits = A.rungs;
const f = (x, d = 2) => x.toFixed(d);

{
  const W = 1000, H = 480, L = 60, R = 255, T = 62, B = 56;
  const panelW = (W - L - R) / 2 - 24;
  const x0 = (p, b) => L + p * (panelW + 48) + ((b - bits[0]) / (bits[bits.length - 1] - bits[0])) * panelW;
  const top = Math.max(...A.series.filter((s) => s.cells && (s.geometry in CTRL) && s.geometry !== "zn_interval").flatMap((s) => s.cells.map((c) => c.ci95[1])), 50);
  const lo = Math.log10(0.8), hi = Math.log10(top * 1.5);
  const y = (k) => T + (1 - (Math.log10(k) - lo) / (hi - lo)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="24" font-size="15" font-weight="bold">Pair-sum coincidence excess κ against log2 N (log scale)</text>\n`;
  for (const v of [1, 3, 10, 30, 100, 300, 1000, 3000, 10000, 30000].filter((v) => v < top * 1.5)) {
    svg += `<line x1="${L}" x2="${W - R - 20}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#999" : "#e5e5e5"}"/><text x="${L - 6}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  }
  [0.3, 0.35].forEach((theta, p) => {
    svg += `<text x="${x0(p, bits[0])}" y="${T - 12}" font-weight="bold">θ = ${theta}</text>\n`;
    for (const b of bits) svg += `<text x="${x0(p, b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
    DELTA.forEach((d, i) => {
      const pts = bits.map((b) => `${x0(p, b)},${y(2 ** ((b - bits[0]) * d))}`).join(" ");
      svg += `<polyline fill="none" stroke="#888" stroke-dasharray="${["3 3", "6 3", "9 3"][i]}" points="${pts}"/>\n`;
    });
    for (const [g, color] of [...Object.entries(CTRL).filter(([k]) => k !== "zn_interval"), ...Object.entries(REAL)]) {
      const s = get(theta, g);
      if (!s || !s.cells) continue;
      const pts = s.cells.map((c) => `${x0(p, c.bits)},${y(c.kappa)}`).join(" ");
      svg += `<polyline fill="none" stroke="${color}" stroke-width="${g in REAL ? 2 : 1.5}" ${g in REAL ? "" : 'stroke-opacity="0.75"'} points="${pts}"/>\n`;
      for (const c of s.cells) {
        svg += `<line x1="${x0(p, c.bits)}" x2="${x0(p, c.bits)}" y1="${y(Math.max(c.ci95[0], 0.8))}" y2="${y(c.ci95[1])}" stroke="${color}" stroke-width="2.5"/><circle cx="${x0(p, c.bits)}" cy="${y(c.kappa)}" r="3" fill="${color}"/>\n`;
      }
    }
  });
  svg += `<text x="${(L + W - R - 20) / 2}" y="${H - 14}" text-anchor="middle">log2 N (rung); one curve family, ${A.series[0].cells.map((c) => c.curves).join(" / ")} curves per rung at θ = 0.30; Z/N control in slopes.svg</text>
<text transform="translate(16 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">κ = S / M (95% curve-cluster bootstrap)</text>\n`;
  let ly = T + 6;
  for (const [g, color] of [...Object.entries(REAL), ...Object.entries(CTRL).filter(([k]) => k !== "zn_interval")]) {
    svg += `<line x1="${W - R + 10}" x2="${W - R + 38}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2"/><text x="${W - R + 44}" y="${ly + 4}">${NAMES[g]}</text>\n`;
    ly += 20;
  }
  ly += 8;
  DELTA.forEach((d, i) => {
    svg += `<line x1="${W - R + 10}" x2="${W - R + 38}" y1="${ly}" y2="${ly}" stroke="#888" stroke-dasharray="${["3 3", "6 3", "9 3"][i]}"/><text x="${W - R + 44}" y="${ly + 4}">break-even δ* = ${f(d, 3)}</text>\n`;
    ly += 18;
  });
  svg += `<text x="${W - R + 10}" y="${ly + 18}" font-size="11" fill="#444">dashed: κ needed for a growing</text><text x="${W - R + 10}" y="${ly + 32}" font-size="11" fill="#444">harvest, 2^((bits − 16)·δ*)</text>
<text x="${W - R + 10}" y="${ly + 52}" font-size="11" fill="#444">source: analysis/analysis.json</text>
</svg>\n`;
  writeFileSync("kappa_ladder.svg", svg);
}

{
  const rows = [];
  for (const theta of [0.3, 0.35]) for (const g of ["window", "gap2", "gap3", "plant_g1_6", "plant_g1_5", "zn_interval"]) {
    const s = get(theta, g);
    if (s && s.slope_ci95) rows.push({ theta, g, s });
  }
  const W = 960, rowH = 22, T = 52, L = 250, R = 40, H = T + rows.length * rowH + 70;
  const xmin = -0.1, xmax = 0.8;
  const x = (v) => L + ((v - xmin) / (xmax - xmin)) * (W - L - R);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="20" y="24" font-size="15" font-weight="bold">Slope d ln κ / d ln N with 95% intervals (16 to 32 bits)</text>\n`;
  for (let v = -0.1; v <= 0.8001; v += 0.1) svg += `<line x1="${x(v)}" x2="${x(v)}" y1="${T - 6}" y2="${H - 56}" stroke="${Math.abs(v) < 1e-9 ? "#999" : "#eee"}"/><text x="${x(v)}" y="${H - 40}" text-anchor="middle">${f(v, 1)}</text>\n`;
  DELTA.forEach((d, i) => {
    svg += `<line x1="${x(d)}" x2="${x(d)}" y1="${T - 6}" y2="${H - 56}" stroke="#888" stroke-dasharray="${["3 3", "6 3", "9 3"][i]}"/><text x="${x(d)}" y="${T - 12}" text-anchor="middle" font-size="11">δ*=${f(d, 2)}</text>\n`;
  });
  rows.forEach((r, i) => {
    const yy = T + i * rowH + 10;
    const color = REAL[r.g] ?? CTRL[r.g];
    const [lo, hi] = r.s.slope_ci95;
    svg += `<text x="${L - 10}" y="${yy + 4}" text-anchor="end">θ ${r.theta} ${NAMES[r.g]}</text>`;
    svg += `<line x1="${x(Math.max(lo, xmin))}" x2="${x(Math.min(hi, xmax))}" y1="${yy}" y2="${yy}" stroke="${color}" stroke-width="3"/><circle cx="${x(r.s.slope)}" cy="${yy}" r="4" fill="${color}"/>`;
    if (r.s.slope_formal_pred != null) svg += `<path d="M ${x(r.s.slope_formal_pred)} ${yy - 8} l 4 -6 l -8 0 z" fill="none" stroke="#333"/>`;
    svg += "\n";
  });
  for (const s of STAGE1) {
    // Stage 1 rows are listed under the plot as text; no marker on the shared axis.
  }
  svg += `<text x="${L}" y="${H - 14}" font-size="11" fill="#444">triangle: slope predicted from the integer count (plants); Z/N expected slope is 1 − θ. Source: analysis/analysis.json</text>
<text x="${L}" y="${H - 26}" text-anchor="start" font-size="11" fill="#444">slope of ln κ on ln N</text>
</svg>\n`;
  writeFileSync("slopes.svg", svg);
}
console.log("wrote kappa_ladder.svg slopes.svg");
