import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const generator = "scripts/build_cover_transfer_assessments.mjs";
const inputs = {
  registry: "docs/curves/registry.json",
  covers: "docs/curves/covers.json",
  graph: "docs/curves/cover-links.yaml",
};
const originals = Object.fromEntries(Object.entries(inputs).map(([key, path]) =>
  [key, JSON.parse(readFileSync(join(root, path), "utf8"))]));

// Invalid fixtures must fail at the catalog join, before reading unrelated source artifacts.
const cases = [
  ["missing graph model", data => data.graph.curves.pop(), /graph rows: not the exact catalog membership/u],
  ["duplicate graph model", data => data.graph.curves.push(data.graph.curves[0]), /duplicate graph model/u],
  ["missing cover model", data => data.covers.curves.pop(), /cover rows: not the exact catalog membership/u],
  ["duplicate registry model", data => data.registry.curves.push(data.registry.curves[0]), /duplicate registry model/u],
  ["incorrect representation binding", data => {
    data.graph.curves.find(row => row.representations.length).representations[0].curve_uid = "wrong-curve";
  }, /registry representation binding mismatch/u],
  ["unreferenced map", data => { data.graph.maps["unreferenced-map"] = {}; }, /typed map bindings: not the exact catalog membership/u],
];
for (const [name, mutate, expected] of cases) {
  test(name, () => {
    const fixture = mkdtempSync(join(tmpdir(), "cover-membership-"));
    try {
      mkdirSync(join(fixture, "scripts"));
      mkdirSync(join(fixture, "docs/curves"), { recursive: true });
      copyFileSync(join(root, generator), join(fixture, generator));
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
