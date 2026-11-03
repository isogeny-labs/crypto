#!/usr/bin/env node
// Weakness analysis of the ECDLP claim corpus.
//
// attack-graph.mjs ranks unestablished claims by how much rests on them. This pass asks a
// different question: where is the structure itself weak, and which weak point is cheapest
// to break? It reports six structural findings and one ranking:
//
//   1. status inflation   established claims (proved, algorithm, measured) whose support
//                         closure contains an open, heuristic or assumed claim
//   2. model-scope leaps  support edges from a narrower model to a broader one that are not
//                         declared model_bridge edges
//   3. source contagion   claims sharing a source with an already refuted claim
//   4. hidden assumptions free-text assumptions repeated across several nodes but never
//                         modelled as a node, so no refutation can propagate from them
//   5. live obstructions  established obstructs/qualifies/does_not_cover pressure on a claim
//                         that still supports others
//   6. author-flagged     nodes whose own failure_modes list is long
//   7. ranking            impact x breakability, with the cheapest discriminating action
//
// Every number is a count over the assembled corpus; the scores are a triage heuristic, not
// evidence about the claims. Usage: node weakness.mjs [--check]
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { loadCorpus } from "./load.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const corpus = loadCorpus(path.join(here, "corpus.json"));
const check = process.argv.includes("--check");

const supportRelations = new Set(["conditions", "enables", "model_bridge"]);
const pressureRelations = new Set(["obstructs", "qualifies", "does_not_cover"]);
// Same ordering as attack-graph.mjs.
const strength = {
  refuted: 0, open: 1, heuristic: 2, assumed: 3, conditional: 4,
  proved_internal_unreviewed: 5, proved_external_unread: 5, measured: 6, algorithm: 7,
  proved_external: 8, definition: 9
};
const established = id => strength[nodes.get(id).status] >= strength.measured;
const weak = id => strength[nodes.get(id).status] <= strength.assumed && nodes.get(id).status !== "refuted";

// How broad a claim's model is. A support edge that goes up this order applies a result
// proved in a narrower setting to a broader one. "mathematical" is model-neutral.
const breadth = {
  repository_measurement: 0, curve_family_restricted: 1, generic_group: 2,
  generic_group_preprocessing: 2, algebraic_coordinate: 2, quantum: 2, classical_unrestricted: 3
};

const leapSources = new Set(["generic_group", "generic_group_preprocessing", "repository_measurement"]);
const nodes = new Map(corpus.nodes.map(n => [n.id, n]));
const children = new Map(corpus.nodes.map(n => [n.id, []]));
const parents = new Map(corpus.nodes.map(n => [n.id, []]));
for (const e of corpus.edges) {
  if (supportRelations.has(e.relation)) {
    children.get(e.from).push(e.to);
    parents.get(e.to).push({ id: e.from, edge: e.id });
  }
}

function downstream(start) {
  const seen = new Set();
  const queue = [...children.get(start)];
  while (queue.length) {
    const id = queue.shift();
    if (seen.has(id) || id === start) continue;
    seen.add(id);
    queue.push(...children.get(id));
  }
  return seen;
}

// Shortest support path from `start` back to the nearest node satisfying `pred`.
function nearestAncestor(start, pred) {
  const prev = new Map([[start, null]]);
  const queue = [start];
  while (queue.length) {
    const id = queue.shift();
    if (id !== start && pred(id)) {
      const pathIds = [];
      for (let x = id; x !== null; x = prev.get(x)) pathIds.push(x);
      return pathIds; // weak ancestor first, start last
    }
    for (const p of parents.get(id)) {
      if (!prev.has(p.id)) { prev.set(p.id, id); queue.push(p.id); }
    }
  }
  return null;
}

const impact = new Map(corpus.nodes.map(n => [n.id, downstream(n.id).size + (n.cited_by ?? []).filter(c => c.use === "depends").length]));

// 1. Status inflation.
const inflation = [];
for (const n of corpus.nodes) {
  if (!established(n.id)) continue;
  const p = nearestAncestor(n.id, weak);
  if (p) inflation.push({ id: n.id, status: n.status, weak_ancestor: p[0], weak_status: nodes.get(p[0]).status, path: p, impact: impact.get(n.id) });
}
inflation.sort((a, b) => b.impact - a.impact || a.id.localeCompare(b.id));

// 2. Model-scope leaps.
const leaps = [];
for (const e of corpus.edges) {
  if (!supportRelations.has(e.relation) || e.relation === "model_bridge") continue;
  const a = nodes.get(e.from), b = nodes.get(e.to);
  if (!(a.model in breadth) || !(b.model in breadth)) continue;
  // A family-restricted attack conditioning a broad exclusion rule is ordinary logic (the
  // rule exists to exclude the family), so only generic-model and measured sources count.
  if (!leapSources.has(a.model)) continue;
  if (breadth[b.model] > breadth[a.model]) {
    leaps.push({ edge: e.id, from: a.id, from_model: a.model, to: b.id, to_model: b.model, relation: e.relation, to_status: b.status, impact: impact.get(b.id), statement: e.statement });
  }
}
leaps.sort((x, y) => (breadth[nodes.get(y.to).model] - breadth[nodes.get(y.from).model]) - (breadth[nodes.get(x.to).model] - breadth[nodes.get(x.from).model]) || y.impact - x.impact || x.edge.localeCompare(y.edge));

// 3. Source contagion. Repository syntheses are excluded: they cite everything.
const sourceKind = new Map(corpus.sources.map(s => [s.id, s]));
const bySource = new Map();
for (const n of corpus.nodes) for (const s of n.source_refs) {
  if (!bySource.has(s)) bySource.set(s, []);
  bySource.get(s).push(n.id);
}
const contagion = [];
for (const [s, ids] of bySource) {
  const src = sourceKind.get(s);
  // Books and standards collect hundreds of results; one refuted line says little about
  // the rest. Repository syntheses cite everything.
  if (!src || ["repository_synthesis", "lead_index", "standard_or_book"].includes(src.kind)) continue;
  const refuted = ids.filter(id => nodes.get(id).status === "refuted");
  if (!refuted.length) continue;
  const exposed = ids.filter(id => nodes.get(id).status !== "refuted" && nodes.get(id).status !== "definition")
    .map(id => ({ id, status: nodes.get(id).status, impact: impact.get(id) }))
    .sort((a, b) => b.impact - a.impact || a.id.localeCompare(b.id));
  if (exposed.length) contagion.push({ source: s, title: src.title, verification: src.verification, refuted, refuted_fraction: Math.round(1000 * refuted.length / ids.length) / 1000, exposed });
}
contagion.sort((a, b) => b.exposed.reduce((t, x) => t + x.impact, 0) - a.exposed.reduce((t, x) => t + x.impact, 0) || a.source.localeCompare(b.source));

// 4. Hidden shared assumptions: greedy clustering of assumption strings by token Jaccard.
const stop = new Set("the a an of and or to is are be in on for by with that this as at from it its into than under each every all any not no".split(" "));
const toks = s => new Set(s.toLowerCase().replace(/[^a-z0-9^ ]+/g, " ").split(/\s+/).filter(w => w.length > 2 && !stop.has(w)));
const jac = (a, b) => { let i = 0; for (const x of a) if (b.has(x)) i++; return i / (a.size + b.size - i || 1); };
const clusters = [];
for (const n of corpus.nodes) for (const text of n.assumptions) {
  const t = toks(text);
  if (t.size < 2) continue;
  let best = null, bestScore = 0;
  for (const c of clusters) { const s = jac(t, c.key); if (s > bestScore) { best = c; bestScore = s; } }
  if (best && bestScore >= 0.5) { best.members.push({ id: n.id, text }); }
  else clusters.push({ key: t, members: [{ id: n.id, text }] });
}
const hidden = clusters
  .map(c => ({ nodes: [...new Set(c.members.map(m => m.id))], texts: [...new Set(c.members.map(m => m.text))] }))
  .filter(c => c.nodes.length >= 3)
  .map(c => ({ ...c, established_users: c.nodes.filter(established), total_impact: c.nodes.reduce((t, id) => t + impact.get(id), 0) }))
  .sort((a, b) => b.nodes.length - a.nodes.length || b.total_impact - a.total_impact);

// 4b. Unwired dependence: a node's own assumption text names a heuristic that exists as a
// node, but no support path connects them, so a refutation of the heuristic would not reach
// it and the heuristic's impact is understated.
const textPatterns = {
  "HEUR-DECOMPOSITION-PROBABILITY": /decomposition probab|relation probab|decomposition (and|yield)/i,
  "HEUR-RELATION-RANK": /relations? (are|is) (sufficiently |independent|.*independent)|enough independent relations|rank (heuristic|model)|relation rank|rank heuristics/i,
  "HEUR-FIRST-FALL-DEGREE": /first (degree )?fall( degree)? (controls|approximates)|first-fall-degree assumption|\bFFDA\b/i
};
function ancestors(id) {
  const seen = new Set(), queue = parents.get(id).map(p => p.id);
  while (queue.length) { const x = queue.shift(); if (seen.has(x)) continue; seen.add(x); queue.push(...parents.get(x).map(p => p.id)); }
  return seen;
}
const unwired = [];
for (const [h, re] of Object.entries(textPatterns)) {
  if (!nodes.has(h)) continue;
  for (const n of corpus.nodes) {
    if (n.id === h || n.status === "refuted") continue;
    const hit = n.assumptions.find(a => re.test(a) && !/height|mordell|canonical/i.test(a));
    if (hit && !ancestors(n.id).has(h)) unwired.push({ heuristic: h, node: n.id, status: n.status, assumption: hit, impact: impact.get(n.id) });
  }
}
unwired.sort((a, b) => a.heuristic.localeCompare(b.heuristic) || b.impact - a.impact || a.node.localeCompare(b.node));

// 5. Live obstructions: established pressure on a claim that still supports something.
const obstructions = [];
for (const e of corpus.edges) {
  if (!pressureRelations.has(e.relation)) continue;
  const from = nodes.get(e.from), to = nodes.get(e.to);
  if (!established(from.id) || to.status === "refuted" || established(to.id)) continue;
  if (impact.get(to.id) === 0) continue;
  obstructions.push({ edge: e.id, relation: e.relation, from: from.id, from_status: from.status, to: to.id, to_status: to.status, impact: impact.get(to.id), statement: e.statement });
}
const relOrder = { obstructs: 0, does_not_cover: 1, qualifies: 2 };
obstructions.sort((a, b) => relOrder[a.relation] - relOrder[b.relation] || b.impact - a.impact || a.edge.localeCompare(b.edge));

// 6. Author-flagged fragility.
const flagged = corpus.nodes
  .filter(n => (n.failure_modes ?? []).length >= 4 && n.status !== "refuted" && !established(n.id))
  .map(n => ({ id: n.id, status: n.status, failure_modes: n.failure_modes, impact: impact.get(n.id) }))
  .sort((a, b) => b.failure_modes.length - a.failure_modes.length || b.impact - a.impact || a.id.localeCompare(b.id));

// 7. Ranking. Breakability is the expected cheapness of a decisive result (higher = cheaper):
// reading a primary text or reviewing an internal proof is cheap; a heuristic stated with a
// formula and parameters in an algebraic or family model can be tested exactly on small
// fields; an open conjecture from the literature is expensive.
function breakability(n) {
  const finite = ["algebraic_coordinate", "curve_family_restricted", "mathematical"].includes(n.model);
  const parametric = Boolean(n.formula) || (n.parameters ?? []).length > 0;
  switch (n.status) {
    case "proved_external_unread": return { score: 0.9, cost: "read", how: "read the primary text; confirm statement, hypotheses and the exact quantifiers the corpus uses" };
    case "proved_internal_unreviewed": return { score: n.formal?.state === "kernel_checked" ? 0.6 : 0.8, cost: "review", how: n.formal?.state === "kernel_checked" ? "audit the Lean statement against the prose claim (statement fidelity), then the axioms" : "independent review; formalise the finite core" };
    case "heuristic": return finite && parametric
      ? { score: 0.8, cost: "exact small-field census", how: "exhaust small fields of a scaling family; compare the formula's prediction with exact counts and report the ratio with its trend" }
      : { score: 0.5, cost: "measurement", how: "measure on a scaling family without censoring" };
    case "conditional": return { score: 0.45, cost: "discharge conditions", how: "attack the weakest condition first; each condition is its own node" };
    case "assumed": return { score: 0.3, cost: "reduction", how: "narrow the scope or reduce to a weaker assumption" };
    case "open": return n.id.startsWith("CONJ-") ? { score: 0.15, cost: "open conjecture", how: "look for a small counterexample only; a proof is out of scope" } : { score: 0.35, cost: "open obligation", how: "prove or refute" };
    default: return null;
  }
}
const contagionBoost = new Map();
for (const c of contagion) for (const x of c.exposed) contagionBoost.set(x.id, (contagionBoost.get(x.id) ?? 0) + 1);
const obstructionBoost = new Map();
for (const o of obstructions) if (o.relation === "obstructs") obstructionBoost.set(o.to, (obstructionBoost.get(o.to) ?? 0) + 1);
const inflationRoots = new Map();
for (const r of inflation) inflationRoots.set(r.weak_ancestor, (inflationRoots.get(r.weak_ancestor) ?? 0) + 1);

const ranking = [];
for (const n of corpus.nodes) {
  if (n.status === "refuted") continue;
  const b = breakability(n);
  if (!b) continue;
  const imp = impact.get(n.id);
  const boosts = (contagionBoost.get(n.id) ?? 0) * 0.25 + (obstructionBoost.get(n.id) ?? 0) * 0.25 + (inflationRoots.get(n.id) ?? 0) * 0.5;
  const score = Math.round(1000 * Math.log2(2 + imp + boosts) * b.score * (1 + 0.5 * (obstructionBoost.get(n.id) ?? 0))) / 1000;
  ranking.push({ id: n.id, label: n.label, status: n.status, model: n.model, impact: imp, inflates: inflationRoots.get(n.id) ?? 0, contagion: contagionBoost.get(n.id) ?? 0, obstructed: obstructionBoost.get(n.id) ?? 0, breakability: b.score, cost: b.cost, action: b.how, score });
}
ranking.sort((a, b) => b.score - a.score || b.impact - a.impact || a.id.localeCompare(b.id));
// Reading a textbook theorem or auditing a kernel-checked statement is verification, not an
// attack: it is cheap but rarely overturns anything. Keep it in its own queue.
const hygieneCosts = new Set(["read", "review"]);
const attackQueue = ranking.filter(r => !hygieneCosts.has(r.cost));
const hygieneQueue = ranking.filter(r => hygieneCosts.has(r.cost));

const out = {
  schema: "ecdlp.weakness-analysis/v1",
  corpus_version: corpus.version,
  counts: {
    nodes: corpus.nodes.length, status_inflation: inflation.length, model_scope_leaps: leaps.length,
    contagious_sources: contagion.length, hidden_assumption_clusters: hidden.length, unwired_dependences: unwired.length,
    live_obstructions: obstructions.length, author_flagged: flagged.length, attack_queue: attackQueue.length, hygiene_queue: hygieneQueue.length
  },
  status_inflation: inflation, model_scope_leaps: leaps, source_contagion: contagion,
  hidden_assumptions: hidden, unwired_dependences: unwired, live_obstructions: obstructions, author_flagged: flagged, attack_queue: attackQueue, hygiene_queue: hygieneQueue
};
const json = JSON.stringify(out, null, 2) + "\n";

// Markdown.
const lbl = id => nodes.get(id).label.replace(/\|/g, "\\|");
const md = [];
md.push("# ECDLP corpus weakness analysis", "",
  "Generated by `node weakness.mjs` from the claim corpus; do not edit by hand.",
  "Counts are exact over the assembled corpus. The breakability scores are a triage heuristic",
  "for choosing what to attack, not evidence about any claim.", "",
  `Corpus ${corpus.version}: ${corpus.nodes.length} claims. ` +
  `${inflation.length} established claims rest on an open, heuristic or assumed claim; ` +
  `${leaps.length} support edges widen the model without a bridge; ` +
  `${contagion.length} sources have a refuted claim and other live claims; ` +
  `${hidden.length} assumption clusters recur across 3+ nodes without being nodes; ` +
  `${obstructions.length} established pressure edges hit claims that still support others.`, "");

md.push("## 1. Attack queue (impact x breakability)", "",
  "Heuristic, conditional, assumed and open claims. `obstructed` counts established",
  "`obstructs` edges into the claim; each multiplies the score by 1.5.", "",
  "| # | claim | status | impact | inflates | obstructed | cost | score | cheapest decisive action |", "|---|---|---|---|---|---|---|---|---|");
attackQueue.slice(0, 30).forEach((r, i) => md.push(`| ${i + 1} | \`${r.id}\` ${lbl(r.id)} | ${r.status} | ${r.impact} | ${r.inflates} | ${r.obstructed} | ${r.cost} | ${r.score} | ${r.action} |`));
md.push("", "### Hygiene queue (read or review; cheap, rarely decisive)", "",
  "| # | claim | status | impact | cost | action |", "|---|---|---|---|---|---|");
hygieneQueue.slice(0, 20).forEach((r, i) => md.push(`| ${i + 1} | \`${r.id}\` ${lbl(r.id)} | ${r.status} | ${r.impact} | ${r.cost} | ${r.action} |`));

md.push("", "## 2. Status inflation", "",
  "Claims marked proved, algorithm or measured whose support closure reaches an open,",
  "heuristic or assumed claim. Either the edge is wrong, the label is too strong, or the",
  "claim is genuinely conditional on the weak ancestor.", "",
  "| claim | status | weakest ancestor | its status | path | impact |", "|---|---|---|---|---|---|");
for (const r of inflation) md.push(`| \`${r.id}\` | ${r.status} | \`${r.weak_ancestor}\` | ${r.weak_status} | ${r.path.map(x => `\`${x}\``).join(" → ")} | ${r.impact} |`);

md.push("", "## 3. Model-scope leaps", "",
  "Support edges from a narrower model to a broader one that are not declared `model_bridge`.",
  "Order: repository_measurement < curve_family_restricted < generic/algebraic/quantum < classical_unrestricted.", "",
  "| edge | from (model) | to (model) | relation | to status | statement |", "|---|---|---|---|---|---|");
for (const l of leaps) md.push(`| ${l.edge} | \`${l.from}\` (${l.from_model}) | \`${l.to}\` (${l.to_model}) | ${l.relation} | ${l.to_status} | ${l.statement.replace(/\|/g, "\\|")} |`);

md.push("", "## 4. Source contagion", "",
  "A source with one refuted claim has shown it can be wrong in exactly the way the corpus",
  "trusts it. Its other live claims are the next things to re-derive.", "");
for (const c of contagion) {
  md.push(`- **${c.source}** (${c.verification}, ${Math.round(100 * c.refuted_fraction)}% of its claims refuted) ${c.title.replace(/\|/g, "\\|")}: refuted ${c.refuted.map(x => `\`${x}\``).join(", ")}; still live:`);
  for (const x of c.exposed) md.push(`  - \`${x.id}\` ${lbl(x.id)} (${x.status}, impact ${x.impact})`);
}

md.push("", "## 5. Hidden shared assumptions", "",
  "Assumption text repeated across three or more nodes but not a node itself. A refutation",
  "of it would not propagate, because no edge carries it.", "");
hidden.forEach((h, i) => {
  md.push(`${i + 1}. **${h.nodes.length} nodes** (${h.established_users.length} established, total impact ${h.total_impact}): ${h.nodes.map(x => `\`${x}\``).join(", ")}`);
  for (const t of h.texts.slice(0, 4)) md.push(`   - "${t}"`);
});

md.push("", "## 5b. Unwired dependences", "",
  "The node's own assumption text invokes a heuristic that is a node, but no support path",
  "connects them. The heuristic's impact above is understated by these rows, and refuting it",
  "would not reach them until the edge is added.", "",
  "| heuristic | node (status) | assumption text | node impact |", "|---|---|---|---|");
for (const u of unwired) md.push(`| \`${u.heuristic}\` | \`${u.node}\` (${u.status}) | ${u.assumption.replace(/\|/g, "\\|")} | ${u.impact} |`);

md.push("", "## 6. Live obstructions", "",
  "An established claim obstructs, qualifies or bounds the scope of a claim that is still",
  "load-bearing. `obstructs` rows are the sharpest: the obstruction is proved and the",
  "obstructed claim still supports others.", "",
  "| edge | relation | from | to (status) | impact | statement |", "|---|---|---|---|---|---|");
for (const o of obstructions) md.push(`| ${o.edge} | ${o.relation} | \`${o.from}\` | \`${o.to}\` (${o.to_status}) | ${o.impact} | ${o.statement.replace(/\|/g, "\\|")} |`);

md.push("", "## 7. Author-flagged fragility", "",
  "Unestablished nodes whose own `failure_modes` list has four or more entries.", "",
  "| claim | status | impact | failure modes |", "|---|---|---|---|");
for (const f of flagged) md.push(`| \`${f.id}\` | ${f.status} | ${f.impact} | ${f.failure_modes.join("; ").replace(/\|/g, "\\|")} |`);
md.push("");
const mdText = md.join("\n");


// SVG: the top of the attack queue. Bar length is the triage score; the label carries the
// counts behind it. Deterministic, no external renderer.
function svgChart(rows) {
  const W = 980, left = 430, right = 150, rowH = 26, top = 64, n = rows.length, H = top + n * rowH + 56;
  const max = Math.max(...rows.map(r => r.score));
  const x = v => left + (W - left - right) * v / max;
  const esc = t => t.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  const fill = r => r.obstructed > 0 ? "#b4443a" : r.cost === "exact small-field census" ? "#2f6f9f" : "#7a8794";
  const o = [`<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}" font-family="DejaVu Sans, Arial, sans-serif">`,
    `<rect width="${W}" height="${H}" fill="#ffffff"/>`,
    `<text x="16" y="26" font-size="17" font-weight="bold" fill="#1d2329">ECDLP claim corpus ${esc(corpus.version)}: attack queue, top ${n} of ${attackQueue.length}</text>`,
    `<text x="16" y="46" font-size="12" fill="#4a535c">score = log2(2 + impact + boosts) x breakability x (1 + 0.5 x obstructs). Triage heuristic, not evidence. Source: weakness.json</text>`];
  for (let t = 0; t <= Math.floor(max); t++) o.push(`<line x1="${x(t)}" y1="${top - 6}" x2="${x(t)}" y2="${top + n * rowH}" stroke="#e3e6ea"/>`, `<text x="${x(t)}" y="${top + n * rowH + 16}" font-size="11" text-anchor="middle" fill="#4a535c">${t}</text>`);
  rows.forEach((r, i) => {
    const y = top + i * rowH;
    o.push(`<text x="${left - 8}" y="${y + 16}" font-size="12" text-anchor="end" fill="#1d2329">${esc(r.id)}</text>`,
      `<rect x="${left}" y="${y + 4}" width="${x(r.score) - left}" height="${rowH - 9}" fill="${fill(r)}"/>`,
      `<text x="${x(r.score) + 6}" y="${y + 16}" font-size="11" fill="#1d2329">${r.score.toFixed(2)}  imp ${r.impact}${r.obstructed ? `, obs ${r.obstructed}` : ""}</text>`);
  });
  const ly = top + n * rowH + 36;
  [["#b4443a", "obstructed by an established claim"], ["#2f6f9f", "testable by exact small-field census"], ["#7a8794", "measurement or condition discharge"]]
    .forEach(([c, t], i) => o.push(`<rect x="${16 + i * 310}" y="${ly - 10}" width="12" height="12" fill="${c}"/>`, `<text x="${34 + i * 310}" y="${ly}" font-size="12" fill="#1d2329">${t}</text>`));
  o.push(`<text x="${left + (W - left - right) / 2}" y="${top + n * rowH + 30}" font-size="11" text-anchor="middle" fill="#4a535c">triage score</text>`, "</svg>");
  return o.join("\n") + "\n";
}
const svgText = svgChart(attackQueue.slice(0, 20));

const jsonPath = path.join(here, "weakness.json"), mdPath = path.join(here, "WEAKNESSES.md"), svgPath = path.join(here, "weakness-queue.svg");
if (check) {
  const stale = [[jsonPath, json], [mdPath, mdText], [svgPath, svgText]].filter(([p, t]) => !fs.existsSync(p) || fs.readFileSync(p, "utf8") !== t);
  if (stale.length) { console.error(`stale: ${stale.map(([p]) => path.basename(p)).join(", ")}; run node weakness.mjs`); process.exit(1); }
} else {
  fs.writeFileSync(jsonPath, json);
  fs.writeFileSync(mdPath, mdText);
  fs.writeFileSync(svgPath, svgText);
}
console.log(`weakness: ${inflation.length} inflated, ${leaps.length} scope leaps, ${contagion.length} contagious sources, ${hidden.length} hidden assumptions, ${unwired.length} unwired, ${obstructions.length} live pressure edges, ${attackQueue.length} attack + ${hygieneQueue.length} hygiene`);
