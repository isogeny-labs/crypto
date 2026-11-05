// One-shot dashboard update for this result (AGENTS.md §7a): one point in
// docs/ic/progress-timeline.json, the embedded copy in the scoreboard
// regenerated from the file, its plotted-point row, and its panel.  Every
// number is copied from analysis/band-prime.json.  Run from the repository root.
import { readFileSync, writeFileSync } from "node:fs";

const DATE = process.argv[2] ?? "2026-10-28";
const band = JSON.parse(readFileSync("research/prime_ksum_table_20261028/analysis/band-prime.json", "utf8"));
const s = (arm, bits) => band.rows.find((r) => r.arm === arm && Math.round(r.log2_r) === bits).mean_s;
const [k4, pair, rho, bsgs] = ["ic-ksum4", "ic-pair", "rho-neg", "bsgs-neg"].map((a) => s(a, 40));
const ratio = k4[0] / rho[0], was = pair[0] / rho[0];
const lo = s("ic-ksum4", 20)[0] / s("rho-neg", 20)[0];
const f = (x, d = 1) => x.toFixed(d);
const ANCHOR = "#prime-ksum-table-20261028";
const SOURCE = "https://github.com/isogeny-labs/crypto/blob/main/research/prime_ksum_table_20261028/README.md";
const LABEL = "Prime-field IC, k = 4 sum table (one probe a target), 40-bit a = -3 curve icv1-fp40-tm924137-0ecaaeb9";

const tlPath = "docs/ic/progress-timeline.json";
const point = {
  date: DATE,
  value: ratio,
  reference: "matched",
  unit: "counted GAE lower-bound diagnostic",
  label: LABEL,
  detail:
    `ic-ksum4 / rho-neg mean S at w = 0 on the frontier campaign's 40-bit curve and test targets (16 x 2 runs, every run verified, replay-all audited): ` +
    `${f(ratio, 2)}x; ${f(k4[1] / rho[1], 2)}x at w = 0.1, ${f(k4[2] / rho[2], 2)}x at w = 1. Was ${f(was, 1)}x (ic-pair, same session). ` +
    `Fitted alpha 0.521 [0.506, 0.538] on 20-32 bits against ic-pair 0.620 [0.586, 0.650]. Generic: still ${f(k4[0] / bsgs[0], 1)}x BSGS, the family's limit. Largest size measured; the ratio is smallest at 20 bits (${f(lo, 2)}x). Lookups unpriced.`,
  class: "engineering",
  anchor: ANCHOR,
  source: SOURCE,
};
// The file keeps Python json formatting (indent 1, ASCII-escaped, float
// literals as written), so the point is spliced into the text of its series and
// every other byte is left alone.  A rerun replaces the point with this anchor in place.
const ascii = (x) => x.replace(/[\u0080-\uffff]/g, (c) => "\\u" + c.charCodeAt(0).toString(16).padStart(4, "0"));
const close = (text, i) => { // index of the bracket closing the one at text[i]
  let depth = 0, str = false;
  for (; i < text.length; i++) {
    const c = text[i];
    if (str) { if (c === "\\") i++; else if (c === "\"") str = false; continue; }
    if (c === "\"") str = true;
    else if (c === "[" || c === "{") depth++;
    else if ((c === "]" || c === "}") && --depth === 0) return i;
  }
  throw new Error("unbalanced timeline file");
};
let tl0 = readFileSync(tlPath, "utf8");
const sid = tl0.indexOf(`"id": "ec-bounded-counts"`);
const open = tl0.indexOf("\"points\": [", sid) + "\"points\": ".length;
const end = close(tl0, open);
const spans = [];
for (let i = tl0.indexOf("{", open); i > 0 && i < end; i = tl0.indexOf("{", i)) {
  const j = close(tl0, i);
  spans.push(tl0.slice(i, j + 1));
  i = j + 1;
}
const kept = spans.slice();
const fresh = ascii(JSON.stringify(point, null, 1)).replace(/\n/g, "\n    ");
const at = kept.findIndex((sp) => JSON.parse(sp).anchor === point.anchor);
if (at >= 0) kept[at] = fresh; else kept.push(fresh);
const tail = tl0.slice(tl0.lastIndexOf("}", end) + 1, end);
tl0 = tl0.slice(0, open + 1) + "\n    " + kept.join(",\n    ") + tail + tl0.slice(end);
tl0 = tl0.replace(/"updated": "([0-9-]+)"/, (_, d) => `"updated": "${d > DATE ? d : DATE}"`);
writeFileSync(tlPath, tl0);
const tl = JSON.parse(readFileSync(tlPath, "utf8"));
const tlText = readFileSync(tlPath, "utf8");

const pagePath = "docs/index-calculus-scoreboard.html";
let page = readFileSync(pagePath, "utf8");
page = page.replace(
  /(<script type="application\/json" id="progress-data">)([\s\S]*?)(<\/script>)/,
  (_, a, _b, c) => a + tlText + c,
);
const row =
  `<tr><td>${DATE}</td><td>Elliptic curve, cold counted-work diagnostics with unpriced native work</td><td>Prime-field IC, k = 4 sum table (one probe a target), 40-bit <code>icv1-fp40-tm924137-0ecaaeb9</code></td>` +
  `<td class="n">${f(ratio, 2)}&times; <small>counted GAE lower-bound diagnostic; was ${f(was, 1)}&times; (pair table)</small></td><td>matched</td><td><span class="chip">engineering</span></td>` +
  `<td><a href="${ANCHOR}">panel</a> &middot; <a href="${SOURCE}">source</a></td></tr>`;
const rowRe = new RegExp(`<tr><td>[0-9-]+</td>(?:(?!</tr>)[\\s\\S])*?href="${ANCHOR}">panel[\\s\\S]*?</tr>`);
if (rowRe.test(page)) page = page.replace(rowRe, row);
else {
  page = page.replace(/(<summary>Every plotted point as a table<\/summary>[\s\S]*?)(<\/tbody>)/, (_, a, b) => a + row + "\n" + b);
}
const panel =
  `<p id="prime-ksum-table-20261028">${DATE}: <a href="${SOURCE}">prime-field index calculus with a k-sum table</a>. ` +
  `Replacing the pair table with a table of every signed 3- or 4-sum of a smaller base (one probe a target) lowers the prime IC baseline on the frontier campaign's six curves and test targets at every size and every band weight: ` +
  `at 40 bits <code>S</code> ${f(k4[0])} (k = 4) against ${f(pair[0])} (pair table), ${f(k4[2])} against ${f(pair[2])} at w = 1; fitted exponent 0.521 [0.506, 0.538] against 0.620 [0.586, 0.650]. ` +
  `Class: engineering. The method is generic: it converges to baby-step giant-step and stays ${f(k4[0] / bsgs[0], 1)}&times; above it and ${f(ratio, 1)}&times; above rho at 40 bits.</p>`;
const panelRe = /<p id="prime-ksum-table-20261028">[\s\S]*?<\/p>/;
if (panelRe.test(page)) page = page.replace(panelRe, panel);
else {
  page = page.replace(/(<p id="m4-narrow-engines-wide-vars-20261020">[\s\S]*?<\/p>)/, (m) => m + "\n" + panel);
}
const best =
  `<tr><td>Prime field, generic table IC, one-target cold counted S (a = &minus;3, 20&ndash;40 bits)</td><td>k = 4 sum table (\`ksum:m=4\`, A = 64) on <code>icv1-fp40-tm924137-0ecaaeb9</code>, frontier campaign curves and test targets, 16 &times; 2 runs a size; the largest size measured, chosen as the frontier-comparable cell (smallest ratio ${f(lo, 1)}&times; at 20 bits)</td>` +
  `<td class="n"><strong>${f(ratio, 1)}&times;</strong> <small>S ${f(k4[0])} against rho-neg ${f(rho[0], 2)}; was ${f(was, 1)}&times; (pair table, S ${f(pair[0])})</small></td><td><span class="chip">engineering</span></td>` +
  `<td>Lowers the prime-field IC baseline at every size and band weight (exponent 0.620 &rarr; 0.521) but is generic: it converges to baby-step giant-step and stays ${f(k4[0] / bsgs[0], 1)}&times; above it. <a href="${ANCHOR}">Panel</a> &middot; <a href="${SOURCE}">report</a></td></tr>`;
if (!page.includes("Prime field, generic table IC")) {
  page = page.replace(/(<tr><td>Whole cold method, multi-target \(historical diagnostic\)<\/td>[\s\S]*?<\/tr>)/, (m) => m + "\n" + best);
}
writeFileSync(pagePath, page);
console.log(`point ${f(ratio, 3)} (was ${f(was, 2)}); updated ${tl.updated}`);
