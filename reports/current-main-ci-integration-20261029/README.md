# Current-main CI integration, 2026-10-29

Normal merge of main `a0aef071f53c9bff03c082dc30eca643aa43833d` into the retained
Lean coverage and catalog repair. The three textual conflicts are resolved by
replaying current inputs and comparing both parent versions. Copilot request
6466890009 had no resolution after more than 15 hours; the documented normal
merge fallback was recorded in PR #2685 before editing. No shared history is
rebased or force-pushed.

## Software checks

- Generic transitive Cargo path dependencies now hydrate exact manifests,
  declared targets, Rust sources and recursively embedded inputs. Unrelated
  archive JSON and Rust remain absent from cold sparse checkouts. All 12
  controls pass, including the separately landed #2805 nested include/path fixture, a generic nested helper crate and native Cargo
  metadata. Absolute or escaping target paths are refused.
- Lean project paths are normalized against the checkout before module names
  and control exclusions are derived. Canonical, leading `./` and trailing `/`
  spellings all reach the actual `sorryAx` audit. All 10 controls pass with
  Lean 4.34.1 and 4.19.0. The GHS projects replay 13 and 1 native theorems with
  zero trusted declarations. No axiom rule or CI trigger is relaxed.
- All 13 current constructor tests pass over exact registry/graph membership
  sets, including every current binary EC1 binding. The standalone executable
  exports all 20,639 models with exact canonical non-provenance parity.
  Complete cover byte-currentness reports 20,639 verified, zero invalid and
  zero unsupported rows.
- The current Jacobian formula catalog has 20,639 entries and 60,770 routes.
  The prior catalog contributes 20,592 identical entries, with 47 additions,
  zero removals and zero changed entries. Both parents use the same CostModel
  and factor budget 200000. These are modeled routes, not measured solver
  improvements or transfer certificates.

## Dated assessment and retained failures

The automatic merge changed report population labels to 20,051 while retaining
an assessment pinned to commit `728304718cb0ab6df70da620a908a5f76f175293`
(15,791 maps and 979 EC1 bindings). The independent verifier correctly found
four presentation mismatches. The exact automatic-merge patch and both parent
hashes are retained here. The three derived presentations again describe their
declared dated cohort; the newer 20,051 population remains independently
traceable at `a857883776846d01b13355996ea433951a6ff9d7`, and the live 20,639
population is checked separately above. No later row inherits a scientific
assessment merely from sharing the software implementation.

After provenance regeneration the independent assessment verifier reports zero
problems, byte-currentness passes, and all 17 scientific assessment sections
remain unchanged. The existing 13-page PDF remains byte-exact at 665062 bytes,
SHA-256 `ab2d8b7a9c002e9ce124e22bc2c04f4b96129f88091b65b3ab0dddb67af3efc1`.
Concrete subgroup-order and per-cover transfer proof obligations remain open.

## Sources and scope

Reviewed the repository gate, Cargo scope controls, dated assessment builder,
snapshot loader, independent verifier, both input cohorts and prior repair
receipts. Lake's documented `+Module` build semantics inform the normalized
module checks: <https://lean-lang.org/doc/reference/latest/Build-Tools-and-Distribution/Lake/#lake-build>.
This is software repair and provenance reconciliation; no experiment or timing
comparison was run. Full-library results and hosted current-head merge gates
are recorded separately when completed. Shell syntax passes; optional
ShellCheck reports the existing literal-backtick, unused-array and intentional
module-list splitting diagnostics, retained in its raw log.

## Completed full suite and latest accepted main

The complete release-library suite at source 2e4b7801 passes 5,017 tests,
zero failures, with 122 pre-existing ignored tests. Its initial 26 missing
runtime-fixture failures and the complete passing rerun are both preserved.
A subsequent normal merge incorporates main 161a5a93, retaining both dated
shared-journal additions. Its accepted catalog contains 20,787 models.

Rebuilt all three native producers with Rust 1.98.0, including main's updated
Cargo dependency graph. All 13 constructor controls pass against those current
inputs. Complete cover byte-currentness verifies 20,787 rows with zero invalid
or unsupported results; standalone non-provenance data matches canonical data
exactly. The Jacobian inventory adds 148 entries to the prior 20,639, with
zero changed or removed existing entries, the same CostModel and factor budget
200000. It is 95,002,491 bytes of ordinary pretty JSON, below 100 MiB, at
SHA-256 6649cb2e3ce1131aef019fc277f0619bcf9055c00aa650dccc242c9c860b6ee4.

Assessment regeneration, byte-currentness, schema validation and independent
verification pass; the independent verifier reports zero problems. All 17
scientific sections remain unchanged relative to the complete-suite source;
only source_artifacts provenance differs. The dated PDF remains byte-exact.
All 12 combined Cargo scope controls pass again. No new mathematical support,
solver, performance, private-data or experiment claim is introduced. Hosted
current-head merge checks and fresh review still determine landing.
