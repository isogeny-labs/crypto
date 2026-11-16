# CT116 — Frobenius-orbit index is the projective source logarithm

**Date 2026-11-14. Scientific status: [pcc:unverified].** This is an exact, bounded statement about the **fixed** 11a1/59-to-2 toy construction. It does not prove a lower bound on all computational cross-characteristic maps or produce a scalable transfer.

## Existing work / external sources checked

- Main [#3877](https://github.com/isogeny-labs/crypto/pull/3877), global eleven-torsion field [#4102](https://github.com/isogeny-labs/crypto/pull/4102), and CT112–115 [#4130](https://github.com/isogeny-labs/crypto/pull/4130), [#4132](https://github.com/isogeny-labs/crypto/pull/4132), [#4136](https://github.com/isogeny-labs/crypto/pull/4136), [#4139](https://github.com/isogeny-labs/crypto/pull/4139). CT115's established Frobenius-four identity is the direct input, and its x-only transfer target is still **conditional**.
- Concurrent genus-three Jacobian/norm work in cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), Richelot cross-factor maps [#1511](https://github.com/isogeny-labs/cryptanalysis/pull/1511), and autoresearcher Prym work were searched and left untouched. This is not a second Cantor implementation.
- Frobenius characteristic-polynomial background: [Stanford elliptic curve point-counting notes](https://crypto.stanford.edu/pbc/notes/elliptic/count.html).
- The distinction between a generic-model lower bound and special-coordinate attacks: Victor Shoup, [*Lower Bounds for Discrete Logarithms and Related Problems*](https://www.shoup.net/papers/dlbounds1.pdf), Eurocrypt 1997. **Do not infer a new Shoup lower bound from this toy calculation.**
- Research-KB MCP server and repository wrapper unavailable in this connector-only execution. Repository evidence and external literature were used instead; no novelty claim. Conductor and project-wide cargo tests not executed in this scoped run.

## Exact theorem: index selection equals projective DLP

Let P=(19,22) generate E_59(F_59)[11] on

    E: y^2 + y = x^3 - x^2 - 10x - 20.

Let T=(t,u) be the fixed target order-eleven point of CT103/115 on

    E_2: y^2 + y = x^3 + x^2,

where t is the class of X modulo 0x130027 in F_(2^20), and u is its ordinate in F_(2^40). CT115 established Frob_2^4(T)=[7]T.

For 0 <= j < 5, define

    a_j = Frob_2^(4j)(t) = x([7^j]T).

The five a_j are pairwise distinct because the order of 7 in
(F_11^x)/{+1,-1} is five. They enumerate all Kummer x-classes
of the ten nonzero points in <T>. The scalar representatives are

    [7^j mod 11] = [1,7,5,2,3].

For an arbitrary public Q=[m]P, its intended sign-relaxed transfer
is x([m]T). Its unique orbit index j is precisely the unique j with

    m = +7^j or -7^j (mod 11).

Therefore **an algorithm that outputs this j has already solved the
source DLP up to sign**. One source comparison [7^j]P == Q, and if
necessary negation, recovers the exact m. This is an equivalence
between *orbit-index extraction* and this finite projective DLP,
**not** a lower bound on producing a field element a_j by another
non-indexed algebraic algorithm. It does not prohibit global
characteristic-zero root recognition, an efficiently computed cover,
or other special-encoding techniques.

## Independent exact toy computation

Python and independently implemented Node use different finite-field
multiplication and separate source group/BSGS implementations.
They compute the binary orbit as

    a_j = [2,65536,4697,17,4956]

in the fixed polynomial basis, with period five. The ten source public
points are mapped using a **five-entry source projective logarithm
table** constructed by source group additions; the target orbit is
then scanned and source sign verification recovers all ten discrete
logarithms. At the source the five distinct x-values at positive
multiples 1..5 are [19,20,43,46,22].

Important: the table is the cryptanalytic step and is explicitly
charged as a *cold* setup. The code takes public Q coordinates for
each online case, but its x lookup is not an algorithmic global
root recognizer. It bypasses the exact degree-60 number-field
construction, and therefore cannot be represented as an independently
discovered cross-characteristic speedup.

The source-only baby-step/giant-step control, freshly built for each
of the ten Q inputs, uses 4 through 6 elliptic additions (50 summed
over the ten independent cold runs), excluding its separate check.
The projective source lookup table takes four source additions to
populate five x classes (plus the initial copy of P), and the
five-element binary orbit takes 16 F_(2^20) squarings. The transfer
also incurs class selection, target orbit search and source sign
checks, which are not normalized to the same cost unit here.
Consequently **no fair cold-cost ratio** against optimized rho,
BSGS or an index-calculus implementation follows from these counts.

These are exact algorithmic counts for the stated source operations,
not benchmark wall times or amortized/whole-pipeline speed claims.
A size-eleven example alone is not evidence about P-192, P-224 or
larger ECDLP instances.

## Reproducible commands

From repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct116
    python3 -B "$D/ct116_orbit_index.py" > /tmp/ct116-py.json
    python3 -O -B "$D/ct116_orbit_index.py" > /tmp/ct116-pyopt.json
    node "$D/ct116_orbit_index_independent.js" > /tmp/ct116-js.json
    node --no-opt "$D/ct116_orbit_index_independent.js" > /tmp/ct116-jsopt.json
    cmp /tmp/ct116-py.json /tmp/ct116-pyopt.json
    cmp /tmp/ct116-py.json /tmp/ct116-js.json
    cmp /tmp/ct116-py.json /tmp/ct116-jsopt.json
    printf '%s  %s\n' e356ec49a6c53a942fa617b1d199e368c000e213c94c2b056448298979eac997 /tmp/ct116-py.json | sha256sum -c

The developer-local scripts matched in ordinary and optimized Python
and Node modes, with SHA-256
e356ec49a6c53a942fa617b1d199e368c000e213c94c2b056448298979eac997.
Scoped exact-source CI is defined in
.github/workflows/ct116-orbit-index.yml; signed scientific PCC,
full repo validation and review gates remain separate. [pcc:unverified]

## Next constructive gate

CT116 shows that a **scalar-labelled choice of a Frobenius orbit root**
is not progress toward solving a hard DLP: the label itself is the
projective logarithm. The route is still open to a *coordinate-native*
algorithm that obtains the correct global x-conjugate without
extracting a projective scalar or building the five-entry table.

Propose a fixed-place exact recognition algorithm with a bounded
height/precision certificate. Require it to accept encoded Q, produce
a characteristic-zero polynomial expression of the corresponding
global root, verify the source specialization and a fixed binary
specialization, and report its cost *including* any root factoring,
enumeration, precomputation and failed recognition attempts. Only
then time the target x-class log and sign check and compare against
matched rho/BSGS/index calculus under the existing cold-cost protocol.

No EKS/Spot budget was spent, no protected merge was attempted, and
this scoped negative/control result does not alter the independent
genus-two/Cantor or Jacobian branches. [pcc:unverified]
