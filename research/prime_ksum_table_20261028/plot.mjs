// Derived graphs for README.md from the native outputs (analysis/band-prime.json,
// test/sessions/*/records.jsonl).  Writes s_vs_bits.svg and cost_law.svg.
// Thin rendering only: every number plotted is read from ecbench's own output.
import { readFileSync, writeFileSync } from "node:fs";

const band = JSON.parse(readFileSync("analysis/band-prime.json", "utf8"));
const arms = [
  ["ic-pair", "pair table (frontier baseline)", "#c0392b", "8 4"],
  ["ic-ksum3", "k = 3 sum table", "#d68910", ""],
  ["ic-ksum4", "k = 4 sum table", "#1e8449", ""],
  ["kangaroo", "kangaroo (vOW)", "#7d3c98", "2 3"],
  ["bsgs-neg", "BSGS (negation)", "#566573", "2 3"],
  ["rho-neg", "rho (negation), reference", "#1f4e79", ""],
];

// Per-run S (w = 0) from the records, for a 95 % interval on each mean.
const runs = {};
for (const b of [20, 24, 28, 32, 36, 40]) {
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

const W = 760, H = 500, L = 70, R = 220, T = 40, B = 60;
const x = (bits) => L + ((bits - 20) / 20) * (W - L - R);
const lo = Math.log10(0.5), hi = Math.log10(300);
const y = (s) => T + (1 - (Math.log10(s) - lo) / (hi - lo)) * (H - T - B);
let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">Prime-field ECDLP: mean S per arm, w = 0 (bars: w = 0.1 and w = 1)</text>\n`;
for (const s of [0.5, 1, 2, 5, 10, 20, 50, 100, 200]) {
  svg += `<line x1="${L}" x2="${W - R}" y1="${y(s)}" y2="${y(s)}" stroke="#ddd"/><text x="${L - 8}" y="${y(s) + 4}" text-anchor="end">${s}</text>\n`;
}
for (const b of [20, 24, 28, 32, 36, 40]) {
  svg += `<text x="${x(b)}" y="${H - B + 18}" text-anchor="middle">${b}</text>\n`;
}
svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">log2 r (bits); 16 targets x 2 rounds per arm and size, every run verified</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">S = group-op-equivalents / sqrt(r) (log scale)</text>\n`;
arms.forEach(([arm, label, color, dash], i) => {
  const pts = band.rows.filter((r) => r.arm === arm).map((r) => [Math.round(r.log2_r), r.mean_s]);
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
svg += `<text x="${W - R + 15}" y="${T + 170}" font-size="11" fill="#444">thick bar: 95% CI of the w = 0 mean</text>
<text x="${W - R + 15}" y="${T + 186}" font-size="11" fill="#444">thin bar: w = 0 to w = 1, dot at w = 0.1</text>
<text x="${W - R + 15}" y="${T + 210}" font-size="11" fill="#444">sessions: test/sessions/P20 ... P40</text>
</svg>\n`;
writeFileSync("s_vs_bits.svg", svg);

// Explanatory diagram: where each arm's cost goes, and the generic exponents.
const box = (bx, by, w, h, fill, lines) =>
  `<rect x="${bx}" y="${by}" width="${w}" height="${h}" rx="6" fill="${fill}" stroke="#333"/>` +
  lines.map((t, i) => `<text x="${bx + 10}" y="${by + 20 + i * 17}">${t}</text>`).join("") + "\n";
let d = `<svg xmlns="http://www.w3.org/2000/svg" width="760" height="330" font-family="Helvetica, Arial, sans-serif" font-size="13">
<rect width="760" height="330" fill="white"/>
<text x="20" y="26" font-size="15" font-weight="bold">Generic table index calculus on E(F_p): one probe per walk step</text>
<defs><marker id="a" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="#333"/></marker></defs>\n`;
d += box(20, 45, 220, 95, "#eaf2f8", ["base F: A points with small x", "(prime-abscissa), logs unknown", "cost: A Legendre + sqrt (native)"]);
d += box(270, 45, 230, 95, "#fef5e7", ["table: every signed k-sum of F", "~2^(k-1) C(A,k) entries and adds", "k = 2: pair table (baseline)", "k = 3, 4: this study"]);
d += box(530, 45, 210, 95, "#eafaf1", ["walk R = [a]G + [b]Q", "one add + one probe per step", "hit prob ~ 2 * entries / p"]);
d += box(150, 190, 260, 95, "#f4ecf7", ["relations: ~c_k * A rows of weight k", "c_2 = 0.46-0.63 (first cycle; measured)", "c_3 = 0.92-1.00, c_4 = 0.98-1.06 (measured)"]);
d += box(450, 190, 290, 95, "#fdedec", ["cost ~ table + c_k A p / (2 entries)", "optimum: exponent k / (2k - 1)", "k=2: 2/3   k=3: 3/5   k=4: 4/7", "limit k -> large: BSGS, never below"]);
d += `<line x1="240" y1="92" x2="268" y2="92" stroke="#333" marker-end="url(#a)"/><line x1="500" y1="92" x2="528" y2="92" stroke="#333" marker-end="url(#a)"/>
<line x1="630" y1="140" x2="350" y2="188" stroke="#333" marker-end="url(#a)"/><line x1="410" y1="237" x2="448" y2="237" stroke="#333" marker-end="url(#a)"/>
<text x="20" y="315" font-size="11" fill="#444">Derived (cost law) and measured (c_k) labels; the law and its BSGS limit hold for any generic decomposition, so this is engineering, not an advance.</text>
</svg>\n`;
writeFileSync("cost_law.svg", d);
