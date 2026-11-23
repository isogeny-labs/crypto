# CT238 — cubic norm-fiber compression has a constant-factor ceiling (2026-11-19)

**Scope:** Mathematical counting statement for the *full signed rational norm-factor base* on the fixed-degree smooth genus-4 cubic-cover family. A written elementary proof is supplied below; no new Lean theorem was compiled or externally reviewed. **[pcc:unverified].**

## Setup and exact identity

Let $p$ be odd with $p\equiv1\pmod3$, and let

$$C/\mathbb F_p:\quad y^2=f(u^3)$$

be a smooth genus-4 hyperelliptic curve with $f\in\mathbb F_p[t]$ cubic of nonzero leading coefficient. The degree-nine equation has one rational point at infinity. Let $W_p$ be the number of rational affine Weierstrass points ($y=0$), so $0\le W_p\le9$. Let $B_C$ count rational affine $u$-coordinates with at least one rational $y$, identifying opposite signs as a single factor-base symbol. Let $b_0=1$ if a rational point lies over $u=0$, and $0$ otherwise. Smoothness implies $f(0)\ne0$.

Let $B_{\mathrm{norm}}$ be the number of distinct source-elliptic norm-$x=u^3$ columns obtained from **all** these rational signed symbols.

**Exact finite-field count:**

$$
B_C=\frac{\#C(\mathbb F_p)-1+W_p}{2},\qquad
B_{\mathrm{norm}}=\frac{B_C-b_0}{3}+b_0
=\frac{\#C(\mathbb F_p)-1+W_p+4b_0}{6}.
$$

**Proof.** There are $B_C-W_p$ non-Weierstrass rational $u$ values, each contributing two rational points with opposite $y$, plus $W_p$ rational branch points contributing one each, and the unique rational point at infinity. Thus $\#C=1+2(B_C-W_p)+W_p$. Since $p\equiv1\pmod3$, a nontrivial cube root of unity $\zeta\in\mathbb F_p$ exists. The automorphism $(u,y)\mapsto(\zeta u,y)$ preserves $f(u^3)$ and therefore the existence of rational $y$. Every orbit on nonzero rational $u$ has exactly three elements, all with the same $u^3$. There is at most one exceptional orbit at zero. Counting these orbits proves the identities.

The genus-4 Hasse–Weil estimate gives $|\#C(\mathbb F_p)-(p+1)|\le8\sqrt p$, so

$$
\frac{p-8\sqrt p}{6}\le B_{\mathrm{norm}}
\le\frac{p+8\sqrt p+13}{6}.
$$

Consequently **the full compressed rational norm factor base remains $\Theta(p)$** as $p\to\infty$ through admissible primes, although its leading size is one third of the uncompressed rational $u$-symbol count. This is a structural obstruction for the **specific full-base strategy**, not a lower bound on all index-calculus algorithms, reduced factor bases, high-genus changes, other transfers, or solver complexity itself. No subexponential prime-field ECDLP algorithm follows from this constant-degree cubic-orbit compression alone.

## Exact F103 instance

The [CT238 Python pilot](ct238_p103_norm_fiber_pilot.py) and [hosted CI #44572120949](https://github.com/isogeny-labs/crypto/actions/runs/44572120949) enumerate 66 rational signed $C$-$u$ symbols (all non-Weierstrass), 22 distinct $E$ norm-$x$ columns and three rational preimages of every such norm column. Here $b_0=0$, $W_{103}=0$, $\#C(\mathbb F_{103})=133$, so $B_{\mathrm{norm}}=(133-1)/6=22$.

The prospective [CT238 native cold Rust protocol](CT238_NORM_FIBER_COMPRESSED_NATIVE_PROTOCOL.md) is a **different experiment**; its timings cannot be inferred from the counting argument or Python execution.

## Scope and next gates

This combinatorial result assumes the stated smooth genus-4 model; it is not a Lean-verified theorem or cryptographic asymptotic lower bound. Native same-F103 cost, complete memory and strongest rho/IC comparisons, original 695/701 norm fixture, physical C169/C13/D and signed PCC remain outstanding. No paid compute or merge.
