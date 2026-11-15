# Independent same-support extracted-point review

Finding: validated at the stated field extraction scope. Seven declarations derive the extracted point's normalized validity and nonzero representation weights from two original Mumford congruences, equal normalized supports, a nonzero slope difference, a noninverse ordinate pair, and four nonzero weights. No extracted-point validity or smoothness premise is hidden in the input.

## Algebra and source comparison

Writing u=X²+qX+r and vi=siX+ti, the two input congruences give u dividing (v1-v2)(v1+v2). Subtracting the matching degree-two multiple leaves a degree-at-most-one remainder divisible by monic degree-two u, so it vanishes. This proves the exact factor identity and coefficient equations. If s1+s2=0 while s1-s2!=0, the coefficient equations force t1+t2=0, contradicting the explicit noninverse premise. Thus both slope factors needed by the root derivation are nonzero. The extracted x=(t2-t1)/(s1-s2) is a support root and y=s1*x+t1 satisfies point divisibility. No characteristic restriction beyond Field is required for this result.

Compared WeightedExtract.sourcePoint to retained add22_weighted lines928-947. Cross-scaled t1 uses z2^5*w2; t2 uses z1^5*w1. Source x=(t2c-t1c)*k, y=((s1*x+t1*(z2²*k²))*k³)*z2³, z=k*(z1*z2),w=w1 match. source_k_scale gives k=(z1*z2)^3*(w1*w2)*(ns1-ns2), then source_point_decode proves the exact normalized x/y and nonzero z/w. The normalized branch premises are explicit; this packet does not prove whole raw-word guard reflection or the final add22 return.

## Singular and inverse controls

The retained singular fixture f=X5,u=X²,v1=X,v2=2X satisfies the original congruences and extraction premises, but gives y=0. Its following native dbl weight is 2*y=0. This is a counterexample to inferring nonzero ordinate from these premises alone, not a failure of the stated extracted-point validity theorem. No smoothness assumption was introduced or used. A separate inverse-pair fixture shows that nonzero slope difference without the noninverse condition need not yield a support root. Both limitations are material and retained.

Reviewer added a nonunit-weight fixture: a=(0,-16,0,96,2,3), b=(0,-81,54,0,3,2), f=X5-X+1. They normalize to u=X²-1,v1=1,v2=X. Applying the actual source_point_valid theorem and independently simplifying sourcePoint yields normalized (1,1), with nonzero representation weights. This checks the cross-weight placement beyond the producer's unit-weight singular controls.

## Fresh evidence

All68 manifest/source bindings matched. Fresh SameUMixed, WeightedExtract, producer Controls, seven-theorem Readback and reviewer Controls passed, each on its first invocation. Named axioms are propext, Classical.choice, Quot.sound. Existing pinned Lean/dependency binaries were reused; no independent kernel-binary rechecker or full dependency rebuild was performed. Producer warnings and historical failures remain retained; reviewer did not repair producer files.

Reviewer did not author this candidate, but authored/reviewed prior mixed branch dependencies. This is an independent extraction review with that lineage involvement disclosed. The result covers normalized field-point validity and weight preservation only. Nonzero ordinate under a suitable smoothness premise, subsequent doubling, UInt64 guards, divisor-class addition, counters and compiler behavior remain separate. No scientific/native cohort or cloud work occurred; cost ratios are null.
