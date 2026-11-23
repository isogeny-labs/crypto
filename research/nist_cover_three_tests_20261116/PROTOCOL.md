# Mixed 3-isogeny transfer test protocol — 2026-11-16

Request: comprehensively test the proposed P-192 d=7 mixed-3-isogeny and projected-relation route; preserve all curve families and eligible controls available in the preceding study.

## Development/static replay scope (no performance experiment)

Inputs: unchanged archived E/Prym torsion witnesses for P-192 candidate139, d7; all306 exported six-dimensional Frobenius/Weil modules in the preceding study. These are producer-unverified inputs. This audit is a new native implementation, not a signed independent PCC verifier.

1. Replay the degree8 finite field's irreducibility, validity/order/independence of four genus2 torsion divisors and two elliptic torsion points, Frobenius action, and six Prym Weil pairings from archived tangent functions.
2. Enumerate every reduced row echelon subspace of F3^6 in dimensions1,2,3. Check exact isotropy and Frobenius stability; compare counts with exports; verify full Frobenius order, multiplier, and alternating-form nonsingularity. Output all eight P-192 d7 maximal kernels, their restriction matrices, point-field degrees and intersections with each factor.
3. Negative controls: corrupted Frobenius, pairing, count and witness; malformed/duplicate/missing module inputs. All must reject. Preserve original evidence unchanged.
4. These checks validate the supplied finite algebra and certificates, conditional on source point/order identities. They do not independently point-count the cryptographic-size genus2 curves or construct their quotient maps.

## Full experiment requirements — not replaced by the replay

A. Build an explicit quotient from each of the eight kernels, with exact field, curve equation, polarization and actual kernel. Detect product/hyperelliptic targets and quadratic descent obstruction. Check all exceptional inputs.
B. For each valid Jacobian target, implement F=phi*pi^* and T=pi_*phi^dagger. Check TF=[6], homomorphism, exact order-r image, public scalar identities and recovery on E. Include zero, ramification, infinity and extension-supported divisors.
C. Derive an explicit algebraic restricted-base solver for T(sum n_i D_i)=aP+bQ, including a free ker(T) residual. Compare the SAME projected relations with direct elliptic norm/decomposition equations. Merge duplicate projected base images before rank; never count rational 3-torsion translations as distinct elliptic points.
D. Use the repository standard prime-field panel, plus the exact NIST/ANSI cases as construction inputs; document the deliberate non-panel P-192 structural test. Freeze public targets after reusable setup, retain hidden scalars only for independent correctness checks. Controls: direct elliptic equations, unmodified cover, product-kernel cases where available, random equivalent factor bases, quotient corruption, A/A, and strongest matched rho. Existing Tian F257 is a positive map control, not a P-192 quotient.
E. Freeze six or more growing sizes spanning24bits plus a held-out size before a scaling claim. Include at least five measured rounds per arm and size, warmup, complete cold per-target setup/failed attempts/maps/relations/linear algebra/descent/recovery/verification, peak memory and operation counts. Report the0/0.1/1GAE accounting band. No extrapolation from kernel field degree.

## Execution status and admission

The latest main journal reports the lab monthly cost above its cap. No paid build/run is admitted while headroom is unknown or exhausted. Preparing source and local unit/static checks is development work, not a substitute backend. Before a scientific launch: finish the actual candidate solver and quotient (no fixture substitution), commit/push source and protocol, obtain current budget/capacity readback, pin image and binary, pass S3 checkpoint and publication gates, and submit on EKS Spot. Proposed first construction attempt:2CPU/8GiB, deadline1800s, zero automatic retries, totalUSD5 ceiling including build/storage/idle; a later performance panel needs a separately frozen resource/cost record. Budget headroom alone does not supply the missing quotient/solver implementation.

Stop on the first arithmetic mismatch, unsupported quotient/descent condition, unverifiable map, or admission failure. Preserve failures; mark original requirements unresolved. No speedup, exponent or global nonexistence claim follows from this development replay. [pcc:unverified]
