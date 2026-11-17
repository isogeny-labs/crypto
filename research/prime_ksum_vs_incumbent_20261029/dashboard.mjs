// One-shot dashboard update for this result (AGENTS.md §7a): one point in
// docs/ic/progress-timeline.json for the session's best cold one-target IC
// cell, the embedded copy in the scoreboard regenerated from the file, its
// plotted-point row and its panel.  Every number is copied from
// analysis/band.json and analysis/compare-*.json.  Run from the repository root.
import { readFileSync, writeFileSync } from "node:fs";

const DATE = process.argv[2] ?? "2026-10-29";
const DIR = "research/prime_ksum_vs_incumbent_20261029";
const band = JSON.parse(readFileSync(`${DIR}/analysis/band.json`, "utf8"));
const s = (arm, bits) => band.rows.find((r) => r.arm === arm && Math.round(r.log2_r) === bits).mean_s;
const cmp = (b, a, c) => JSON.parse(readFileSync(`${DIR}/analysis/compare-P${b}-${a}__${c}-w0.json`, "utf8")).ops;
const f = (x, d = 2) => x.toFixed(d);
const ic = ["inc", "ksum4", "ksum5"];
// Best cold one-target IC cell: the lowest IC / rho-neg at w = 0 over every IC arm and size.
let best = null;
for (const b of [20, 24, 28, 32]) for (const a of ic) {
  const v = s(a, b)[0] / s("rho-neg", b)[0];
  if (!best || v < best.v) best = { b, a, v };
}
const was = s("inc", best.b)[0] / s("rho-neg", best.b)[0];
const r = (b) => cmp(b, "inc", "ksum5").ratio_b_over_a;
const ANCHOR = "#prime-ksum-vs-incumbent-20261029";
const SOURCE = `https://github.com/isogeny-labs/crypto/blob/main/${DIR}/README.md`;
const CURVE = { 20: "icv1-fp20-tm565-6d7d1d32" }[best.b];
const NAME = { ksum4: "k = 4", ksum5: "k = 5" }[best.a];
const LABEL = `Prime-field IC, generic decomposition track, ${NAME} sum table vs the mitm incumbent on its ${best.b}-bit holdout curve ${CURVE}`;

const tlPath = "docs/ic/progress-timeline.json";
const point = {
  date: DATE,
  value: best.v,
  reference: "matched",
  unit: "counted GAE lower-bound diagnostic",
  label: LABEL,
  detail:
    `${best.a} / rho-neg mean S at w = 0 on prime_base_oracle_tuning_20261029's ${best.b}-bit holdout curve and targets, same session as the incumbent recipe (48 x 2 runs per arm, every run verified, replay-all audited; inc and rho-neg reproduce the incumbent's recorded means exactly): ` +
    `${f(best.v)}x. Was ${f(was)}x (mitm negation-folded N 45, the incumbent). ksum5 / inc ${f(r(20))} / ${f(r(24))} / ${f(r(28))} / ${f(r(32))} at 20 / 24 / 28 / 32 bits, below 1 at every band weight; ksum4 vs ksum5 unresolved at 20 and 24 bits. ` +
    `Track: generic decomposition (bounded by sqrt(r); cannot beat rho). Same host only (unqualified; second-host replay owed). Lookups unpriced.`,
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
  `<tr><td>${DATE}</td><td>Elliptic curve, cold counted-work diagnostics with unpriced native work</td><td>Prime-field IC, ${NAME} sum table vs the mitm incumbent, ${best.b}-bit holdout <code>${CURVE}</code></td>` +
  `<td class="n">${f(best.v)}&times; <small>counted GAE lower-bound diagnostic; was ${f(was)}&times; (mitm incumbent, same session)</small></td><td>matched</td><td><span class="chip">engineering</span></td>` +
  `<td><a href="${ANCHOR}">panel</a> &middot; <a href="${SOURCE}">source</a></td></tr>`;
const rowRe = new RegExp(`<tr><td>[0-9-]+</td>(?:(?!</tr>)[\\s\\S])*?href="${ANCHOR}">panel[\\s\\S]*?</tr>`);
if (rowRe.test(page)) page = page.replace(rowRe, row);
else {
  page = page.replace(/(<summary>Every plotted point as a table<\/summary>[\s\S]*?)(<\/tbody>)/, (_, a, b) => a + row + "\n" + b);
}
const panel =
  `<p id="prime-ksum-vs-incumbent-20261029">${DATE}: <a href="${SOURCE}">the k-sum table against the prime-field mitm incumbent, same session and targets</a>. ` +
  `On the incumbent's own holdout curves, targets and seeds (its recorded means reproduced exactly), the k = 5 sum table costs ${f(r(20))}&times;, ${f(r(24))}&times;, ${f(r(28))}&times; and ${f(r(32))}&times; the incumbent at 20, 24, 28 and 32 bits (w = 0), below 1 at every band weight; registered test at 24&ndash;32 bits. ` +
  `Track: generic decomposition (bounded by &radic;r; cannot beat rho); a ranking of two generic recipes, not algebraic progress. Still ${f(s("ksum5", 32)[0] / s("rho-neg", 32)[0], 1)}&times; rho and ${f(s("ksum5", 32)[0] / s("bsgs-neg", 32)[0], 1)}&times; BSGS at 32 bits. Same host only; a second-host replay is owed.</p>`;
const panelRe = /<p id="prime-ksum-vs-incumbent-20261029">[\s\S]*?<\/p>/;
if (panelRe.test(page)) page = page.replace(panelRe, panel);
else {
  page = page.replace(/(<p id="prime-ksum-k5-20261028">[\s\S]*?<\/p>)/, (m) => m + "\n" + panel);
}
writeFileSync(pagePath, page);
console.log(`point ${f(best.v, 3)} (${best.a} at ${best.b} bits), was ${f(was, 3)}; updated ${JSON.parse(tlText).updated}`);
