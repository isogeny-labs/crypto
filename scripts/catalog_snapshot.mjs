import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const CATALOG_FILES = new Set([
  "docs/curves/registry.json",
  "docs/curves/covers.json",
  "docs/curves/cover-links.yaml",
]);

// The transfer assessment describes a dated catalog_commit, independently of
// the live geometry catalog. Never substitute working-tree data on a Git error.
export function catalogSnapshotBytes(root, relativePath, catalogCommit) {
  if (!CATALOG_FILES.has(relativePath)) {
    return readFileSync(resolve(root, relativePath));
  }
  return execFileSync("git", ["show", `${catalogCommit}:${relativePath}`], {
    cwd: root,
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
}

// A dated negative finding cites this historical formula inventory, not the
// newly refreshed live catalog. Never permit a version pin on implementation
// evidence, and never substitute live bytes if the pinned object is unavailable.
export function legacyFormulaSnapshotBytes(root, relativePath, sourceCommit) {
  if (relativePath !== "docs/curves/jacobian-transfers.json") {
    throw new Error("only the legacy formula inventory may use source_commit");
  }
  if (!/^[0-9a-f]{40}$/u.test(sourceCommit)) {
    throw new Error("source_commit must be an immutable full Git commit");
  }
  return execFileSync("git", ["show", `${sourceCommit}:${relativePath}`], {
    cwd: root,
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
}
