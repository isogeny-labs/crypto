# CT65 — Frobenius Galois orbit of explicit elliptic quotients and trace projector

**Status: [pcc:unverified].** This is a further same-characteristic structure/control result following CT62–64, not a prime-to-binary source-point lift, an ECDLP attack, or a runtime speedup.

## Concrete object

Over F4 take C: y²+y=x⁵+omega x³, omega²+omega+1=0, with Jacobian J. Over F1024 use the bit polynomial 0x409 and omega=236. The five nonzero translation parameters with genuine order-two lifts in CT64 are exactly

    20, 534, 619, 809, 832.

The q=4 Frobenius sends an automorphism's coefficients (r,a,b,c) to their fourth powers, preserving the defining curve because omega lies in F4. It permutes the parameters in the **single five-cycle**

    20 -> 832 -> 619 -> 809 -> 534 -> 20.

Each r has two Artin–Schreier choices of c (differing by 1); the ten resulting involutions form **two disjoint length-five Frobenius orbits**. In particular, the CT64 involution (20,6,120,432) is carried by the **fourth** Frobenius power to (534,794,636,668). The two quotient maps used in CT64 are *Galois conjugates* rather than unrelated arbitrary factor choices.

An individual involution in these orbits cannot descend to F4 or F16, and this exact parameterization requires definition over F_(4^5)=F1024. This does not say that every conceivable geometric quotient has the same coefficients; CT63's F4-simplicity separately rules out any elliptic factor over F4.

## An explicit, noninvertible descent projector on 31-torsion

CT63 gives the Weil polynomial of Frobenius F=Frob4 on J:

    W(X)=X⁴+2X³+4X²+8X+16=(X⁵−32)/(X−2).

By Cayley–Hamilton for abelian varieties, F⁵=[32] on J. Therefore F⁵=1 on J[31]. Modulo 31, W(X) has **four distinct roots 1,4,8,16**. Consequently J(F1024)=J[31] has dimension four over F31 and the F-fixed subspace has dimension exactly one.

Since 5 is invertible modulo 31, the operator

    e = [25] (1+F+F²+F³+F⁴)    on J[31]

is an explicit idempotent with image **J(F4)[31]**, a one-dimensional subspace, and kernel of dimension three. On J(F4)[31], e is the identity. Thus it is a computable Frobenius-trace *retraction*, but **not** an isogeny/isomorphism on J[31]; it kills 31³ of the 31⁴ torsion points.

For Mumford representations over F1024, F is coefficient-wise fourth powering and the five summands use the ordinary Jacobian group law. The formulas are algorithmic once full F1024 Cantor arithmetic is available; this report does not claim an implemented arbitrary-F1024 divisor transport or a cross-characteristic inverse specialization.

The exact Frobenius companion matrix is independently checked in Python via modular matrix exponentiation, idempotency, row reduction and kernel rank. JavaScript independently checks the eigenvalue action, while both independently enumerate binary-field Galois orbits. Four local execution modes matched for their common JSON invariants (Python regular/optimized; JS regular/no-opt).

## Proof scope, prior art, next steps

The algebra follows standard Weil polynomial/Cayley–Hamilton, finite Galois descent and Maschke averaging; no novelty claim for these general theorems. Reuses the exact CT62–64 construction and the CT64 sources (Kani, Howe–Zhu). Shared research-KB MCP and checkout wrapper were unavailable; fallback reviewed repository evidence, issue #3904 and root PR #3877. The projection is **restricted to 31-torsion**, not an integer endomorphism e=[25]Tr satisfying e²=e on all of J.

Reproduction from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct65
    python3 -B "$D/ct65_galois_projector.py"
    python3 -O -B "$D/ct65_galois_projector.py"
    node "$D/ct65_independent.js"
    node --no-opt "$D/ct65_independent.js"

Next: independent arbitrary-F1024 Mumford/Cantor and dual-isogeny tests, including nonsplit and ramified divisor controls; then any attempted source-prime-to-binary lift must supply compatible arbitrary encoded P,Q without prior DLP and charge global-field/transfer/solve costs. This trace projector does not furnish that missing step. No paid compute, root/Cantor changes, protected merges or speedup claim.

[pcc:unverified]
