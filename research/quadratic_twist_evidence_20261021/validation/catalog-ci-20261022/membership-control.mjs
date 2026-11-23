// Metadata-only negative control: remove one exact map membership.
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";

const root = fileURLToPath(new URL("../../../../", import.meta.url));
const fixture = fs.mkdtempSync(path.join(os.tmpdir(), "twist-membership-control-"));
const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
try {
  const assessmentPath = path.join("docs", "curves", "cover-transfer-assessments.json");
  const positiveBytes = fs.readFileSync(path.join(root, assessmentPath));
  const assessment = JSON.parse(positiveBytes.toString("utf8"));
  // Physical copies keep the verifier's realpath containment checks active and
  // work across filesystems. Only the assessment's contents are altered.
  for (const artifact of assessment.source_artifacts) {
    const source = path.resolve(root, artifact.path);
    if (!source.startsWith(root) || artifact.path === assessmentPath) {
      throw new Error("unsafe or self-referential source artifact");
    }
    const destination = path.join(fixture, artifact.path);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.copyFileSync(source, destination);
  }
  fs.symlinkSync(path.join(root, ".git"), path.join(fixture, ".git"));
  const selected = assessment.memberships.map_base.find(
    (group) => group.profile_id === "map.odd-extension-quadratic.direct",
  );
  if (selected == null || selected.members.length !== 2287) {
    throw new Error("positive fixture has an unexpected extension profile");
  }
  const removed = selected.members.pop();
  const alteredBytes = JSON.stringify(assessment);
  fs.writeFileSync(path.join(fixture, assessmentPath), alteredBytes);

  let receipt;
  try {
    execFileSync(process.execPath, [
      path.join(root, "research/cover_transfer_assessment_20261013/verify_assessment.mjs"),
      fixture,
    ], { cwd: root, encoding: "utf8", maxBuffer: 20_000_000 });
    throw new Error("verifier accepted an assessment missing an exact map member");
  } catch (error) {
    if (error.status !== 1 || typeof error.stdout !== "string") throw error;
    receipt = JSON.parse(error.stdout);
    if (!receipt.problems.some((problem) => /membership|map coverage|base profile/u.test(problem))) {
      throw new Error("verifier failed without detecting the removed membership: " +
        JSON.stringify(receipt.problems));
    }
  }
  console.log(JSON.stringify({
    status: "rejected_missing_map_membership",
    removed_map_uid: removed,
    positive_assessment_sha256: digest(positiveBytes),
    altered_assessment_sha256: digest(alteredBytes),
    verifier_sha256: digest(fs.readFileSync(path.join(root,
      "research/cover_transfer_assessment_20261013/verify_assessment.mjs"))),
    verifier_exit: 1,
    problem_count: receipt.problem_count,
    problems: receipt.problems,
  }, null, 2));
} finally {
  // This control owns every file in this temporary fixture.
  fs.rmSync(fixture, { recursive: true, force: true });
}
