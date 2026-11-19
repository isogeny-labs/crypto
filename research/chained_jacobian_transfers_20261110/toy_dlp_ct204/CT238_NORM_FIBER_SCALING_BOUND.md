# CT238 — norm-fiber compression has a constant-factor ceiling (2026-11-19)

**Scope:** mathematical counting statement for the *full signed rational norm-factor base* on the fixed-degree smooth genus-4 cubic-cover family. A written elementary proof is supplied below; no new Lean theorem was compiled or externally reviewed. **[pcc:unverified].**

## Setup and exact identity

Let (p) be odd with (p\equiv1\pmod 3), and let
[
 C/\mathbb F_p:\quad y^2=f(u^3)
]
be a **smooth** genus-4 hyperelliptic curve with (f\in\mathbb F_p[t]) cubic of nonzero leading coefficient. The degree-nine equation has one rational point at infinity. Let (W_p) denote the number of affine rational Weierstrass points ((y=0)), so (0\leq W_p\leq 9). Let (B_C) count rational affine (u)-coordinates with at least one rational (y), with opposite signs identified as a single factor-base symbol. Define (b_0=1) if a rational point lies over (u=0), and (0) otherwise. Under smoothness, (f(0)\neq0), hence (b_0\in\{0,1\}) and no rational branch point at zero.

Let (B_{\mathrm{norm}}) be the number of distinct E-source norm-(x=u^3) columns obtained from **all** these rational signed symbols.

**Claim, exact finite-field count:**
[
 B_C=\frac{\#C(\mathbb F_p)-1+W_p}{2},\qquad
 B_{\mathrm{norm}}=\frac{B_C-b_0}{3}+b_0
 =\frac{\#C(\mathbb F_p)-1+W_p+4b_0}{6}.
]

**Proof.** There are (B_C-W_p) non-Weierstrass rational (u) values, each contributing two points with opposite (y), plus (W_p) branch points contributing one each, and the unique rational point at infinity. Thus (#C=1+2(B_C-W_p)+W_p). Since (p\equiv1\pmod3), there is a nontrivial cube root of unity (ζ\in\mathbb F_p). The automorphism (u\mapstoζu) preserves (f(u^3)) and hence preserves the existence of rational (y); every orbit on (u\neq0) has exactly three elements and all give the **same** value (u^3). There is at most one exceptional orbit at zero. Counting these orbits proves the displayed identity. □

The genus-4 Hasse–Weil estimate gives (|#C-(p+1)|\le 8\sqrt p), so
[
 \frac{p-8\sqrt p}{6}\le B_{\mathrm{norm}}
 \le\frac{p+8\sqrt p+13}{6}.
]
Consequently **the full compressed rational norm factor base remains Θ(p)** as (p\to\infty) through admissible primes, although its leading size is roughly one third of the uncompressed rational (u)-symbol count. This is a structural obstruction for the **specific full-base strategy**, not a lower bound on all index-calculus algorithms, arbitrary reduced bases, high-genus changes, other transfers, or solver complexity itself. In particular no new subexponential prime-field ECDLP algorithm follows from this cubic-orbit compression.

## Exact F103 instance

The [CT238 Python pilot](ct238_p103_norm_fiber_pilot.py) and [hosted CI #44572120949](https://github.com/isogeny-labs/crypto/actions/runs/44572120949) independently enumerate 66 rational signed C-(u) symbols (all with opposite nonzero (y)), 22 distinct E norm-(x) columns, every nonzero orbit of size 3, and full rank 22 from 27 raw split relations. Here (b_0=0), (W_{103}=0), (#C(\mathbb F_{103})=133), and (B_{\mathrm{norm}}=133-1\over6=22), consistent with the identity.

The prospective [CT238 native cold Rust protocol](CT238_NORM_FIBER_COMPRESSED_NATIVE_PROTOCOL.md) is a **different and independently required experiment**; its timings may not be inferred from this counting argument or Python execution.

## Scope and next gates

This argument is a combinatorial deduction under the explicitly stated smooth genus-4 assumptions; it is not a verified Lean theorem or full cryptographic asymptotic lower bound. Native same-F103 comparator, complete memory and strongest rho/IC, original 695/701 norm fixture, physical C169/C13/D and signed PCC remain outstanding. No paid compute or merge.
