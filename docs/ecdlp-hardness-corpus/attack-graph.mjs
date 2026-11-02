#!/usr/bin/env node
// Attack graph over the claim corpus: which claims rest on what, how weak the weakest
// support is, and which nodes to attack (prove, refute, measure, review) first.
//
//   node attack-graph.mjs            write attack-graph.json, attack-graph.dot, ATTACK_GRAPH.md
//   node attack-graph.mjs --check    also fail if a tracked lemma is cited in the repository
//                                    by a file its node does not list in cited_by
//
// Support edges (A -> B means B rests on A): conditions, enables, model_bridge.
// `instantiates` (A is an instance of B) and `attacks_when` (A attacks problem B) are
// not support: a general route does not rest on one instance of it. Taint propagates along them from every node whose status
// is weaker than established. `qualifies` and `obstructs` are recorded as pressure on a
// node, not as support.

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { loadCorpus } from "./load.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, "../..");
const corpus = loadCorpus(path.join(here, "corpus.json"));
const check = process.argv.includes("--check");

const supportRelations = new Set(["conditions", "enables", "model_bridge"]);
const pressureRelations = new Set(["refutes", "obstructs", "qualifies", "does_not_cover"]);
// Lower is weaker. A node's support strength is the weakest status among itself and
// everything it rests on.
const strength = {
  refuted: 0, open: 1, heuristic: 2, assumed: 3, conditional: 4,
  proved_internal_unreviewed: 5, measured: 6, algorithm: 7, proved_external: 8, definition: 9
};
const action = {
  refuted: "audit every dependent; repair or retract the estimates that use it",
  open: "prove or refute",
  heuristic: "prove, or measure on a scaling family without censoring",
  assumed: "assumption: narrow its scope or reduce it to a weaker one",
  conditional: "discharge each condition, or keep the row labelled conditional",
  proved_internal_unreviewed: "independent review, then kernel-check"
};

const nodes = new Map(corpus.nodes.map(node => [node.id, node]));
const children = new Map(corpus.nodes.map(node => [node.id, []]));
const parents = new Map(corpus.nodes.map(node => [node.id, []]));
const pressure = new Map(corpus.nodes.map(node => [node.id, []]));
for (const edge of corpus.edges) {
  if (supportRelations.has(edge.relation)) {
    children.get(edge.from).push(edge.to);
    parents.get(edge.to).push(edge.from);
  } else if (pressureRelations.has(edge.relation)) {
    pressure.get(edge.to).push({ from: edge.from, relation: edge.relation });
  }
}

function closure(start, next) {
  const seen = new Set();
  const queue = [...next.get(start)];
  while (queue.length > 0) {
    const id = queue.shift();
    if (seen.has(id) || id === start) continue;
    seen.add(id);
    queue.push(...next.get(id));
  }
  return [...seen].sort();
}

const targetsByNode = new Map();
for (const target of corpus.research_targets) {
  for (const dep of target.depends_on) {
    if (!targetsByNode.has(dep)) targetsByNode.set(dep, []);
    targetsByNode.get(dep).push(target.id);
  }
}

const rows = [];
for (const node of corpus.nodes) {
  const upstream = closure(node.id, parents);
  const downstream = closure(node.id, children);
  const weakAncestors = upstream.filter(id => strength[nodes.get(id).status] < strength.conditional);
  const weakest = [node.id, ...upstream].reduce((low, id) =>
    strength[nodes.get(id).status] < strength[nodes.get(low).status] ? id : low, node.id);
  const citations = node.cited_by ?? [];
  rows.push({
    id: node.id,
    label: node.label,
    status: node.status,
    formal: node.formal?.state ?? "none",
    weakest_support: nodes.get(weakest).status,
    weakest_support_node: weakest,
    tainted_by: weakAncestors,
    refuted_ancestors: upstream.filter(id => nodes.get(id).status === "refuted"),
    downstream,
    pressure: pressure.get(node.id),
    depends_citations: citations.filter(c => c.use === "depends").map(c => c.path),
    research_targets: targetsByNode.get(node.id) ?? []
  });
}

// Frontier: every node that is not established, ranked by how much rests on it.
const frontier = rows
  .filter(row => action[row.status] !== undefined)
  .map(row => ({
    ...row,
    action: action[row.status],
    impact: row.downstream.length + row.depends_citations.length
  }))
  .sort((a, b) => b.impact - a.impact || strength[a.status] - strength[b.status] || a.id.localeCompare(b.id));

// Citation lint: files that match a tracked node's citation_patterns but are not linked.
function trackedFiles() {
  const out = execFileSync("git", ["ls-files", "-z"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 });
  return out.split("\0").filter(file =>
    /\.(md|tex|rs|typ|py|lean|txt)$/.test(file) &&
    !/(^|\/)(target|node_modules)\//.test(file) &&
    !file.startsWith("docs/session-logs/") &&
    !/\/verification\/[^/]+\/run[^/]*\/source\//.test(file));
}
const unlinked = [];
const tracked = corpus.nodes.filter(node => (node.citation_patterns ?? []).length > 0);
if (tracked.length > 0) {
  const files = trackedFiles();
  for (const node of tracked) {
    const patterns = node.citation_patterns.map(p => new RegExp(p, "i"));
    const linked = new Set((node.cited_by ?? []).map(c => c.path));
    for (const file of files) {
      if (linked.has(file)) continue;
      let text;
      try { text = fs.readFileSync(path.join(repoRoot, file), "utf8"); } catch { continue; }
      if (text.length > 4_000_000) continue;
      const hit = patterns.find(p => p.test(text));
      if (hit) unlinked.push({ node: node.id, path: file, pattern: hit.source });
    }
  }
}

const statusCounts = {};
for (const row of rows) statusCounts[row.status] = (statusCounts[row.status] ?? 0) + 1;
const formalCounts = {};
for (const row of rows) formalCounts[row.formal] = (formalCounts[row.formal] ?? 0) + 1;
const contaminated = rows.filter(row => row.refuted_ancestors.length > 0);

const report = {
  schema: "ecdlp.attack-graph/v1",
  corpus_version: corpus.version,
  support_relations: [...supportRelations],
  counts: {
    nodes: rows.length,
    frontier: frontier.length,
    contaminated_by_refuted: contaminated.length,
    unlinked_citations: unlinked.length,
    by_status: statusCounts,
    by_formal_state: formalCounts
  },
  frontier: frontier.map(({ id, label, status, formal, action: act, impact, downstream, depends_citations, research_targets, pressure: p }) =>
    ({ id, label, status, formal, action: act, impact, downstream, depends_citations, research_targets, pressure: p })),
  contaminated: contaminated.map(({ id, status, refuted_ancestors, depends_citations }) => ({ id, status, refuted_ancestors, depends_citations })),
  unlinked_citations: unlinked
};
fs.writeFileSync(path.join(here, "attack-graph.json"), `${JSON.stringify(report, null, 2)}\n`);

// DOT: frontier nodes that support something or are refuted, and everything they support.
// Isolated frontier nodes are listed only in ATTACK_GRAPH.md.
const keep = new Set();
for (const row of frontier) {
  if (row.downstream.length === 0 && row.status !== "refuted") continue;
  keep.add(row.id);
  for (const id of row.downstream) keep.add(id);
}
const fill = { refuted: "#f8c9c9", open: "#fff1b8", heuristic: "#f4e8fb", assumed: "#fff4cc", conditional: "#fde2e2", proved_internal_unreviewed: "#e6f1ff" };
const q = value => JSON.stringify(String(value));
const dot = [
  "digraph attack_graph {",
  "  graph [rankdir=LR, bgcolor=\"white\", pad=0.2, nodesep=0.22, ranksep=0.6, fontname=\"DejaVu Sans\", fontsize=16, labelloc=t, label=\"ECDLP attack graph: unestablished claims and what rests on them\"];",
  "  node [shape=box, style=\"rounded,filled\", fontname=\"DejaVu Sans\", fontsize=9];",
  "  edge [fontname=\"DejaVu Sans\", fontsize=8, color=\"#66788a\", arrowsize=0.7];"
];
const byId = new Map(frontier.map(row => [row.id, row]));
for (const id of [...keep].sort()) {
  const node = nodes.get(id);
  const row = byId.get(id);
  const label = `${id}\n${node.status}${row ? ` · impact ${row.impact}` : ""}${node.formal ? `\nformal: ${node.formal.state}` : ""}`;
  const color = rows.find(r => r.id === id).refuted_ancestors.length > 0 || node.status === "refuted" ? "#a33a3a" : "#526575";
  dot.push(`  ${q(id)} [label=${q(label)}, fillcolor=${q(fill[node.status] ?? "#eef2f5")}, color=${q(color)}${node.status === "refuted" ? ", penwidth=2" : ""}];`);
}
for (const edge of corpus.edges) {
  if (keep.has(edge.from) && keep.has(edge.to) && supportRelations.has(edge.relation)) {
    dot.push(`  ${q(edge.from)} -> ${q(edge.to)} [label=${q(edge.relation)}];`);
  }
  if (edge.relation === "refutes" && keep.has(edge.to)) {
    dot.push(`  ${q(edge.from)} [fillcolor="#dff4e5", color="#257942"];`);
    dot.push(`  ${q(edge.from)} -> ${q(edge.to)} [label="refutes", color="#a33a3a", penwidth=2];`);
  }
}
dot.push("}");
fs.writeFileSync(path.join(here, "attack-graph.dot"), `${dot.join("\n")}\n`);

const cell = value => String(value).replaceAll("|", "\\|");
const md = [
  "# ECDLP attack graph",
  "",
  "Generated by `node attack-graph.mjs` from the claim corpus; do not edit by hand.",
  "Support edges are `conditions`, `enables` and `model_bridge` (B rests on A).",
  "Impact counts the claims that rest on a node",
  "transitively plus the repository files whose estimates cite it as `depends`.",
  "",
  `Corpus ${corpus.version}: ${rows.length} claims, ${frontier.length} on the frontier, ` +
    `${contaminated.length} resting on a refuted claim, ${unlinked.length} unlinked citations.`,
  "",
  "## Resting on a refuted claim",
  "",
  "| claim | status | refuted ancestor | depends citations |",
  "|---|---|---|---|",
  ...contaminated.map(row => `| \`${row.id}\` | ${row.status} | ${row.refuted_ancestors.map(id => `\`${id}\``).join(", ")} | ${row.depends_citations.length} |`),
  "",
  "## Frontier (attack in this order)",
  "",
  "| # | claim | status | formal | impact | action | targets |",
  "|---|---|---|---|---|---|---|",
  ...frontier.map((row, index) => `| ${index + 1} | \`${row.id}\` ${cell(row.label)} | ${row.status} | ${row.formal} | ${row.impact} | ${cell(row.action)} | ${row.research_targets.join(", ") || "-"} |`),
  "",
  "## Unlinked citations",
  "",
  unlinked.length === 0 ? "None: every repository file matching a tracked claim's `citation_patterns` is listed in its `cited_by`." :
    ["| claim | file | pattern |", "|---|---|---|", ...unlinked.map(u => `| \`${u.node}\` | \`${u.path}\` | \`${cell(u.pattern)}\` |`)].join("\n"),
  ""
];
fs.writeFileSync(path.join(here, "ATTACK_GRAPH.md"), md.join("\n"));

process.stdout.write(`attack graph: ${frontier.length} frontier, ${contaminated.length} on refuted support, ${unlinked.length} unlinked citations\n`);
if (check && unlinked.length > 0) {
  for (const u of unlinked) process.stderr.write(`UNLINKED ${u.node} ${u.path} (/${u.pattern}/)\n`);
  process.exit(1);
}
