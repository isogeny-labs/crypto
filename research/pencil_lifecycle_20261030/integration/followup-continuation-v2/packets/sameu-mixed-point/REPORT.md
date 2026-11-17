# Same-u mixed extraction: validity and a singular boundary

Seven field lemmas compiled. From the two original congruences u|(f-v1²) and u|(f-v2²), where u=X²+qX+r and vi=siX+ti, the proof derives

(v1-v2)(v1+v2) = C((s1-s2)(s1+s2))*u.

The derivation subtracts the leading-coefficient multiple of u and proves the remaining divisible polynomial of degree at most one is zero. Nonzero s1-s2 and the noninverse guard imply s1+s2 is nonzero. Therefore x=(t2-t1)/(s1-s2) is a root of u, and (x,s1*x+t1) satisfies the curve congruence. No output-validity or smoothness premise was added.

The second module transcribes native gate.rs lines930–958 extraction operations as sourceK/sourcePoint. It proves the crossed slope difference scale, nonzero extracted z/w, and exact normalized x/y. The composed theorem takes original weighted input validity, nonzero input weights, equal normalized q/r, nonzero normalized slope difference and the normalized noninverse guard. These are stated explicitly. It returns extracted-point validity; it does not yet compose the literal UInt64 guard reflection or the subsequent doubling call.

## Checked counterexample to dropping the smoothness boundary

Over Q, take f=X^5, u=X², v1=X, v2=2X and unit weights. Both original input congruences hold, the slopes differ, and the ordinates are not inverses. The extracted point is (0,0), represented by sourcePoint with nonzero z/w. The fixed Lean control invokes the composed theorem on these inputs and separately proves its raw y=0. The following native dbl at line509 returns weight 2*y, hence zero on this singular input. Thus polynomial input validity and the mixed guards alone cannot establish nonzero doubling output weight. This is a singular-curve boundary, not a counterexample under smooth-curve hypotheses.

A second negative control omits the noninverse guard: u=X²+1, v1=X, v2=-X, f=X^5+X³+X². Both congruences and nonzero slope difference hold, but the candidate x=0 is not a root of u. This retains the need for the ordered inverse exclusion.

## Validation and next obligations

SameUMixed attempt3, WeightedExtract weighted4, controls4 and readback2 exited zero. All seven declaration readbacks list only propext, Classical.choice and Quot.sound. Earlier proof/import/algebra-normalization failures are retained. Native commands use existing pinned dependencies and a120-second cap; this is not a scientific run. Independent source review is pending. The PDF and SVG summarize component scope without cost estimates.

Remaining obligations: literal word/canonical guard composition; a precisely stated smoothness or local nonsingularity premise deriving nonzero extracted ordinate; composition with reviewed doubling; any divisor-class interpretation, counter execution, compiler semantics or whole-walk invariant. Full cost ratios remain null.

Source/literature: retained projective_law_followup_20261022/PROOF_AUDIT.md same-u mixed derivation, gate.rs literal extraction and dbl output, and previously reviewed MixedCancellation.extracted_valid. The prior publisher DOI lookup was unavailable; this task builds on retained source-bound algebra, not an unverified paper-completeness claim. Exact paths/hashes are in sources.json.
