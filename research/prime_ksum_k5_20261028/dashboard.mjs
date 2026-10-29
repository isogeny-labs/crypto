// One-shot dashboard update for this result (AGENTS.md §7a): one point in
// docs/ic/progress-timeline.json, the embedded copy in the scoreboard
// regenerated from the file, its plotted-point row, its panel, and the prime
// table-IC best row (replaced, it was #2808's).  Every number is copied from
// analysis/band-prime.json and analysis/bounds/.  Run from the repository root.
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";

const DATE = process.argv[2] ?? "2026-10-29";
const DIR = "research/prime_ksum_k5_20261028";
const band = JSON.parse(readFileSync(`${DIR}/analysis/band-prime.json`, "utf8"));
const s = (arm, bits) => band.rows.find((r) => r.arm === arm && Math.round(r.log2_r) === bits).mean_s;
const alpha = (arm) => JSON.parse(readFileSync(`${DIR}/analysis/bounds/P-${arm}-medium-w0.json`, "utf8")).fit;
const [k5, k4, rho, bsgs] = ["ic-ksum5", "ic-ksum4", "rho-neg", "bsgs-neg"].map((a) => s(a, 40));
const ratio = k5[0] / rho[0], was = k4[0] / rho[0];
const r48 = s("ic-ksum5", 48)[0] / s("rho-neg", 48)[0];
const a5 = alpha("ic-ksum5"), a4 = alpha("ic-ksum4");
const f = (x, d = 1) => x.toFixed(d);
const ci = (a) => `${f(a.alpha, 3)} [${f(a.alpha_ci95[0], 3)}, ${f(a.alpha_ci95[1], 3)}]`;
const ANCHOR = "#prime-ksum-k5-20261028";
const SOURCE = `https://github.com/isogeny-labs/crypto/blob/main/${DIR}/README.md`;
const LABEL = "Prime-field IC, generic decomposition track, k = 5 sum table (one probe a target), 40-bit a = -3 curve icv1-fp40-tm924137-0ecaaeb9";

const tlPath = "docs/ic/progress-timeline.json";
const point = {
  date: DATE,
  value: ratio,
  reference: "matched",
  unit: "counted GAE lower-bound diagnostic",
  label: LABEL,
  detail:
    `ic-ksum5 / rho-neg mean S at w = 0 on the frontier campaign's 40-bit curve and test targets (16 x 2 runs, every run verified, replay-all audited): ` +
    `${f(ratio, 2)}x; ${f(k5[1] / rho[1], 2)}x at w = 0.1, ${f(k5[2] / rho[2], 2)}x at w = 1. Was ${f(was, 2)}x (ic-ksum4, same session, reproducing #2808 run for run). ` +
    `Below ic-ksum4 across the band at 24-48 bits; fitted alpha ${ci(a5)} on 36-48 bits against ${ci(a4)}. ${f(r48, 1)}x rho at 48 bits. Track: generic decomposition (bounded by sqrt(r); cannot beat rho); not ranked against Table B (different curves). Still ${f(k5[0] / bsgs[0], 1)}x BSGS, the family's limit. Lookups unpriced.`,
  class: "engineering",
  anchor: ANCHOR,
  source: SOURCE,
};
// The file is written by Python's json (indent 1, ASCII-escaped, float
// literals kept), so the edit goes through it and leaves every other line alone.
execFileSync("python3", ["-c", `
import json, sys
path, date, point = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
tl = json.load(open(path))
s = next(x for x in tl["series"] if x["id"] == "ec-bounded-counts")
s["points"] = [p for p in s["points"] if p["anchor"] != point["anchor"]] + [point]
tl["updated"] = max(tl["updated"], date)
open(path, "w").write(json.dumps(tl, indent=1) + "\\n")
`, tlPath, DATE, JSON.stringify(point)]);
const tl = JSON.parse(readFileSync(tlPath, "utf8"));
const tlText = readFileSync(tlPath, "utf8");

const pagePath = "docs/index-calculus-scoreboard.html";
let page = readFileSync(pagePath, "utf8");
page = page.replace(
  /(<script type="application\/json" id="progress-data">)([\s\S]*?)(<\/script>)/,
  (_, a, _b, c) => a + tlText + c,
);
const row =
  `<tr><td>${DATE}</td><td>Elliptic curve, cold counted-work diagnostics with unpriced native work</td><td>Prime-field IC, k = 5 sum table (one probe a target), 40-bit <code>icv1-fp40-tm924137-0ecaaeb9</code></td>` +
  `<td class="n">${f(ratio, 2)}&times; <small>counted GAE lower-bound diagnostic; was ${f(was, 2)}&times; (k = 4 table)</small></td><td>matched</td><td><span class="chip">engineering</span></td>` +
  `<td><a href="${ANCHOR}">panel</a> &middot; <a href="${SOURCE}">source</a></td></tr>`;
if (!page.includes(`href="${ANCHOR}">panel`)) {
  page = page.replace(/(<summary>Every plotted point as a table<\/summary>[\s\S]*?)(<\/tbody>)/, (_, a, b) => a + row + "\n" + b);
}
const panel =
  `<p id="prime-ksum-k5-20261028">${DATE}: <a href="${SOURCE}">prime-field index calculus with a 5-sum table</a>. ` +
  `A table of every signed 5-sum of a smaller base lowers the prime IC baseline below the k = 4 table at every size from 24 to 48 bits and every band weight (registered test: 36 and 40 bits): ` +
  `at 40 bits <code>S</code> ${f(k5[0], 2)} against ${f(k4[0], 2)}, ${f(k5[2])} against ${f(k4[2])} at w = 1; fitted exponent ${ci(a5)} against ${ci(a4)} on 36&ndash;48 bits. ` +
  `Track: generic decomposition (bounded by &radic;r; cannot beat rho); class engineering within it. The method is generic: it converges to baby-step giant-step and stays ${f(k5[0] / bsgs[0], 1)}&times; above it and ${f(ratio, 1)}&times; above rho at 40 bits (${f(r48, 1)}&times; at 48 bits).</p>`;
if (!page.includes('id="prime-ksum-k5-20261028"')) {
  page = page.replace(/(<p id="prime-ksum-table-20261028">[\s\S]*?<\/p>)/, (m) => m + "\n" + panel);
}
const best =
  `<tr><td>Prime field, generic table IC, one-target cold counted S (a = &minus;3, 20&ndash;48 bits)</td><td>k = 5 sum table (\`ksum:m=5\`, A = 33) on <code>icv1-fp40-tm924137-0ecaaeb9</code>, frontier campaign curves and test targets, 16 &times; 2 runs a size; the frontier-comparable cell (also measured at 44 and 48 bits: ${f(r48, 1)}&times; at 48)</td>` +
  `<td class="n"><strong>${f(ratio, 1)}&times;</strong> <small>S ${f(k5[0], 2)} against rho-neg ${f(rho[0], 2)}; was ${f(was, 1)}&times; (k = 4 table, S ${f(k4[0], 2)})</small></td><td><span class="chip">engineering</span></td>` +
  `<td>Generic decomposition (bounded by &radic;r; cannot beat rho). Lowers the prime-field IC baseline at every size from 24 bits and every band weight (medium-tier exponent 0.565 &rarr; 0.547) but is generic: it converges to baby-step giant-step and stays ${f(k5[0] / bsgs[0], 1)}&times; above it. <a href="${ANCHOR}">Panel</a> &middot; <a href="${SOURCE}">report</a></td></tr>`;
const bestRe = /<tr><td>Prime field, generic table IC[\s\S]*?<\/tr>/;
if (!bestRe.test(page)) throw new Error("best row of #2808 not found");
page = page.replace(bestRe, best);
writeFileSync(pagePath, page);
console.log(`point ${f(ratio, 3)} (was ${f(was, 2)}); updated ${tl.updated}`);
