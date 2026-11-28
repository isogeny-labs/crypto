# CT105 — The degree-120 transfer is a degree-10 automorphism torsor over a degree-12 field

**2026-11-14 · scientific standing: [pcc:unverified].** New exact algebraic decomposition, two independent finite-field verifiers, and a working *local* scalar-blind public-point lift. This is **not** an arbitrary-input global lift in the fixed field or an ECDLP speedup.

## Prior evidence and scope

Reviewed crypto root PR #3877, CT102–104 in stacked PR #4102, newest session log / AGENTS.md, cryptanalysis PR #1523 (degree-13 Jacobian transfers), crypto-autoresearcher PR #3282 (genus-5/Humbert cover lead). Those independent branches remain untouched. Repository research-KB MCP, local wrapper checkout, Conductor and EKS credentials are not exposed in this GitHub connector execution; no novelty or exhaustiveness claim. Primary classical background: [Snowden, Elliptic curves over DVRs, Lecture 8](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html) (finite étale prime-to-residue-characteristic torsion), [Hensel's lemma](https://en.wikipedia.org/wiki/Hensel%27s_lemma), and standard elementary Galois theory. The proof below relies on concrete CT102–103 certificates rather than a full GL2 image assumption.

For E: y²+y=x³−x²−10x−20, CT102 proves that the oriented point field

    K = Q(T), T=(X,Y)∈E[11],  [K:Q]=120

is a field. CT103 fixes source rho59(X,Y)=(19,22) with #E(F59)=55 and fixed binary rho2(X,Y)=(t,u)∈E(F_(2^40)), each a quotient of the same global order. CT103 also proves the source good-reduction specialization identifies E(K)[11] **bijectively** with E(F59)[11]≅Z/11. In particular E(K)[11]=<T>; this is crucial.

## Theorem A: exact automorphism group, unconditional for this K

Every unit m∈(Z/11Z)× gives an automorphism sigma_m of K over Q, defined on its two generators by

    sigma_m(X)=x([m]T),   sigma_m(Y)=y([m]T).

To prove existence, [m]T belongs to E(K)[11], and [m^(-1)]([m]T)=T, so Q([m]T)=Q(T)=K; its pair of coordinates satisfies the same defining eleven-division polynomial and curve equation. Thus the map of generators extends to an automorphism of K. These maps compose by sigma_m sigma_n=sigma_(mn), and their images of T are distinct.

For the reverse containment, a Q-automorphism sigma fixes the curve equation and eleven-division relations; hence sigma(T)∈E(K)[11]=<T>. Since sigma(T)≠O, sigma(T)=[m]T for exactly one unit m. The coordinates of T generate K, so sigma=sigma_m. Therefore

    Aut_Q(K) ≅ (Z/11Z)× ≅ C10       (exactly ten automorphisms).

No Galois-image surjectivity or unproved automorphism enumeration is needed. **K/Q is not asserted to be Galois.** By Artin's fixed-field theorem the fixed subfield M=K^Aut has degree

    [M:Q]=12, [K:M]=10,   Gal(K/M)≅C10.

The coefficients of the subgroup kernel polynomial

    H_T(Z)=product_(m=1..5) (Z−x([m]T))

are invariant under all ten automorphisms and generate this degree-12 cyclic-subgroup field: their geometric conjugate records the *unoriented* one-dimensional eleven-torsion subgroup, and 120 nonzero vectors / 10 units give twelve possible lines. No global explicit degree-12 defining polynomial is constructed in this rung; obtaining a short such model is a next-step experiment.

## Theorem B: different splitting at the fixed two primes

Use CT103's actual Frobenius companion matrices modulo eleven:

    A59=[[0,7],[1,5]],      A2=[[0,9],[1,9]].

Direct exact matrix powers give A59^5=I and **A2^4=7I**. The unit 7 has multiplicative order ten modulo eleven. Thus Frob2^4=[7] *on all eleven-torsion*.

For the fixed binary residue map of CT103 this gives the exact identity of homomorphisms on the global order:

    rho2 ∘ sigma_7 = (z ↦ z^(2^4)) ∘ rho2.

It suffices to check the two generators X,Y: the binary Frobenius-four point equals [7]U. Every sigma_m is a power of sigma_7. Hence each of the three binary primes of K is fixed by the entire C10 action. Its residue field F_(2^40) has the faithful order-ten Frobenius-four subgroup; the corresponding residue field in M is F_(2^4). Each fixed binary prime is **relatively inert of residue degree ten** in K/M, and

    splitting of 2 in M: 4^3;
    splitting of 2 in K: 40^3.

For source 59, Frobenius has two projectively fixed lines and two projective orbits of length five. The degree-12 line field therefore splits as

    splitting of 59 in M: 1^2 5^2.

One fixed line has Frobenius eigenvalue one, containing all ten oriented rational source points; the other fixed line has eigenvalue four and its oriented points have degree five. Accordingly, **the ten degree-one source primes of K above 59 lie over a single degree-one prime of M and split completely in K/M**. This is a sharp contrast with the binary prime's relative degree ten, not a cross-characteristic field morphism.

Two separately coded Python/JavaScript programs exhaustively enumerate all 120 nonzero vectors and all twelve projective lines, reproducing source oriented 1^10·5^22, source projective 1^2·5^2, binary oriented 40^3 and binary projective 4^3. They verify A2^4=7I and that 7 generates F11×. This finite-module check is not a second independent recomputation of CT102's field-degree proof.

## Scalar-blind online *local* inverse specialization (implemented)

A public source Q=(x0,y0) is admitted **without a subgroup generator or multiplier** by checking

    psi11(x0)=0 mod 59,  y0²+y0=x0³−x0²−10x0−20 mod 59,
    psi11'(x0)≠0 mod 59,  2y0+1≠0 mod 59.

The complete list of such x-coordinates is 19,20,22,43,46; each has two ordinates. Thus its public encoding directly defines the unramified degree-one prime ideal

    p_Q=(59,X−x0,Y−y0)

of the common global order. No lookup of Q in an mP table is needed for **this prime identification**.

A digitwise Hensel implementation and an independently written BigInt precision-doubling Newton implementation then uniquely lift that input to the root (xhat,yhat) over Z59 modulo 59^8:

    psi11(xhat)=0 and yhat²+yhat=h(xhat) mod 59^8.

Both programs process **all ten** oriented source points, preserve their input orientation, reject (19,23) and (0,0), and independently recount 54 affine source points plus the identity. Four local modes produced byte-identical output SHA-256:

    84f85287e92d6113581703ddd8da5b4089af26d3b3c67f1fa4eaa33605509265

For example, input Q=(19,22) uniquely lifts to

    (xhat,yhat)=(102016143499836,102915451177665) mod 146830437604321.

The separately coded Frobenius certificate passes four modes, SHA-256:

    f79acb7467a3a50f745ef6032aa2823e6933b09e2ffb72cadc21402c7825fafb

The p-adic algorithm constructs the local embedding of **T at p_Q**, not a global expression for [m]T in the *fixed* generator presentation of K. These are not interchangeable online tasks: the latter requires identifying the unique sigma_m that moves p_P to p_Q, or an equivalent global-algebraization operation. The current local algorithm does **not** evaluate rho2([m]T), run the binary DLP, or give the sought complete transfer. The fact that rho2∘sigma_7 is a cheap Frobenius power makes the last step explicit **once sigma_m has been recognized**, not before.

## Reproduction from repository root

~~~sh
D=research/cross_characteristic_global_torsion_20261112/ct105
python3 -B "$D/ct105_source_hensel.py" > /tmp/ct105-py.json
python3 -O -B "$D/ct105_source_hensel.py" > /tmp/ct105-pyopt.json
node "$D/ct105_source_hensel_independent.js" > /tmp/ct105-js.json
node --no-opt "$D/ct105_source_hensel_independent.js" > /tmp/ct105-jsopt.json
cmp /tmp/ct105-py.json /tmp/ct105-pyopt.json
cmp /tmp/ct105-py.json /tmp/ct105-js.json
cmp /tmp/ct105-py.json /tmp/ct105-jsopt.json
printf '%s  %s\n' 84f85287e92d6113581703ddd8da5b4089af26d3b3c67f1fa4eaa33605509265 /tmp/ct105-py.json | sha256sum -c

python3 -B "$D/ct105_frobenius_torsor.py" > /tmp/ct105-tor-py.json
python3 -O -B "$D/ct105_frobenius_torsor.py" > /tmp/ct105-tor-pyopt.json
node "$D/ct105_frobenius_torsor_independent.js" > /tmp/ct105-tor-js.json
node --no-opt "$D/ct105_frobenius_torsor_independent.js" > /tmp/ct105-tor-jsopt.json
cmp /tmp/ct105-tor-py.json /tmp/ct105-tor-pyopt.json
cmp /tmp/ct105-tor-py.json /tmp/ct105-tor-js.json
cmp /tmp/ct105-tor-py.json /tmp/ct105-tor-jsopt.json
printf '%s  %s\n' f79acb7467a3a50f745ef6032aa2823e6933b09e2ffb72cadc21402c7825fafb /tmp/ct105-tor-py.json | sha256sum -c

python3 -B "$D/ct105_source_hensel.py" --point 19,22
node "$D/ct105_source_hensel_independent.js" --point 19,22
! python3 -B "$D/ct105_source_hensel.py" --point 19,23
! node "$D/ct105_source_hensel_independent.js" --point 19,23
~~~

Independent exact-head CI, separate signed PCC certification, broader Rust/Conductor gates, and real cold benchmarks remain separate from the four-mode local replay.

## New decisive target and cost accounting

**Compute the C10-torsor coordinate from p_Q without scalar enumeration**, preferably by explicitly constructing the degree-12 fixed field M, a degree-10 cyclic presentation K/M, and recognizing the selected source root via 59-adic precision + exact global certificate. Verify that its image at the *same fixed* binary prime follows the Frobenius-four action. A lookup over ten sigma_m is only a public toy control and would disclose the scalar; it is not the desired algorithm.

Then charge the CT104 cold protocol for field setup, relative-field basis, input root lifting, global reconstruction, certificate, F2^40/Frobenius work, target DLP, and matched strongest source rho/BSGS/index-calculus. The current degree-12/10 decomposition is a **representation opportunity**, not measured cost reduction. No paid EKS work, protected merge, leaderboard promotion, or claimed attack.
