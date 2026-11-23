# Frozen reviewer boundary cases

1. Over Q, q=r=t=0,s=1, both representations identical with z=w=1. Equal crossed guards hold, inverse is false, B=0 and s!=0. This makes the permitted b_zero_point arm nonvacuous.
2. Direct call over Q with s=t=0 has B=0 and identity branch; it must not be excluded without inverse-false reachability.
3. Over F2, s=0,t=1 gives B=0 but t!=0. This attacks any omitted 2!=0 premise in the local scalar implication. Equal-arm precedence is a distinct issue.
4. Zero weights force both crossed ordinates zero, so equal and inverse predicates can coincide. Inverse-first must remain explicit; no automatic nonzero-weight premise should be imported.

These are proof/software fixtures, not scientific samples. Review source and producer before deciding exact Lean syntax. No producer repairs.
