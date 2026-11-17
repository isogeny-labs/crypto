// Derived graphs for README.md from the native outputs (analysis/compare-*.json,
// analysis/band.json).  Writes ratio.svg, per_curve.svg and rho_multiple.svg.
// Thin rendering only: every measured number plotted is read from ecbench's own output.
import { readFileSync, writeFileSync } from "node:fs";

const SIZES = [20, 24, 28, 32];
const cmp = (b, a, c, w) => JSON.parse(readFileSync(`analysis/compare-P${b}-${a}__${c}-w${w}.json`, "utf8")).ops;
const band = JSON.parse(readFileSync("analysis/band.json", "utf8"));
const PRED = { 20: [0.65, 0.95], 24: [0.4, 0.55], 28: [0.3, 0.45], 32: [0.2, 0.32] };
const head = (W, H, title) => `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">\n<rect width="${W}" height="${H}" fill="white"/>\n<text x="70" y="22" font-size="15" font-weight="bold">${title}</text>\n`;
const W = 840, H = 460, L = 70, R = 290, T = 40, B = 60;
const x = (bits) => L + ((bits - 18) / 16) * (W - L - R);
const ylog = (lo, hi) => (v) => T + (1 - (Math.log10(v) - Math.log10(lo)) / (Math.log10(hi) - Math.log10(lo))) * (H - T - B);
const axes = (y, ticks, xlabel, ylabel) => {
  let s = "";
  for (const v of ticks) s += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#999" : "#ddd"}"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const b of SIZES) s += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
  return s + `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">${xlabel}</text>\n<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">${ylabel}</text>\n`;
};
const note = (lines, y0) => lines.map((t, i) => `<text x="${W - R + 15}" y="${y0 + i * 16}" font-size="11" fill="#444">${t}</text>`).join("\n") + "\n";

{ // ratio.svg: pooled paired ratio to the tuned incumbent
  const y = ylog(0.15, 1.25);
  let svg = head(W, H, "k-sum tables against the tuned mitm incumbent, 4 fresh curves per size (pooled)") + axes(y, [0.2, 0.3, 0.5, 0.7, 1], "bits; 4 fresh holdout curves x 24 targets x 2 rounds per size", "paired ratio S(candidate) / S(inc-sel) (log scale)");
  SIZES.forEach((b) => { svg += `<rect x="${x(b) - 14}" y="${y(PRED[b][1])}" width="28" height="${y(PRED[b][0]) - y(PRED[b][1])}" fill="#7d3c98" fill-opacity="0.12"/>`; });
  [["ksum5-sel", "#7d3c98", -9, "k = 5 sum table / inc-sel"], ["ksum4-sel", "#1e8449", 9, "k = 4 sum table / inc-sel"]].forEach(([arm, color, dx, label], i) => {
    const pts = SIZES.map((b) => [b, ["0", "0.1", "1"].map((w) => cmp(b, "inc-sel", arm, w))]);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" points="${pts.map(([b, o]) => `${x(b) + dx},${y(o[0].band.ratio_b_over_a[0])}`).join(" ")}"/>\n`;
    for (const [b, o] of pts) o.forEach((c, k) => {
      const xx = x(b) + dx + (k - 1) * 4, op = [1, 0.7, 0.4][k];
      svg += `<line x1="${xx}" x2="${xx}" y1="${y(c.ci95_bootstrap[0])}" y2="${y(c.ci95_bootstrap[1])}" stroke="${color}" stroke-opacity="${op}" stroke-width="${k ? 1.5 : 3}"/><circle cx="${xx}" cy="${y(c.band.ratio_b_over_a[k])}" r="${k ? 2.2 : 3.5}" fill="${color}" fill-opacity="${op}"/>`;
    });
    const ly = T + 20 + i * 22;
    svg += `\n<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2"/><text x="${W - R + 52}" y="${ly + 4}">${label}</text>\n`;
  });
  svg += `<rect x="${W - R + 15}" y="${T + 54}" width="30" height="10" fill="#7d3c98" fill-opacity="0.12"/><text x="${W - R + 52}" y="${T + 63}">registered k = 5 range</text>\n` + note(["bars: cluster-bootstrap 95% CI (target level),", "left to right w = 0, 0.1, 1 (fading)", "below 1: cheaper than the tuned incumbent", "interval covers targets, not curves: see per_curve.svg", "source: analysis/compare-P*-inc-sel__*.json"], T + 100) + "</svg>\n";
  writeFileSync("ratio.svg", svg);
}
{ // per_curve.svg: the 16 per-curve ratios (test a)
  const y = ylog(0.15, 1.25);
  let svg = head(W, H, "Per-curve ratio to the tuned incumbent, w = 0 (every fresh curve)") + axes(y, [0.2, 0.3, 0.5, 0.7, 1], "bits", "mean S(candidate) / mean S(inc-sel) on one curve");
  const rows = {};
  for (const r of band.rows) ((rows[+/^icv1-fp(\d+)-/.exec(r.curve)[1]] ??= {})[r.curve] ??= {})[r.arm] = r.mean_s[0];
  [["ksum5-sel", "#7d3c98", 5, "k = 5"], ["ksum4-sel", "#1e8449", -5, "k = 4"]].forEach(([arm, color, dx, label], i) => {
    for (const b of SIZES) Object.values(rows[b]).forEach((a, j) => { svg += `<circle cx="${x(b) + dx + (j - 1.5) * 3}" cy="${y(a[arm] / a["inc-sel"])}" r="3.2" fill="${color}" fill-opacity="0.8"/>`; });
    svg += `\n<circle cx="${W - R + 25}" cy="${T + 20 + i * 22}" r="4" fill="${color}"/><text x="${W - R + 52}" y="${T + 24 + i * 22}">${label} / inc-sel, one dot per curve</text>\n`;
  });
  svg += note(["1 = the tuned incumbent on that curve", "source: analysis/band.json (per-curve means)"], T + 80) + "</svg>\n";
  writeFileSync("per_curve.svg", svg);
}
{ // rho_multiple.svg: distance remaining
  const y = ylog(0.8, 60);
  const S = (arm, b) => band.rows.filter((r) => +/^icv1-fp(\d+)-/.exec(r.curve)[1] === b && r.arm === arm).map((r) => r.mean_s[0]).reduce((a, c) => a + c, 0) / 4;
  let svg = head(W, H, "Distance remaining: mean S over rho-neg, same session, w = 0") + axes(y, [1, 2, 5, 10, 20, 50], "bits (4 fresh curves per size)", "S(arm) / S(rho-neg) (log scale, mean of per-curve means)");
  [["inc-sel", "#c0392b", "", "tuned mitm incumbent"], ["ksum4-sel", "#1e8449", "", "k = 4 sum table"], ["ksum5-sel", "#7d3c98", "", "k = 5 sum table"], ["bsgs-neg", "#566573", "2 3", "BSGS (negation)"]].forEach(([arm, color, dash, label], i) => {
    const pts = SIZES.map((b) => [b, S(arm, b) / S("rho-neg", b)]);
    svg += `<polyline fill="none" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""} points="${pts.map(([b, v]) => `${x(b)},${y(v)}`).join(" ")}"/>` + pts.map(([b, v]) => `<circle cx="${x(b)}" cy="${y(v)}" r="3.5" fill="${color}"/>`).join("");
    const ly = T + 20 + i * 22;
    svg += `\n<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2" ${dash ? `stroke-dasharray="${dash}"` : ""}/><text x="${W - R + 52}" y="${ly + 4}">${label}</text>\n`;
  });
  svg += note(["1 = rho with negation map", "generic track: the limit is BSGS", "source: analysis/band.json"], T + 124) + "</svg>\n";
  writeFileSync("rho_multiple.svg", svg);
}
