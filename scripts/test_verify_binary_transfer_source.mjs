import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import {
  copyFileSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

import {
  SOURCE_PATHS,
  framedSourceDigest,
  parseCli,
  producerPathsFromSource,
  run,
  verifyBinaryTransferSource,
} from "./verify_binary_transfer_source.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const verifier = join(root, "scripts/verify_binary_transfer_source.mjs");
const corpusPath = join(root, "docs/curves/binary-subgroup-transfer-certificates.json");
const receiptPath = join(
  root,
  "research/curve_transfer_infrastructure_20261118/binary-transfer-catalog-source-replay-5c6e43a6.json",
);

function corpus() {
  return JSON.parse(readFileSync(corpusPath, "utf8"));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function git(cwd, args) {
  return execFileSync("git", args, {
    cwd,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  }).trim();
}

function initializeSourceRepository(directory) {
  git(directory, ["init", "--quiet"]);
  git(directory, ["config", "user.name", "Binary Transfer Test"]);
  git(directory, ["config", "user.email", "binary-transfer-test@example.invalid"]);
  for (const path of SOURCE_PATHS) {
    mkdirSync(dirname(join(directory, path)), { recursive: true });
    copyFileSync(join(root, path), join(directory, path));
  }
  git(directory, ["add", "."]);
  git(directory, ["commit", "--quiet", "-m", "complete source closure"]);
  return git(directory, ["rev-parse", "HEAD"]);
}

test("framed digest binds path order and content bytes", () => {
  const entries = [
    { path: "a", contents: Buffer.from([0, 1, 2]) },
    { path: "β", contents: Buffer.from("value", "utf8") },
  ];
  assert.equal(
    framedSourceDigest(entries),
    "96250a886e5b00e8577d74f4c34217c8036b15aa03203a2ee447db06663a6583",
  );
  assert.notEqual(framedSourceDigest(entries), framedSourceDigest(entries.toReversed()));
  assert.notEqual(
    framedSourceDigest(entries),
    framedSourceDigest([{ ...entries[0], contents: Buffer.from([0, 1, 3]) }, entries[1]]),
  );
});

test("bounded producer parser recovers the reviewed exact closure", () => {
  const source = readFileSync(join(root, "src/bin/hyperelliptic_cover.rs"), "utf8");
  const start = source.indexOf("fn transfer_catalog_producer_source_digest() -> String {");
  const end = source.indexOf("\nfn model_error", start);
  assert.ok(start >= 0 && end > start);
  const body = source.slice(start, end);
  const replaceBody = mutated => `${source.slice(0, start)}${mutated}${source.slice(end)}`;
  assert.deepEqual(producerPathsFromSource(source), SOURCE_PATHS);
  assert.throws(
    () => producerPathsFromSource(replaceBody(body.replace(
      '("Cargo.lock", include_str!("../../Cargo.lock")),',
      "",
    ))),
    /closure has 20 paths/u,
  );
  const reordered = [...SOURCE_PATHS];
  [reordered[0], reordered[1]] = [reordered[1], reordered[0]];
  const reorderedBody = body
    .replace(`"${SOURCE_PATHS[0]}"`, '"temporary-logical-path"')
    .replace('include_str!("curve_cover_check/checker.rs")', 'include_str!("temporary-target")')
    .replace(`"${SOURCE_PATHS[1]}"`, `"${SOURCE_PATHS[0]}"`)
    .replace('include_str!("curve_cover_check/links.rs")', 'include_str!("curve_cover_check/checker.rs")')
    .replace('"temporary-logical-path"', `"${SOURCE_PATHS[1]}"`)
    .replace('include_str!("temporary-target")', 'include_str!("curve_cover_check/links.rs")');
  const parsed = producerPathsFromSource(replaceBody(reorderedBody));
  assert.deepEqual(parsed.slice(0, 2), reordered.slice(0, 2));
  assert.notDeepEqual(parsed, SOURCE_PATHS);
  assert.throws(
    () => producerPathsFromSource(replaceBody(body.replace(
      'include_str!("curve_cover_check/checker.rs")',
      'include_str!("curve_cover_check/links.rs")',
    ))),
    /include_str target .* does not normalize to the logical repository path/u,
  );
});

test("CLI parser rejects missing, duplicate, and unknown arguments", () => {
  assert.deepEqual(parseCli(["--help"]), { help: true });
  assert.throws(() => parseCli([]), /--input is required/u);
  assert.throws(
    () => parseCli(["--input", "a", "--repo-root", ".", "--input", "b"]),
    /duplicate argument --input/u,
  );
  assert.throws(() => parseCli(["--wat", "x"]), /unknown argument/u);
  assert.throws(() => parseCli(["--input"]), /requires a value/u);
});

test("exact recorded Git tree reproduces the checked-in receipt", () => {
  const actual = verifyBinaryTransferSource({
    input: corpusPath,
    repoRoot: root,
    verifierPath: verifier,
  });
  assert.deepEqual(actual, JSON.parse(readFileSync(receiptPath, "utf8")));
});

test("source digest and producer commit mutations fail closed", () => {
  const directory = mkdtempSync(join(tmpdir(), "binary-transfer-source-mutation-"));
  try {
    const mutatedDigest = corpus();
    mutatedDigest.producer.source_sha256 = `${mutatedDigest.producer.source_sha256.slice(0, -1)}0`;
    const digestPath = join(directory, "digest.json");
    writeJson(digestPath, mutatedDigest);
    assert.throws(
      () => verifyBinaryTransferSource({ input: digestPath, repoRoot: root, verifierPath: verifier }),
      /source digest differs from the recorded Git tree/u,
    );

    const staleCommit = corpus();
    staleCommit.producer.git_commit = "e0ae0fe5d09c0492a8b50600e88d0b9da68afef1";
    const stalePath = join(directory, "stale.json");
    writeJson(stalePath, staleCommit);
    assert.throws(
      () => verifyBinaryTransferSource({ input: stalePath, repoRoot: root, verifierPath: verifier }),
      /source digest differs from the recorded Git tree/u,
    );

    const malformed = corpus();
    malformed.producer.git_commit = malformed.producer.git_commit.toUpperCase();
    const malformedPath = join(directory, "malformed.json");
    writeJson(malformedPath, malformed);
    assert.throws(
      () => verifyBinaryTransferSource({ input: malformedPath, repoRoot: root, verifierPath: verifier }),
      /40 lowercase hexadecimal digits/u,
    );

    const invalidMetadata = [
      ["schema", value => { value.schema_version = "wrong/v1"; }, /unsupported certificate corpus schema/u],
      ["attestation", value => { value.producer.attestation = "verified"; }, /caller_supplied_unverified/u],
      ["uppercase digest", value => {
        value.producer.source_sha256 = value.producer.source_sha256.toUpperCase();
      }, /64 lowercase hexadecimal digits/u],
    ];
    for (const [name, mutate, expected] of invalidMetadata) {
      const value = corpus();
      mutate(value);
      const path = join(directory, `${name.replaceAll(" ", "-")}.json`);
      writeJson(path, value);
      assert.throws(
        () => verifyBinaryTransferSource({ input: path, repoRoot: root, verifierPath: verifier }),
        expected,
      );
    }
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("missing and non-UTF-8 recorded blobs fail closed", () => {
  const directory = mkdtempSync(join(tmpdir(), "binary-transfer-source-git-"));
  try {
    initializeSourceRepository(directory);

    writeFileSync(join(directory, "Cargo.lock"), Buffer.from([0xff, 0xfe, 0xfd]));
    git(directory, ["add", "Cargo.lock"]);
    git(directory, ["commit", "--quiet", "-m", "non utf8 source"]);
    const nonUtf = corpus();
    nonUtf.producer.git_commit = git(directory, ["rev-parse", "HEAD"]);
    nonUtf.producer.source_sha256 = "0".repeat(64);
    const nonUtfPath = join(directory, "non-utf.json");
    writeJson(nonUtfPath, nonUtf);
    assert.throws(
      () => verifyBinaryTransferSource({ input: nonUtfPath, repoRoot: directory, verifierPath: verifier }),
      /Cargo\.lock is not valid UTF-8/u,
    );

    copyFileSync(join(root, "Cargo.lock"), join(directory, "Cargo.lock"));
    rmSync(join(directory, "Cargo.toml"));
    git(directory, ["add", "-A"]);
    git(directory, ["commit", "--quiet", "-m", "missing source"]);
    const missing = corpus();
    missing.producer.git_commit = git(directory, ["rev-parse", "HEAD"]);
    missing.producer.source_sha256 = "0".repeat(64);
    const missingPath = join(directory, "missing.json");
    writeJson(missingPath, missing);
    assert.throws(
      () => verifyBinaryTransferSource({ input: missingPath, repoRoot: directory, verifierPath: verifier }),
      /read [0-9a-f]{40}:Cargo\.toml failed/u,
    );
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("Git replace refs cannot substitute producer blobs", () => {
  const directory = mkdtempSync(join(tmpdir(), "binary-transfer-source-replace-"));
  try {
    const original = initializeSourceRepository(directory);
    const entries = SOURCE_PATHS.map(path => ({
      path,
      contents: readFileSync(join(directory, path)),
    }));
    const expectedDigest = framedSourceDigest(entries);
    const originalLock = readFileSync(join(directory, "Cargo.lock"), "utf8");
    writeFileSync(join(directory, "Cargo.lock"), `${originalLock}\n# replacement object\n`);
    git(directory, ["add", "Cargo.lock"]);
    git(directory, ["commit", "--quiet", "-m", "replacement source tree"]);
    const replacement = git(directory, ["rev-parse", "HEAD"]);
    git(directory, ["replace", original, replacement]);
    assert.match(git(directory, ["show", `${original}:Cargo.lock`]), /replacement object/u);
    assert.doesNotMatch(
      git(directory, ["--no-replace-objects", "show", `${original}:Cargo.lock`]),
      /replacement object/u,
    );

    const value = corpus();
    value.producer.git_commit = original;
    value.producer.source_sha256 = expectedDigest;
    const input = join(directory, "replace-ref.json");
    writeJson(input, value);
    const receipt = verifyBinaryTransferSource({
      input,
      repoRoot: directory,
      verifierPath: verifier,
    });
    assert.equal(receipt.producer_commit, original);
    assert.equal(receipt.source_sha256, expectedDigest);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("bounded input, atomic output, and input-output alias controls", () => {
  const directory = mkdtempSync(join(tmpdir(), "binary-transfer-source-output-"));
  try {
    const oversized = join(directory, "oversized.json");
    writeFileSync(oversized, Buffer.alloc(4 * 1024 * 1024 + 1));
    assert.throws(
      () => verifyBinaryTransferSource({ input: oversized, repoRoot: root, verifierPath: verifier }),
      /4194304-byte limit/u,
    );

    const output = join(directory, "receipt.json");
    assert.equal(run([
      "--input", corpusPath,
      "--repo-root", root,
      "--output", output,
    ]), 0);
    assert.deepEqual(
      JSON.parse(readFileSync(output, "utf8")),
      JSON.parse(readFileSync(receiptPath, "utf8")),
    );
    assert.throws(() => run([
      "--input", corpusPath,
      "--repo-root", root,
      "--output", corpusPath,
    ]), /must not alias/u);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
