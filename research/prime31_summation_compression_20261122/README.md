# Prime31 summation polynomial research

The current research checkpoint is [PR4977](https://github.com/isogeny-labs/crypto/pull/4977). It combines the exact rational quadratic representation, a constructive dense-factor-base argument, the signed pair-spectrum elimination, and the new rank-three S3 pencil analysis. The relation-solving asymptotic objective remains open. All16 registered lane claims currently have unverified PCC standing. [pcc:unverified]

## Current mathematical results

- [Rational quadratic system](iteration6/MATHEMATICAL_NOTE.md): exact prime-field orientation, explicit field guards, free-order comb completeness and the fixed-balanced-tree counterexample.
- [Fixed-source fibre density](iteration7/FIXED_CURVE_FIBRE_DENSITY.md): for the stated odd-order source and capped power-of-two fibre, at least1/6 of regular fibres have at leastN/4 liftable coordinates; expected setup includes rejected trials. [pcc:unverified]
- [Exact signed contraction and yield](iteration8/EXACT_CONTRACTION_AND_YIELD.md), followed by the [pair-spectrum resultant](iteration9/PAIR_SPECTRUM_RESULTANT.md): complete zero-subtotal and signed-output semantics, including misses.
- [Reusable S3 pencil](iteration10/S3_PENCIL_CONTRACTION.md): a rank-at-most-three value matrix, exact quotient target degree2D-1, and the restricted costs of dense representations. The discriminant test does not prune the actual rational pair spectrum. [pcc:unverified]
- [Smooth-modulus family correction](iteration10/FAMILY_SIZE_CORRECTION.md): Chang's existing prime theorem supports every fixed exponent0<beta<5/12 in the selected-field construction, with the source-curve and input-construction costs kept explicit. [pcc:unverified]

The new [root review](iteration10/ROOT_REVIEW.md) and the separate pencil/family reviews bind the exact written proofs. The [illustrated report](iteration6/REPORT.pdf), [state](STATE.json), [claim standing](CLAIM_STANDING.json) and [continuation](CONTINUATION.md) distinguish written results, original-source compile attempts and unfinished obligations.

## Actual native correctness outcome

The latest dedicated original-root job is [45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392) at source14a9f021b630fec0a7dd38cb724d71d68c99626b. Its five command steps attempted compilation, but an unchanged binary-hyperelliptic source module prevented every test and candidate program from executing. No new identity was issued; the subsequent separate-seed replay was not reached. The exact [failure receipt and artifact hashes](iteration6/evidence/ci-framework-restored-failure/receipt.json) and [source-history audit](iteration6/evidence/third-blocker-source-audit/AUDIT.md) are retained. [pcc:unverified]

The earlier spec and framework compile failures remain separately preserved. The manifest, boundary and spec repairs already present upstream were not replaced. The only source patch in merged[PR4962](https://github.com/isogeny-labs/crypto/pull/4962) restored four exact missing framework guard lines, with no deletions. Its merge does not establish a successful original-root build. [pcc:unverified]

Historical iteration1 [original-lock correctness](https://github.com/isogeny-labs/crypto/actions/runs/45089569480) passed41 library and5 component tests at its own source. Its multiplication diagnostic was adverse:17923 versus8816 for the sparse-product control. Those results do not qualify the newer rational-chain verifier. The earlier [report](REPORT.md), [protocol](PROTOCOL.md) and [development evidence](evidence/README.md) retain their original scopes. [pcc:unverified]

## Next exact query gate

For the actual Dickson-derived pair spectrumX, seek an index for zero components of the column product ofS3 values, including empty targets. In the D=Theta(B^2) regime and conventional four-summand full-row model, near-linear setup inD must be combined with queryD^theta for a fixedtheta<1/2. The matrix's ordinary rank, quadratic input degree and compact evaluation circuit do not supply that query algorithm. Root recovery, signed rows, rank, failures, storage and any certificate output remain charged. [pcc:unverified]

## Ongoing record

The existing research task remains enabled on its unchanged schedule. The [research Space](https://chatgpt.com/space/page_729b0b80bdb4819193f457c80ecabe84) was initially saved and read back; its latest update is unconfirmed because reconciliation reads did not complete. [The intended content](SPACE_PAGE.md) and [sync status](iteration10/SPACE_SYNC_STATUS.json) are retained here. Do not recreate the Page or replay an uncertain write.
