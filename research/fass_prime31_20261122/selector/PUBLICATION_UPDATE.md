# Follow-up publication after parent merge

Active draft: [PR #4936](https://github.com/isogeny-labs/crypto/pull/4936), branch `research/fass-selector-resultant-qualified-20261123`. Parent PR #4921 was merged by another session before the final expected-head update. It is left untouched. The follow-up starts from main `30645895aa234c2ae4bf563664ecc3aa4a5dbba4`, with tracking commit `ab0fb9fb1d22170785aa9d65ae133ef34efedfb9`.

The original source/evidence commit `883e2d2dda223ae1b8eafdc83b289a49fa178855`, metadata commit `a2569c6234f4896b19bf99a9036f7698199061da` and first prepared-publication checkpoint `bfd94571a89cb8fcb538c99e8497bc5704a2382a` remain in the publication ancestry. The passing 15-test observation remains limited to the frozen source inventory. [pcc:unverified]

## Current source boundary

At the fixed follow-up head, the projection blob is still `63984e18b2a3b014616fa7138baff7e300850cc1`, and the PKM blob is still `9a801f27ae779cd35a6ee21a80252922324a9089`. Their inspected limitations match the retained [current-head audit](evidence/current-head-integration-audit.json). The registry has advanced to `8e379760334cf328bdc1c2b687dce000fef825e1`; it still has duplicate `hec_dlp` declarations, and publication retains every existing line while adding the selector declaration. No fresh current-head build or qualifier run occurred. [pcc:unverified]

Reproduce the frozen result from `883e2d2d`. Current-head qualification needs repair of the existing malformed later projection functions and duplicate registry declaration, exact source rebinding, the complete component suite and the whole-library gate. The earlier pinned main audit at `856dfeb…` and the later audit at `dda2a…` retain their original scopes.

## Final ledger reconciliation

The original three unpublished unsigned envelopes and both claim payloads remain byte-identical in `evidence/pcc-reconciliation/`. The first proposed canonical mapping against `dda2a…` never reached a branch ref because its parent PR merged. Those proposed envelopes and their checkpoint are retained, not silently relabeled as published history.

The final [before-state receipt](evidence/pcc-reconciliation-followup/before.json) and [registration mapping](evidence/pcc-reconciliation-followup/receipt.json) preserve the complete canonical prefix at the fixed follow-up head, then use the maintained Python administrative `append_entry` API to register the same event types and bodies in new envelopes. The November 8/9 documented unsigned-suffix procedure is cited in the original archive. No signed event, claim payload or verdict is edited. [pcc:unverified]

`PCC-20261123-b6b455` remains the intended unsigned statement. The earlier `PCC-20261123-f307d3` remains superseded with its original producer-commit field and explanatory note. Repository standing remains unverified. [pcc:unverified]

The original report and PDF are frozen source-qualified research artifacts. This update records the newer publication state without rewriting their mathematical derivations or raw qualification evidence. No benchmark or paid job was launched, and no merge is requested for this follow-up. Pages lookups returned no usable result before runtime resets; no Page was edited.
