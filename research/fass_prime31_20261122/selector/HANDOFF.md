# Selector resultant continuation

Parent: [PR #4921](https://github.com/isogeny-labs/crypto/pull/4921), branch `research/fass-prime31-20261122`. This is candidate C of the original FASS protocol. The original objective, matched algebraic comparisons and full cold prime-field index-calculus accounting remain open.

## Delivered component

`src/cryptanalysis/semaev_selector.rs` implements the division-free quadratic/selector norm, complete coordinate extraction for split `X^d-1`, and rational signed lifting. The exact frozen source inventory passed the 15-test native qualifier, including the three Prime31 target outcomes retained in `development-receipt.json`. These are unsigned development observations. [pcc:unverified]

The manuscript, proof map, source diagram and visually inspected four-page PDF are in this directory. Three evaluated identity controls and two same-team source reviews are under `identities/` and `evidence/`; none establishes signed PCC or Lean standing. The source review's manuscript hashes precede the final integration paragraph, as the report explains.

## First next action

Read the current main revision and `evidence/main-integration-audit.json` before merging. At the inspected main revision `856dfeb0be25df78ab3877741544f64940128fac`, `semaev_projection.rs` changed the quadratic wrappers and added imports outside this qualifier's dependency closure. Do not replace newer main code with old code to manufacture a match. Preserve main's additional module declarations and reconcile the actual dependency closure, then create a new source binding and rerun qualification. The prior 15-test outcome applies only to its recorded sources. [pcc:unverified]

The full-library command is `cargo test --release --lib --offline -j 1`. The retained 180-second and subsequent 300-second attempts both exited 124 during dependency compilation. Neither produced a library test result. Both logs remain under `evidence/library/`. [pcc:unverified]

## Admitted comparison after integration

Compare the selector norm against same-base direct/chained F4 and current main's `resultant_decompose_m2` / `ResultantOracleM2`. Preserve the separate linear-size signed-point scan as a reference. Charge setup, polynomial work, extraction, rejected and empty targets, signed lifting and useful rank. The current norm uses schoolbook multiplication; the fast-polynomial bound is prospective.

Use the original protocol and repository checkpoint, runner, source/image, capacity and budget admission. No benchmark or paid job was launched here. Earlier checkpoint attempts lacked a supported authenticated transport; their observed status is in the dated session log. Lean/lake was unavailable in this runtime; `PROOF_MAP.md` identifies the exact formal obligations.

## Publication and state

The PR stays draft because current-main integration, whole-library qualification and the broader admitted comparison remain unfinished. Update its existing branch rather than opening a duplicate. Preserve the append-only main PCC prefix before registering any new claim. Never promote the producer attestation to a verified claim without the signed verdict. [pcc:unverified]

Pages searches and an inventory request returned no usable result before the execution runtime reset. No Page was edited. When the connector responds, find and freshly read the existing FASS research page or ECDLP benchmark tracker before reconciling its status; do not create a duplicate from a failed lookup.

There are no research runs in flight from this work.

The immutable selector source/evidence commit is `883e2d2dda223ae1b8eafdc83b289a49fa178855`. The intended unsigned statement is `PCC-20261123-b6b455`; `PCC-20261123-f307d3` is retained only as a superseded administrative record with the earlier producer commit. Preserve concurrent coupled-study commit `2ac638f69c95e9966ce8dd29505b67337d6a9bba` when updating the lane. [pcc:unverified]
