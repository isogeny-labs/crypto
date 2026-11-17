// Tooling: figures for README.md from translate_test.json and slabs_more.json (exact counts).
//   reach_sharpness.svg   W*E/c_1 against |M|: random polytopes and thin slabs, with Theorem K's bound 1
//   eform_vs_targets.svg  proved e-form bound 2/(m(m+1)D) against the targets
//   proof_diagram.svg     the fibre-injection step of Theorem K (static drawing)
import { readFileSync, writeFileSync } from "node:fs";
const t = JSON.parse(readFileSync("translate_test.json", "utf8"));
const sl = JSON.parse(readFileSync("slabs_more.json", "utf8"));
const FONT = `font-family="Helvetica, Arial, sans-serif"`;

{ // ---- reach_sharpness
  const W = 860, H = 500, L = 70, R = 270, T = 44, B = 64;
  const lx0 = 1.0, lx1 = 7.6;
  const x = (n) => L + ((Math.log10(n) - lx0) / (lx1 - lx0)) * (W - L - R);
  const y = (v) => T + (1 - v / 1.05) * (H - T - B);
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" ${FONT} font-size="12"><rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="24" font-size="15" font-weight="bold">Theorem K: W_i * E(M) / c_i(M) never exceeds 1, and thin slabs approach it</text>\n`;
  for (const v of [0, 0.2, 0.4, 0.6, 0.8, 1]) s += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="${v === 1 ? "#c0392b" : "#ddd"}" ${v === 1 ? 'stroke-width="1.6" stroke-dasharray="6 3"' : ""}/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const e of [2, 3, 4, 5, 6, 7]) s += `<line x1="${x(10 ** e)}" x2="${x(10 ** e)}" y1="${T}" y2="${H - B}" stroke="#eee"/><text x="${x(10 ** e)}" y="${H - B + 18}" text-anchor="middle">10^${e}</text>\n`;
  s += `<text x="${(L + W - R) / 2}" y="${H - 20}" text-anchor="middle">|M| = number of monomials (log scale)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">W_i * E_trans(M) / c_i(M), coordinate i = the thin one</text>\n`;
  // random polytopes: max over coordinates of rho_i
  const shown = t.rows.filter((r) => r.E > 0 && r.pts >= 16);
for (const r of shown) s += `<circle cx="${x(r.pts)}" cy="${y(r.maxrho)}" r="2.6" fill="${r.name === "s3/3" ? "#1e8449" : "#7d3c98"}" fill-opacity="0.55"/>\n`;
  // slabs, closed form (S3 in 2 unknowns), one polyline per a
  const cols = { 4: "#2874a6", 16: "#d68910", 64: "#a93226", 128: "#17202a" };
  for (const a of [4, 16, 64, 128]) {
    const pts = sl.filter((r) => r.a === a);
    s += `<polyline fill="none" stroke="${cols[a]}" stroke-width="1.8" points="${pts.map((r) => `${x(((a + 1)) * (r.L + 1))},${y(r.rho1)}`).join(" ")}"/>`;
    for (const r of pts) s += `<circle cx="${x(((a + 1)) * (r.L + 1))}" cy="${y(r.rho1)}" r="3" fill="${cols[a]}"/>`;
    s += "\n";
  }
  let ly = T + 14;
  const leg = (c, txt, kind) => { s += (kind === "dot" ? `<circle cx="${W - R + 22}" cy="${ly}" r="3.5" fill="${c}"/>` : `<line x1="${W - R + 10}" x2="${W - R + 36}" y1="${ly}" y2="${ly}" stroke="${c}" stroke-width="2" ${kind === "dash" ? 'stroke-dasharray="6 3"' : ""}/>`) + `<text x="${W - R + 44}" y="${ly + 4}">${txt}</text>\n`; ly += 21; };
  leg("#c0392b", "Theorem K: bound 1", "dash");
  leg("#1e8449", "random polytopes, S3, 3 unknowns", "dot");
  leg("#7d3c98", "random polytopes, S3, 2 unknowns", "dot");
  for (const a of [4, 16, 64, 128]) leg(cols[a], `slab [0,${a}] x [0,L], S3 in 2 unknowns`, "line");
  s += `<text x="${W - R + 10}" y="${ly + 14}" font-size="11" fill="#444">${t.rows.length} random polytopes tested, ${t.rows.length - shown.length} with E=0 or |M| &lt; 16 not drawn;</text><text x="${W - R + 10}" y="${ly + 29}" font-size="11" fill="#444">0 violations; largest value ${t.worst.toFixed(3)}.  Slab maximum 0.991.</text>
<text x="${W - R + 10}" y="${ly + 49}" font-size="11" fill="#444">E_trans = sum_s #{y : y + s*supp f in M}</text>
<text x="${W - R + 10}" y="${ly + 64}" font-size="11" fill="#444">exact integer counts; no timings.</text></svg>\n`;
  writeFileSync("reach_sharpness.svg", s);
}

{ // ---- eform_vs_targets
  const W = 760, H = 420, L = 80, R = 40, T = 50, B = 70;
  const rows = [["S3 (m=2, D=2)", 1 / 6, 1 / 2, "3x"], ["S4 (m=3, D=4)", 1 / 24, 1 / 4, "6x"], ["S5 (m=4, D=8)", 1 / 80, 1 / 5, "16x"]];
  const lo = Math.log10(0.008), hi = Math.log10(0.8);
  const y = (v) => T + (1 - (Math.log10(v) - lo) / (hi - lo)) * (H - T - B);
  let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" ${FONT} font-size="12"><rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="26" font-size="15" font-weight="bold">Symmetric e-form: proved bound on delta = log_p B against the target</text>\n`;
  for (const v of [0.01, 0.02, 0.05, 0.1, 0.2, 0.5]) s += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="#ddd"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  const bw = 62, gap = (W - L - R) / rows.length;
  rows.forEach(([lab, bound, target, f], i) => {
    const cx = L + gap * (i + 0.5);
    s += `<rect x="${cx - bw - 4}" y="${y(bound)}" width="${bw}" height="${H - B - y(bound)}" fill="#7d3c98"/><rect x="${cx + 4}" y="${y(target)}" width="${bw}" height="${H - B - y(target)}" fill="#bbb"/>
<text x="${cx - bw / 2 - 4}" y="${y(bound) - 6}" text-anchor="middle">1/${Math.round(1 / bound)}</text><text x="${cx + bw / 2 + 4}" y="${y(target) - 6}" text-anchor="middle">1/${Math.round(1 / target)}</text>
<text x="${cx}" y="${H - B + 20}" text-anchor="middle">${lab}</text><text x="${cx}" y="${H - B + 38}" text-anchor="middle" fill="#a93226">short by ${f}</text>\n`;
  });
  s += `<rect x="${W - 250}" y="34" width="12" height="12" fill="#7d3c98"/><text x="${W - 232}" y="45">proved: 2/(m(m+1)D) (any lattice)</text><rect x="${W - 250}" y="54" width="12" height="12" fill="#bbb"/><text x="${W - 232}" y="65">target</text>
<text x="${L}" y="${H - 8}" font-size="11" fill="#444">log scale; convex monomial sets, one block, HG with dimension factors dropped. Supports: eform_support.out.</text></svg>\n`;
  writeFileSync("eform_vs_targets.svg", s);
}

{ // ---- proof_diagram (static)
  const s = `<svg xmlns="http://www.w3.org/2000/svg" width="860" height="470" ${FONT} font-size="13"><rect width="860" height="470" fill="white"/>
<defs><marker id="a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0 0 L6 3 L0 6 Z" fill="#555"/></marker></defs>
<text x="30" y="26" font-size="15" font-weight="bold">Theorem K, fibre step: every y in A_s lands in two fibres of M</text>
<text x="30" y="50" font-size="12">y -> y + s*nu is a lattice translation (injective), so  #fibre_t(A_s) &lt;= min( l(t + s*m), l(t + s*M&apos;) ),  l(t) = #{mu in M : mu_i = t}.</text>
<text x="30" y="68" font-size="12">Summing over s and t counts pairs (t&apos;, t&apos;&apos;) whose difference is a positive multiple of w = M&apos; - m (m, M&apos; = min and max of nu_i over supp f).</text>
<line x1="60" y1="400" x2="800" y2="400" stroke="#333"/><line x1="60" y1="400" x2="60" y2="96" stroke="#333"/>
<text x="805" y="404">x_i</text><text x="40" y="92">other coordinates</text>
<path d="M60 400 L60 110 L420 130 L700 270 L700 400 Z" fill="#eaf2f8" stroke="#2874a6" stroke-width="2"/><text x="440" y="190" fill="#2874a6">M (lattice points of a convex R)</text>
<path d="M60 400 L60 210 L230 220 L330 310 L330 400 Z" fill="#fdebd0" stroke="#d68910" stroke-width="2"/><text x="72" y="250" fill="#9c640c" font-size="12">A_s = { y : y + s*supp f in M }</text>
<line x1="120" y1="400" x2="120" y2="300" stroke="#a93226" stroke-width="4"/><text x="100" y="420" fill="#a93226">fibre t of A_s</text>
<line x1="260" y1="400" x2="260" y2="180" stroke="#1e8449" stroke-width="4"/><text x="200" y="438" fill="#1e8449">fibre t + s*m of M</text>
<line x1="540" y1="400" x2="540" y2="240" stroke="#1e8449" stroke-width="4"/><text x="470" y="420" fill="#1e8449">fibre t + s*M&apos; of M</text>
<path d="M124 320 C 160 280 220 280 256 300" fill="none" stroke="#555" marker-end="url(#a)"/><text x="150" y="272" fill="#555" font-size="12">+ s*nu_min</text>
<path d="M124 350 C 220 392 440 392 536 350" fill="none" stroke="#555" marker-end="url(#a)"/><text x="300" y="368" fill="#555" font-size="12">+ s*nu_max</text>
<text x="30" y="462" font-size="11" fill="#444">The layer-cake and counting lemma (class_count_le, pairs_le_sum in the Lean file) then give  w * E &lt;= sum_{mu in M} mu_i = c_i.  Editable source: plot.mjs.</text></svg>\n`;
  writeFileSync("proof_diagram.svg", s);
}
console.log("ok");
