// Metadata comparison only. Preserve integer tokens beyond JavaScript's safe range.
import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../../../../", import.meta.url));
const priorHead = "3e442fd3310d192495e5277a278aa2a0c0aab605";
const priorCatalog = "a11984a573441d5bf0d60ab1d26cfc34e71badf5";
const ciMerge = "ca2236699e36cd8ef391e208059d03d6933020ad";
const ciBase = "412b60289db97ad1ba13a2bffe1120fadc7fb7d6";
const declaredBase = "f8e72869e284a99f16d17b545f0d7666ef943c35";
const git = (...args) => execFileSync("git", args, { cwd: root, maxBuffer: 32 * 1024 * 1024 });
const read = (name) => fs.readFileSync(path.join(root, name));
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
const parse = (bytes) => JSON.parse(bytes.toString(),
  (key, value, context) => typeof value === "number" ? JSON.rawJSON(context.source) : value);
function canonical(value) {
  if (JSON.isRawJSON(value)) return value.rawJSON;
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value !== null && typeof value === "object") return `{${Object.keys(value).sort()
    .map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  return JSON.stringify(value);
}
const exact = (before, after, name) => assert.equal(canonical(after), canonical(before), name);
const text = (value) => JSON.isRawJSON(value) ? value.rawJSON : String(value);
const registryPath = "docs/curves/registry.json";
const beforeRegistryBytes = git("show", `${priorHead}:${registryPath}`);
const registryBytes = read(registryPath);
const registry = parse(registryBytes);
exact(parse(beforeRegistryBytes), registry, "registry values changed during formatting");
assert.equal(registry.curves.length, 4493);
const aliasPath = "src/cryptanalysis/curve_aliases.json";
exact(parse(git("show", `${priorHead}:${aliasPath}`)), parse(read(aliasPath)), "alias mappings changed");
const catalogInputs = {};
for (const name of [registryPath, "docs/curves/covers.json", "docs/curves/cover-links.yaml"]) {
  const predecessor = git("show", `${priorCatalog}:${name}`);
  const current = read(name);
  catalogInputs[name] = { predecessor_sha256: hash(predecessor), current_sha256: hash(current) };
  if (name !== registryPath) {
    const before = parse(predecessor), after = parse(current);
    assert.equal(after.registry_sha256, hash(registryBytes));
    delete before.registry_sha256;
    delete after.registry_sha256;
    exact(before, after, `${name}: geometry or identity changed`);
  }
}
git("merge-base", "--is-ancestor", priorCatalog, priorHead);

const boardPath = "docs/ic/leaderboard.json";
const beforeBoard = parse(git("show", `${priorHead}:${boardPath}`));
const afterBoard = parse(read(boardPath));
const oldRoster = new Map(beforeBoard.roster.map((row) => [row.slug, row]));
assert.equal(afterBoard.roster.length, registry.curves.length);
let correctedFields = 0;
for (const curve of registry.curves) {
  const row = afterBoard.roster.find((item) => item.slug === curve.slug);
  assert(row, `missing roster row ${curve.slug}`);
  if (curve.family === "extension") {
    assert.equal(row.field, `GF(${curve.params.p}^${text(curve.params.k)})`);
    correctedFields += 1;
  }
  const old = oldRoster.get(curve.slug);
  if (old) {
    const lhs = { ...old }, rhs = { ...row };
    if (curve.family === "extension") { delete lhs.field; delete rhs.field; }
    exact(lhs, rhs, `existing roster values changed for ${curve.slug}`);
  }
}
assert.equal(correctedFields, 2287);
const oldRosterCount = beforeBoard.roster.length;
delete beforeBoard.roster; delete afterBoard.roster;
delete beforeBoard.sources.registry.sha256; delete afterBoard.sources.registry.sha256;
exact(beforeBoard, afterBoard, "measured leaderboard, protocol or non-registry source changed");

const browserPath = "docs/browser/data.json";
const beforeBrowserBytes = git("show", `${priorHead}:${browserPath}`);
const browserBytes = read(browserPath);
const beforeBrowser = parse(beforeBrowserBytes), afterBrowser = parse(browserBytes);
const oldBrowserCount = beforeBrowser.curves.length;
const browserBySlug = new Map(afterBrowser.curves.map((row) => [row.slug, row]));
assert.equal(browserBySlug.size, registry.curves.length);
for (const row of beforeBrowser.curves) {
  exact(row, browserBySlug.get(row.slug), `existing browser row changed: ${row.slug}`);
}
for (const curve of registry.curves) {
  const row = browserBySlug.get(curve.slug);
  assert(row, `missing browser identity ${curve.slug}`);
  assert.equal(row.model_json, curve.model_json);
  assert.equal(row.icv1, curve.icv1);
  exact(row.curve_uid, curve.representations.map((rep) => rep.curve_uid),
    `browser EC1 values changed: ${curve.slug}`);
  if (curve.family === "extension") {
    assert.equal(row.a, `[${curve.params.a.map(text).join(", ")}]`);
    assert.equal(row.b, `[${curve.params.b.map(text).join(", ")}]`);
    assert.equal(row.field.p, curve.params.p);
    assert.equal(text(row.field.degree), text(curve.params.k));
  }
}
for (const name of [registryPath, "docs/curves/covers.json", "docs/curves/cover-links.yaml", boardPath]) {
  assert.equal(afterBrowser.sources[name], hash(read(name)));
  delete beforeBrowser.sources[name]; delete afterBrowser.sources[name];
}
delete beforeBrowser.curves; delete afterBrowser.curves;
delete beforeBrowser.counts.curves; delete afterBrowser.counts.curves;
exact(beforeBrowser, afterBrowser, "browser measurements, sessions, candidates or other frozen metadata changed");

const inheritedPaths = ["research/sea_point_count_20261016/lean", "tools/lean",
  ".github/workflows/lean-proof-gate.yml"];
const inheritedSources = {};
for (const name of inheritedPaths) {
  const snapshots = [declaredBase, ciBase, priorHead, ciMerge].map((ref) =>
    git("ls-tree", ref, "--", name).toString().trim());
  assert(snapshots[0], `missing inherited source ${name}`);
  assert(snapshots.every((snapshot) => snapshot === snapshots[0]), `inherited source differs: ${name}`);
  inheritedSources[name] = snapshots[0];
}
const study = "research/quadratic_twist_evidence_20261021";
for (const graph of ["typed_correspondence", "typed_transfer_context"]) {
  for (const format of ["dot", "svg", "png"]) {
    assert.equal(hash(read(`${study}/reports/metadata-20261022T0500Z/${graph}.${format}`)),
      hash(read(`${study}/reports/catalog-20261022/${graph}.${format}`)),
      `unchanged diagram differs: ${graph}.${format}`);
  }
}
const frozenPaths = ["historical", "certificates", "lean", "native/runs", "reports", "audits"]
  .map((suffix) => `${study}/${suffix}`);
assert.equal(git("diff", "--name-only", "--diff-filter=MD", priorHead, "--", ...frozenPaths)
  .toString().trim(), "", "frozen study evidence or earlier reports changed");
console.log(JSON.stringify({
  status: "passed", prior_head: priorHead, predecessor_catalog_commit: priorCatalog,
  registry_models_unchanged: registry.curves.length,
  registry_before_sha256: hash(beforeRegistryBytes), registry_after_sha256: hash(registryBytes),
  alias_mappings_unchanged: true, all_geometry_and_identity_values_unchanged: true,
  integer_tokens_preserved_exactly: true,
  roster_before: oldRosterCount, roster_after: registry.curves.length,
  odd_extension_field_labels: correctedFields,
  measured_leaderboard_and_protocol_unchanged: true,
  browser_before: oldBrowserCount, browser_after: registry.curves.length,
  browser_before_sha256: hash(beforeBrowserBytes), browser_after_sha256: hash(browserBytes),
  existing_browser_rows_and_all_measurement_indexes_unchanged: true,
  frozen_evidence_and_earlier_reports_unchanged: true,
  diagram_sources_and_svg_png_renders_unchanged: true,
  catalog_inputs: catalogInputs,
  inherited_lean: { declared_base: declaredBase, ci_base: ciBase, head: priorHead,
    ci_merge: ciMerge, source_objects_identical: inheritedSources },
}, null, 2));
