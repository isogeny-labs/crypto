#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import {
  existsSync,
  lstatSync,
  readFileSync,
  realpathSync,
  renameSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { basename, dirname, posix, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export const CORPUS_SCHEMA = "binary-subgroup-transfer-certificates/v1";
export const RECEIPT_SCHEMA = "binary-subgroup-transfer-source-verification/v1";
export const RECEIPT_STATUS = "producer_source_digest_bound_to_recorded_git_tree";
export const SOURCE_DIGEST_ENCODING =
  "u64be(path_utf8_bytes)||path_utf8||u64be(content_utf8_bytes)||content_utf8, repeated in source-path order";
export const VERIFIER_PATH = "scripts/verify_binary_transfer_source.mjs";
export const CLAIM_BOUNDARY =
  "this repository-mode check binds the binary corpus's saved producer source digest and reviewed 21-path closure to blobs in the recorded Git commit; it does not attest that an executable was built from that commit, independently verify cover, Jacobian, transfer, or subgroup arithmetic, provide signed PCC standing, or establish a solver or computational advantage";

export const SOURCE_PATHS = Object.freeze([
  "src/bin/curve_cover_check/checker.rs",
  "src/bin/curve_cover_check/links.rs",
  "src/bin/curve_cover_check/models.rs",
  "src/bin/hyperelliptic_cover.rs",
  "src/binary_ecc/cover_transfer.rs",
  "src/binary_ecc/curve.rs",
  "src/binary_ecc/f2m.rs",
  "src/binary_ecc/hyperelliptic.rs",
  "src/binary_ecc/mod.rs",
  "src/binary_ecc/operation_tally.rs",
  "src/binary_ecc/poly_f2m.rs",
  "src/cryptanalysis/ecc2k130_guard.rs",
  "src/cryptanalysis/group_op_count.rs",
  "src/prime_hyperelliptic/fp2.rs",
  "src/prime_hyperelliptic/mod.rs",
  "src/prime_hyperelliptic/primality.rs",
  "src/hash/sha256.rs",
  "src/utils/mod.rs",
  "docs/curves/binary-subgroup-transfer-certificates.schema.json",
  "Cargo.toml",
  "Cargo.lock",
]);

const PRODUCER_SOURCE_PATH = "src/bin/hyperelliptic_cover.rs";
const PRODUCER_FUNCTION_MARKER = "fn transfer_catalog_producer_source_digest() -> String {";
const PRODUCER_FUNCTION_END_MARKER = "\nfn model_error";
const LOWER_COMMIT = /^[0-9a-f]{40}$/u;
const LOWER_SHA256 = /^[0-9a-f]{64}$/u;
const MAX_CORPUS_BYTES = 4 * 1024 * 1024;
const MAX_SOURCE_BLOB_BYTES = 8 * 1024 * 1024;
const MAX_AGGREGATE_SOURCE_BYTES = 32 * 1024 * 1024;
const UTF8 = new TextDecoder("utf-8", { fatal: true });
const SCRIPT_PATH = fileURLToPath(import.meta.url);
const CLI_OPTIONS = Object.freeze({
  "--input": "input",
  "--repo-root": "repoRoot",
  "--output": "output",
});

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function requireObject(value, context) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`${context} must be a JSON object`);
  }
  return value;
}

function decodeUtf8(bytes, context) {
  try {
    return UTF8.decode(bytes);
  } catch {
    throw new Error(`${context} is not valid UTF-8`);
  }
}

function u64be(value) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error("framed byte length is not a safe nonnegative integer");
  }
  const bytes = Buffer.alloc(8);
  bytes.writeBigUInt64BE(BigInt(value));
  return bytes;
}

export function framedSourceDigest(entries) {
  const digest = createHash("sha256");
  for (const entry of entries) {
    const path = Buffer.from(entry.path, "utf8");
    const contents = Buffer.isBuffer(entry.contents)
      ? entry.contents
      : Buffer.from(entry.contents);
    digest.update(u64be(path.length));
    digest.update(path);
    digest.update(u64be(contents.length));
    digest.update(contents);
  }
  return digest.digest("hex");
}

export function producerPathsFromSource(source) {
  const start = source.indexOf(PRODUCER_FUNCTION_MARKER);
  if (start < 0 || source.indexOf(PRODUCER_FUNCTION_MARKER, start + 1) >= 0) {
    throw new Error("producer source has a missing or duplicate transfer source-digest function");
  }
  const end = source.indexOf(PRODUCER_FUNCTION_END_MARKER, start);
  if (end < 0) {
    throw new Error("producer source-digest function has no bounded end marker");
  }
  const body = source.slice(start, end);
  const tuple = /\(\s*"([^"]+)"\s*,\s*include_str!\(\s*"([^"]+)"\s*\)\s*,?\s*\)/gu;
  const entries = [...body.matchAll(tuple)].map((match) => ({
    path: match[1],
    target: match[2],
  }));
  const paths = entries.map(({ path }) => path);
  if (paths.length !== SOURCE_PATHS.length) {
    throw new Error(
      `producer source-digest closure has ${paths.length} paths; expected ${SOURCE_PATHS.length}`,
    );
  }
  for (const { path, target } of entries) {
    if (target.startsWith("/") || target.includes("\\")) {
      throw new Error(`include_str target for ${path} is not a relative POSIX path`);
    }
    const normalized = posix.normalize(posix.join("src/bin", target));
    if (normalized !== path) {
      throw new Error(
        `include_str target for ${path} does not normalize to the logical repository path (got ${normalized})`,
      );
    }
  }
  return paths;
}

function requireExpectedProducerPaths(actual) {
  if (actual.length !== SOURCE_PATHS.length
      || actual.some((path, index) => path !== SOURCE_PATHS[index])) {
    throw new Error("producer source path list differs from the reviewed verifier closure");
  }
}

function gitBytes(repoRoot, args, context, maxBytes = MAX_SOURCE_BLOB_BYTES) {
  let output;
  try {
    output = execFileSync("git", ["--no-replace-objects", ...args], {
      cwd: repoRoot,
      encoding: null,
      maxBuffer: maxBytes + 64 * 1024,
      stdio: ["ignore", "pipe", "pipe"],
    });
  } catch (error) {
    const detail = Buffer.isBuffer(error?.stderr)
      ? error.stderr.toString("utf8").trim().slice(0, 1000)
      : String(error?.message ?? error).slice(0, 1000);
    throw new Error(`${context} failed${detail ? `: ${detail}` : ""}`);
  }
  if (output.length > maxBytes) {
    throw new Error(`${context} exceeds the ${maxBytes}-byte limit`);
  }
  return output;
}

function validateCorpus(corpus) {
  requireObject(corpus, "certificate corpus");
  if (corpus.schema_version !== CORPUS_SCHEMA) {
    throw new Error("unsupported certificate corpus schema");
  }
  const producer = requireObject(corpus.producer, "certificate corpus producer");
  if (!LOWER_COMMIT.test(producer.git_commit ?? "")) {
    throw new Error("producer.git_commit must be exactly 40 lowercase hexadecimal digits");
  }
  if (!LOWER_SHA256.test(producer.source_sha256 ?? "")) {
    throw new Error("producer.source_sha256 must be exactly 64 lowercase hexadecimal digits");
  }
  if (producer.attestation !== "caller_supplied_unverified") {
    throw new Error("producer.attestation must remain caller_supplied_unverified");
  }
  return producer;
}

function readBounded(path, maxBytes, context) {
  const info = statSync(path);
  if (!info.isFile()) throw new Error(`${context} is not a regular file`);
  if (info.size > maxBytes) throw new Error(`${context} exceeds the ${maxBytes}-byte limit`);
  const bytes = readFileSync(path);
  if (bytes.length > maxBytes) {
    throw new Error(`${context} exceeds the ${maxBytes}-byte limit after read`);
  }
  return bytes;
}

export function verifyBinaryTransferSource({ input, repoRoot, verifierPath = SCRIPT_PATH }) {
  const root = realpathSync(resolve(repoRoot));
  const inputPath = realpathSync(resolve(input));
  const corpusBytes = readBounded(inputPath, MAX_CORPUS_BYTES, "certificate corpus");
  let corpus;
  try {
    corpus = JSON.parse(decodeUtf8(corpusBytes, "certificate corpus"));
  } catch (error) {
    if (error instanceof SyntaxError) throw new Error(`parse certificate corpus: ${error.message}`);
    throw error;
  }
  const producer = validateCorpus(corpus);
  const commit = producer.git_commit;

  gitBytes(root, ["cat-file", "-e", `${commit}^{commit}`], "resolve producer commit", 1024);

  const sourceEntries = [];
  let totalBytes = 0;
  let producerSource = null;
  for (const path of SOURCE_PATHS) {
    const bytes = gitBytes(root, ["show", `${commit}:${path}`], `read ${commit}:${path}`);
    totalBytes += bytes.length;
    if (totalBytes > MAX_AGGREGATE_SOURCE_BYTES) {
      throw new Error(`producer source closure exceeds ${MAX_AGGREGATE_SOURCE_BYTES} bytes`);
    }
    const contents = decodeUtf8(bytes, `${commit}:${path}`);
    if (path === PRODUCER_SOURCE_PATH) producerSource = contents;
    sourceEntries.push({ path, contents: bytes });
  }
  if (producerSource === null) throw new Error("producer source closure lacks its defining source");
  requireExpectedProducerPaths(producerPathsFromSource(producerSource));

  const sourceSha256 = framedSourceDigest(sourceEntries);
  if (sourceSha256 !== producer.source_sha256) {
    throw new Error(
      `producer source digest differs from the recorded Git tree: saved ${producer.source_sha256}, computed ${sourceSha256}`,
    );
  }

  const verifierBytes = readBounded(realpathSync(verifierPath), 1024 * 1024, "source verifier");
  return {
    schema_version: RECEIPT_SCHEMA,
    status: RECEIPT_STATUS,
    corpus: {
      schema_version: corpus.schema_version,
      bytes: corpusBytes.length,
      sha256: sha256(corpusBytes),
    },
    producer_commit: commit,
    source_sha256: sourceSha256,
    source_digest_encoding: SOURCE_DIGEST_ENCODING,
    source_files_checked: SOURCE_PATHS.length,
    source_files: sourceEntries.map(({ path, contents }) => ({
      path,
      bytes: contents.length,
      sha256: sha256(contents),
    })),
    verifier: {
      path: VERIFIER_PATH,
      sha256: sha256(verifierBytes),
    },
    checks: {
      corpus_schema_and_producer_metadata_valid: true,
      commit_object_resolved: true,
      producer_source_path_list_matches_verifier: true,
      all_source_blobs_utf8: true,
      individual_source_hashes_computed: true,
      framed_source_digest_equal: true,
    },
    claim_boundary: CLAIM_BOUNDARY,
  };
}

export function parseCli(argv) {
  if (argv.length === 1 && (argv[0] === "--help" || argv[0] === "-h")) {
    return { help: true };
  }
  const values = {};
  for (let index = 0; index < argv.length; index += 2) {
    const name = argv[index];
    const value = argv[index + 1];
    const key = CLI_OPTIONS[name];
    if (key === undefined) throw new Error(`unknown argument ${name ?? "<missing>"}`);
    if (value === undefined || value.startsWith("--")) throw new Error(`${name} requires a value`);
    if (Object.hasOwn(values, key)) throw new Error(`duplicate argument ${name}`);
    values[key] = value;
  }
  if (!values.input) throw new Error("--input is required");
  if (!values.repoRoot) throw new Error("--repo-root is required");
  return {
    help: false,
    input: values.input,
    repoRoot: values.repoRoot,
    output: values.output ?? null,
  };
}

function writeAtomic(output, bytes, input) {
  const outputPath = resolve(output);
  const inputPath = realpathSync(resolve(input));
  if (existsSync(outputPath) && realpathSync(outputPath) === inputPath) {
    throw new Error("--output must not alias --input");
  }
  if (!existsSync(outputPath) && outputPath === resolve(input)) {
    throw new Error("--output must not alias --input");
  }
  const parent = realpathSync(dirname(outputPath));
  if (!statSync(parent).isDirectory()) throw new Error("output parent is not a directory");
  if (existsSync(outputPath) && lstatSync(outputPath).isDirectory()) {
    throw new Error("--output must not be a directory");
  }
  const temporary = resolve(
    parent,
    `.${basename(outputPath)}.tmp-${process.pid}-${randomBytes(8).toString("hex")}`,
  );
  try {
    writeFileSync(temporary, bytes, { flag: "wx", mode: 0o600 });
    renameSync(temporary, outputPath);
  } finally {
    rmSync(temporary, { force: true });
  }
}

function help() {
  return `Usage: node ${VERIFIER_PATH} --input CORPUS --repo-root REPOSITORY [--output RECEIPT]\n\nBinds the binary transfer corpus's recorded producer source digest and reviewed 21-path closure to the recorded Git commit. This is a provenance check, not an independent arithmetic or solver check.\n`;
}

export function run(argv) {
  const options = parseCli(argv);
  if (options.help) {
    process.stdout.write(help());
    return 0;
  }
  const receipt = verifyBinaryTransferSource(options);
  const bytes = Buffer.from(`${JSON.stringify(receipt, null, 2)}\n`, "utf8");
  if (options.output === null) process.stdout.write(bytes);
  else writeAtomic(options.output, bytes, options.input);
  return 0;
}

if (process.argv[1] && realpathSync(process.argv[1]) === realpathSync(SCRIPT_PATH)) {
  try {
    process.exitCode = run(process.argv.slice(2));
  } catch (error) {
    process.stderr.write(`binary transfer source verification failed: ${error.message}\n`);
    process.exitCode = 3;
  }
}
