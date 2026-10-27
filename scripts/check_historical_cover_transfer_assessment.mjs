#!/usr/bin/env node

// Replay the immutable 2,572-model study from its complete original revision.
// The live catalog and its current assessment are checked separately.
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const baseline = "a53a7e2bfa20080a424b9a2484a16932b6e3c116";
const assessmentPath = "docs/curves/cover-transfer-assessments.json";
const assessment = JSON.parse(execFileSync(
  "git", ["show", `${baseline}:${assessmentPath}`],
  { cwd: root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
));
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
