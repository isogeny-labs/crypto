// Derived graphs for README.md from analysis/regime.json (itself derived by
// derive.mjs from the native records).  Writes amortised.svg and pt_plane.svg.
// Thin rendering only: every number plotted is read from regime.json.
import { readFileSync, writeFileSync } from "node:fs";

const { rows, crossovers } = JSON.parse(readFileSync("analysis/regime.json", "utf8"));
const row = (bits, arm, w) => rows.find((r) => r.bits === bits && r.arm === arm && r.w === w);
const COL = { "kl-ksum4": "#1e8449", "pre-eqS": "#1f4e79", "pre-eqP": "#d68910", "rho-neg": "#566573", "ic-ksum4": "#c0392b" };
const NAME = {
  "kl-ksum4": "known-log k = 4 table (kl-ksum4)",
  "pre-eqS": "preprocessing rho, equal memory (pre-eqS)",
  "pre-eqP": "preprocessing rho, small table (pre-eqP)",
  "rho-neg": "rho (negation), per target",
  "ic-ksum4": "cold IC k = 4 table, per target",
};
const head = (W, H, title) => `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="70" y="22" font-size="15" font-weight="bold">${title}</text>\n`;

// 1. Amortised S per target against the number of targets L, 40 bits.
{
  const W = 840, H = 500, L = 70, R = 310, T = 40, B = 60;
  const lx0 = 0, lx1 = 6, ly0 = Math.log10(0.0005), ly1 = Math.log10(5000);
  const x = (n) => L + ((Math.log10(n) - lx0) / (lx1 - lx0)) * (W - L - R);
  const y = (s) => T + (1 - (Math.log10(s) - ly0) / (ly1 - ly0)) * (H - T - B);
  let svg = head(W, H, "40-bit curve: amortised cost per target after L targets (w = 0; band edge w = 1 dashed)");
  for (const s of [0.001, 0.01, 0.1, 1, 10, 100, 1000]) svg += `<line x1="${L}" x2="${W - R}" y1="${y(s)}" y2="${y(s)}" stroke="#ddd"/><text x="${L - 8}" y="${y(s) + 4}" text-anchor="end">${s}</text>\n`;
  for (let e = 0; e <= 6; e++) svg += `<line x1="${x(10 ** e)}" x2="${x(10 ** e)}" y1="${T}" y2="${H - B}" stroke="#eee"/><text x="${x(10 ** e)}" y="${H - B + 18}" text-anchor="middle">10^${e}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">L = targets sharing one precomputation (log scale)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">amortised S = P/(L sqrt r) + T/sqrt r (log scale)</text>\n`;
  const Ls = Array.from({ length: 61 }, (_, i) => 10 ** (i / 10));
  ["kl-ksum4", "pre-eqS", "pre-eqP", "rho-neg", "ic-ksum4"].forEach((arm, i) => {
    for (const [w, dash] of [[0, ""], [1, "6 4"]]) {
      const r = row(40, arm, w);
      const f = (n) => (r.setup_s ? r.setup_s / n : 0) + r.online_s;
      svg += `<polyline fill="none" stroke="${COL[arm]}" stroke-width="${w ? 1.2 : 2.2}" ${dash ? `stroke-dasharray="${dash}"` : ""} points="${Ls.map((n) => `${x(n).toFixed(1)},${y(f(n)).toFixed(1)}`).join(" ")}"/>\n`;
    }
    const ly = T + 20 + i * 22;
    svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${COL[arm]}" stroke-width="2.2"/><text x="${W - R + 52}" y="${ly + 4}" font-size="11">${NAME[arm]}</text>\n`;
  });
  const c = crossovers.find((c) => c.bits === 40 && c.w === 0 && c.a === "kl-ksum4" && c.b === "pre-eqS");
  const v = row(40, "kl-ksum4", 0).setup_s / c.crossover_targets + row(40, "kl-ksum4", 0).online_s;
  svg += `<circle cx="${x(c.crossover_targets)}" cy="${y(v)}" r="5" fill="none" stroke="#000"/><text x="${x(c.crossover_targets) + 8}" y="${y(v) - 8}" font-size="11">kl = pre-eqS at L = ${Math.round(c.crossover_targets).toLocaleString("en")}</text>\n`;
  svg += `<text x="${W - R + 15}" y="${T + 150}" font-size="11" fill="#444">16 targets x 1 round per arm, all verified,</text>
<text x="${W - R + 15}" y="${T + 165}" font-size="11" fill="#444">replay-all audited (test/sessions/P40).</text>
<text x="${W - R + 15}" y="${T + 185}" font-size="11" fill="#444">Accounting only: no single-target</text>
<text x="${W - R + 15}" y="${T + 200}" font-size="11" fill="#444">cost falls below rho.</text>
</svg>\n`;
  writeFileSync("amortised.svg", svg);
}

// 2. The precomputation-time plane: P/sqrt(N) against T/sqrt(N), with P*T = N.
{
  const W = 840, H = 520, L = 70, R = 310, T = 40, B = 60;
  const lx0 = -4, lx1 = 1, ly0 = -0.5, ly1 = 4;
  const x = (t) => L + ((Math.log10(t) - lx0) / (lx1 - lx0)) * (W - L - R);
  const y = (p) => T + (1 - (Math.log10(p) - ly0) / (ly1 - ly0)) * (H - T - B);
  let svg = head(W, H, "Precomputation P against online cost T, both / sqrt(N), N = r/2 (w = 0)");
  for (let e = lx0; e <= lx1; e++) svg += `<line x1="${x(10 ** e)}" x2="${x(10 ** e)}" y1="${T}" y2="${H - B}" stroke="#eee"/><text x="${x(10 ** e)}" y="${H - B + 18}" text-anchor="middle">10^${e}</text>\n`;
  for (let e = 0; e <= ly1; e++) svg += `<line x1="${L}" x2="${W - R}" y1="${y(10 ** e)}" y2="${y(10 ** e)}" stroke="#eee"/><text x="${L - 8}" y="${y(10 ** e) + 4}" text-anchor="end">10^${e}</text>\n`;
  for (const [k, lab] of [[1, "P·T = N"], [10, "P·T = 10 N"]]) {
    const pts = [];
    for (let e = lx0; e <= lx1 + 1e-9; e += 0.05) { const t = 10 ** e, p = k / t; if (Math.log10(p) <= ly1 && Math.log10(p) >= ly0) pts.push(`${x(t).toFixed(1)},${y(p).toFixed(1)}`); }
    svg += `<polyline fill="none" stroke="#888" stroke-dasharray="${k === 1 ? "4 3" : "1 3"}" points="${pts.join(" ")}"/>`;
    const t = k / 10 ** 3.6; svg += `<text x="${x(t) + 6}" y="${y(10 ** 3.6) + 4}" font-size="11" fill="#666">${lab}</text>\n`;
  }
  const s2 = Math.SQRT2; // T/sqrt(N) = online_s * sqrt(2), likewise P
  const SH = { 32: 3.5, 36: 4.5, 40: 6 };
  ["kl-ksum4", "pre-eqS", "pre-eqP"].forEach((arm, i) => {
    const pts = [32, 36, 40].map((b) => [b, row(b, arm, 0)]);
    svg += `<polyline fill="none" stroke="${COL[arm]}" stroke-width="1.2" points="${pts.map(([, r]) => `${x(r.online_s * s2)},${y(r.setup_s * s2)}`).join(" ")}"/>`;
    for (const [b, r] of pts) svg += `<circle cx="${x(r.online_s * s2)}" cy="${y(r.setup_s * s2)}" r="${SH[b]}" fill="${COL[arm]}" fill-opacity="0.8"/>`;
    const ly = T + 20 + i * 22;
    svg += `<circle cx="${W - R + 30}" cy="${ly}" r="5" fill="${COL[arm]}"/><text x="${W - R + 45}" y="${ly + 4}" font-size="11">${NAME[arm]}</text>\n`;
  });
  svg += `<text x="${W - R + 15}" y="${T + 95}" font-size="11" fill="#444">marker size: 32, 36, 40 bits</text>
<text x="${W - R + 15}" y="${T + 115}" font-size="11" fill="#444">rho per target: T/sqrt(N) = 1.1-1.3, P = 0</text>
<text x="${W - R + 15}" y="${T + 135}" font-size="11" fill="#444">source: analysis/regime.json</text>
<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">T / sqrt(N), online cost per target (log scale)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">P / sqrt(N), target-blind precomputation (log scale)</text>
</svg>\n`;
  writeFileSync("pt_plane.svg", svg);
}
