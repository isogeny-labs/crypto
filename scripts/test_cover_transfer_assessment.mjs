import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const generator = "scripts/build_cover_transfer_assessments.mjs";
const catalogCommit = "728304718cb0ab6df70da620a908a5f76f175293";
const inputs = {
  registry: "docs/curves/registry.json",
  covers: "docs/curves/covers.json",
  graph: "docs/curves/cover-links.yaml",
};
const originals = Object.fromEntries(Object.entries(inputs).map(([key, path]) => [
  key,
  JSON.parse(execFileSync("git", ["show", `${catalogCommit}:${path}`], {
    cwd: root,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  })),
]));

function retargetModel(data, predicate, mutate) {
  const registryRow = data.registry.curves.find((row) => predicate(JSON.parse(row.model_json)));
  assert.notEqual(registryRow, undefined, "negative fixture model not found");
  const oldModelHash = createHash("sha256").update(registryRow.model_json).digest("hex");
  const model = JSON.parse(registryRow.model_json);
  mutate(model);
  const modelJson = JSON.stringify(model);
  const newModelHash = createHash("sha256").update(modelJson).digest("hex");
  registryRow.model_json = modelJson;

  const coverRow = data.covers.curves.find((row) => row.model_sha256 === oldModelHash);
  const graphRow = data.graph.curves.find((row) => row.model_sha256 === oldModelHash);
  assert.notEqual(coverRow, undefined, "negative fixture cover row not found");
  assert.notEqual(graphRow, undefined, "negative fixture graph row not found");
  coverRow.model_sha256 = newModelHash;
  graphRow.model_sha256 = newModelHash;
  graphRow.model_json = modelJson;
  const mapUid = graphRow.cover_links[0].map_uid;
  data.graph.maps[mapUid].record.target_model_sha256 = newModelHash;
  const coverUid = graphRow.cover_links[0].cover_uid;
  data.graph.covers[coverUid].record.field = {
    label: model.field,
    p: String(model.p),
  };
}

const directShort = (model) =>
  model.form === "y^2=x^3+a*x+b" && model.field.startsWith("fp-");
const widePilot = (model) =>
  directShort(model) && model.p === "5902977611310272157103";
const normalizedPrime = (model) =>
  model.field.startsWith("fp-") && model.form !== "y^2=x^3+a*x+b";

// Join mutations must fail at the catalog join; proof-boundary mutations must
// fail at their dedicated classifier/count controls. Every case must stop
// before reading unrelated source artifacts.
const cases = [
  ["missing graph model", data => data.graph.curves.pop(), /graph rows: not the exact catalog membership/u],
  ["duplicate graph model", data => data.graph.curves.push(data.graph.curves[0]), /duplicate graph model/u],
  ["missing cover model", data => data.covers.curves.pop(), /cover rows: not the exact catalog membership/u],
  ["duplicate registry model", data => data.registry.curves.push(data.registry.curves[0]), /duplicate registry model/u],
  ["incorrect representation binding", data => {
    data.graph.curves.find(row => row.representations.length).representations[0].curve_uid = "wrong-curve";
  }, /registry representation binding mismatch/u],
  ["unreferenced map", data => { data.graph.maps["unreferenced-map"] = {}; }, /typed map bindings: not the exact catalog membership/u],
  ["composite direct-prime modulus", data => {
    retargetModel(data, directShort, model => {
      model.p = "15";
      model.field = "fp-15";
    });
  }, /composite prime-field modulus/u],
  ["psi12 pseudoprime rejected by base 41", data => {
    retargetModel(data, widePilot, model => {
      model.p = "318665857834031151167461";
      model.field = `fp-${model.p}`;
    });
  }, /composite prime-field modulus/u],
  ["all-passing value above psi13 leaves proved profile", data => {
    for (let index = 0; index < 2; index += 1) {
      retargetModel(data, widePilot, model => {
        model.p = "3317044064679887385962123";
        model.field = `fp-${model.p}`;
      });
    }
  }, /map\.odd-prime-quadratic\.direct\.proved-psi13: expected 2, got 0/u],
  ["normalized row cannot enter proved direct backend", data => {
    retargetModel(data, normalizedPrime, model => {
      model.p = "11";
      model.field = "fp-11";
    });
  }, /normalized profile proof boundary changed/u],
];
for (const [name, mutate, expected] of cases) {
  test(name, () => {
    const fixture = mkdtempSync(join(tmpdir(), "cover-membership-"));
    try {
      mkdirSync(join(fixture, "scripts"));
      mkdirSync(join(fixture, "docs/curves"), { recursive: true });
      copyFileSync(join(root, generator), join(fixture, generator));
      // The production helper deliberately reads the immutable catalog commit
      // with `git show`.  This temporary tree is not a Git repository, so give
      // the negative fixture an explicit adapter that reads only the mutated
      // catalog files below.  If the generator reaches the legacy formula
      // inventory, the intended early membership failure did not occur.
      writeFileSync(join(fixture, "scripts/catalog_snapshot.mjs"), `
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

export function catalogSnapshotBytes(root, relativePath) {
  return readFileSync(resolve(root, relativePath));
}

export function legacyFormulaSnapshotBytes() {
  throw new Error("negative catalog fixture reached legacy formula input");
}
`);
      const data = structuredClone(originals);
      mutate(data);
      for (const [key, path] of Object.entries(inputs)) {
        writeFileSync(join(fixture, path), JSON.stringify(data[key]));
      }
      const result = spawnSync(process.execPath, [join(fixture, generator), "--write"], {
        encoding: "utf8", timeout: 30_000,
      });
      assert.equal(result.error, undefined);
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, expected);
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
}
