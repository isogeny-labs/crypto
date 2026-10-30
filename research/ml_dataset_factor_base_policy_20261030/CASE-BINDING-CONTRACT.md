# Proposed V2 policy worker/case binding integration

Scratch-only proposal, 2026-10-30. The active guarded worktree is unchanged.
No build, native generation, Git mutation, S3 action, or external coordination
was performed. This is implementation preparation, not completed CLI support.

## Exact proposed representation

Keep `config.json` as the existing strict CaseConfig for both origins. It is
already consumed by capture, execution and solver code; adding policy fields
would change V1 bytes and break those readers. Add `case-policy.json` only for
V2, including explicit legacy-only V2. Its strict typed fields are:

- schema `crypto.ml-dataset.case-policy/v1`, origin `v2`;
- full versioned typed plan SHA-256 and exact retained plan-file SHA-256;
- unchanged typed-config SHA-256;
- global cell index, axes-cell index, policy index, and explicit policy.

The sidecar is derived only by parsing original plan bytes and calling its checked
cell selector. Verification recreates the same expected record. It never trusts
an embedded policy or index independently. Duplicate sidecar/config fields and
unknown fields fail typed parsing. Sidecar input is bounded at16KiB; config/plan
inputs use the existing32MiB limit. This is metadata sizing, not process RSS.

CellManifest retains its existing field layout. V2 uses
`crypto.ml-dataset.cell/v2` and exactly one sidecar artifact with role
`restricted-case-policy-binding`, encoding `json`, exact byte length and digest.
Changing or dropping both its path and role cannot avoid the required sidecar.
V1 keeps `crypto.ml-dataset.cell/v1` and emits no sidecar. Proposed corresponding
run/attempt versions are `crypto.ml-dataset.run/v2` and
`crypto.ml-dataset.attempt/v2`; these choices require explicit integration
acceptance before production, because old capture/start readers require v1.

## Small independent helper

`case_binding.rs` is proposed as `types::case_binding`, registered by the
separate `types-registration.patch`. `BoundCase` can only be constructed from
strict original-byte plan admission; its normalized case and hashes are private.
It exposes occurrence production, strict config/sidecar verification, manifest
binding, parent run/attempt version/hash/index checks, and expected-policy
factor-base replay. It performs no filesystem writes and no implicit generation.

The constructor already retains restricted history in
`witness.json.factor_base_policy_construction`. Reuse that exact field rather
than duplicate the large history into another artifact. For signed policies,
full replay requires it and calls `factor_base_policy::verify` with the expected
plan policy and requested size. The public mathematical verifier alone is not
sufficient. For both V1 and explicit V2 legacy policies, require the original base schema before calling
its verifier, so the new mathematical schema dispatch cannot silently substitute
an orbit population into an explicitly legacy case. Public family/n/m/twist and
requested size are checked against the selected config. Known-answer/scalar
replay stays a distinct existing phase. No private field enters public features.

Existing verification of public arithmetic/scalar relations is preserved. This
helper does not independently regenerate seeded curve/generator/key sampling;
it must not be described as a new proof of that sampling process.

## Held mod.rs patch

`held-mod.patch` is an exact proposed diff against the inspected mod.rs. It:

1. Adds producer sidecar installation and worker admission helpers.
2. Keeps bare V1 worker use unchanged; requires complete V2 plan/run/attempt,
   exact config and sidecar before selecting the policy constructor.
3. Dispatches constructor and full expected-policy factor-base replay in the
   existing construction/replay phases.
4. Reserves the sidecar path against backend artifact collisions and gives it
   the explicit role during incomplete-worker artifact collection.
5. Selects the manifest schema from the admitted occurrence, permits both cell
   schema versions, authenticates the sidecar against the sealed manifest, and
   requires the restricted witness role for a completed V2 factor-base artifact.
6. Uses expected-policy replay for V2 cells; V1 also uses the expected legacy-policy gate before historical mathematical replay. Status without replay checks custody/occurrence and never gains
   a new semantic-verification claim.

**This patch intentionally does not turn on V2 generation by itself.** The
existing producer and bound_run still decode DatasetPlan, and capture still
rejects V2 run/attempt schemas. They fail closed until the remaining coordinated
integration below is implemented. Do not bypass those gates to demonstrate a
partial V2 worker run. No output from this proposal is a completed CLI fixture.

## Remaining wiring, with concrete source boundaries

- mod.rs plan/preflight: add explicit policy selectors and strict PlanDocument
  parsing; no selector retains exact V1 serialization. Validate the full V2
  document/protocol and print the actual multiplied count and policy caps.
- mod.rs generate: retain the selected V2 plan bytes exactly (both run root and
  cell copy), use full versioned plan hash, select `CaseSpec.config`, emit run/v2
  and attempt/v2, then mint the sidecar from those actual retained plan bytes.
  V1 storage calls remain as before. Do not hash only `axes` for budget binding.
- mod.rs bound_run, reconciliation, verify_run and resume: change normalized plan
  handling to PlanDocument and compare `.cell(index)?.config`; maintain full
  policy origin through all indices. Verify both root and copied cell bindings.
- capture.rs::AttemptBinding::from_cell_files: strict PlanDocument, correct run/
  attempt schemas, and sidecar admission before creating capture context. The
  existing attempt digest should bind the sidecar hash for V2 so a policy record
  cannot be detached from capture custody. Keep all V1 binding bytes unchanged.
- execution.rs: audit each transitive use of AttemptBinding and context hashes;
  preserve worker deadline identity and reject substitution across policy blocks.
- startup_gate.rs: current frozen-plan decode is DatasetPlan at~1291. Admit full
  V2 plan/preflight hash and count; preserve protocol/file/body hash distinctions.
- budget paths: launch/preflight RunBinding, per-cell scopes, journal, restore and
  grant checks must all bind the full V2 plan hash and global index.
- solvers.rs: use the exact verified population and signed-point/distinct-x maps;
  preserve alternative PKM domain semantics and unsupported backend outcomes.
  The helper does not enable unsupported native routes.
- export.rs/report.rs: readers call verify_cell and use its CaseConfig. Retain
  policy occurrence identity separately, add explicit V2 allowlists/projection,
  distinguish policy cells/actual populations in reports, and keep restricted
  sidecar/history out of public model inputs. Preserve V1 exported bytes/IDs.
- archive/publish/readback: allow and bind the new reserved artifact/schema using
  the existing immutable registry and publication machinery, including failures.

Before calling CLI integration complete, exercise omitted-policy V1, explicit
legacy-only V2, and signed V2 on both twist positions; verify policy substitution
with recomputed outer hashes, missing history, wrong role/origin/index, strict
parsing, partial workers, source immutability and export retry. Tests in the
helper are authored but unexecuted. They do not replace the native acceptance
or full dataset workflow checks.

## Independent review fixes (revision02)

V1 replay now always calls BoundCase::verify_factor_base: a recomputed outer
manifest cannot substitute a signed schema into an implicit legacy occurrence.
V2 direct worker admission calls require_worker_activation before any constructor
or optional capture/execution handling. This is an explicit unconditional gate;
missing custody files do not enable execution. Remove it only in a coordinated
activation patch after capture, budget, startup, solver admission and export
readers are ready. An authored regression invokes the actual worker with a V2
plan and attempt index but no custody and checks that no constructor artifacts
were created; it has not been run.

Future signed solver admission retains public/base/witness first, then returns a
censored non-solver outcome for a verified empty population. Nonempty outcomes
carry the actual domain digest, constructor terminal, and explicit
actual_retained_population_only scope. They never assert intended-domain UNSAT.
These paths remain unreachable while the V2 activation gate is present.

## Implementation freeze amendment

This document now freezes the independently reviewed revision02 helper representation
for implementation under T960, before case_binding.rs is added. V1 bytes and public
serialization remain unchanged. This is an occurrence/custody component, not full
CLI activation. The restricted sidecar is mandatory only for explicit V2. Full
reader and producer integration remains required before removing activation gates.
Root accepts this routine design decision within the authorized dataset task.
