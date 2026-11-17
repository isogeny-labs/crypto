# Independent field CRT and quotient-shape review — joint B

Reviewer: /root/quintic_fidelity_review. Inputs: field-crt at e45227d1b8f420414c71de1cbecd787b887f69e7 and optimized-quotient-shape at 660bbb72be (full resolved SHAs in metadata/commits.txt). Review plan at the latter commit was read before inspection. No new sibling reports read. Reviewer authored neither component. Historical context was inherited; this is independent source-fidelity review and fresh replay, not mutually blind re-derivation.

## Result

**Machine status: passed**, for all five field-CRT declarations, nine quotient-shape declarations and ten fixed controls, by fresh reviewer compilation/readback. **Fidelity: faithful_scoped** for their field-polynomial claims. **Outcome: validated_scoped for these named algebraic claims under reused dependency-cache trust**, not full native-word or Jacobian-class correctness. All hypotheses below remain explicit. No mathematical or source-formula mismatch found.

## CRT construction

support is the monic quadratic X^2+qX+r; delta is sX+t. Native differences are A=qa-qb, B=ra-rb, C=sb-sa, D=tb-ta. N=B^2-A*B*qb+A^2*rb, S1=C*B-A*D, S0=D*(B-A*qb)+A*C*rb. These agree with the inspected Rust optimized branch, including the ordinate orientation b.v-a.v.

support_bezout divides the support certificate by nonzero N and provides explicit polynomial coefficients giving 1. This is a field theorem, not an unjustified inversion over the previous arbitrary ring. numerator_composition is an identity; ordinate_congruences proves V=Va+S*Ua agrees with both ordinate residues, using N!=0. residual_of_congruence transfers each input residual divisibility by an explicit square-difference identity. product_residual combines the two via proved coprimality, not an assumed product-divisibility principle.

The field CRT statements deliberately do not require S1!=0: interpolation still holds at zero slope. Support squarefreeness and curve smoothness are not required for these polynomial identities and divisibility statements. The repeated-support fixture demonstrates this scope, without asserting smoothness of its curve.

## Quotient degree and leading coefficient

support_shape gives monic degree two without restrictions on lower coefficients. numerator_shape requires S1!=0 to give degree one. slope_shape uses both N!=0 and S1!=0 to give degree one and leading coefficient S1/N. Since Ua is monic degree two, V=Va+S*Ua has degree three: Va has degree at most one, so it cannot cancel the cubic term.

For deg(f)=5, deg(V)=3, the square has degree six and dominates f. Therefore f-V^2 has degree six and leading coefficient -LC(V)^2 over any field, including characteristic two. A monic degree-four divisor U gives a quotient of degree two and leading coefficient -LC(V)^2. The generic lemma assumes U divides the residual; the source_quotient_exists endpoint obtains that premise from product_residual, so the source endpoint does not assume the exact quotient identity it claims to construct. Its two input residual assumptions and N,S1 guards are retained.

Finally LC(Q)=-(S1/N)^2. Multiplication by -(N/S1)^2 makes leading coefficient one; inverse_lead_identity checks this cancellation with both quantities nonzero. The native expression is -square(norm*si), conditional on si=1/S1 from the batched inverse. The report states that condition and does not claim to have proved the word batch inverse or the pscale/trim bridge.

## Boundary attacks and controls

- N=0: the fixed control proves the equal-support norm-zero case is excluded. Bezout/interpolation claims do not silently extend across division by zero.
- S1=0 with N!=0: zero_slope_still_interpolates proves interpolation remains available, while degree-one/cubic/degree-two quotient conclusions explicitly require S1!=0.
- Degree six: the replayed sextic_cancellation control f=X^6, V=X^3 gives residual zero. The degree-five hypothesis prevents this cancellation. No weakened sextic theorem is claimed.
- Missing minus sign: missing_minus_fails over Q proves positive (3/2)^2 times negative (2/3)^2 is not 1. The actual scaling has both negative factors.
- Nonunit coefficients: inverse_lead_identity instantiated with N=3,S1=2 checks scaling beyond +/-1.
- Full source fixture: f=X^5-X^4-X^3+1 with Ua=X^2, Ub=X^2+1, Va=1, Vb=X+1 yields V=1-X^3. The controls establish input residual divisibility, f degree five, product residual and quotient degree two/leading coefficient -1. They prove hypotheses are jointly satisfiable; they do not prove a smooth Jacobian fixture.

All ten producer controls were freshly elaborated in this reviewer session. The independent audit additionally rejects empty output, forbidden sorryAx replacement and wrong theorem name for each readback set. The known-false 0=1 theorem was freshly rejected with exit 1. No theorem attempt unexpectedly failed in this review; successful logs and the expected-failure log are retained.

## Custody and exact verification scope

Sources were extracted from the two immutable commits into input/. Seven named modules were compiled with the same pinned Lean binary in a private directory; reviewed target .olean files were generated anew there. The search path puts this directory first, so quotient proofs consume the freshly built field-CRT modules. Original producer directories were not modified. Standard and helper dependency caches were reused and hashed before and after. Hash checks passed for helper sources/caches, recursively enumerated dependency unions, all initial review sources, and compiler binary. This does not constitute an independent clean build or binary rechecker of the dependencies. Original helper source-to-cache provenance is a retained trust boundary.

The native checker in audit.rs is new reviewer tooling, compiled with rustc. It verifies seven successful module exits, expected false exit 1, exact ordered declaration inventories derived from the immutable readback sources, total count 24, exact accepted line syntax and the allowed axiom set. It executes six malformed-output controls. acceptance.json records those outcomes. This machine evidence supports the named declarations, not all adjacent archived modules.

Current main was fetched and inspected separately at 27b3b8403914c2483bfbc0605300c5e68785e91b. Its root tree contains no ledger/ directory; the dated session journal was read and retained instead. That journal preserves expired scientific authorization and pending continuation. No producer-branch merge or source mutation was made. No official hypothesis promotion is part of this technical review.

## Remaining obligations

Native batch inverse decoding; UInt64 coefficient operation-order correspondence; quotient uniqueness and pexact mapping; pscale/trim equality with the monic operation; final negative remainder and strict storage; fast/fallback output comparison; divisor-class interpretation. These are explicitly outside the reviewed claims. No timing, cohort, cloud, scientific or budget change occurred.

## Sources read and attestation

Owned joint: B only. Read immutable review-plan.md; field-crt FastCRTInterpolation, FastCRTInterpolationControls, FastCRTInterpolationReadback, report/status; quotient FastInterpolationShape, FastQuotientShape, FastQuotientControls, FastQuotientReadback, report/status; original check.sh and dependency list; exact native three_leads.rs formulas in metadata/native.rs; source and logs of this review's replay and audit; current-main tree and session journal. Prior relevant AGENTS and review-evidence Lean-route instructions were retained from this session. No sibling new report was read. The prior strict-storage implementation is unrelated to the assigned B declarations and was not used as independent evidence for them.
