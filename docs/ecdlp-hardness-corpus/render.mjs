#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { loadCorpus } from "./load.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const corpus = loadCorpus(path.join(here, "corpus.json"));

function groupBy(items, keyFn) {
  const groups = new Map();
  for (const item of items) {
    const key = keyFn(item);
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(item);
  }
  return groups;
}

const colors = {
  definition: ["#e8eef5", "#334e68"],
  assumption: ["#fff4cc", "#8a6200"],
  structural_theorem: ["#dff4e5", "#257942"],
  reduction: ["#e6f1ff", "#2468a2"],
  algorithmic_upper_bound: ["#fce8d8", "#b45309"],
  model_lower_bound: ["#dff4e5", "#257942"],
  conditional_attack: ["#fde2e2", "#a33a3a"],
  measured_bound: ["#ece7fb", "#6047a8"],
  heuristic: ["#f4e8fb", "#7b3fa1"],
  open_obligation: ["#fff1b8", "#9a6700"]
};

const clusters = [
  ["foundations", "Problems and assumption lattice", ["definition", "assumption", "reduction"]],
  ["bounds", "Rigorous model bounds and structural theorems", ["model_lower_bound", "structural_theorem"]],
  ["attacks", "Algorithms and special-family attacks", ["algorithmic_upper_bound", "conditional_attack"]],
  ["evidence", "Measured and heuristic evidence", ["measured_bound", "heuristic"]],
  ["open", "Open proof and attack surfaces", ["open_obligation"]]
];

function quote(value) {
  return JSON.stringify(value.replaceAll("\n", " "));
}

function label(node) {
  const formula = node.formula ? `\\n${node.formula}` : "";
  return `${node.id}\\n${node.label}${formula}`;
}

const lines = [
  "digraph ecdlp_claims {",
  "  graph [rankdir=LR, newrank=true, bgcolor=\"white\", pad=0.2, nodesep=0.25, ranksep=0.65, fontname=\"DejaVu Sans\", fontsize=16, labelloc=t, label=\"ECDLP claim graph — theorem scope, attacks, and open obligations\"];",
  "  node [shape=box, style=\"rounded,filled\", fontname=\"DejaVu Sans\", fontsize=9, margin=\"0.10,0.07\"] ;",
  "  edge [fontname=\"DejaVu Sans\", fontsize=8, color=\"#66788a\", fontcolor=\"#445566\", arrowsize=0.7];"
];

const emitted = new Set();
for (const [clusterId, clusterLabel, kinds] of clusters) {
  lines.push(`  subgraph cluster_${clusterId} {`);
  lines.push(`    label=${quote(clusterLabel)}; color=\"#cad5df\"; style=\"rounded\";`);
  for (const node of corpus.nodes.filter(item => kinds.includes(item.kind))) {
    const [fill, stroke] = colors[node.kind];
    const style = node.status === "open" || node.status === "heuristic" ? "rounded,filled,dashed" : "rounded,filled";
    lines.push(`    ${quote(node.id)} [label=${quote(label(node))}, fillcolor=\"${fill}\", color=\"${stroke}\", style=\"${style}\"];`);
    emitted.add(node.id);
  }
  lines.push("  }");
}

for (const node of corpus.nodes.filter(item => !emitted.has(item.id))) {
  const [fill, stroke] = colors[node.kind] ?? ["#eeeeee", "#555555"];
  lines.push(`  ${quote(node.id)} [label=${quote(label(node))}, fillcolor=\"${fill}\", color=\"${stroke}\"];`);
}

for (const edge of corpus.edges) {
  const dashed = ["does_not_cover", "leaves_open", "qualifies"].includes(edge.relation);
  const color = edge.relation === "obstructs" ? "#a33a3a" : edge.relation === "hardness_implies" ? "#2468a2" : "#66788a";
  lines.push(`  ${quote(edge.from)} -> ${quote(edge.to)} [label=${quote(edge.relation)}, color=\"${color}\", style=\"${dashed ? "dashed" : "solid"}\"];`);
}

lines.push("}");
fs.writeFileSync(path.join(here, "graph.dot"), `${lines.join("\n")}\n`);

const dimensions = ["kind", "status", "model"];
const stats = [];
for (const dimension of dimensions) {
  for (const [value, nodes] of [...groupBy(corpus.nodes, node => node[dimension])].sort()) {
    stats.push(`${dimension}\t${value}\t${nodes.length}`);
  }
}
stats.push(`total\tnodes\t${corpus.nodes.length}`);
stats.push(`total\tedges\t${corpus.edges.length}`);
stats.push(`total\tsources\t${corpus.sources.length}`);
stats.push(`total\tresearch_targets\t${corpus.research_targets.length}`);
fs.writeFileSync(path.join(here, "stats.tsv"), `dimension\tvalue\tcount\n${stats.join("\n")}\n`);

const safeCell = value => {
  const text = String(value ?? "").replaceAll("\t", " ").replaceAll("\n", " ");
  return text.length === 0 ? "-" : text;
};
const claimRows = corpus.nodes.map(node => [
  node.id, node.kind, node.status, node.model, node.evidence_class ?? "",
  node.formula ?? "", node.source_refs.join(",")
].map(safeCell).join("\t"));
fs.writeFileSync(path.join(here, "claims.tsv"), `id\tkind\tstatus\tmodel\tevidence_class\tformula\tsource_refs\n${claimRows.join("\n")}\n`);

const sourceRows = corpus.sources.map(source => [
  source.id, source.verification, source.kind, source.citation,
  source.url ?? source.repository_path ?? "", source.sha256 ?? ""
].map(safeCell).join("\t"));
fs.writeFileSync(path.join(here, "sources.tsv"), `id\tverification\tkind\tcitation\tlocation\tsha256\n${sourceRows.join("\n")}\n`);

function domain(node) {
  if (node.kind === "open_obligation") return "open";
  if (node.model === "quantum") return "quantum";
  if (node.model === "repository_measurement" || node.kind === "measured_bound") return "evidence";
  if (node.model === "algebraic_coordinate") return "algebraic";
  if (node.model === "curve_family_restricted") return "special";
  if (node.model.startsWith("generic_group")) return "generic";
  return "foundation";
}

const domainLabels = {
  foundation: "Definitions, assumptions, reductions",
  generic: "Generic algorithms and model bounds",
  algebraic: "Coordinate / index-calculus route",
  special: "Special-family and pairing attacks",
  quantum: "Quantum algorithms and resources",
  evidence: "Measured repository evidence",
  open: "Open proof obligations"
};
const domainColors = {
  foundation: "#e8eef5", generic: "#dff4e5", algebraic: "#f4e8fb",
  special: "#fde2e2", quantum: "#e6f1ff", evidence: "#ece7fb", open: "#fff1b8"
};
const nodeById = new Map(corpus.nodes.map(node => [node.id, node]));
const domainCounts = groupBy(corpus.nodes, domain);
const aggregateEdges = new Map();
for (const edge of corpus.edges) {
  const from = domain(nodeById.get(edge.from));
  const to = domain(nodeById.get(edge.to));
  if (from === to) continue;
  const key = `${from}\u0000${to}`;
  aggregateEdges.set(key, (aggregateEdges.get(key) ?? 0) + 1);
}
const overview = [
  "digraph ecdlp_overview {",
  "  graph [rankdir=LR, bgcolor=\"white\", pad=0.2, nodesep=0.45, ranksep=0.8, fontname=\"DejaVu Sans\", fontsize=16, labelloc=t, label=\"ECDLP hardness corpus — logical layers\"];",
  "  node [shape=box, style=\"rounded,filled\", fontname=\"DejaVu Sans\", fontsize=11, margin=\"0.14,0.10\"];",
  "  edge [fontname=\"DejaVu Sans\", fontsize=9, color=\"#66788a\", arrowsize=0.8];"
];
for (const [id, labelText] of Object.entries(domainLabels)) {
  overview.push(`  ${quote(id)} [label=${quote(`${labelText}\\n${domainCounts.get(id)?.length ?? 0} claims`)}, fillcolor=\"${domainColors[id]}\", color=\"#526575\"];`);
}
for (const [key, count] of [...aggregateEdges].sort()) {
  const [from, to] = key.split("\u0000");
  overview.push(`  ${quote(from)} -> ${quote(to)} [label=${quote(`${count} typed edges`)}];`);
}
overview.push("}");
fs.writeFileSync(path.join(here, "overview.dot"), `${overview.join("\n")}\n`);
