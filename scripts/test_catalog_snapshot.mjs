import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { catalogSnapshotBytes } from "./catalog_snapshot.mjs";

function fixture(t) {
  const root = mkdtempSync(join(tmpdir(), "catalog-snapshot-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const git = (...args) => execFileSync("git", args, { cwd: root, stdio: ["ignore", "pipe", "pipe"] });
  git("init");
  mkdirSync(join(root, "docs/curves"), { recursive: true });
  for (const name of ["registry.json", "covers.json", "cover-links.yaml"]) {
    writeFileSync(join(root, "docs/curves", name), `frozen ${name}\n`);
  }
  git("add", ".");
  git("-c", "user.name=Catalog fixture", "-c", "user.email=catalog@example.invalid",
      "-c", "commit.gpgsign=false", "commit", "-m", "Freeze fixture");
  const commit = git("rev-parse", "HEAD").toString().trim();
  for (const name of ["registry.json", "covers.json", "cover-links.yaml"]) {
    writeFileSync(join(root, "docs/curves", name), `new live ${name}\n`);
  }
  return { root, commit };
}

test("all catalog inputs come from the declared commit after the live catalog grows", (t) => {
  const { root, commit } = fixture(t);
  for (const name of ["registry.json", "covers.json", "cover-links.yaml"]) {
    assert.equal(catalogSnapshotBytes(root, `docs/curves/${name}`, commit).toString(), `frozen ${name}\n`);
  }
});

test("ordinary source artifacts still bind the working tree", (t) => {
  const { root, commit } = fixture(t);
  writeFileSync(join(root, "source.txt"), "current source\n");
  assert.equal(catalogSnapshotBytes(root, "source.txt", commit).toString(), "current source\n");
});

test("an unavailable snapshot fails instead of falling back to live input", (t) => {
  const { root } = fixture(t);
  assert.throws(() => catalogSnapshotBytes(root, "docs/curves/registry.json", "0".repeat(40)));
});
