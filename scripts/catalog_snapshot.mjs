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
