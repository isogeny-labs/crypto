// One-shot dashboard update for this result (AGENTS.md §7a): one point in
// docs/ic/progress-timeline.json for the session's best cold one-target IC
// cell, the embedded copy in the scoreboard regenerated from the file, its
// plotted-point row and its panel.  Every number is copied from
// analysis/band.json and analysis/compare-*.json.  Run from the repository root.
import { readFileSync, writeFileSync } from "node:fs";

const DATE = process.argv[2] ?? "2026-10-29";
const DIR = "research/prime_ksum_pooled_20261029";
const cmp = (b, a, c) => JSON.parse(readFileSync(`${DIR}/analysis/compare-P${b}-${a}__${c}-w0.json`, "utf8")).ops;
const rho = (b, arm) => cmp(b, "rho-neg", arm).band.ratio_b_over_a[0]; // pooled IC / rho-neg at w = 0
const f = (x, d = 2) => x.toFixed(d);
const ic = ["inc-sel", "ksum4-sel", "ksum5-sel"];
// Best cold one-target IC cell: the lowest IC / rho-neg at w = 0 over every IC arm and size.
let best = null;
for (const b of [20, 24, 28, 32]) for (const a of ic) {
  const v = rho(b, a);
  if (!best || v < best.v) best = { b, a, v };
}
const was = rho(best.b, "inc-sel");
const r = (b) => cmp(b, "inc-sel", "ksum5-sel").band.ratio_b_over_a[0];
const ANCHOR = "#prime-ksum-pooled-20261029";
const SOURCE = `https://github.com/isogeny-labs/crypto/blob/main/${DIR}/README.md`;
const NAME = { "ksum4-sel": "k = 4", "ksum5-sel": "k = 5", "inc-sel": "tuned mitm incumbent" }[best.a];
const LABEL = `Prime-field IC, generic decomposition track, ${NAME} sum table vs the tuned mitm incumbent, ${best.b}-bit, 4 fresh curves pooled`;

const tlPath = "docs/ic/progress-timeline.json";
const point = {
  date: DATE,
  value: best.v,
  reference: "matched",
  unit: "counted GAE lower-bound diagnostic",
  label: LABEL,
  detail:
    `${best.a} / rho-neg pooled mean S at w = 0 on 4 fresh ${best.b}-bit curves (24 targets x 2 rounds each, every run verified, replay-all audited; incumbent re-tuned on separate train curves by the same rule): ` +
    `${f(best.v)}x. Was ${f(was)}x (tuned mitm incumbent, same session and curves). ksum5 / inc-sel ${f(r(20), 3)} / ${f(r(24), 3)} / ${f(r(28), 3)} / ${f(r(32), 3)} at 20 / 24 / 28 / 32 bits, on all 16 curves and below 1 at every band weight. ` +
    `Track: generic decomposition (bounded by sqrt(r); cannot beat rho). Same host only (unqualified; second-host replay owed). Pooled over four curves, so not comparable with the single-curve ksum4 point of research/prime_ksum_vs_incumbent_20261029. Lookups unpriced.`,
  class: "engineering",
  anchor: ANCHOR,
  source: SOURCE,
};
// The file keeps Python json formatting (indent 1, ASCII-escaped, float
// literals as written), so the point is spliced into the text of its series and
// every other byte is left alone.  A rerun replaces the point with this anchor.
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
const kept = spans.filter((sp) => JSON.parse(sp).anchor !== point.anchor);
kept.push(ascii(JSON.stringify(point, null, 1)).replace(/\n/g, "\n    "));
const tail = tl0.slice(tl0.lastIndexOf("}", end) + 1, end);
tl0 = tl0.slice(0, open + 1) + "\n    " + kept.join(",\n    ") + tail + tl0.slice(end);
tl0 = tl0.replace(/"updated": "([0-9-]+)"/, (_, d) => `"updated": "${d > DATE ? d : DATE}"`);
writeFileSync(tlPath, tl0);
const tlText = readFileSync(tlPath, "utf8");

const pagePath = "docs/index-calculus-scoreboard.html";
let page = readFileSync(pagePath, "utf8");
page = page.replace(
  /(<script type="application\/json" id="progress-data">)([\s\S]*?)(<\/script>)/,
  (_, a, _b, c) => a + tlText + c,
);
const row =
  `<tr><td>${DATE}</td><td>Elliptic curve, cold counted-work diagnostics with unpriced native work</td><td>Prime-field IC, ${NAME} sum table vs the tuned mitm incumbent, ${best.b}-bit, 4 fresh curves pooled</td>` +
  `<td class="n">${f(best.v)}&times; <small>counted GAE lower-bound diagnostic; was ${f(was)}&times; (tuned mitm incumbent, same session)</small></td><td>matched</td><td><span class="chip">engineering</span></td>` +
  `<td><a href="${ANCHOR}">panel</a> &middot; <a href="${SOURCE}">source</a></td></tr>`;
const rowRe = new RegExp(`<tr><td>[0-9-]+</td>(?:(?!</tr>)[\\s\\S])*?href="${ANCHOR}">panel[\\s\\S]*?</tr>`);
if (rowRe.test(page)) page = page.replace(rowRe, row);
else {
  page = page.replace(/(<summary>Every plotted point as a table<\/summary>[\s\S]*?)(<\/tbody>)/, (_, a, b) => a + row + "\n" + b);
}
const alpha = (arm) => JSON.parse(readFileSync(`${DIR}/analysis/bounds/P-${arm}-toy-w0.json`, "utf8")).fit.alpha.toFixed(3);
const panel =
  `<p id="prime-ksum-pooled-20261029">${DATE}: <a href="${SOURCE}">the k-sum tables against a tuned prime-field mitm incumbent, on 16 fresh curves</a>. ` +
  `With the incumbent re-tuned on separate train curves by the same rule as the k-sum tables, the k = 5 sum table costs ${f(r(20))}&times;, ${f(r(24))}&times;, ${f(r(28))}&times; and ${f(r(32))}&times; the tuned incumbent at 20, 24, 28 and 32 bits (w = 0, pooled over 4 fresh curves a size), below it on all 16 curves and at every band weight; the registered primary test at 24&ndash;32 bits holds. ` +
  `Toy-tier exponent ${alpha("ksum5-sel")} against ${alpha("inc-sel")}. ` +
  `Track: generic decomposition (bounded by &radic;r; cannot beat rho); a ranking of generic recipes, not algebraic progress. Still ${f(cmp(32, "rho-neg", "ksum5-sel").band.ratio_b_over_a[0], 1)}&times; rho and ${f(cmp(32, "bsgs-neg", "ksum5-sel").band.ratio_b_over_a[0], 1)}&times; BSGS at 32 bits. Same host only; a second-host replay is owed.</p>`;
const panelRe = /<p id="prime-ksum-pooled-20261029">[\s\S]*?<\/p>/;
if (panelRe.test(page)) page = page.replace(panelRe, panel);
else {
  page = page.replace(/(<p id="prime-ksum-vs-incumbent-20261029">[\s\S]*?<\/p>)/, (m) => m + "\n" + panel);
}
writeFileSync(pagePath, page);
console.log(`point ${f(best.v, 3)} (${best.a} at ${best.b} bits), was ${f(was, 3)}; updated ${JSON.parse(tlText).updated}`);
