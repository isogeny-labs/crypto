#!/usr/bin/env node

// The v2 assessment is an immutable 2,572-model study. Replay it against the
// complete input revision it assessed, even after the live registry grows.
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const baseline = "a53a7e2bfa20080a424b9a2484a16932b6e3c116";
const assessmentPath = "docs/curves/cover-transfer-assessments.json";
const assessment = JSON.parse(readFileSync(join(root, assessmentPath), "utf8"));
const paths = new Set([
  assessmentPath,
  "scripts/build_cover_transfer_assessments.mjs",
  "research/cover_transfer_assessment_20261013/verify_assessment.mjs",
  ...assessment.source_artifacts.map((artifact) => artifact.path),
  ...Object.values(assessment.report).map((artifact) => artifact.path),
]);
const temp = mkdtempSync(join(tmpdir(), "cover-transfer-replay-"));
let added = false;

function git(...args) {
  return execFileSync("git", args, { cwd: root, stdio: "pipe" });
}

try {
  git("cat-file", "-e", `${baseline}^{commit}`);
  const originalBlob = git("rev-parse", `${baseline}:${assessmentPath}`).toString().trim();
  const publishedBlob = git("hash-object", assessmentPath).toString().trim();
  if (originalBlob !== publishedBlob) {
    throw new Error("published v2 assessment differs from its pinned baseline");
  }
  git("worktree", "add", "--detach", "--no-checkout", temp, baseline);
  added = true;
  git("-C", temp, "sparse-checkout", "set", "--no-cone", ...[...paths].sort().map((path) => `/${path}`));
  git("-C", temp, "reset", "--hard", baseline);
  for (const command of [
    ["node", "scripts/build_cover_transfer_assessments.mjs", "--check"],
    ["node", "research/cover_transfer_assessment_20261013/verify_assessment.mjs"],
  ]) {
    execFileSync(command[0], command.slice(1), { cwd: temp, stdio: "inherit" });
  }
  console.log(`historical cover-transfer assessment replayed at ${baseline}`);
} finally {
  if (added) git("worktree", "remove", "--force", temp);
  rmSync(temp, { recursive: true, force: true });
}
