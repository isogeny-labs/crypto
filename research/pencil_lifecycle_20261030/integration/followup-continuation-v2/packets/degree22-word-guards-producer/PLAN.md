# UInt64 ordered degree-two guard correspondence: producer plan

Implement the previously reviewed NEXT-WORD-PLAN. Primary local sources: projective_law_followup_20261022/PROOF_AUDIT.md and src/bin/projective_law_gate.rs lines727-753 and911-935; reviewed Guard22 and CrossScaling. This is a producer construction, not independent review.

Prime5<=p<2^31, degree-one Field with no lookup tables, canonical inputs. Preserve literal multiplication/subtraction association in B. Prove B canonical and decoded equal to doubleB. Under exact crossPreludeWord=some sameUDouble and Bword=0, derive a.s!=0 using the earlier inverse guard exclusion. No Mumford validity or nonzero-weight hypotheses are needed for this guard-only statement. No output-degree, payload-validity, compiler, counter or classsum statement.

Controls: a nonunit p7 sameUDouble/B0 fixture, p5 lower-bound fixture, zero ordinate inverse, zero-weight overlap returning inverse before equal, noncanonical input rejected by the hypotheses. Explicit separate axiom readback, all attempts preserved. Independent review remains pending. No scientific workload or timing claim.
