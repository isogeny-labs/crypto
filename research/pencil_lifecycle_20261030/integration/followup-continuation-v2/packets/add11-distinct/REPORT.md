# Distinct 1+1 branch: normalized validity preservation

The actual add11 distinct formula, restricted to unit-weight degree-one inputs, now has a compiled field-level preservation proof. From two degree-one Mumford-valid inputs with distinct x-coordinates, the returned weights z=1 and w=x1-x2 are nonzero, and the normalized monic degree-two support divides f-v². A separately compiled branch-precedence lemma selects distinct: both earlier inverse and doubling guards require equal x.

## Exact formula and proof

For input points (x1,y1),(x2,y2) with unit weights, source add11 returns q=-(x1+x2), r=x1*x2, s=y1-y2, t=x1*y2-x2*y1, z=1, w=x1-x2. Normalization yields u=X²-(x1+x2)X+x1*x2 and v=((y1-y2)/(x1-x2))X+(x1*y2-x2*y1)/(x1-x2).

The theorem assumes (X-C x_i) divides f-(C y_i)² for each input. This is the degree-one Mumford condition, equivalent mathematically to point validity; no theorem here connects native j.valid to it. Define m=(y1-y2)/(x1-x2), c=(x1*y2-x2*y1)/(x1-x2), v=C m*X+C c. The proof shows v-C y_i=(X-C x_i)*C m, so each support divides f-v². An explicit constant Bezout identity for the distinct supports proves their product divides f-v². Expanding that product gives the exact returned u. No factor-theorem oracle or additional dependency was installed.

The source order is inverse first, doubling second, distinct last. Since x1!=x2, neither preceding guard holds, and the native distinct-x assertion is satisfied at this field-model scope. The overlapping inverse/equal point (x,0),(x,0) takes inverse, retained in the controls. The output-weight proof uses precisely the distinct-x hypothesis; it does not silently assume all arbitrary inputs have nonzero difference.

## Source mapping and previous results

Gate projective_law_gate.rs lines511–532 contains add11; from_affine lines569–580 assigns z=w=1 to degree-one inputs. add_projective lines606–625 dispatches a degree-one pair to add11. Collector lines153–160 checks mapped affine inputs before conversion and then uses this dispatcher throughout scalar/jump construction. Source hashes match the retained copies already checked against current HEAD in the constructor packet. Prior InvariantBranches proves negation/reweighting and weight guard facts, but not this output congruence; inverse-to-identity alone would have added only the immediate identity invariant. This proof therefore addresses the next nontrivial formula case without claiming general dispatcher closure.

This is a mathematical field theorem for unit input weights. It does not prove UInt64/counter correspondence for this payload, the general nonunit-weight 1+1 formula, the doubling branch, divisor-class addition semantics, or loop closure. The curve polynomial is arbitrary in the theorem; smoothness/odd-characteristic quintic requirements enter the intended Jacobian interpretation separately.

## Checks, controls and errors

Both Add11 compilation stages pass; stage2 adds branch dispatch and explicit normalized-output corollary. Separate axiom readback and controls are retained. Controls instantiate f=X^5-X+1 over Q at points (0,1),(1,1), prove both input divisibilities explicitly and apply the full output invariant. Distinct, doubling and overlapping inverse precedence are checked. Equal x makes w=0 in the negative boundary control.

The initial API probe failed because the pinned minimal library lacks polynomial evaluation/factor-theorem modules. That output is preserved; AMENDMENT.md was frozen before implementation to use degree-one divisibility and an explicit Bezout proof. Lean reports only stylistic unnecessarySeqFocus warnings for the successful proof, retained unmodified. The first control compile required marking the polynomial abbreviation noncomputable; that failed attempt is retained and a subsequent compile checks the correction. Initial PDF rendering lacked the amssymb package import and is preserved before correction. No scientific execution, new dependency installation, root edit or cost comparison occurred. Independent source-fidelity review is pending.

Literature/background: prior source-hashed InvariantBranches obligations and Ozbay Gurler–Hisil complete-law paper, DOI10.1007/s13389-024-00351-7 sections2–4, establish the Mumford representation and formula assumptions. This packet proves a source-local preservation property, not a new addition law.
