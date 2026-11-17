// Byte-preserving source-prefix selection and host metadata only; no EC arithmetic.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import os from 'node:os';
import {execFileSync} from 'node:child_process';
const [sourceRoot, out] = process.argv.slice(2);
if (!sourceRoot || !out || !fs.statSync(out).isDirectory()) throw Error('SOURCE_ROOT EXISTING_OUTPUT_DIR');
const study = import.meta.dirname;
const p = JSON.parse(fs.readFileSync(path.join(study, 'protocol.json')));
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
for (const f of p.extensions.source_files) {
  const b = fs.readFileSync(path.join(sourceRoot, f.path));
  if (b.length !== f.bytes || sha(b) !== f.sha256) throw Error('Frozen source mismatch: ' + f.path);
}
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], {cwd: sourceRoot, encoding: 'utf8'}).trim();
if (sourceCommit !== p.extensions.source_checkout) throw Error('Wrong frozen source checkout');
const raw = fs.readFileSync(path.join(sourceRoot, 'research/theorem_census_20261109/native/semaev.cpp'), 'utf8');
const marker = '\nint main(int argc,char** argv){\n';
const at = raw.indexOf(marker);
if (at < 0 || raw.indexOf(marker, at + 1) !== -1) throw Error('Historical entry marker is not unique');
const entry = fs.readFileSync(path.join(study, 'cpp-entry.cpp'), 'utf8');
const generated = raw.slice(0, at + 1) + entry;
fs.writeFileSync(path.join(out, 's3-only.cpp'), generated);
fs.writeFileSync(path.join(out, 'source.json'), JSON.stringify({schema: 'hosted-s3-cpp-source/v1',
  source_commit: sourceCommit, original_cpp_sha256: sha(raw), unchanged_prefix_bytes: at + 1,
  unchanged_prefix_sha256: sha(raw.slice(0, at + 1)), entry_sha256: sha(entry),
  generated_translation_unit_sha256: sha(generated), os: os.type(), release: os.release(),
  arch: os.arch(), cpu: os.cpus()[0]?.model, threads: 1, clock_valid: false}, null, 2) + '\n');
