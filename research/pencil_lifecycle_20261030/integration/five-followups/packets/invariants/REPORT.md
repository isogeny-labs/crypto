# Constructor invariants and exceptional branches

Five field-level declarations compiled successfully. A separate process reads back their types and axioms. This is producer-side proof checking, not independent source-fidelity review. No scientific trial, cohort, cloud action, benchmark or repository edit was performed. Sources and command receipts are hashed in the accompanying JSON.

## New component results

1. `weight_decode`: reweighting `(q,r,s,t)` by `(z²,z⁴,z³w,z⁵w)` and decoding by the same nonzero factors returns the original coefficients.
2. `weight_validity`: the same round trip preserves `u | f-v²`, literally after substituting the weighted coordinates. This is a constructor invariant over a field; it does not prove machine multiplication semantics.
3. `negation_validity`: replacing v by -v preserves `u | f-v²`.
4. `shared_zero_q_implies_same_support`: if dq=q1-q2=0 and B=dq*(q2*r1-q1*r2)-(r1-r2)²=0, then q1=q2 and r1=r2.
5. `shared_zero_q_excluded`: after the distinct-support guard, B=0 implies dq!=0. This excludes the field interpretation of `shared_zero_q_delta` at gate.rs:963–966. It requires no Mumford-validity assumption, only the field domain and the exact preceding guard.

A first compile failed because the lemma name `pow_eq_zero` was unavailable. That source, stdout and receipt are retained in `attempt1/`; its `sorryAx` output is not evidence of a proved result. The corrected proof uses `mul_self_eq_zero`. Successful receipt and readback are distinct from that failed attempt.

## Representation and entry boundary

Define the proposed invariant I by cases: Identity; degree one with canonical words, z*w!=0 and decoded y²=f(x); degree two with canonical words, z*w!=0, monic decoded u of degree two, v of degree less than two, and u | f-v². Smoothness and a shared fixed field/curve identify the intended Jacobian. Primality and operation-counter headroom belong to the implementation environment, not the algebraic tuple alone.

The collector asserts `j.valid(p)` and `j.valid(q)` before `from_affine` (projective_state_collector.rs:153–160). `Jacobian::valid` checks degree, monic leading coefficient and polynomial remainder (three_leads.rs:972–981); it does not check primality, canonical ranges, or the smoothness of the quintic. `from_affine` uses z=w=1 and delegates degree-two construction to `weight_degree_two` (gate.rs:569–580,708–724). The new weight lemmas establish the algebraic coordinate/congruence part for degree two. Degree-one conversion, UInt64 correspondence, curve validity and canonicality of the transfer output remain source-refinement obligations.

Identity addition passes through its operand (606–625), hence preserves any operand invariant directly. Every other repeated transition calls `add_projective`: scalar multiplication, jump construction and walk initialization at collector:169–206, restart at232–238 and trial transition at255. Thus proving one closed dispatcher invariant would support an induction over these loops. The loop currently provides no independent validity assertion after each projective addition. Normalization (gate:531–568) decodes using two inversions for nonidentity outputs; it assumes nonzero weights, and does not itself establish the output congruence.

## Branch partition and remaining algebra

The table in branches.json records source guard order. Do not equate returned branch labels with prelude paths: equal-v dispatch returns the nested doubling label.

- 1+1 inverse returns identity; equal point after inverse exclusion doubles; remaining point pairs require distinct x. Field y² equality at a common x is the missing local bridge excluding a third ordinate. Distinct-x gives nonzero output w; nonzero y after inverse exclusion gives nonzero doubled w=2y in odd characteristic.
- 1+2 K!=0 generic output weights are z'=K*z2*z1*w2,w'=1, nonzero from the guard and input weights. K=0 cancellation returns the other support. Positive coincidence splits repeated support/tripling from double-plus-other; validity of extracted points and secondary generic nonvanishing remain.
- 2+2 same-u tests inverse first, equal-v second, then mixed. Existing weighted same-support theorem excludes mixed zero slope under validity; the new proof does not replace it. Extracted-point validity and the returned double remain obligations.
- Distinct-u B=0 shared support now has a proved field-level dq!=0 guard. Output point extraction, matching ordinate alternatives and secondary compositions remain to connect to validity.
- Distinct-u B!=0,C=0 outputs degree one with z'=B*w1*w2*z1*z2,w'=1. B!=0,C!=0 outputs degree two with z'=z1*z2*C,w'=B*w1*w2. Nonzero weights follow algebraically from guard/input nonvanishing; full numerator congruence and divisor-class meaning remain separately required.
- Doubling B=0,S=0 returns identity. B=0,S!=0 extracts then doubles a point: validity and nonzero ordinate must be established before claiming nonzero returned W. Other doubling branches have z'=z*B*w,w'=1 (C=0) or z'=z*C,w'=B*w (C!=0), so direct nonzero-weight reasoning applies.

## Literature and prior evidence

Read Ozbay Gurler–Hisil, Complete group law for genus 2 Jacobians on Jacobian coordinates, DOI10.1007/s13389-024-00351-7, sections2–4: https://link.springer.com/article/10.1007/s13389-024-00351-7 . Field characteristic !=2, smooth quintic model and valid reduced Mumford inputs are explicit. This work checks representation/dispatch connections, and makes no novelty claim about the law. Prior local `published-law-scope-review.md` and `law-remaining-obligations.md` distinguish formula identities, global class semantics and compiled execution. Existing `WeightedSameSupport.lean` handles a different zero-slope branch. Historical exact source bytes were compared to the current Git HEAD blob before relying on their line references.

## Next dependency

Independently review the five proposition statements and source mapping. Connect the residue/multiplication proof lane to these constructor and guard statements, then prove the branch output congruence and nonzero weights for generic 1+2 before using it in positive shared-support composition. No cost ratio is produced by these proof checks.
