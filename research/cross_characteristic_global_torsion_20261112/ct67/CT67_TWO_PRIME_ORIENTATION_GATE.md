# CT67 — A two-prime orientation gate despite fixed binary Frobenius

**Standing: [pcc:unverified].** An exact, bounded arithmetic and Galois-label obstruction for the *fixed* Cremona 11a1 mod-3 instance. Not a complexity lower bound, a cross-characteristic algorithm or a complete ECDLP attack. This extends CT51/52/54, without changing CT64–66 or native genus-three work.

## Exact proposition and proof

Let E/Q be `y²+y=x³−x²−10x−20`. Source P=(18,5) on E(F29) has 2P=(18,23)=−P and exact order three; #E(F29)=30. Its binary reduction has #E(F2)=5, hence Frobenius trace t₂=−2. Its action on geometric E[3] has irreducible characteristic polynomial X²+2X+2 (mod 3); in an appropriate basis take

    F = [[0,1],[1,1]]  (mod 3).

We verify F⁴=−I and F⁸=I. As F has irreducible quadratic minimal polynomial, the centralizer of F in GL₂(F₃) is F₃[F]* ≅ F₉*, of order eight. In the one-dimensional F₉-module E[3], multiplication by F₉* acts *regularly* on the eight nonzero vectors.

**Therefore even choosing the literal binary Frobenius matrix, the binary curve and the extension degree 8 does not canonically determine which of the eight nonzero binary 3-torsion points is the image of the oriented prime-field generator.** Centralizer relabelings preserve F but can send a given generator to every nonzero point. In this example they are the eight powers of the binary Frobenius; these are cheap residue-field relabelings, not eight independent isogenies or evidence of large transport work.

Using CT52's proof of the *actual* global mod-3 image GL₂(F₃), |GL₂|=48, there are six conjugate Frobenius matrices, and the stabilizer of any nonzero oriented vector has size six. The full binary 3-torsion is defined over F₂⁸, not a smaller binary field. CT51 *does* supply explicit global point-field coordinates and both selected specializations, which fix one legitimate bridge. CT67 does not invalidate that earlier positive toy example.

**Important distinction:** once P's actual destination U has been specified, a group homomorphism necessarily sends every Q=[x]P to [x]U. However, merely specifying U does not compute [x]U from an arbitrary encoded Q. A globally coherent encoded-point evaluation mechanism, or the source discrete logarithm, remains necessary. Thus the result is a **noncanonical embedding / insufficient-label obstruction** to *local-data-only* bridges; it does not rule out efficient algorithms that supply the missing global data.

## Reproduction and independent checks

The Python program exhaustively enumerates all 48 invertible 2×2 matrices over F₃, their centralizer and conjugacy orbit; it also enumerates affine points of E(F29) and E(F2), and verifies source doubling. JavaScript independently works in F₉=F₃[t]/(t²−t−1), uses Frobenius powers rather than scanning GL₂, counts E(F29) by discriminant characters, and independently checks the same tangent witness. Both test that the target generator is *not uniquely fixed* by fixed F, while all scalar relations remain preserved for consistent orientation.

Run from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct67
    python3 -B "$D/ct67_relative_embedding.py" > ct67-py.json
    python3 -O -B "$D/ct67_relative_embedding.py" > ct67-py-opt.json
    node "$D/ct67_relative_embedding_independent.js" > ct67-js.json
    node --no-opt "$D/ct67_relative_embedding_independent.js" > ct67-js-noopt.json
    cmp ct67-py.json ct67-py-opt.json
    cmp ct67-py.json ct67-js.json
    cmp ct67-py.json ct67-js-noopt.json

All four producer-local executions produced byte-identical JSON SHA-256 `6a87f395affeadd1b492888b979e16fb256555ecff3c8ed67897592aadbce9c7`. This is **not an independently signed scientific verification**. The separate preexisting CT66 integrated job from [run 43406131237](https://github.com/isogeny-labs/crypto/actions/runs/43406131237) completed successfully; CT67 requires its own exact-head CI verdict.

## Required contract for a proposed cross-characteristic chain

Each hop must specify: (1) a common characteristic-zero torsion coordinate algebra / number-field model, (2) a source-prime specialization mapping a global point T to the *encoded* public P, (3) a chosen target-prime specialization, residue presentation and target generator U, (4) a *joint* procedure evaluating arbitrary encoded Q without an oracle for x=log_P(Q), and (5) construction, height, degree, inverse, transport, complete solving, and baseline costs. CT66 satisfies a separate **same-characteristic** quotient/dual portion, not these global obligations.

**Restricted negative finding:** binary local group structure, Frobenius trace, extension degree and even a fixed mod-3 Frobenius matrix do not determine a target orientation. **Unresolved:** arbitrary-input compatible global lifting and full-cost matched rho/IC comparison. No project-wide cargo benchmark, paid compute or protected merge.

## Research provenance, prior art and KB

Internal: CT51 README; CT52 Galois-orbit report; CT54 Frobenius filter; CT66 explicit dual; root [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877) and CT [PR #4015](https://github.com/isogeny-labs/crypto/pull/4015), plus cryptanalysis #1520/#1521 and the chained-transfer next-steps note. CT54 already warns about labeled single-prime Frobenius; CT67 provides a more concrete **two-prime fixed-Frobenius centralizer falsifier**, not a new general theorem. Literature: [Snowden, elliptic curves over DVRs](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html) for finite-étale torsion reduction; [Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack* (2000)](https://doi.org/10.1023/A:1008312401197) for a distinct, historically studied cross-characteristic lifting failure.

**KB unavailable:** no research-kb MCP tool and no checked-out wrapper in this environment. Fallback: inspected the cited repository files, cross-repo PRs and primary web sources. This is not evidence of novelty or absence of alternative algorithms. [pcc:unverified]
