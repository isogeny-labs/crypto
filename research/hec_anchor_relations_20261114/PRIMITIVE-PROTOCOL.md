# Primitive normalization: frozen algebra and relation-yield gate

Frozen before implementation/run, 2026-11-15. Previous goal turn made progress:
the signed rational family passed its algebra/rank gates, but its cold-runtime
benefit is unmeasured. This gate asks whether discarded common-factor pairs
contain additional useful lower-degree relations without additional sign draws.

## Prior art and source receipt

D'Andrea, Krick and Szanto, Proposition2.1 and Theorems2.2/2.3:
https://arxiv.org/html/1211.6895v3 . Reducing a rational function to coprime
numerator/denominator is established. A homogeneous interpolation equation
at a simultaneous zero does NOT guarantee the reduced rational function
interpolates that datum; this is the accounting condition tested here.

Sources checked (KB): revision9141e386254340a9bdba3036f2bb706a, lexical-only,
refresh2026-11-14T21:44:48Z (predates current study). Searches:
"rational interpolation" "common factors"; "hyperelliptic" "primitive"
"relations". The first returned irrelevant hits, not novelty evidence.
Opened source:b3c90d39a347ef661ccf10680a742186 with metadata/context,
crypto:research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md:
retain phases, exact workload and cyclic ell-part gate. Primary paper read
directly for the normalization condition. No novelty claim follows.

## Algebra

For a nonzero pair let h=gcd(a,b) be monic, a'=a/h, b'=b/h. The norm identity is

    a^2-f*b^2 = h^2 * (a'^2-f*b'^2).

For each original distinct anchor xi, retain its signed place only if h(xi)!=0.
Let A_keep be the product over retained abscissae. At those anchors, dividing
the interpolation equation by h(xi) is valid, so A_keep divides the primitive
norm. Let Q'=(a'^2-f*b'^2)/A_keep. If original norm degree<=M+g, t=deg h and
k original anchors were removed, deg Q'<=g-2t+k<=g-t, since k<=t.
The removed anchors need not be zeros of a'-b'*y: never retain them by assumption.
Any zeros of the primitive function there reappear naturally as roots of Q'.
For a primitive pair, b' cannot vanish at a norm root unless b'=0; that case
has a' constant nonzero and gives an empty principal relation. Orient split
residual roots with a'/b', merge multiplicities and signed place coefficients,
then verify the full Jacobian identity. Branch points and repeated factors
of h are retained as controls. The zero pair remains rejected.

## Frozen checks and comparison

1. Unit gate: original9408signed curve cases from RATIONAL-PROTOCOL.md,
   now normalize every nonzero pair. Check primitive gcd, exact divisions,
   retained-node equations, residual bound, and all split principal identities.
   Require at least one recovered nonprimitive split case per genus.
2. Synthetic controls, explicitly outside the generator's degree bounds:
   y^2=x^5+x over F101, a=0,b=h; branch anchors chosen from x=0,1,-1,10.
   Test common factors at an anchor, repeated there, and away from anchors.
   Also test b=0 constant primitive function and zero-pair rejection.
3. Add an explicit --primitive arm to rational_rank.rs. The default rejection
   arm must remain byte-identical to RATIONAL-RANK-VERIFIED.jsonl. Run the
   same36arms, curves, H/d, tuple order, seeds, stop rules and caps. Primitive
   mode retains rather than rejects nonzero common-factor pairs, removes the
   invalidated anchors and continues factorization/deduplication/forest/rank.
4. Independently verify every resulting anchor rank with rank(R)-rank(R_off).
   Record nonprimitive count plus normalized admissions and normalized
   projected rows, all failures/exhaustions/caps, constructed and consumed
   outputs. Compare visited functions to required rank as a count ratio only.
   Rank stopping may change consumed prefixes; tuple generation is unchanged
   and independent of algebra/rank outcomes until the arm stops.

Falsifiers: any identity/rank/division mismatch fails correctness. If the
normalized pairs are duplicates, empty rows or still inadmissible, retain that
negative yield result. A count reduction alone is not a time ratio. There is
no timing experiment in this gate, no hidden target input, no external worker
or spend. Local native Rust correctness/count screen; <=60s per invocation,
<=256MiB target memory. Preserve all original protocols and raw outputs.

## Full-objective cost boundary

Even a strong rank-count result must pay all cold setup, subset construction,
unused batch outputs, common-factor normalization, orientations, target descent,
recombination, algebra and final verification. Do not multiply it by previous
timing ratios from different curves. A complete same-target performance
protocol and novelty assessment remain necessary for the >=10x objective.

## Synthetic-input correction, before rerun

The specified branch coordinates0,1,-1,10overF101lie on y^2=x^5-x,
not the mistakenly written y^2=x^5+x. The first run caught this by exact
polynomial division: all9408generated curve normalization cases passed,
but the synthetic control failed. Preserve PRIMITIVE-GATE-INITIAL.txt.
Correct only the synthetic curve's x coefficient to-1; retain every coordinate,
assertion, generated fixture, seed, count cap and pass criterion unchanged.
