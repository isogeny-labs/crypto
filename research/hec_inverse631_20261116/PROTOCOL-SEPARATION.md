# Separating-form amendment after retained B failure

The original solve stopped on B: msolve exit1, empty parametrization, reported elimination degree11 and recommendation to change separating form. Positive and D1 both pass exact twelve-equation verification at degree12. Preserve all original files.

One deterministic amendment is permitted: replace only the final separating equation for B with t=x1+2y1+4x2+8y2+16x3+32y3. The other eleven equations remain byte-identical. Do not change the source coordinates, alignment, target sections or solver options. Execute B-alt once with the same300s/two-thread cap and same independent verifier, still requiring degree6or12, squarefree eliminant, invertible denominator and every exact residual zero. No additional fallback is authorized by this protocol. Positive and D1 are not rerun. This tests the separation hypothesis; failure is retained without implying an invalid correspondence.

Freeze amended equation, collector, existing verifier and solver hashes; commit/checkpoint/gate before execution. Recovered target classes/signs and global transfer remain unverified even if this algebraic gate passes.
