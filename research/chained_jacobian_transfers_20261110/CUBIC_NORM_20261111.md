# Cubic covers: arbitrary closed-point norm and chained pullbacks

This is bounded implementation work on the existing prime-field cover family,
not a new cover construction. All results below have unsigned PCC standing.
The source implementation is `cryptanalysis:experiments/chained-transfer-reference/src/cubic_transfer.rs`
at f91a5e061ca5cfd777688afb65ab6972306618d1, ready PR1475;
the independent replay is [`examples/chained_cubic_norm_reference.rs`](../../examples/chained_cubic_norm_reference.rs).
No benchmark has run. [pcc:unverified]

## Prior art and the exact gap

The existing `research/enge_primefield_cover_20261024/REPORT.md` constructs
`C_{d,s}: y²=f(t^d+s)`, discusses the published Enge bound and implements
rational linear/target collectors. Its open problem
`problems/prime-cover-projected-relation-cost-missing.md` asks for closed
primes, a complete divisor norm, replay and total-cost evidence. This work
addresses the bounded genus4 norm prerequisite. It does not repeat the old
collector or close its relation-yield and linear-algebra gaps.

Galbraith, corrected ch.8 §8.3, especially Theorem8.3.8 and Corollary8.3.10,
supplies the Picard pullback/norm identities; ch.10 §§10.3–10.4 supplies the
odd-degree Mumford model. The source PDFs and retrieval gaps are in
`SOURCES_20261111.md`. The internal survey's generic upper-bound comparisons
are not lower bounds excluding every specialized algorithm. KB/atlas access
remains unavailable, and no novelty claim is made.

## Explicit maps and their domains

Use the pre-existing public Enge fixture

    E/F101: y²=x³+x+46, #E=109, G=(2,37), Q=(64,19).
    C9: y²=x9+x3+46, genus4.
    C27: y²=x27+x9+46, genus13.

Both curve morphisms are `(x,y) ↦ (x³,y)`, of degree3. They have a unique
rational point at infinity, with ramification index3. In characteristic
different from2,3, smoothness follows from the squarefree cubic and its
nonzero constant. For a canonical class `(u,v)` on genus g=1 or4, its whole
pullback is `(u(X³),v(X³))`. This includes irreducible and repeated support,
and is already reduced: its degree is at most3g≤3g+1, the target genus.
No rational-root choice or unknown DLP scalar is an input. [pcc:unverified]

The pullback is injective on these represented rational class groups:
coefficient deflation recovers the exact canonical source pair. The code
checks both deflation and re-expansion. This is an image-restricted inverse,
not a norm on arbitrary genus13 classes. Four core Lean structural lemmas
prove coefficient left-inverse/injectivity and closed-orbit trace invariance;
they do not construct finite fields or Picard maps (PCC-e1a827).
[pcc:unverified]

On the **entire canonical genus4 domain**, the implemented norm factors monic
`u=∏q_i^e_i`, with irreducible degrees at most4. It strips all linear factors;
a remaining quadratic/cubic is irreducible, while degree4 is tested against
every monic quadratic. Reconstruction of `u` is mandatory. For each factor,
in `Fp[X]/q_i` it computes the elliptic point `(X³,v(X))`, sums all
`deg(q_i)` Frobenius conjugates, checks orbit closure and descent to Fp,
then multiplies the trace by `e_i`. Summing those contributions, with
infinity as the elliptic identity, is the divisor pushforward. This handles
the residue-degree weight automatically, even if the image point has a
shorter Frobenius orbit. The construction uses only the input divisor.
[pcc:unverified]

The prototype checks primes5≤p≤101, nonzero constant and smooth models using
the pre-existing BigUint curve validator. Factorization is deliberately
bounded to degree4. No arbitrary-genus13 norm is implemented; the identity
`N₂(f₂*D)=3D` supplies only an image-restricted second norm. On E[109],
`N₁f₁*=[3]` and the composed restricted norm gives[9]; inverses are73 and97
modulo109. These recover points, not DLP scalars. [pcc:unverified]

## Independent arithmetic and retained controls

The development tests cover every109 source scalar, both pullbacks, actual
first norm and restricted recovery. A secondary predeclared
`E/F17:y²=x³+2x+2`, order19, generator(5,1), checks all361 ordered subgroup
pairs through both pullbacks. Multiples0..16 of the ramified point(0,6) on
C9 check multiplicities and reach degree12 on C27. [pcc:unverified]

The fixed set `kA` and `kA+B`, k=0..64, A=(26,37), B=(4,19) on C9, produces
130 canonical norm witnesses with closed support degrees1,2,3,4. Their norms
are independently reconstructed as `kG` and `kG+Q`. The root replay imports
no new norm code: its pre-existing genus4/13 Cantor engine reconstructs the
source/pulled classes, its affine elliptic engine reconstructs norms, and a
different Frobenius-gcd criterion confirms irreducible cubic/quartic support.
PCC-2d5c08 binds the TSV and passing replay receipt. Both Jacobian engines
use Cantor; this remains producer-authored development evidence, without a
signed independent verdict. [pcc:unverified]

The initial compile's integer-type error is retained in the consumer's
`evidence/cubic-transfer-01.txt`; its passing four-test development run is
`cubic-transfer-02.txt`, PCC-daf60f. The first full wrapper attempt stopped
before compilation because Cargo was absent from PATH; the corrected attempt
adds the installed toolchains. No prior receipt is overwritten.

## Quadratic-route restriction, at its actual scope

If an elliptic curve over an odd prime field has odd full rational group
order, its short Weierstrass cubic has no rational root and is irreducible.
For a root α of `f(X²+s)`, `α²+s` has degree3 over Fp; hence the tower law
gives3 dividing `[Fp(α):Fp]`. Thus the sextic has no linear or irreducible
quadratic factor. This prevents our constructor's three **individually
Fp-rational quadratic factors** and its rational-Weierstrass odd chart.
The E109 fixture exhaustively checks the factor restriction. [pcc:unverified]

It does **not** rule out an Fp-rational Richelot kernel represented by a
Frobenius-stable unordered triple of nonrational quadratics. Sextic Frobenius
type(3,3) can permit such triples. Balanced even-sextic arithmetic and an
explicit descended correspondence are prerequisites for assessing that route;
neither is supplied by the old rational-factor constructor. This is the next
bounded implementation task, not a universal no-transfer theorem.

## Computational and integration limits

The chain preserves full prime-order109 and19 groups but remains inside
explicitly recognizable pullback images. An ambient genus increase is no
evidence of a faster solver. No relation search, solver comparison, cost
ratio, cryptographic-size support or cross-characteristic descent is claimed.
Prime-field proper-subfield Weil restriction remains unavailable; this is a
same-field construction. The original Enge negatives and outstanding cost
problem remain open.
[pcc:unverified]

The original crypto PR3877 still has its PCC ledger-prefix fork and archived
receipt blockers. The completed root library run is6184 pass/13 fail/133
ignored, PCC-8ffeb3. Restoring tracked files fixes6 failing tests; the remaining
5 lack archived raw data and2 lack container time-namespace metadata. The
full run remains failed. No history, receipt or test has been rewritten.
[pcc:unverified]

S3 checkpoint admission, live IsoLab budget/capacity, KB and the encrypted
autoresearcher vault remain unavailable. The persistent queue is `QUEUE.md`
and crypto issue3904. These gates block experiments, not local proof and
correctness development. Source hashes, exact consumer commit and reproducible
commands accompany the checkpoint manifest.
