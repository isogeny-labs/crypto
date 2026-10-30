// Tables and figures for README.md from the native output (results/*.jsonl).
// Thin rendering only: every number is copied from digit_fall's own records;
// the closed form and D_null columns are PROTOCOL.md section 4 and the records'
// own law_ffd / D_null fields.  Writes analysis/tables.md, analysis/summary.json,
// degree_vs_s.svg and plain_vs_digit.svg.  Run from this directory.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

mkdirSync("analysis", { recursive: true });
const load = (f) => readFileSync(`results/${f}.jsonl`, "utf8").split("\n").filter((l) => l.trim()).map((l) => JSON.parse(l));
const all = [...load("A"), ...load("B"), ...load("C"), ...load("D")].filter((r) => !r.error);
// A first-fall-only record with the same seed, prime, curve and target as a closure record is a replay of that
// instance, not a new one (panel D, m = 3, s = 4).
const inst = (r) => `${r.m}|${r.s}|${r.seed}|${r.p}|${r.a}|${r.b}|${r.kind}`;
const closureInst = new Map(all.filter((r) => r.closure).map((r) => [inst(r), r]));
const replays = all.filter((r) => !r.closure && closureInst.has(inst(r)));
const rows = all.filter((r) => !replays.includes(r));
const errors = [...load("A"), ...load("B"), ...load("C"), ...load("D")].filter((r) => r.error);
const plain = load("E");

const key = (r) => `${r.m}|${r.arm}|${r.s}|${r.closure ? "c" : "f"}`;
const cells = new Map();
for (const r of rows) (cells.get(key(r)) ?? cells.set(key(r), []).get(key(r))).push(r);
const uniq = (a) => [...new Set(a)];
const fmtSet = (xs) => {
  const u = uniq(xs).sort((a, b) => a - b);
  return u.length === 1 ? `${u[0]}` : `{${u.join(",")}}`;
};
const summary = [];
let md = "| m | arm | s | draws | distinct (p,a,b,x_R) | primes | nsol | closed form | ffd | d_last | censored | D_null | cols at d_last | cols / 2^n |\n|--:|:--|--:|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|\n";
const order = ["U", "P", "Usupp", "Ublock"];
const keys = [...cells.keys()].sort((x, y) => {
  const [mx, ax, sx, cx] = x.split("|"), [my, ay, sy, cy] = y.split("|");
  return +mx - +my || order.indexOf(ax) - order.indexOf(ay) || +sx - +sy || cx.localeCompare(cy);
});
for (const k of keys) {
  const c = cells.get(k), r0 = c[0];
  const done = c.filter((r) => !r.censored);
  const ids = uniq(c.map((r) => `${r.p},${r.a},${r.b},${r.kind}`)).length;
  const row = {
    m: r0.m, arm: r0.arm, s: r0.s, closure: r0.closure, draws: c.length, distinct: ids,
    primes: uniq(c.map((r) => r.p)).length,
    nsol: fmtSet(c.map((r) => r.nsol)),
    law: r0.law_ffd,
    ffd: fmtSet(c.map((r) => r.ffd)),
    d_last: r0.closure ? fmtSet(done.map((r) => r.d_last)) : "n/a",
    censored: c.length - done.length,
    D_null: r0.D_null,
    cols: r0.closure && done.length ? fmtSet(done.map((r) => r.cols_at_dlast)) : "n/a",
    secs_max: Math.max(...c.map((r) => r.secs)),
  };
  summary.push(row);
  md += `| ${row.m} | ${row.arm}${row.closure ? "" : " (first fall)"} | ${row.s} | ${row.draws} | ${row.distinct} | ${row.primes} | ${row.nsol} | ${row.law} | ${row.ffd} | ${row.d_last} | ${row.censored} | ${row.D_null} | ${row.cols} | ${row.cols === "n/a" ? "n/a" : (+row.cols / 2 ** (r0.m * r0.s)).toFixed(2)} |\n`;
}

// the registered predictions, checked on every independent instance
const chk = (name, ok, bad) => ({ name, ok, bad });
const real = rows.filter((r) => (r.arm === "U" || r.arm === "P") && r.m === 2 && r.closure);
const pred = [
  chk("P1 ffd = 4 + floor(s/2), m = 2, real draws", real.filter((r) => r.ffd === 4 + Math.floor(r.s / 2)).length, real.filter((r) => r.ffd !== 4 + Math.floor(r.s / 2))),
  chk("P1 ffd = 13, m = 3, s = 4, 5 (U), independent instances", rows.filter((r) => r.m === 3 && r.ffd === 13).length, rows.filter((r) => r.m === 3 && r.ffd !== 13)),
  chk("P2 d_last = ffd, real unsatisfiable draws", real.filter((r) => r.arm === "U" && !r.censored && r.d_last === r.ffd).length, real.filter((r) => r.arm === "U" && (r.censored || r.d_last !== r.ffd))),
  chk("P5 d_last = ffd, planted draws", real.filter((r) => r.arm === "P" && !r.censored && r.d_last === r.ffd).length, real.filter((r) => r.arm === "P" && (r.censored || r.d_last !== r.ffd))),
  chk("P3 Usupp: ffd = d_last = D_null = s + 3", rows.filter((r) => r.arm === "Usupp" && r.ffd === r.s + 3 && r.d_last === r.s + 3).length, rows.filter((r) => r.arm === "Usupp" && !(r.ffd === r.s + 3 && r.d_last === r.s + 3))),
  chk("P4 Ublock: ffd = law and d_last = ffd", rows.filter((r) => r.arm === "Ublock" && r.ffd === r.law_ffd && r.d_last === r.ffd).length, rows.filter((r) => r.arm === "Ublock" && !(r.ffd === r.law_ffd && r.d_last === r.ffd))),
];
const replayOk = replays.filter((r) => closureInst.get(inst(r)).ffd === r.ffd);
pred.push(chk("replay check: the first-fall-only rerun of the m = 3, s = 4 instances gives the closure run ffd", replayOk.length, replays.filter((r) => !replayOk.includes(r))));
md += "\n| registered prediction | draws holding | draws failing |\n|:--|--:|--:|\n";
for (const p of pred) {
  const tot = p.ok + p.bad.length;
  md += `| ${p.name} | ${p.ok} / ${tot} | ${p.bad.length} |\n`;
}
if (errors.length) md += `\nCells with no unsatisfiable instance found: ${errors.map((e) => `m=${e.m} s=${e.s} p=${e.p}`).join("; ")}\n`;

// plain control
const pc = new Map();
for (const r of plain) (pc.get(r.p) ?? pc.set(r.p, []).get(r.p)).push(r);
md += "\n| p | draws | nsol | ffd | d_last | d_last - p | censored | grid points p^2 |\n|--:|--:|:--|:--|:--|:--|--:|--:|\n";
const plainRows = [];
for (const p of [...pc.keys()].sort((a, b) => a - b)) {
  const c = pc.get(p), done = c.filter((r) => !r.censored);
  md += `| ${p} | ${c.length} | ${fmtSet(c.map((r) => r.nsol))} | ${fmtSet(c.map((r) => r.ffd))} | ${fmtSet(done.map((r) => r.d_last))} | ${fmtSet(done.map((r) => r.d_last - p))} | ${c.length - done.length} | ${p * p} |\n`;
  plainRows.push({ p, ffd: uniq(c.map((r) => r.ffd)), d_last: uniq(done.map((r) => r.d_last)) });
}
writeFileSync("analysis/tables.md", md);
writeFileSync("analysis/summary.json", JSON.stringify({ cells: summary, predictions: pred.map((p) => ({ name: p.name, holding: p.ok, failing: p.bad.length, failing_draws: p.bad.slice(0, 5).map((r) => ({ m: r.m, s: r.s, arm: r.arm, seed: r.seed, p: r.p, ffd: r.ffd, d_last: r.d_last })) })), plain: plainRows }, null, 1) + "\n");

// ---- figure 1: degrees against s
{
  const W = 840, H = 460, L = 70, R = 290, T = 40, B = 60;
  const xs = [2, 3, 4, 5, 6, 7, 8];
  const x = (s) => L + ((s - 1.5) / 7) * (W - L - R);
  const y0 = 4, y1 = 12;
  const y = (v) => T + (1 - (v - y0) / (y1 - y0)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">Digit presentation, m = 2: first and last fall degree against s</text>\n`;
  for (let v = 4; v <= 12; v++) svg += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="#ddd"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const s of xs) svg += `<text x="${x(s)}" y="${H - B + 18}" text-anchor="middle">${s}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">digits per unknown s (n = 2s Boolean variables)</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">degree (d_last; circles: draws)</text>\n`;
  const series = [
    ["U", "real, unsatisfiable (d_last)", "#7d3c98", -5],
    ["P", "real, planted (d_last)", "#c0392b", 0],
    ["Ublock", "block-factored null (d_last)", "#1e8449", 5],
    ["Usupp", "support-matched null (d_last)", "#566573", 10],
  ];
  // closed form and D_null
  svg += `<polyline fill="none" stroke="#555" stroke-dasharray="5 4" points="${xs.map((s) => `${x(s)},${y(4 + Math.floor(s / 2))}`).join(" ")}"/>\n`;
  svg += `<polyline fill="none" stroke="#999" stroke-dasharray="2 3" points="${xs.map((s) => `${x(s)},${y(s + 3)}`).join(" ")}"/>\n`;
  series.forEach(([arm, label, color, dx], i) => {
    const pts = [];
    for (const s of xs) {
      const c = (cells.get(`2|${arm}|${s}|c`) ?? []).filter((r) => !r.censored);
      for (const r of c) svg += `<circle cx="${x(s) + dx}" cy="${y(r.d_last)}" r="3" fill="${color}" fill-opacity="0.45"/>`;
      if (c.length) pts.push([s, c.reduce((a, r) => a + r.d_last, 0) / c.length]);
    }
    svg += `<polyline fill="none" stroke="${color}" stroke-width="1.6" points="${pts.map(([s, v]) => `${x(s) + dx},${y(v)}`).join(" ")}"/>\n`;
    const ly = T + 20 + i * 22;
    svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${ly}" y2="${ly}" stroke="${color}" stroke-width="2"/><text x="${W - R + 52}" y="${ly + 4}">${label}</text>\n`;
  });
  svg += `<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 108}" y2="${T + 108}" stroke="#555" stroke-dasharray="5 4"/><text x="${W - R + 52}" y="${T + 112}">closed form 4 + floor(s/2)</text>
<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 130}" y2="${T + 130}" stroke="#999" stroke-dasharray="2 3"/><text x="${W - R + 52}" y="${T + 134}">null bound D_null = s + 3</text>
<text x="${W - R + 15}" y="${T + 170}" font-size="11" fill="#444">each circle: one draw (own prime, curve,</text>
<text x="${W - R + 15}" y="${T + 186}" font-size="11" fill="#444">target); lines: cell means</text>
<text x="${W - R + 15}" y="${T + 210}" font-size="11" fill="#444">source: results/A.jsonl, B, C</text>
</svg>\n`;
  writeFileSync("degree_vs_s.svg", svg);
}

// ---- figure 2: plain against digit
{
  const W = 840, H = 460, L = 70, R = 290, T = 40, B = 60;
  const ps = plainRows.map((r) => r.p);
  const lo = Math.log2(4.5), hi = Math.log2(40);
  const x = (v) => L + ((Math.log2(v) - Math.log2(4)) / (Math.log2(128) - Math.log2(4))) * (W - L - R);
  const y = (v) => T + (1 - (Math.log2(v) - lo) / (hi - lo)) * (H - T - B);
  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" font-family="Helvetica, Arial, sans-serif" font-size="12">
<rect width="${W}" height="${H}" fill="white"/>
<text x="${L}" y="22" font-size="15" font-weight="bold">Plain presentation against digit presentation (m = 2)</text>\n`;
  for (const v of [5, 8, 12, 20, 32]) svg += `<line x1="${L}" x2="${W - R}" y1="${y(v)}" y2="${y(v)}" stroke="#ddd"/><text x="${L - 8}" y="${y(v) + 4}" text-anchor="end">${v}</text>\n`;
  for (const v of [4, 8, 16, 32, 64, 128]) svg += `<text x="${x(v)}" y="${H - B + 18}" text-anchor="middle">${v}</text>\n`;
  svg += `<text x="${(L + W - R) / 2}" y="${H - 18}" text-anchor="middle">p (plain) or the number of digit-values 2^s (digit); log scale</text>
<text transform="translate(18 ${(T + H - B) / 2}) rotate(-90)" text-anchor="middle">d_last (log scale)</text>\n`;
  const plainPts = plainRows.filter((r) => r.d_last.length).map((r) => [r.p, r.d_last[0]]);
  svg += `<polyline fill="none" stroke="#c0392b" stroke-width="2" points="${plainPts.map(([p, d]) => `${x(p)},${y(d)}`).join(" ")}"/>`;
  for (const [p, d] of plainPts) svg += `<circle cx="${x(p)}" cy="${y(d)}" r="3.5" fill="#c0392b"/>`;
  const dig = [2, 3, 4, 5, 6, 7].map((s) => { const c = (cells.get(`2|U|${s}|c`) ?? []).filter((r) => !r.censored); return c.length ? [2 ** s, c.reduce((a, r) => a + r.d_last, 0) / c.length] : null; }).filter(Boolean);
  svg += `<polyline fill="none" stroke="#7d3c98" stroke-width="2" points="${dig.map(([v, d]) => `${x(v)},${y(d)}`).join(" ")}"/>`;
  for (const [v, d] of dig) svg += `<circle cx="${x(v)}" cy="${y(d)}" r="3.5" fill="#7d3c98"/>`;
  svg += `\n<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 20}" y2="${T + 20}" stroke="#c0392b" stroke-width="2"/><text x="${W - R + 52}" y="${T + 24}">plain F_p[x1,x2]/(x^p - x)</text>
<line x1="${W - R + 15}" x2="${W - R + 45}" y1="${T + 42}" y2="${T + 42}" stroke="#7d3c98" stroke-width="2"/><text x="${W - R + 52}" y="${T + 46}">digit, 2^s values (measured)</text>
<text x="${W - R + 15}" y="${T + 80}" font-size="11" fill="#444">plain: measured d_last, p = 7 .. 31</text>
<text x="${W - R + 15}" y="${T + 96}" font-size="11" fill="#444">digit: measured d_last (panel A)</text>
<text x="${W - R + 15}" y="${T + 120}" font-size="11" fill="#444">different unknown ranges: plain covers</text>
<text x="${W - R + 15}" y="${T + 136}" font-size="11" fill="#444">all of F_p, digit covers 2^s values</text>
<text x="${W - R + 15}" y="${T + 160}" font-size="11" fill="#444">source: results/E.jsonl, results/A.jsonl</text>
</svg>\n`;
  writeFileSync("plain_vs_digit.svg", svg);
}
console.log(md);
