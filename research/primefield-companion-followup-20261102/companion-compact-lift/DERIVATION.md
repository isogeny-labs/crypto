# Compact scalar Schur unwind frames — 2026-11-02

Status: derived construction with static source checks. Numerical trials,
mutation controls, heap measurements and performance trials: **zero**.

## Question and source context

Can a scalar Schur lift use only the coordinate it consumes, and stop retaining
the source operator throughout descent? Inspected local references:
`companion-schur-lift/lift.py`, `companion-border-cache/border_schur.py`,
`companion-border-cache/solve_border.py`, and `structured-block-kernel/kernel.py`.
The preserved reference computes a whole vector and consumes its first entry.

[Bostan, Jeannerod, Mouilleron and Schost (2017), Sections 6.2–6.3](https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf)
supplies the established displacement/Schur context. This component specializes
ordinary block elimination to the already acquired borders. No novelty claim
for block elimination or inherited fast randomized complexity bound is made.
Prime coefficient field p>3; no field change or elliptic search is introduced.

## Derived identities

After the existing column and row permutations, write

    B = [ a  t^T ]       S = D - c a^{-1} t^T
        [ c   D  ]       h = b_tail - c a^{-1} b_0.

Here a is nonzero. The cached first row supplies t and the cached first
column supplies c; scalar a^{-1} is already computed by the caller.
For S z=h, set x=(a^{-1}(b_0-t^T z), z). Direct block multiplication gives
B x=b. For S^T v=0 and v^T h=1, set y=(-a^{-1}c^T v,v).
Then B^T y=0 and y^T b=v^T h=1. The existing final permutations undo the
column swap for x and row swap for y. Both swaps are involutions.

Thus the lift needs one dot product, one scalar multiplication and one
subtraction. Retaining both tails is necessary for this single-pass route
because the solution versus exclusion-witness outcome is unknown at capture.
The reference root equation/normalization check is unchanged.

## Implementation and ownership

`LiftFrame` copies the two canonical border tails before border release and
stores only these tails, inverse, b_0, row/column indices, shared meter, profile
and consumed flag. It retains no Schur/source/generator object, full RHS,
callback or bound method. The tails are tuples. Source and inverse provenance
is internal; a forged inverse or border is outside this contract. This is not
an independent certificate of the borders.

The frame validates the lift tail and direction before reserving one-use
ownership. Arithmetic failure after reservation leaves it consumed; it must
not be retried. Successful use clears both tuples and the two field scalars.
The profile is shared with the trace so its final values reflect unwind.
A caller retaining or mutating internal objects violates the normal solver
ownership contract. The shared meter does not retain source operators in the
inspected Meter implementation, but its audit blocks can still grow.

`solve_compact.py` is a separate route. It changes one import, frame capture
and release, and the unwind loop. A normalization audit restores those exact
substitutions and compares complete ASTs against the preserved solver.
The audit also checks declared frame slots and absence of numerical source
calls. These are syntax/inspection checks, not dynamic heap validation.

## Derived costs and storage

For one nonterminal node of dimension n and transported width w:

| Quantity | Released-border reference | Compact frame |
|---|---:|---:|
| Lift full source applications | ell | 0 |
| New dot field products | 0 | n-1 |
| New dot field additions | 0 | n-1 |
| Common lift scalar products / subtractions | 1 / 1 | 1 / 1 |
| Setup + RHS + lift source applications | 3+ell | 3 |
| Tail-copy slots at capture | 0 | 2(n-1) |

ell=1 for a nonzero input, else 0. The compact dot deliberately includes zero
coefficients and zero inputs. For zero solution tails the reference already
skips its full source application, so this branch introduces arithmetic rather
than saving a query. No runtime dominance claim follows. Dimension-one and
zero-source terminals create no frame.

The preserved kernel records 4w+8 native integer products per successful
full source application. Compact lifting removes ell(4w+8) such products;
it adds n-1 field products and n-1 additions. These are different units.
It also avoids the associated encoding/reduction work but no unmeasured
bit-length or elapsed-time gain is assigned. Setup cache scaling n(2w+5)
and all other phases remain charged.

Each live frame retains exactly 2n field-value slots (2n-2 border entries,
one inverse and one RHS entry), excluding integer metadata, object overhead,
shared meter and profile. At complete scalar descent from N, the sum over
n=N,...,2 is N(N+1)-2 field-value slots. This remains quadratic. The prior
frames retained references to 2nw generator entries, n RHS entries and a
scalar inverse per node; counting unique resident storage requires alias and
lifetime analysis. New frames do not keep those source/RHS objects alive.
Temporary border copying coexists with the current source/cache, and local
solver variables can retain current operators. Compression certificates and
trace history remain outside the frame-slot bound; no total-space bound or
measured peak reduction is claimed.

## Validation obligation and next opportunity

Keep the full upstream cohort and acceptance gate. Before acceptance, compare
primal/dual vectors and original-system equations across all pivot permutations,
singular/zero terminals, zero/nonzero tails and n=2. Independently recompute
field/native counters. Test malformed tails, consumed/released frames and
corrupted borders/inverses. Check reference lifetimes and meter aliases using
heap instrumentation; compare retained frames and audit memory separately.
A genuine total-cost comparison needs bit volumes and matched runtime.

Next investigate source-coordinate extraction from displacement generators to
avoid border retention, or stream/replay compressed certificates with explicit
recomputation cost. Neither construction is supplied here. The coefficient
representation and feature-annihilation requirements remain separate upstream
work. Existing cost admission forbids new numerical runs in this round.
