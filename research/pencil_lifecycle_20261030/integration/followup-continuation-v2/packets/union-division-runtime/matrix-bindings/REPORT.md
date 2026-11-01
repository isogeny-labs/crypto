# Retained Kummer matrix certificate binding audit

The native read-only checker verifies all 105 ordered action matrices in the retained 21-model set. Every modulus is prime and at most 2^32; every matrix is exactly 4×4 with canonical entries; every square is cI with nonzero c, matching the retained square_scalar at the same action index. Each original root is canonical and distinct within its model, and the output preserves order while recording the literal loader normalization (root-r0)/delta.

## Exact inputs and source revisions

The checker compares theta/generic JSONL digests to the N16 summary, requires exactly the frozen 21 prime keys across all three inputs, and rejects duplicate model keys. It also hashes all twelve source_digests entries against locally recovered bytes: theta and generic data, historical protocol, N16 source, historical walk source and seven target-gate files. Target-gate files are hashed only; no point records are enumerated or recomputed. The preprovided theta file supplies its exact matching pin; an initial local-path collector could not locate that file in its candidate checkouts, and the resulting fail-closed audit is preserved in unavailable-pin-attempt1.

Historical walk pin: 8ce693805a5814327e88872dd33e95d47434d602a3dfe076c8f80cb9381ea2e2.
Supplied walk source: 0bd17210577ba0107c28082318c91b8acc6b375760ba30c3761199a6ffd72ff7.
These are different revisions. The historical source caches theta inverses; the supplied source uses uncached inversion. walk-source.diff preserves the change. The extracted literal linear and model-loader sections compare byte-for-byte equal. This source agreement does not identify an executed binary or establish its actual runtime inputs.

## Arithmetic and membership

The fixed prime list is explicit in src/main.rs. Trial division checks primality; modulo multiplication and four-term square sums use u128. With p≤2^32 and entries<p, each product is below2^64 and the four-term sum below2^66. The normalization denominator is required nonzero and canonical; exponentiation by p-2 gives its inverse in the checked prime field. The retained action index and scalar comparison prevent silently reordering the five matrices. Exact model membership and five-action checks prevent silent downscaling.

The supplied native walk uses narrow u64 products and successive modular additions. The existing word-multiplication theorem addresses products under canonical p≤2^32 inputs. This packet does not attest the full native row-fold or compiled execution semantics. It supplies concrete fixed coefficients and scalar-square premises for the separately checked field repair theorem. Matrix identities alone do not supply the geometric interpretation of the action or xadd.

## Fixed controls and fail-closed behavior

Eleven built-in controls cover identity and doubled permutation success; zero matrix, unequal diagonal square, wrong shape, noncanonical coefficient, altered digest, composite/oversized modulus, zero normalization denominator and duplicate prime key rejection. Three additional copied-input controls reject reordered saved scalar certificates, a missing summary model, and an altered source digest. They terminate with nonzero native exit and accepted=false. Each retained negative receipt records the actual error.

The final audit returns accepted=true only after all checks and the unchanged-input readback. accept.jq independently requires21 models,105 actions, twelve matching pins, scalar equality and all fixed controls. Final replay matches the earlier passing result exactly. Exact source/lock/toolchain/binary and before/after input hashes are retained. No scientific trials, point enumeration, trajectory generation, cloud operation, cost measurement or repository edits occurred.

## Review status

This is a producer packet pending independent source-fidelity review and publication binding. Historical pin agreement is a custody consistency check, not independent authentication of the historical experiments. Native row-fold and compiled runtime-input provenance remain explicitly false in the standardized result; no cost ratio is generated.
