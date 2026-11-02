#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { loadCorpus } from "./load.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const corpusPath = process.argv[2] ?? path.join(here, "corpus.json");
const corpus = loadCorpus(corpusPath);
const errors = [];

function requireValue(condition, message) {
  if (!condition) errors.push(message);
}

function uniqueIds(items, collection) {
  const seen = new Set();
  for (const item of items) {
    requireValue(typeof item.id === "string" && item.id.length > 0, `${collection}: missing id`);
    requireValue(!seen.has(item.id), `${collection}: duplicate id ${item.id}`);
    seen.add(item.id);
  }
  return seen;
}

requireValue(corpus.schema === "ecdlp.claim-corpus/v1", "wrong corpus schema");
requireValue(/^\d{4}-\d{2}-\d{2}$/.test(corpus.version ?? ""), "version must be YYYY-MM-DD");
for (const key of ["sources", "nodes", "edges", "research_targets"]) {
  requireValue(Array.isArray(corpus[key]), `${key} must be an array`);
}

const sourceIds = uniqueIds(corpus.sources ?? [], "sources");
const nodeIds = uniqueIds(corpus.nodes ?? [], "nodes");
uniqueIds(corpus.edges ?? [], "edges");
uniqueIds(corpus.research_targets ?? [], "research_targets");

const allowedKinds = new Set([
  "definition", "assumption", "structural_theorem", "reduction",
  "algorithmic_upper_bound", "model_lower_bound", "conditional_attack",
  "measured_bound", "heuristic", "open_obligation"
]);
const allowedStatuses = new Set([
  "definition", "assumed", "proved_external", "proved_internal_unreviewed",
  "algorithm", "conditional", "measured", "heuristic", "open"
]);
const allowedModels = new Set([
  "mathematical", "classical_unrestricted", "generic_group",
  "generic_group_preprocessing", "algebraic_coordinate",
  "curve_family_restricted", "quantum", "repository_measurement"
]);
const allowedRelations = new Set([
  "defines", "reduces_to", "hardness_implies", "upper_bounds", "lower_bounds",
  "attacks_when", "enables", "conditions", "instantiates", "model_bridge",
  "does_not_cover", "obstructs", "leaves_open", "measures", "qualifies"
]);
const allowedEvidenceClasses = new Set([
  "theorem", "algorithm", "conditional_theorem", "asymptotic_heuristic",
  "concrete_estimate", "measurement", "assumption", "definition", "open"
]);

for (const source of corpus.sources ?? []) {
  requireValue(typeof source.title === "string" && source.title.length > 0, `${source.id}: missing source title`);
  requireValue(["primary_read", "repository_read", "metadata_only", "secondary_only"].includes(source.verification), `${source.id}: unknown verification ${source.verification}`);
  if (source.verification === "primary_read") {
    requireValue(/^[0-9a-f]{64}$/.test(source.sha256 ?? ""), `${source.id}: primary-read source lacks a SHA-256 digest`);
  }
  if (source.repository_path) {
    const repoRoot = path.resolve(here, "../..");
    requireValue(fs.existsSync(path.resolve(repoRoot, source.repository_path)), `${source.id}: missing repository path ${source.repository_path}`);
  }
}

function checkSourceRefs(owner, refs) {
  requireValue(Array.isArray(refs), `${owner}: source_refs must be an array`);
  for (const ref of refs ?? []) requireValue(sourceIds.has(ref), `${owner}: unknown source ${ref}`);
}

for (const node of corpus.nodes ?? []) {
  requireValue(allowedKinds.has(node.kind), `${node.id}: unknown kind ${node.kind}`);
  requireValue(allowedStatuses.has(node.status), `${node.id}: unknown status ${node.status}`);
  requireValue(allowedModels.has(node.model), `${node.id}: unknown model ${node.model}`);
  requireValue(typeof node.statement === "string" && node.statement.length > 0, `${node.id}: missing statement`);
  requireValue(typeof node.scope === "string" && node.scope.length > 0, `${node.id}: missing scope`);
  if (node.evidence_class !== undefined) {
    requireValue(allowedEvidenceClasses.has(node.evidence_class), `${node.id}: unknown evidence_class ${node.evidence_class}`);
  }
  checkSourceRefs(node.id, node.source_refs);
  if (!["definition", "assumption", "open_obligation"].includes(node.kind)) {
    requireValue(node.source_refs.length > 0, `${node.id}: evidence-bearing node has no source`);
  }
  if (node.kind === "measured_bound") {
    requireValue(node.model === "repository_measurement" && node.status === "measured", `${node.id}: measured bound must be measured repository evidence`);
  }
  if (node.kind === "model_lower_bound") {
    requireValue(node.model.includes("generic_group"), `${node.id}: model lower bound must name a generic model`);
  }
}

const inventoryPath = path.join(here, "galbraith-2018-inventory.json");
requireValue(fs.existsSync(inventoryPath), "missing Galbraith book inventory");
if (fs.existsSync(inventoryPath)) {
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
  requireValue(inventory.schema === "ecdlp.galbraith-book-inventory/v1", "wrong Galbraith inventory schema");
  requireValue(inventory.source?.id === "S-GALBRAITH-MPKC-2018", "wrong Galbraith inventory source id");
  requireValue(inventory.source?.sha256 === "ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d", "wrong Galbraith PDF digest");
  requireValue(inventory.source?.pdf_pages === 696, "wrong Galbraith PDF page count");
  requireValue(Array.isArray(inventory.results), "Galbraith results must be an array");
  requireValue(Array.isArray(inventory.sections), "Galbraith sections must be an array");
  requireValue(inventory.results?.length === 717, "Galbraith inventory must contain 717 labeled results");
  requireValue(inventory.sections?.length === 375, "Galbraith inventory must contain 375 sections");

  const inventoryIds = uniqueIds(inventory.results ?? [], "Galbraith results");
  const resultKeys = new Set();
  const algorithmNumbers = [];
  for (const result of inventory.results ?? []) {
    const key = `${result.type}:${result.number}`;
    requireValue(!resultKeys.has(key), `Galbraith results: duplicate label ${key}`);
    resultKeys.add(key);
    requireValue(Number.isInteger(result.pdf_page) && result.pdf_page >= 1 && result.pdf_page <= 696, `${result.id}: invalid PDF page`);
    requireValue(typeof result.chapter === "string" && result.chapter.length > 0, `${result.id}: missing chapter`);
    requireValue(["ecdlp_core", "protocol_context", "mathematical_dependency", "background"].includes(result.relevance), `${result.id}: invalid relevance`);
    if (result.type === "Algorithm") algorithmNumbers.push(Number(result.number));
  }
  algorithmNumbers.sort((left, right) => left - right);
  requireValue(algorithmNumbers.join(",") === Array.from({ length: 29 }, (_, index) => index + 1).join(","), "Galbraith algorithms must be numbered 1 through 29");

  const sectionNumbers = new Set();
  for (const section of inventory.sections ?? []) {
    requireValue(!sectionNumbers.has(section.number), `Galbraith sections: duplicate ${section.number}`);
    sectionNumbers.add(section.number);
    requireValue(Number.isInteger(section.pdf_page) && section.pdf_page >= 1 && section.pdf_page <= 696, `Galbraith section ${section.number}: invalid PDF page`);
  }
  requireValue(inventory.counts?.labeled_results === inventory.results?.length, "Galbraith labeled-result count is stale");
  requireValue(inventory.counts?.sections === inventory.sections?.length, "Galbraith section count is stale");
  requireValue(inventoryIds.size === inventory.results?.length, "Galbraith result IDs are not unique");
  for (const dimension of ["type", "chapter", "relevance"]) {
    const actual = {};
    for (const result of inventory.results ?? []) actual[result[dimension]] = (actual[result[dimension]] ?? 0) + 1;
    requireValue(JSON.stringify(actual, Object.keys(actual).sort()) === JSON.stringify(inventory.counts?.[`by_${dimension}`], Object.keys(actual).sort()), `Galbraith counts by ${dimension} are stale`);
  }

  const galbraithSource = corpus.sources.find(source => source.id === "S-GALBRAITH-MPKC-2018");
  requireValue(galbraithSource?.sha256 === inventory.source?.sha256, "Galbraith source ledger and inventory digests differ");

  for (const node of corpus.nodes ?? []) {
    if (!node.source_refs.includes("S-GALBRAITH-MPKC-2018")) continue;
    requireValue(Array.isArray(node.source_locations) && node.source_locations.length > 0, `${node.id}: Galbraith-backed node lacks a source locator`);
    for (const location of node.source_locations ?? []) {
      const labels = [...location.matchAll(/(Definition|Theorem|Lemma|Proposition|Corollary|Conjecture|Heuristic|Algorithm)\s+((?:[0-9]+|A)(?:\.[0-9]+)*|[0-9]+)/g)];
      for (const [, type, number] of labels) {
        requireValue(resultKeys.has(`${type}:${number}`), `${node.id}: unknown Galbraith locator ${type} ${number}`);
      }
    }
  }
}

for (const edge of corpus.edges ?? []) {
  requireValue(nodeIds.has(edge.from), `${edge.id}: unknown from node ${edge.from}`);
  requireValue(nodeIds.has(edge.to), `${edge.id}: unknown to node ${edge.to}`);
  requireValue(edge.from !== edge.to, `${edge.id}: self edge`);
  requireValue(allowedRelations.has(edge.relation), `${edge.id}: unknown relation ${edge.relation}`);
  checkSourceRefs(edge.id, edge.source_refs);
}

for (const target of corpus.research_targets ?? []) {
  requireValue(["working", "ready", "blocked", "open"].includes(target.status), `${target.id}: unknown target status ${target.status}`);
  requireValue(Number.isInteger(target.priority) && target.priority >= 1, `${target.id}: invalid priority`);
  for (const key of ["label", "objective", "acceptance", "next_step"]) {
    requireValue(typeof target[key] === "string" && target[key].length > 0, `${target.id}: missing ${key}`);
  }
  for (const dep of target.depends_on ?? []) requireValue(nodeIds.has(dep), `${target.id}: unknown dependency ${dep}`);
  checkSourceRefs(target.id, target.source_refs);
}

const sourceVerification = new Map((corpus.sources ?? []).map(source => [source.id, source.verification]));
for (const node of corpus.nodes ?? []) {
  if (node.status === "proved_external") {
    requireValue(node.source_refs.some(ref => sourceVerification.get(ref) === "primary_read"), `${node.id}: externally proved claim lacks a primary-read source`);
  }
}

if (errors.length > 0) {
  for (const error of errors) process.stderr.write(`ERROR ${error}\n`);
  process.stderr.write(`FAIL ${errors.length} corpus validation error(s)\n`);
  process.exit(1);
}

process.stdout.write(`PASS ${corpus.nodes.length} nodes, ${corpus.edges.length} edges, ${corpus.sources.length} sources, ${corpus.research_targets.length} research targets\n`);
