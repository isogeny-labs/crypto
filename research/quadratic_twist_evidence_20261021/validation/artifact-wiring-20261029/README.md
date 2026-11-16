# Artifact workflow wiring repair — 2026-10-29

This is ordinary software maintenance, not a new relation-yield, degree, scalar,
timing or formal-proof experiment. The input is PR2381 head
`ade900ef9ae73c76b101d557b0722703aca361e0`, a normal merge of the previous
`740e2ca6c` head with main `bc6927b6b18dc4a910ff7df83154d6a68c592a21`.

## Failure and repair

The required [artifact job123253014058](https://github.com/isogeny-labs/crypto/actions/runs/40814722020/job/123253014058)
passes the data-file policy, then rejects eight quadratic-twist workflow pull
blocks as stale. The unchanged repository generator, with all1,412 tracked
artifact manifests materialized, reproduces the same eight findings (exit1).
Regenerated only those eight workflows. The primary sparse checkout explicitly
includes the root `.artifacts/` configuration as well as `tools/artifacts/`.
Its existing sources manifest alone does not supply that root configuration.

The generator and artifact policy are unchanged. Structural YAML comparison
checks that triggers, jobs, permissions, native commands, verification steps,
budgets and upload steps are identical after removing only the generated
read-only hydration blocks and their tool/config checkout additions. No local
store pull, upload, AWS authentication probe or checkpoint gate was invoked.

## Validation

- Full repository workflow wiring check: exit0 after repair.
- Existing artifact/bootstrap unit suite:28 passed; uses local temporary
  fixtures, not the remote artifact store.
- Existing workflow metadata/parser suite:4 passed.
- All eight structural comparisons pass, including tool/config availability.
- Diff whitespace check passes. Artifact/file-size checks for the committed
  repair are recorded separately in `committed-checks.json` when available.

No native algorithm, Rust/Cargo input, test predicate, registry/catalog,
benchmark, proof, certificate or older frozen evidence changes. The full root
library suite is not rerun for this workflow-only repair; its existing
incomplete/non-green state is not promoted. No native study, corruption CLI,
uniform-sampler execution or Lean elaboration is launched locally.

## Separate registration and launch holds

The exact input-head [native job123253013852](https://github.com/isogeny-labs/crypto/actions/runs/40814722004/job/123253013852)
passes historical audit/replay/controls and42 metadata backend tests, then fails
the unchanged zero-new-registration assertion. Downloaded artifact12872470042
is217,829 bytes, SHA256
`c681874554f95f61370db90182f1434769f0e8207cf9264359bf6c910931dd03`.
Its `registry-update.json` reports1,921 new rows and3,842 alias keys against
registry SHA256
`842a28358e7e39075d63136e0e848642fe0970a14f6e7e6a5b5ac9b1e077758a`.
That input registry has20,787 stored records. These are source-bound CI and
literal-file observations, not validation of a combined active catalog.

Seven separate exact-input-head native replay jobs, including F13, passed.
Later mixed/group-map/pairing/yield/S3 steps in the primary job were skipped
after the registration failure. Required gate/size passed on that input head;
the required artifacts job failed. Those historical results never stand in
for checks on a later repair head. No submitted reviews or threads existed.

Keep registration/cover/native validation and the strict extension assertion
open. Do not lower either assertion or replace a whole registry side. The
approved checkpoint-only scope persists, but secure token presence remains
false, so new scientific/proof launch is still blocked. Conductor exits127;
no task reservation or replacement coordinator is claimed. Keep PR2381 open,
auto-merge off, and all broader mathematical/completion holds intact.

## Evidence and reproduction

The official `tools/artifacts/wire_workflows.py`, its28-test suite, the current
artifact policy and AGENTS sections13/13a are the primary maintenance references.
`runs/input-identities.json` binds their exact blobs, the manifest inventory
digest, input registry, metadata assertion and downloaded CI artifact.
`runs/workflow-diff-review.json` binds all before/after workflow SHA256s.
Selected CI diagnostics and exact local commands/stdout/stderr/exits are
preserved. Preparation failures remain explicit; none is a scientific result.

```sh
./restore-runs.sh
sha256sum -c OUTPUTS.sha256
python3 tools/artifacts/wire_workflows.py --check
python3 scripts/test_workflows_parse.py
python3 -m unittest discover -s tools/artifacts -p test_artifacts.py
```

Run the last three commands from the repository root. Restoration checks every
raw hash and deterministically repacks the archive. No report/PDF is added for
this software-only repair; earlier research reports remain unchanged.
