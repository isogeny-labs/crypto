# Actual S3 yield/degree validation

**Publication hold:** source, compact canonical cases/certificates and proof
receipts are committed. The inspected PDF, two PNGs and raw archive remain
local, ignored pending upload approval. The [exact pending manifest](ARTIFACTS_UPLOAD_PENDING.json)
names all four files,684,341 bytes, hashes and intended S3 keys. Automatic
approval review rejected the upload for lack of explicit authorization of
this payload/destination, citing possible non-public research data. No upload
was performed or retried; no genuine remote hydration manifest is claimed.

Run `s3-yield-20261024`, started2026-10-24 after published head
`5fed419c1a9818a524abc66f7912fbe7c35672c5`. Protocol commit
`55e7eed4` preceded execution. Exact sources, pins and registry bytes are bound
by `INPUTS.sha256`; code developed after the protocol is not mislabelled as
already present in the starting head.

| Check | Result | Evidence |
| --- | --- | --- |
| Native actual S3 replay |124 comparisons /1,904 target systems | frozen `../../native/s3-yield-20261024/cases/`, `audit.json`; archived `runs/ci-delivery/` |
| Independent arithmetic/matrices | every point lift, signed row, rank, degree journal and two-way map control matches | same case files and independent Rust audit |
| Boundaries/caps |942 models,274 shared cases,186 r3 cases too small;600sec caps not hit; no closure cap miss | native index and raw commands |
| Rehashed semantic corruptions | false rank, false degree, omitted case, false scaling reject exit101 /no receipt | archived exact corrupted case trees and logs |
| Full final Lean gate |185 audited declarations,0 failing/extra trusted in12 modules | `lean-gate-final/`, `proof-gate-final-02.log` |
| Named research theorems |144 total,38 new;41 other/generated declarations also audited | new modules and `#print axioms`; full audit log |
| Earlier successful gate |180 declarations at the intermediate source state | `lean-gate-01/`, `proof-gate-final.log`; not substituted for final185 |
| Source-bound identities | S3 scaling IDC1hf15c17737ed713d5 and cleared unit IDC1hcee4710ff9dcf7e7;32+16 each; false/source controls reject | `../../native/s3-yield-20261024/certificate/` |
| Exact new CI command | passes every step, case/audit/certificate bytes and controls | `ci-delivery.log`, `.exit`; archived complete source/data replay |
| Isolated backend tests |22 passed,0 failed/ignored | archived `runs/ci-delivery/backend-tests.log` |
| Full release library suite |4,479 passed,0 failed,119 existing ignores | `cargo-lib-network.log`, `.exit` |
| First full library run |4,470 passed,9 failed,119 ignored; loopback socket permission denied | `cargo-lib-final.log`, `.exit`; preserved environment failure |
| Workflow metadata |4 passed | `workflow-tests.log`, `.exit` |
| Full ICV1 naming metadata check |0 problems after exact tracked prose/pin hydration | `curve-names-hydrated.log`; sparse failures in `curve-names-initial.log` and `curve-names-missing-pins.log` are superseded |
| Current-main artifact policy check | no data files committed | `artifact-policy-check.log`; four selected files remain ignored/pending approval |
| Report | four final local pages rendered/inspected; large assets awaiting publication approval | `pdf-build-02.log`, `pdf-inspection.txt`, committed report source/SVGs and pending manifest |
| Conductor | CLI127; no reservation or replacement coordinator | `conductor.log`, `.exit`; initial tool result before edits recorded in protocol |

## Exact commands and environment

```sh
tools/lean/proof-gate.sh --out /tmp/twist-s3-proof research/quadratic_twist_evidence_20261021/lean
bash research/quadratic_twist_evidence_20261021/run-s3-yield.sh /tmp/twist-s3-fresh
cargo test --release --locked --lib --quiet
python3 -m unittest scripts.test_workflows_parse
```

The full library command needs permission to bind127.0.0.1 for the existing
network tests. The first sandbox run denied those nine binds with OS error1;
the full rerun with network permission passes. No source or test is edited,
skipped, quarantined or weakened. Source/Cargo trees remain identical to the
starting head. Library test duration is diagnostic transcript metadata, not a
research timing or benchmark.

Lean4.34.1/Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned.
Supported `ELAN_HOME=/workspace/twist-tooling/elan` and
`MATHLIB_CACHE_DIR=/workspace/twist-tooling/cache/mathlib` are used; the global
gate is unmodified. Only propext, Classical.choice and Quot.sound occur in
accepted proofs. Failed elaboration logs can contain provisional sorryAx;
those are failures, not proof evidence. Additional lean4checker is unavailable.
Rust1.99.1 uses `RUSTUP_HOME=/workspace/twist-tooling/rustup`,
`CARGO_HOME=/workspace/twist-tooling/cargo`, `CARGO_BUILD_JOBS=2`.
Native field/curve arithmetic is unchanged in full.cpp; the new Rust auditor
shares only the existing hash/identity backend, not its arithmetic or matrix code.

Both native600sec caps are correctness limits, not speed measurements. C++
uses packed prime arithmetic, binary scalar multiplication, polynomial
convolution and incremental echelon insertion. Rust uses independent modular
arithmetic/Euclidean inversion, linear scalar chains, explicit S3 coefficients,
symmetric S3 evaluation and batch RREF with ascending monomial degrees.
Every matrix is rebuilt; no producer rank is trusted as an input.

## Failed and provisional attempts

- Native attempt01 fails only under `-Werror=misleading-indentation`. The exact
  source/log is retained. Attempt02 passes the first experiment; review then
  found its matched control chose new least-y representatives instead of the
  actual transported points. Attempt03 corrects that, records exact coefficient
  rows and passes1,632 systems. Attempt04 adds the inverse pullback control,
  giving the final1,904 systems. All earlier outputs remain provisional and are
  retained in the raw archive; they are never rewritten as final controls.
- Lean attempts01 and02 fail a nonexistent import/noncomputable definitions;
  03 fails a tactic syntax;04 passes the initial system/fixture proofs.
  05 fails a cubic leading-component simplification;06 fixes it.07 and08 retain
  failed degree-fixture coefficient/characteristic simplifications;09 passes
  the completed actual leading-ideal proofs.10 fails only the characteristic
  wrapper,11 passes the degree4 dual/degree5 certificate, and12 passes the final
  cleared identity and exact pullback relations. Snapshots end`.lean.txt`.
  Exit files for early build attempts are transcribed from their completed
  execution results; later shell exit files are captured directly.
- The first native certificate presentation used pretty JSON. The final
  producer uses ordinary compact JSON text, with identical statements/IDs and
  unchanged semantic checks. Both provisional and final trees are retained.
  No compressed certificate masquerades as text, and each final data file is
  below64KiB. The new Cargo.lock is explicitly tracked despite the global ignore.
- Plot/Graphviz rendering initially warned about read-only default font caches.
  Graphs still rendered. The first PDF had a tiny left-to-right diagram and
  one overflowing certificate paragraph; the final top-down layout and proof
  table/paragraph are corrected. All four final pages were visually inspected;
  final LaTeX has no overfull boxes.
- `proof-gate-final.log` is the earlier180-declaration pass; the final source's
  separate `proof-gate-final-02.log` is185. Old gate receipts are retained without
  repurposing or changing their counts.
- The extra full naming metadata check first lacked3,618 tracked Markdown/HTML
  files in the sparse checkout. After materializing them with the repository
  helper, it reported seven findings while the corresponding frozen-data pin
  manifests were still off disk. Materializing the five applicable tracked
  manifest/checksum paths makes the unchanged full `check_curve_names.py
  --diff 5fed419c1a9818a524abc66f7912fbe7c35672c5` pass with zero problems.
  No old prose, registry, matcher, exemption, or validation gate is patched.
  These are sparse-environment diagnostics, not current-head CI failures or
  a claim that an inherited naming failure needs repair.

## Data and artifact policy

Final per-case JSONs remain loose because `index.json` and the verifier name
every exact path/hash. All old evidence stays untouched. The raw research run
tree is archived deterministically as `runs.tar.xz`, with `RUNS.sha256` for its
members and `runs.tar.xz.sha256` for the archive. Extract in this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c RUNS.sha256
```

Packing the extracted tree with GNU tar's sorted names, mtime2026-10-24T00:00Z,
uid/gid0 and mode u+rwX,go+rX,go-w, then xz-6-T1, gives identical bytes. Binaries,
Cargo/Lake caches and rendered review PNGs are not evidence inputs and are not
included; code, exact commands, raw outputs/controls, source snapshots and
transcripts suffice to reproduce them. Nothing older is repacked.

No new non-code file exceeds1MiB. Current-main policy also selects PDF/PNG
assets over64KiB, as well as the85,956-byte raw archive. Those four files are
explicitly ignored and pending approval, not committed in violation of the
policy. The pinned current-main artifact tool's read-only dry run lists exactly
those files. INPUTS binds all final code/pins; OUTPUTS includes their prepared
local hashes alongside canonical cases, certificates, gate records and source
assets. Complete `sha256sum -c OUTPUTS.sha256` on another checkout requires
hydrating those pending files once upload is approved and verified. All earlier
frozen evidence remains unchanged; no existing file is moved by this round.
The initial/current AGENTS blobs, unchanged protected inputs and both catalog
provenance dependencies are recorded in `git-provenance.txt`.

The atlas metadata match ran before the new mechanism discussion. A small
handoff retains existing lead dispositions for the next atlas review. Python
only queries that metadata/existing workflow YAML tests and lays out recorded
JSON values; no Python research implementation or verifier is introduced.

## Scope and completion

The finite F43 useful-row/closure example, actual scalar reduction and main
degree certificates are checked. Homogeneous first-empty degree is equal in
the tested encoding and the twist result is exactly reproduced on the main
curve by base pullback. No uniform useful-yield law, improved regularity for
another actual encoding, F4/F5 work bound or complete speedup follows. Keep
the broader requirements/problem record open. Complete exact-head CI/review,
merge conflicts/integration and shared Conductor access remain distinct gates.
Pending artifact publication is an additional access/authorization gate; it is
not a mathematical obstruction. The artifact tool and AWS CLI are available,
but credential use/remote verification was not attempted after the automatic
review rejection. Do not retry an upload until the user approves this exact
manifest and destination. The safe read-only plan does not authorize an upload.
