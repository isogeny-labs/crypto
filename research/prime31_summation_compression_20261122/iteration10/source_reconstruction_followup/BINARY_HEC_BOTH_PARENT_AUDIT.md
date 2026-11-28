# Both-parent repair audit for the inherited binary-HEC root-build blocker

**Date:** 2026-11-23 UTC. **PR:** #4977, Prime31 S3/pair-spectrum continuation. **Status:** source-reconstruction *plan*, not a repaired Rust file, build, original-root correctness result, signed independent validation or faster query. [pcc:unverified]

## Exact source custody and observed failure

The inherited malformed source is `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs`, Git blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`. The source first appears in merge commit `77d388738a48eb5a6e2abd80636c2e31cceab254` ("Merge main into prime-ic-mitm-20261118"), with parents:

| Revision | Git blob of the exact file | Lines | Role |
|---|---|---:|---|
| merge 77d388… | `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` | 1,234 | Corrupted; ends with orphaned code and loses braces, derives and tests |
| first parent `a9d68fbd9c2dc5b6db267746c483febd22cb5501` | `12758d4edc3f1d5f6c6c66ebb32397d04c991af7` | 1,475 | Original unprojected solver and existing tests |
| second parent `0d4721abfeb0c402cb7de6227e0f2108101162dc` | `89a190d53bcd20a8141c219db2f0f8fd963b9ce2` | 1,540 | Adds projected solver/stage API and accounting; preserves legacy wrapper |

The third original-root dedicated CI job [45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392) attempted five compilation commands but ran **no** native tests, issuer or replay. This note inspects the source lineage, not that artifact's execution as a replacement. A same-session file read shows the malformed merge source ending in dangling statements, whereas the two parent files have complete test-module endings. [pcc:unverified]

## Independent read-only comparison of the two parents

An in-memory line-level longest-common-subsequence comparison of the exact Git blobs identified **15 parent-to-parent difference blocks** (alignment counts, not a semantic proof). The distinguishing code is concentrated in two places, not a wholesale rewrite:

1. Around the relation collector: first parent uses `match classified { Some(entries) => ..., None => {} }`; second parent uses equivalent `if let Some(entries) = classified { ... }`. Both retain the same identity-point branch, smoothness time charge and relation coefficients. The first parent's `let pos = pos;` is a no-op and is not essential behavior.
2. Around end-to-end solving: the second parent adds `BhecIndexCalculusStage`, `binary_hec_projected_index_calculus_dlp<F>`, modular multiplication of both collected relation labels by `label_scale`, explicit phase/verification counters, and a historical `binary_hec_index_calculus_dlp` wrapper passing scale one. This projected adapter must not be dropped.
3. The first parent writes `let g = curve.genus as u32;` and the second writes `let g = curve.genus;` inside `hasse_interval`. The type is already `u32` in `src/binary_ecc/hyperelliptic.rs`; the explicit cast is redundant, though it may be retained to make the reconstructed origin clear.
4. The parent diff has no later changed hunk in their shared test-module region. This is a source-level preservation observation, not a passing test.

A separate LCS against the corrupted merge found 395 parent-A lines and 479 parent-B lines not aligned, together with 154 / 173 merged lines not aligned against those parents. These are **diff-alignment counts** including shifted or edited lines; they must not be misdescribed as an exact number of physically missing statements. The merged file has 1,234 lines versus 1,475 and 1,540 in its parents. [pcc:unverified]

## Reviewed *candidate recipe*, not a substitute-parent publication

A read-only source assembly starts from the second parent's complete API, replaces its `if let` classification body with the first parent's explicit `match` body, and restores the first parent's explicit `as u32` genus cast. This yields a **1,543-line distinct candidate**, not byte-identical to either parent. A structural source-text check found the projected API, the historical scale-one wrapper, stage enum, the restored relation-classification body, the `#[cfg(test)]` module and its terminal parallel case. The first parent's harmless `let pos = pos` is omitted with the reason recorded.

**Nothing here establishes compilation or semantic equivalence**, and the candidate was not written into the production path or PR. Before any repair is admitted, an independent reviewer must compare all 15 parent differences and confirm the exact intended API/behavior and test union, including all interaction with currently maintained callers. Do not remove the module, drop tests, substitute a smaller crate or copy one parent unreviewed. [pcc:unverified]

```text
                  77d388 merge
                /       |       \
   parent A (legacy)  malformed  parent B (projected + legacy)
                \       |       /
                 proposed complete union
                         |
             independent source review
                         |
             original-root compile/test gate
                         |
        issuer, signed replay and PCC standing
```

## Sequential qualification gates

**Source reconstruction:** work on an isolated repair branch after checking live ownership. Freeze the candidate and record the parent blob identities, full diff and expected base. Preserve A's legacy collection semantics and B's projected scale/stage/timing functionality, as well as both parent tests. Obtain a distinct review before modifying the shared root file. The repo's original-root source also includes the unchanged lockfile and spec/main guard material from #4962; do not rewrite it.

**Build:** compile under the original repository manifest/lock and rerun the five exact compilation/test commands from the failed job against the frozen **actual checkout SHA**, with no `continue-on-error`, no test removal, no alternate crate. Retain failed receipts. Additional caller tests should compare scale-one wrapper behavior with the original path and independently exercise nonunit scale against a sound homomorphism fixture.

**Scientific validation:** only after compile success, rerun the original rational-root guard and independent signed completeness, rational-root, sign, duplicate, identity, miss and useful-rank tests at a pinned source; compare against the signed pair-spectrum control. A repair to binary HEC **does not** itself validate the Prime31 relation solver or reduce its query complexity. [pcc:unverified]

## Coordination, literature and next mathematical gate

Source and task read: AGENTS.md, docs/ACTIVE.md, newest dated session log, iteration10/PENCIL_FAMILY_STATE.json, #4977/#4979 current metadata and the inherited #4962 failure. Relevant prior mathematics: Semaev (2015) [arXiv:1504.01175](https://arxiv.org/abs/1504.01175); Bostan–Flajolet–Salvy–Schost (2006), *Fast Computation of Special Resultants*. The repository already contains `research/prime31_ic_20261115/native/examples/prime31_s9_support_ceiling.rs`, which implements the general cancellation-signature coefficient count; a proposed new arity-ceiling lemma was therefore **not** rebranded as novel or duplicated. The shared KB and Conductor services were not available in this runtime.

Only one coordinator was available. It performed parallel, read-only GitHub reads and source comparisons. No native algebra, Rust or validation worker could be dispatched; these are **work packages for a subsequent qualified execution**, not fictitious worker results. No Rust compiler, Lean runtime, authenticated EKS route, paid compute, original arithmetic, benchmark, PCC signature or native Page write ran here. No new asynchronous job or automation was created.

The separate open exact-query objective remains a source-specific target-to-pair zero-component oracle, with near-linear-in-`D` setup and fixed empty-inclusive query exponent `theta<1` (stronger conditional pipeline target `theta<1/2`), including rational outputs, signs, duplicates, proof traffic and rank. This source-repair audit only makes the original-root gate more actionable. [pcc:unverified]
