# Review: "Conditional Subexponential Attacks on Structured LWE" and the tensor-contraction route to ECDLP index calculus

- **Date:** 2026-11-21
- **Tracking PR:** isogeny-labs/crypto#4904 (`claude/youthful-hawking-4n6jhp`); first drafted in isogeny-labs/workspace#60 and moved here on the owner's instruction that research PRs belong in `crypto`
- **Reviewed document:** *Conditional Subexponential Attacks on Structured Learning with Errors: Tensor Encodings, Noise Distributions, and Comparisons with Lattice Reduction*, "ChatGPT (with a lot of prompting from Nigel Paul Smart)", 49 pp., dated 21 November 2026. Uploaded as `main_4.pdf`; SHA-256 in the receipts section.
- **Primary sources read for this review** (all fetched today, text extracted locally):
  - Akshima, Bacho, Huang, Kiltz, Nishaburi, Nooripoor, Tittelbach, Wiedijk, *A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm*, v0.2, 2026-10-29 (`bochum-crypto/subexponential-ecdl`). "Bochum" below.
  - OpenAI, *The Exponential Time Hypothesis is False*, 2026-10-17 rev. 2026-10-25 (`openai/math`). Appendix D carries Theorem D.1, the Boolean tensor contraction theorem.
  - Anthropic, `anthropics/formal-math/eth`: Lean statement files, `Solution.lean`, `scripts/PrintAxioms.lean`, README (the k-CNF counting theorem; proof folder not read).
- **Lab context:** `cryptanalysis/crates/hashed-ecdl-tensor` (Bochum encoding, wired as `ic.hashed_native_v1`), `workspace:docs/notes/prime-pdp-relation-leads-20261119.md` Lead 2C, `workspace:docs/notes/rho-parity-20261120/MEASUREMENT_TABLE.md` row `ic.hashed_native_v1` (fp20, 0/16 verified, censored ~47–55×).
- **Standing:** documentary review and hand-verification of selected lemmas. No measurement, no speed claim, no change to any leaderboard row.

---

## 0. Verdict

1. **Nothing in this paper changes the lab's ECDLP picture, its leaderboard, or deployed-curve security.** The paper says so itself (title page: "expository derivation for mathematical review, not a claim of a practical attack"). Its only non-classical ingredient is the same unverified black box the Bochum ECDLP preprint uses: OpenAI's Theorem D.1 (Boolean tensor contraction in $2^{O(s/\sqrt{\log s})}$).
2. **The unconditional mathematics in the paper is sound where I checked it** (chain-ring structure, collision bound, exact first moment, modulus projection, information-theoretic converse, and the shape of the triadic multiplication network). It is careful, and it is unusually honest about what is and is not established.
3. **Its real lever is LWE-specific and has no ECDLP analogue:** exact projection of the instance from $q$ to a small divisor $D=2^r\approx 4W$, so that each coefficient costs $r=O(\log W)$ bits instead of $\log q$. For prime-field ECDLP the "modulus" is a prime field and the group law is nonlinear in coordinates; there is nothing to project.
4. **The one piece of this whole chain that is machine-checked (Anthropic's Lean k-CNF counter) cannot substitute for Theorem D.1** in either the LWE or the ECDLP application: routing through CNF multiplies the instance size by the word width and destroys subexponentiality in the natural parameter. Both the LWE paper (§1.3, §11.1) and Bochum (§1.3) state this; I re-derive the arithmetic for ECDLP in §3.3 below.
5. **By the source proof's own parameters, Theorem D.1 saves nothing below $2^{2^{17}}$ edges** ($\varepsilon=2^{-17}$, $u_0=\lfloor\varepsilon\lambda\rfloor$, $\lambda=\lceil\log_2 s\rceil$; display (D.5) needs $\lambda\ge 2^{85}$). This is exactly the `root_u == 0` stall the lab already found in `hashed-ecdl-tensor`. The fp20 `ic.hashed_native_v1` result (0/16, censored) is therefore the expected outcome of the method, not an implementation defect to chase.

What the lab should take from it is in §5: two concrete, cheap items and a short "what would change our mind" list.

---

## 1. What the paper is

### 1.1 Provenance and dependency chain

```
OpenAI "ETH is False" (Oct 2026, unreviewed, AI-written)
  ├── Thm 1.1: #k-CNF in (n+m)^{O(1)} 2^{O(n log log n / sqrt(log n))}
  │     Lean comparator statement in openai/math: `:= by sorry` (statement only)
  └── Thm D.1: Boolean tensor contraction, s edges, degree <= s^{9/10},
        succinct poly-time predicates, time 2^{O(s/sqrt(log s))}, poly space
        NOT Lean-covered anywhere.  <-- the only engine both papers below need
            │
            ├── Bochum "Subexponential ECDL" v0.2 (Oct 29): index calculus,
            │     hashed factor bases, BTN relation finding, 2^{O(b/(log b)^{1/4})}
            │     conditional on D.1; footnote 4: u_0 = 0 below 2^{2^17} edges
            │
            └── THIS PAPER (Nov 21): binary-secret Ring-LWE mod 2^t,
                  chain-ring collision bound + triadic BTN multiplier,
                  2^{O(mnr/sqrt(log(mnr)))}, conditional on "Assumption 1.1" = D.1

Anthropic formal-math/eth (Lean 4.33.1, Mathlib pinned): MainTheorem =
  deterministic TM counts #k-CNF in 2^{C min(k m/sqrt(log m), k^2 n log log n/sqrt(log n))}
  for k>=1, m>=2 clauses, n>=4 variables.  Expected `#print axioms`:
  [propext, Classical.choice, Quot.sound].  Restricted to CNF; no tensor interface.
```

All five repositories/documents exist and were fetched today (receipts in §7). The LWE paper's Appendix B records SHA-256 hashes of the three PDFs it consulted; I did not compare them to the current GitHub bytes.

### 1.2 Scope the paper claims

For $R=\mathbb Z[X]/(X^n+1)$, $n=2^k$, $q=2^t$, a **binary secret** $s\in\{0,1\}^n$, and errors **independent and uniform on a known interval of $W$ integers**, the paper gives (conditional on Assumption 1.1) an algorithm with time
$$\mathrm{poly}(n,m,t)+2^{O(mnr/\sqrt{\log(mnr)})},\qquad r=\lceil\log_2 W\rceil+2,$$
polynomial space, failure probability at most the exact first moment of false candidates, and never a wrong answer. With $q\ge 4W$ one ring sample suffices with failure $\le 2^{-n}$. The bound is subexponential in $n$ iff $\log W=o(\sqrt{\log n})$ (§5.1). Extensions cover odd moduli, other defining polynomials, bounded non-binary secrets, Gaussian-like noise via acceptance sets, and sparse LWE; dense LWE and generic SVP/BKZ are explicitly **not** covered.

---

## 2. The theory, layer by layer

I separate what is proved outright from what rides on Assumption 1.1, and record my spot checks.

### 2.1 The chain ring (unconditional, checked)

Put $T=R_D=\mathbb Z_{2^r}[X]/(X^n+1)$ and $Y=X+1$. Lemma 2.1: $(X+1)^n=X^n+1$ in characteristic 2, so $T/2T\cong\mathbb F_2[Y]/(Y^n)$; expanding $(Y-1)^n+1=Y^n+2h(Y)$ with $h(0)=1$ gives $2=uY^n$ for a unit $u$; the ideals are exactly $(Y^j)$, $0\le j\le nr$, with $|(Y^j)|=2^{nr-j}$. Checked: the constant term of $(Y-1)^n$ is $(-1)^n=1$, so $h$ has constant term 1 and is a unit; $|T|=2^{nr}$ matches the chain of $nr$ index-2 steps.

Corollary 2.2 is the load-bearing fact: a polynomial $\delta$ with an **odd** coefficient is not in $2T=(Y^n)$, so $(\delta)\supseteq(Y^n)=2T$ and $|\delta T|\ge(D/2)^n$. Lemma 2.3: multiplication by a uniform $a$ is uniform on $\delta T$, and conditioned on a coset of $2T$ every coefficient is independently uniform on its parity class mod $D$. This is why nilpotents do **not** rescue a competing secret: the image is small modulo 2 but contains every even vector.

### 2.2 Collision bound and exact first moment (unconditional, checked)

Lemma 3.1 / 3.2: for a competing binary secret $s'\ne s$, $\delta=s'-s$ has coefficients in $\{-1,0,1\}$ with some odd entry, so per sample
$$\Pr_a[e-a\delta\in I^n]\le(2\lceil W/2\rceil/D)^n\quad\text{(any bounded law)},\qquad\le(\alpha_W/D)^n\quad\text{(uniform)},$$
with $\alpha_W=W$ for even $W$ and $W+1/W$ for odd $W$ (the odd case counts same-parity ordered pairs $N_0^2+N_1^2$ in the interval). Theorem 3.3 union-bounds over $2^n-1$ competitors: ambiguity $\le[2(\alpha_W/D)^m]^n$.

Corollary 3.4 check: $r=\lceil\log W\rceil+2$ gives $D=4H_W$ with $H_W=2^{\lceil\log W\rceil}$ even, so $4\lceil W/2\rceil/D\le 2H_W/D=1/2$ and the failure bound is $(1/2)^n=2^{-n}$. Correct; the union-bound factor $2^n$ is absorbed into the 4.

Theorem 3.6 computes $\mathbb E N_{\text{false}}$ **exactly** by stratifying competitors by chain-ring valuation $v_Y(s'-s)=j$ (exactly $2^{n-j-1}$ of them, Lemma 3.5) and using the weight enumerator $A_j$ of the binary code $\mathcal V_j=(Y^j)\bmod 2$, whose Euclidean dual is $\mathcal V_{n-j}$. The one-sample leading term becomes $(2W/D)^n(1+o(1))$ (Lemma 3.7 recurrence, checked at the stated values $S_n(0)=2^n-1$, $S_n(1)=(n/2)2^n$). The paper correctly warns (p. 11) that the multi-sample bound keeps the $A_j(\epsilon_W)^m$ factors; the worked counterexample ($n=64,m=8,W=3,D=8$, ratio $>56$) is a nice touch.

Proposition 3.8 (information-theoretic converse) is the standard counting argument and matches the sufficient condition $m\log_2(D/W)>1$ up to the reliability margin.

### 2.3 Exact modulus projection: the actual lever (unconditional)

Lemma 2.5: reduction $R_q\to R_D$ for $D\mid q$ is a ring homomorphism, maps uniform to uniform, and sends $b=as+e$ to the **identical** equation mod $D$ with no rounding term. A binary secret lifts uniquely for $D\ge 2$. Hence the search instance has $r=O(\log W)$ bits per coefficient regardless of $t=\log q$. This, not the tensor network, is what makes the size parameter $mnr$ rather than $mnt$; everything downstream (the subexponentiality condition $\log W=o(\sqrt{\log n})$, the "$2^{64}$ example" table with $r=23$ for 20-bit noise) follows from it.

Section 6.6 then makes the right methodological point: a conventional attacker is **not** obliged to discard the high bits, and discarding them makes the primal lattice target *less* exceptional by the exact factor $(q/D)^{\ell/d}$ (6.9). The correct comparison is with the best conventional attack on the original-modulus instance. This is the same rule as the lab's matched-rho floor.

### 2.4 The linear-size multiplication network (unconditional; structure checked, constants not re-derived)

Section 4 builds a Boolean tensor network (BTN) checking $au\pm v=c$ in $R_D$ with $E\le 325nr$ edges (reference) or $24nr+n$ (compact binary version, Prop. 4.4) and degree $\le 23n^{3/4}r$. The construction is Bochum's one-split Schönhage–Strassen network transplanted from integer coefficients to $\mathbb Z_{2^r}$ coefficients. Because 2 is not a unit in $\mathbb Z_{2^r}$, a power-of-two transform length cannot be inverted; the paper therefore uses a **triadic** auxiliary ring $A=\mathbb Z_{2^r}[Y]/(Y^{2J}+Y^J+1)$ with $\omega=Y^{3J/L}$, $L$ a power of 3 ($\omega^L=Y^{3J}=1$ since $Y^{3J}-1=(Y^J-1)(Y^{2J}+Y^J+1)$), and proves the principal-root sum condition by grouping exponents in triples. The single split keeps the degree at $O(n^{3/4}r)$, which is below the $s^{9/10}$ cap of D.1 (this is the whole reason for not using a recursive FFT). Parsimony (unique auxiliary bits) is argued layer by layer. I did not recount the edge constants; the paper's own Appendix A.4 reports exhaustive small-case checks.

This is the paper's genuine technical contribution over Bochum; it is also entirely irrelevant to $\mathbb F_p$ with $p$ odd, where Bochum's $\mathbb Z/(2^J+1)$ transform already works.

### 2.5 Count-to-search and the main theorem (conditional)

Lemma 1.2 / Bochum Lemma 1: fix interface bits one at a time inside existing predicates; $n+1$ exact counts recover the unique secret; parsimony makes the count equal $|L|$. Theorem 5.1 then is: exact reduction (poly), network with $s_\star=O(mnr)$, D.1 count, union/first-moment failure bound. There is no further idea here; the whole running time is D.1's.

### 2.6 The BKZ comparison and crossover audit (model calculation; honest)

Proposition 6.1 shows that in the standard primal GSA model with **fixed** $q$ and fixed noise, the required block size satisfies $\beta/d\to 1$, i.e. $\beta=\Theta(n)$, so the modelled sieve cost is $2^{\Theta(n)}$ and the conditional $2^{O(n/\sqrt{\log n})}$ eventually beats it. The paper is explicit that this is an implication of the model, not a lower bound on BKZ, and that the embedding contains short homogeneous $q$-vectors at large $\ell$ (§6.7).

Section 7 is the part most relevant to us: it audits the source theorem's own sufficient inequalities. With $\lambda=\lceil\log_2 s\rceil$ and $u_0=\lfloor 2^{-17}\lambda\rfloor$, the recursion does nothing below $\lambda=2^{17}$ ($s\ge 2^{2^{17}}$), and display (D.5) ($\lambda^{4/5}\le u$) needs $\lambda\ge 2^{85}$. The illustrative crossover scales (7.5) are $\log_2(Amnr)\approx 3.9\cdot 10^6$ to $6.2\cdot 10^9$ with $K=1$. I confirmed the parameter block in the OpenAI PDF (§D.4.2: $\varepsilon=1/(128\cdot 1024)$, $u_d=\lfloor\varepsilon\lambda/2^d\rfloor$, $H=\lfloor s/\lfloor\sqrt\lambda\rfloor\rfloor$ leaf size, $\Delta=\lceil s^{9/10}\rceil$, $\lambda=\lceil\log_2 s\rceil$ in §D.4.1).

### 2.7 Generalizations (what survives)

| Change | Status in the paper | Comment |
|---|---|---|
| $q=2^t q_{\text{odd}}$ | immediate (project to the 2-part) | still needs $D=2^r\ge 4W$ to divide $q$ |
| odd prime divisor $p>W$ | CRT image argument, Prop. 8.2 | bound $(1+[2(W/p)^m]^{d_p})^{g_p}-1$; needs $d_p=\Theta(n)$ |
| several small odd factors | coupled CRT, Prop. 8.4 | positive but weak; NTT primes $p\equiv1\pmod{2n}$ excluded |
| other monic $f$ | multiplication verifier extends; recovery needs $f\bmod 2$ irreducible or squarefree | cyclic $X^n-1$ breaks the containment lemma (§9.4) |
| bounded alphabets | Lemma 3.1 with $H=2^{\lceil\log L\rceil}$ | $r=O(\log L+\log W)$ |
| unrestricted secrets | eliminate $s$ via a unit base sample; witness = error pair (Prop. 10.1) | retained modulus $\approx W^2$ instead of $W$ |
| dense LWE | **fails**: $O(mn)$ edges with $m=\Theta(n)$ gives $2^{O(n^2/\sqrt{\log n})}$ | same obstruction as CNF for ECDLP |
| generic SVP / BKZ oracle | **not** obtained; projected blocks lose ring and coefficient promises | §11.3 |
| Gaussian-like noise | acceptance sets, entropy threshold $r>H(E\bmod D)+1$, norm test with explicit Chernoff | entropy bits saved are not security bits |

### 2.8 Correctness assessment

- Spot-checked lemmas (2.1, 2.2, 2.5, 3.1, 3.2, 3.4, 3.5, 3.6 values, 4.1 root identity, 9.1 idea, 10.1 idea, 12.1 union bound) are correct.
- The paper does not overclaim: every runtime statement is tagged conditional; the BKZ comparison is labelled a model calculation; §7.4 refuses to state a crossover.
- Weak points are all inherited: Assumption 1.1 is unverified and the paper says so (p. 5: "We do not identify the latter with a Lean-verified theorem"). The constants $A\in\{25,33,1000\}$ are bookkeeping bounds, not an implementation.
- Minor: the title promises "attacks"; the body delivers a reduction. Nothing in it is an attack on any parameter set anyone uses.

---

## 3. The engine: what Theorem D.1 is, what is verified, and why CNF cannot replace it

### 3.1 Mechanism (from the OpenAI technique overview and Appendix D)

The count is written as $\sum_x\prod_e\Phi_e(x)$, a polynomial of degree $O(n)$ in the assignment. A deformation parameter $T$ replaces each clause input $X_v$ by a linear form $\langle\vec P_{e,v}(T),\vec X\rangle$ that restores the original at $T=0$ and, at chosen nodes $T=1..Q$, makes different variable groups interact only through a small set of **shared linear summaries**, so the sum factorises over groups once the summaries are fixed; interpolation in $T$ recovers the $T=0$ value. Appendix D recurses this on the wires of a tensor network over $\mathbb Z_{2^{P_0}}[U]/(f)$ with 2-adic valuation tapering of Hermite multiplicities ("modes of weight $j$ have coefficient divisible by $2^j$"), giving $2^{O(M/\sqrt u)}$ mode monomials per call and depth-geometric summation to $2^{O(s/\sqrt{\log s})}$. I did not verify this proof and the lab should not pretend to; it is a serious document and an independent audit is a complexity-theory project, not a cryptanalysis task.

### 3.2 Verification status

- **OpenAI**: Lean comparator `SubexponentialSAT.lean` states a k-CNF counting bound with exponent $n(\log\log n)^2/\sqrt{\log n}$ and ends in `sorry`; the repo README says ~42% of top-line results are formalised. Theorem D.1 (tensor interface) has no Lean statement.
- **Anthropic**: `MainResults.lean` defines `MainTheorem` (deterministic `Turing.FinTM2`, counts #k-CNF in $\lfloor 2^{C\min(km/\sqrt{\log m},\,k^2n\log\log n/\sqrt{\log n})}\rfloor$ steps for $k\ge1$, $m\ge2$, $n\ge4$, all variables occurring) and `ETHIsFalse`; `Solution.lean` proves both from `CountedRoute.mainTheoremHolds`; `scripts/PrintAxioms.lean` expects only `propext, Classical.choice, Quot.sound`. README: "Lean checks the proofs, but no machine checks that this file says what the paper says" and "purely asymptotic and says nothing about solving formulas in practice." I read statements and entry points, not `proof/`, and did not run `lake`. Its README's informal key idea mentions contracting tensor networks on complete graphs $K_K$ with $\lambda$ bits per edge, a different interface from D.1.
- **Neither** verified artefact is the succinct-predicate, degree-$s^{9/10}$ interface that the LWE paper and Bochum need.

### 3.3 Why the verified CNF theorem does not give subexponential ECDLP (or LWE)

Take the Anthropic bound with $m_{\text{cl}}$ clauses, exponent $O(k\,m_{\text{cl}}/\sqrt{\log m_{\text{cl}}})$. Encode Bochum's decomposition relation (m point additions plus m hashed-membership tests on a b-bit prime field) as 3-CNF via circuits: each field multiplication needs $\Theta(b\log b)$ gates even with FFT multipliers, so $m_{\text{cl}}=\Theta(mb\log b)$ and the per-query cost is
$$2^{O(mb\log b/\sqrt{\log b})}=2^{O(mb\sqrt{\log b})}\gg 2^{b/2}.$$
With the variable-count form the factor is $(\log\log b)^2\sqrt{\log b}$ instead; still super-linear in $b$. Bochum §1.3 states the same conclusion ($2^{O(b^2\log b)}$ for a double-and-add verifier). The LWE paper's §1.3 and §11.1 are the same obstruction with $n$ in place of $b$. Hence the entire conditional edifice rests on D.1's **succinct predicates**: a vertex may read $O(s^{9/10})$ bits and compute a polynomial-time function of them for free. That is what lets a multiplication cost $O(b)$ edges instead of $O(b\log b)$.

### 3.4 Even granting D.1: the ECDLP crossover arithmetic

Bochum: $2^{O(b/m+mb/\sqrt{\log b})}$, optimised to $2^{O(b/(\log b)^{1/4})}$. For this to beat $2^{b/2}$ at $b=256$ the exponent constant must satisfy $C<(\log_2 256)^{1/4}/2\approx 0.84$ for the leading term alone, before the $r^{1/m}$ relation count and the $N^3$ linear algebra. The source proof's leaf rule ($u_0=0$ below $2^{2^{17}}$ edges; brute-force fallback) makes the question moot at every size any machine will see. Bochum says this in its own footnote 4 and limitations paragraph.

---

## 4. Connection to ECDLP index calculus

### 4.1 Same architecture, three instances

| Component | LWE paper (this review) | Bochum ECDL | Lab (`hashed-ecdl-tensor`, `ic.hashed_native_v1`) |
|---|---|---|---|
| Unknown | binary $s\in\{0,1\}^n$ | decomposition $(A_1..A_m)\in F_1\times\dots\times F_m$ of $R=[u]P+[v]Q$ | same as Bochum |
| Relation checked | $b_j-a_js\in I^n$ in $R_D$ | $A_1+\dots+A_m=R$, $H_i(\mathrm{enc}(A_i))<\tau$ | same; prime cofactor-one adapter |
| Compact encoding | triadic one-split negacyclic multiplier, $24nr+n$ edges, degree $23n^{3/4}r$ | one-split SS multiplier mod $2^J+1$, $<1100(d+1)$ edges, degree $<136(d+1)^{3/4}$; addition chain with guessed slopes and case selectors, $O(b)$ edges per addition | `encoding`, `network`, `packing`, `ring`, `arithmetic` modules |
| Existence / uniqueness | uniqueness via chain-ring collision bound (first moment) | existence via second moment, $\Pr[X>0]\ge 2/5$ (Lemma 10), coverage $f\ge 1/5$ | yield identity $\binom{B+m-1}{m}/N$ (Tier 4 note) |
| Search from counting | Lemma 1.2, $n+1$ counts | Lemma 1, $s+1$ counts, uniform sampling | `contraction`, `recursive`, `partial` |
| Solver | Assumption 1.1 = D.1 | Theorem 2 = D.1 | native re-implementation; `root_u == 0` on all finite inputs |
| Outer loop | none (one or few samples) | $N+1$ relations, $N=O(mr^{1/m})$, Gaussian elimination over $\mathbb F_r$ | ecbench `ic` family |
| Cost | $\mathrm{poly}+2^{O(mnr/\sqrt{\log(mnr)})}$ | $2^{O(b/m+mb/\sqrt{\log b})}$ | measured fp20: exhausted, 0/16, censored ~47–55× rho |

The LWE paper is thus a **sibling application** of the Bochum construction, not an advance on the solver. Reading it confirms the model the lab already works under: tensor-contraction index calculus = {linear-size parsimonious BTN of the decomposition relation} + {black-box exact counter} + {classical relation collection and linear algebra}.

### 4.2 What the LWE paper adds, and whether each item transfers to prime-field ECDLP

1. **Exact modulus projection ($q\to D$).** Does not transfer. In $E(\mathbb F_p)$ there is no sub-modulus of the field; reducing coordinates modulo anything destroys the group law. The nearest lab objects are divisor structure in the *group order or auxiliary tower* (P-256 $T_2$ torus, $2^t\mid p+1$ towers, Lead 2A), which are a different mechanism (lower solving degree), not a word-width reduction. Lead 2C component 1 ("prime inverse hash slopes reducing slot membership to 1D short-box intervals") is the honest analogue of the paper's "interval test on a linear image" (Lemma 3.1 / Lemma 8.3), but it shrinks the membership gadget, not the arithmetic words that dominate $mb$.

2. **Exact moments by valuation strata.** Transfers as a *diagnostic*, cheaply. Bochum needs only $\mathbb E[X^2]\le\mu(1+(L+1)^m/n)$ to get constant coverage for hashed bases. For the lab's **structured** bases (interval, Kummer, torus, signed boxes) the first moment is base-independent (Tier 4, 288 cells), so any coverage difference lives in the second moment. The LWE paper's technique (stratify competitors by the algebraic invariant that controls the image size, then sum exact weight enumerators) is a template for computing $\Pr[X>0]$ exactly for a structured base rather than estimating it. This is the one item worth a few hours of desk work; it feeds the `w_cal` / Kaarelson coverage thread in PR #32.

3. **Error-first elimination of the secret (Prop. 10.1).** No transfer. In ECDLP index calculus the unknown $\xi$ never enters the relation network; only $R$ does. The target-free single-relation variant (Amadori–Pintore–Sala, cited by Bochum) is already closed in the lab (Tier 4, "Hecke/Heegner target-free relations").

4. **Distribution-aware acceptance sets / norm tests (§12).** Already present in the lab as signed-coefficient boxes (`wsum`) and weight-capped jump tables; the paper adds nothing beyond the reminder that "entropy bits saved are not security bits".

5. **Dense-LWE failure (§11.1) and the SAT obstruction (§1.3).** Transfers as a *structural lesson* and is worth stating plainly for the lab's solver work: the compact object is the **un-eliminated addition chain with slope witnesses** ($\lambda_i$ guessed, $\lambda(x_2-x_1)=y_2-y_1$, $x_3=\lambda^2-x_1-x_2$), $O(b)$ edges per addition; the **eliminated** Semaev polynomial $S_{m+1}$ is dense, degree $2^{m-1}$ per variable, and has no linear-size encoding. Any solver that profits from compact encodings (CDCL(T) with an $\mathbb F_p$ propagator in Lead 2B; any counting/contraction engine) should consume the chain, not $S_{m+1}$. Lead 2B's chained-$S_3$ design already follows this; F4-on-$S_{m+1}$ pipelines (`pkm-semaev`, bottleneck `solver_F_p_multiply-adds`) are on the wrong side of this line by construction, which is consistent with their measured $10^5$–$10^6\times$ gaps.

6. **Compare with the best conventional attack on the original instance (§6.6).** Identical to the lab's matched-rho floor and "reduced instance is not the benchmark" rule. No action.

### 4.3 Consequence for `ic.hashed_native_v1` and Lead 2C

- The fp20 remasure (0/16 verified, `factor-base column policy`, censored ~47–55× rho) is what the method must produce at every finite size: the only solver with a subexponential bound falls back to brute force below $2^{2^{17}}$ edges, and brute-force or treewidth-style exact contraction of the decomposition network costs $2^{\Theta(b)}$ per query (the one-split transform layers are dense), times $r^{1/m}$ queries. There is no parameter at fp20..fp32 where this method approaches rho, and no measurement will change that.
- Lead 2C components 1–2 (short-box membership, floor-sum cancellation) reduce the *encoding* size by constants; they cannot touch the solver gap. Component 3 (parsimonious parity queries, small-$K_0$ crossing schedule, $\rho\approx0.896$) is an attempt to re-derive D.1 with different constants. That is legitimate complexity-theory work but it is **not an ECDLP lead** under the lab protocol until D.1 itself (or the lab's variant) has an independent proof. It should be re-tagged accordingly.

---

## 5. Recommendations

1. **Close `ic.hashed_native_v1` as a measured ECDLP method** (keep the crate as an encoding testbed). Record in `docs/ACTIVE.md` and the ecbench bounds that the censored fp20 row is the method's expected behaviour, citing Bochum footnote 4, OpenAI §D.4.2 and LWE paper §7.3. No further remasures at any rung. *(haiku-level bookkeeping.)*
2. **Re-tag Lead 2C component 3** from "ECDLP lead" to "conditional on an unverified contraction theorem; complexity-theory track". Components 1–2 stay as encoding-size notes. *(haiku-level.)*
3. **Harvest the exact second-moment technique** (§4.2 item 2): for one structured factor base already in ecbench (interval base at $m=3$, fp20), compute $\mathbb E[X]$ and $\mathbb E[X^2]$ of the decomposition count exactly or by exhaustive enumeration and compare the implied $\Pr[X>0]$ with the measured relation yield. Preregister per `controlled-experiment`; hashed base as the control arm. *(sonnet-level, half a day, no GPU.)*
4. **Record the structural lesson** (§4.2 item 5) in the PDP leads note as a selection rule for solver inputs: compact engines take the slope-witness addition chain, never $S_{m+1}$. *(haiku-level edit.)*
5. **Watch list, with explicit mind-changers.** The lab should revisit the tensor route only if one of these appears: (a) an independent or machine-checked proof of a succinct-predicate contraction theorem (D.1 or equivalent); (b) a version whose recursion saves work at $\lambda\ll 2^{17}$, i.e. a finite-regime statement; (c) a public implementation beating brute-force contraction on a network of $10^5$–$10^6$ edges with degree $\sim s^{9/10}$. Until then the ECDLP programme is unaffected. *(Mythos/Fable-level judgement if (a) appears; otherwise nothing.)*
6. **Owner-facing security line:** none of the three documents claims, and this review finds no path to, any effect on P-256, P-521, Ed25519, or any lab rung. The LWE paper's own illustrative crossover scale is a logarithmic instance size of order $10^6$–$10^9$.

---

## 5a. Can the contraction recursion be brought down to small sizes? (owner question, 2026-11-21)

Short answer: **not by re-parameterising Appendix D.** The $2^{-17}$ is proof slack and can be tightened, but the recursion's own branching constant is roughly 40 times larger than brute force's per edge, and the only thing that divides it is $\sqrt{u}$ with $u\le\lambda=\lceil\log_2 s\rceil$. A finite-regime version needs a new saving mechanism, not new constants. Derivation and the one experiment that could change this follow.

### 5a.1 How the recursion saves work (OpenAI §D.4, read in full)

At an active call with $M$ edges: label vertices by $[64]^a$, pack edges into packets of $L=2^a$ slots (Lemma D.3); deform the edge identification by a Cauchy matrix $W(T)$ with $W(0)=I$ (D.7); the contraction becomes $F(0)$ for a polynomial $F$ of degree $M(2L-1)$ (D.8). $F(0)$ is recovered by **tapered Hermite interpolation** from jets at $t\in S\setminus\{0\}$ with multiplicity $q_t=h\max(0,u-v_2(t))$, $h=\lceil 3M/u\rceil$ (Lemma D.5). Near a node of 2-adic valuation $i<u$, interactions between the 64 groups expand in **modes** of weight $w$ whose coefficients are divisible by $2^w$, so only mode monomials of total weight below the working precision $p_t=p+8hD_t+8M(a+1)$ survive (D.14); each surviving monomial is a product of 64 child contractions with $\le 3M/32$ edges (D.16). The number of surviving monomials is bounded by $\exp(2\pi\sqrt{Mp_t/D_t})$ (D.15 with its generating-function proof). The retained valuation levels are $u_d=\lfloor\varepsilon\lambda/2^d\rfloor$, $\varepsilon=2^{-17}$.

### 5a.2 Where the $2^{-17}$ comes from, and how far tightening could go

- $a_d\le 4\varepsilon\lambda$ and $L=2^a\le s^{1/1000}$, $D_{\text{arr}}=64\cdot 2^a\le s^{1/1000}$ (D.5) force $\varepsilon\lesssim 1/4000$; the stated $\varepsilon$ has a further factor 32 of slack. The $1/1000$ itself is convenience: the proof only needs packet-entry evaluation $\exp(O(L\log s))\le\exp(O(s^{9/10}))$ and $2D_{\text{arr}}^2\Delta\le M$, i.e. $D_{\text{arr}}\lesssim s^{1/20}$.
- $u\ge\lambda^{4/5}$ in (D.5) is used only so that $2^u$ dominates fixed powers of $\lambda$; $u\ge C\log\lambda$ would do. This is the inequality the LWE paper's §7.3 turns into $\lambda\ge 2^{85}$.
- 64 groups and child size $3M/32$ give depth $\le\log_2\lambda/6+O(1)$; the factors 8 and 3 in $p_t$ and $h$ are generous.

Tightening all of these plausibly moves the **nominal** start of recursion from $\lambda\ge 2^{17}$ to $\lambda$ in the low thousands. That is still $s\gtrsim 2^{1000}$ edges and it is not the binding constraint.

### 5a.3 The binding constraint: the branching constant

Evaluate the paper's own bound. With $p_t\ge 8hD_t$ and $h\ge 3M/u$, $Mp_t/D_t\ge 24M^2/u$, so
$$\log N_{\text{modes}}\;\approx\;2\pi\sqrt{24}\,\frac{M}{\sqrt u}\;\approx\;30.8\,\frac{M}{\sqrt u}\quad\text{nats per active call}.$$
Summing over depths with $M_d=s\,8^{-d}$ and $u_d=\varepsilon\lambda\,2^{-d}$ gives total exponent $\approx 30.8\,(s/\sqrt{\varepsilon\lambda})\sum_d 2^{-5d/2}\approx 37\,s/\sqrt{\varepsilon\lambda}$ nats, against $s\ln 2\approx 0.69\,s$ for direct summation. The recursion beats brute force only when
$$\sqrt{\varepsilon\lambda}>54,\qquad\text{i.e.}\qquad \varepsilon\lambda>2.9\times 10^{3}.$$

| Reading | Requirement | Edge count $s$ |
|---|---|---|
| as stated, $\varepsilon=2^{-17}$ | $\lambda>3.8\times10^8$ | $s>2^{3.8\times10^8}$ |
| nominal recursion start as stated ($u_0\ge1$) | $\lambda\ge 2^{17}$ | $s\ge 2^{131072}$ (Bochum fn. 4, LWE §7.3) |
| all proof slack removed, every valuation level retained ($\varepsilon\to1$) | $\lambda>2.9\times10^3$ | $s>2^{2900}$ |
| lab networks | $\lambda\approx 20$–$30$ | $s\approx 10^6$–$10^9$ |

The (D.15) count is an upper bound, but it is a partition-count estimate that is tight up to polynomial factors for arbitrary predicates, so the order of magnitude stands. The main-body algorithm has the same character: its group count $r=\lfloor\sqrt{\lambda}/\nu^2\rfloor$ and node count $Q=\lfloor\sqrt\lambda/(128\nu)\rfloor$ (eq. 2.19 and §5.5) need $\lambda\gtrsim 2^{20}$ before there are two groups and one node, i.e. $n\gtrsim 2^{10^6}$. The "$n\ge 20$" in Theorem 1.1 is the validity threshold of the inequalities, not the point where anything is saved.

### 5a.4 What a small-size version would have to be instead

1. **Structured exact contraction** (treewidth or a chosen contraction order on the decomposition network). The natural cut between consecutive point additions carries one point, about $2b$ bits, so pathwidth contraction costs $2^{O(b)}$ per query and, reorganised, is exactly two-list meet-in-the-middle over the factor bases. That is the lab's `mitm2`/`tw2l`/`bsgs` family, generic, and bounded by Shoup/CGK/DKM. The tensor framing does not escape the generic lower bounds unless the solver computes on coordinate bits in a non-generic way, which is precisely what the 2-adic mode machinery does at the constant above.
2. **Effective branching on arithmetic networks.** (D.15) is worst-case over arbitrary predicates. Our predicates are carries, comparisons and the one-split transform layers; many children $Z_{g,\beta}$ of a parsimonious arithmetic network may be identically zero. If the nonzero fraction were exponentially small in $M$, the constant would move. This is the **only** cheap, decisive experiment: implement one deformation level (no recursion) on a toy network with $s\approx10^3$–$10^4$ edges using the crate's `deformation`/`packing` modules, count nonzero children against the (D.15) bound and against $2^M$. Falsifier: a nonzero fraction that is $\Theta(1)$ or $2^{-o(M)}$ kills the route at every size. Preregister per `controlled-experiment` with random predicates as the control arm.
3. **Word-level edges.** Theorem D.1 already allows entries in $\mathbb Z_{2^{P_0}}[V]/(V^\ell)$, so $b$-bit residues could ride on single edges and $s$ would count words. The Hermite taper and the modes act on the 2-adic structure of the lifted entries, so the exponent ratio against brute force is unchanged; only the bookkeeping shrinks.
4. **Dropping exactness** (parity counting, Valiant–Vazirani isolation, witness search only; Lead 2C component 3). Parity of the same contraction has the same mode expansion, so the per-edge constant is unchanged; what changes is the number of calls, which is already polynomial.
5. **Reducing $s$ by better encodings** (Lead 2C components 1–2, the LWE paper's $24nr+n$ network). Constant factors in $s$ cannot matter against a $37/\sqrt{\varepsilon\lambda}$ versus $0.69$ comparison.
6. **For ECDLP even with a perfect engine:** the Bochum bound $2^{c\,b/(\log b)^{1/4}}$ beats $2^{b/2}$ at $b=256$ only if $c<0.84$. Any engine with a per-edge constant visible above $\ln 2$ fails before the index-calculus arithmetic even starts.

So the honest position for the lab: making the recursion work at small sizes is equivalent to finding an exact counting primitive for arithmetic tensor networks whose branching constant is below $\ln 2$ per edge by a margin that survives $(\log b)^{1/4}$. Nothing in either paper supplies one; item 2 is the one place where structure might, and it costs a day to test.

---

## 5b. Literature search: has anyone built a small-size version, or a counting primitive that would do? (2026-11-21)

Searched for (i) independent analyses of the OpenAI/Anthropic constants and any attempt to lower them, (ii) reported errors in Theorem D.1, (iii) existing exact or decision algorithms for arithmetic/polynomial systems with per-variable constant below brute force, (iv) reactions to the Bochum ECDL preprint. Web search plus full-text reads of the sources below (PDFs extracted locally where the HTML was truncated). research-kb was unavailable in this session.

### 5b.1 Independent constant extractions agree with §5a

- **Dias & Lima, "Limits and Constants of OpenAI's Claimed ETH Refutation", arXiv:2611.00517 (31 Oct 2026).** Extract constants from Appendix D only. Per-edge constant $K_0=2\pi\sqrt{24\cdot 2^{17}}/\ln 2\approx 16{,}077$ bits per edge per $\sqrt\lambda$, which is exactly §5a.3's $2\pi\sqrt{24}\,M/\sqrt{u}$ with $u=\varepsilon\lambda$ converted to bits. Crossover: "the certified running time is at least $2^{n'}$ unless $\log_2(n+m)>(2K_0)^2\approx 1.03\times 10^9$"; random 3-CNF at density 4.26: $\log_2 n>5.3\times10^{11}$. They report the exact mode count is within 10–13 % of the generating-function estimate in the exponent, so the (D.15) bound is tight in practice, not just up to polynomial factors. They claim no logical error ("We take the preprint's proofs at face value") and say nothing about cryptography.
- **Majdoddin, "Lowered crossover points of Anthropic's subexponential 3-SAT algorithm", arXiv:2610.41622 (30 Oct 2026; written with Claude models per its AI statement).** This is the "extend it to smaller sizes" exercise, done for the Anthropic algorithm with every lever available: least admissible group sizes, traced constant $c=30$, a contraction-tree base case in place of brute force ($e_0=1/2\to0.2554$), exact edge-bit counts, a top-level clustering lemma. Ablation (Table 4, density one, $\log_2 m^\ast$): $33858\to28761\to22264\to5396\to1538\to1685\to417$. Density 4.26: $552040\to10766$. General 3-SAT in $n$: $2^{2.2\times10^{12}}\to2^{8.6\times10^9}$ variables. Own verdict: "No SAT running time in this paper is usable" and "The distance is structural, not a matter of constants." On the **OpenAI Appendix D route** (§3.9, read verbatim): $C_w=18{,}535$ bits per root wire, "dominated by $1/\sqrt\varepsilon=362$", thresholds $\log_2 s^\ast\approx4.4\times10^{10}$ (density one) and $1.4\times10^{12}$; the $u\ge\lambda^{4/5}$ condition "serves only to let $2^u$ absorb fixed powers of $\lambda$, for which $u$ of a few hundred suffices"; "With every improvement that we could justify, but did not prove, $C_w$ falls to 106–134 and the thresholds to $\log_2 s^\ast\approx3.7\times10^5$–$5.9\times10^5$". So the best anyone has argued for the tensor route is networks of $2^{370{,}000}$ edges. For comparison, our decomposition networks have $\lambda\approx20$–$30$.
- **Szeider** (ETH discussion site, 27 Oct / 1 Nov): "the construction of the main text is exhaustive search until $n$ exceeds $2^{10^{10}}$, and the Appendix D algorithm behind Theorem 1.1 at least until $n+m$ exceeds $2^{131071}$." Same reading as Bochum footnote 4 and the LWE paper §7.3.
- **O'Donnell, "Some exposition of 'The Exponential Time Hypothesis Is False'" (30 Oct 2026).** Recasts the OpenAI main-body algorithm as interpolating $P(t)=\mathrm{per}(A(t))$ for a sparse matrix; the engine is that a block-diagonal matrix with blocks of size $s$ plus a rank-$r$ perturbation has permanent cost $2^{O(s)}n^{O(r)}$. This is the cleanest statement of the mechanism: the saving is a block-size/rank trade, and both the block size and the deformation degree are governed by "sufficiently large $n$" constants throughout. No numerical constants are given.

### 5b.2 Review status of the engine

Impagliazzo, Paturi and Williams run `ethdiscussion.blog` (set up 26 Oct 2026): "Our involvement should not be taken as certification of the claim." As of the newest comment seen (16 Nov) no post reports an error in either proof. Commenter-reported Lean replications: the Anthropic proof accepted by Lean's default kernel and by `nanoda`; the OpenAI comparator accepted by the default kernel only, with `nanoda` runs hitting resource limits on the 2.45 GB export. The open gap discussed there is the step from each lab's machine model to textbook ETH ("a reading, not a theorem"). Theorem D.1 has no Lean statement anywhere. One commenter (Tal, 5 Nov) posts an unedited AI-generated draft applying "arithmetic-counting reductions to Theorem D.1" using the Bochum encodings; another (Gupte) keeps a spreadsheet of cryptographic consequences, not opened. Fortnow's blog (28 Oct) takes ETH as "toast" without checking and records no technical objection; one commenter notes the galactic threshold. The Hacker News thread on the Bochum paper has one comment ("very impractical").

### 5b.3 Existing primitives that beat brute force on polynomial systems

The closest real thing to the primitive §5a.4 asks for is the Razborov–Smolensky / parity-counting line:

| Work | Problem | Bound | Notes |
|---|---|---|---|
| Lokshtanov, Paturi, Tamaki, Williams, Yu (SODA 2017) | degree-$d$ systems over $\mathbb F_q$ | $q^{n(1-1/(Cd))}$; $2^{0.8765n}$ at $q=d=2$ | includes a deterministic counting variant |
| Björklund, Kaski, Williams (ICALP 2019) | $\mathbb F_2$, symbolic interpolation | $2^{0.804n}$ at $d=2$ | parity counting |
| Dinur (SODA 2021) | $\mathbb F_2$, multiple parity counting | $2^{0.695n}$ at $d=2$ | |
| Dell, Haak, Kallmayer, Wennmann (SODA 2025, arXiv:2410.20162) | all $\mathbb F_q$ | $q^{\zeta_{q,d}n}$ with $\zeta\le1-\min\{1/(8\ln q),\,1/(4d)\}$ | randomized decision via Valiant–Vazirani isolation; **not** a counter; conditional lower bound: no $q^{(1-\delta)n}$ for all $d$ under SETH |

What this means for us: these do have per-variable constants below brute force, but (a) the saving is $1/(8\ln q)$ at best, about $7\times10^{-4}$ for a 256-bit prime, so over $\mathbb F_p$ directly they are worthless; (b) over the Boolean encoding they are measured against the **total** variable count including the $\Theta(10^3\,mb)$ auxiliaries of a parsimonious network, so $2^{0.7N}\gg2^{b}$; (c) they need bounded degree in the variables they count over, and eliminating the auxiliaries from the point-addition chain gives the summation-polynomial degrees $2^{m-1}$ per variable. None of them is a counting primitive for arithmetic tensor networks, and none approaches the $(\log b)^{1/4}$ margin. Tensor-network counting work (Kourtis et al., SciPost 2019; planar/bounded-degree contraction in $D^{O(\sqrt{\Delta N})}$) is subexponential only for small-separator graph classes, which our dense transform layers are not; Holant dichotomies (Cai–Lu–Xia) classify our signature sets as #P-hard with no tractable special case.

### 5b.4 Noise

`rohanarun/subexponential-ecdl-hillclimb` (unaffiliated AI-agent harness) advertises "beating Pollard rho requires $m\ge7$ and $b>2^{193}$". Its derivation sets every hidden constant to 1, drops the $b/m$ term from its own cost table (restoring it gives $b>2^{381}$ at $m=7$), mixes group operations with bit operations, and its "$C_0\le256$" threshold claim multiplies stated factors $1\times8\times2$ to 256. Its only code is exhaustive search on a 101-element curve. Not a source.

### 5b.5 Conclusion of the search

Nobody has a small-size version, and the two groups who tried hardest say in their own words that the gap is structural. The best argued-for threshold on the tensor route is $2^{3.7\times10^5}$ edges (Majdoddin, improvements "justified but not proved"); the certified one is $2^{1.03\times10^9}$ (Dias–Lima). Independent constant extractions reproduce §5a.3's branching constant to within rounding. No primitive in the literature counts arithmetic tensor networks with a per-edge constant below $\ln2$ by any margin, let alone $(\log b)^{1/4}$. The recommendation in §5 stands; item 2 of §5a.4 (effective branching on structured arithmetic networks) remains the only cheap experiment with any chance of moving the number, and even Majdoddin's "justified" improvements put the target at $C_w\approx100$, still two orders of magnitude above what ECDLP at any size would need.

---

## 5c. Can the density-one #3-SAT crossover go below $2^{420}$, and would it matter for ECDLP? (owner question, 2026-11-21)

### 5c.1 Majdoddin's threshold model, as he states it

- **Bar** (Def. 3.1): the best known algorithm's exponent per edge bit. For 3-CNFs at density $\Delta$ with the exact edge-bit count $U=2(km-n)$: $\bar e_U(\Delta)=\min\{0.5283,\,0.711/\Delta\}/(6-2/\Delta)$, i.e. **0.1321 at density one** (Peng–Sheng–Xiao $1.4423^m$ and Kutzkov $1.637^n$). For general 3-SAT through sparsification and the parity kill, about $3.2\times10^{-4}$ per edge bit (Liu's $2^{0.4092n}$ over $\approx1270$ edge bits per variable).
- **Threshold**: $\log_2 m^\ast=q\log_2 K_L$ for the first level $L$ whose exponent $e_L$ is below the bar, with $q\ge2$ the balancing exponent ($q=2.03$ after his Lemma C).
- **Dichotomy** (Prop. 4.1): if $e(K)\ge C(\log_2K)^{-\alpha'}$ then $\log_2 m^\ast\ge q\,(C/\bar e)^{1/\alpha'}$; if $e(K)\le CK^{-\beta}$ then $\log_2 m^\ast\lesssim(q/\beta)\log_2(C/\bar e)+q$. The squaring recursion $K_{\ell+1}=K_\ell^2$ with per-level factor $\rho$ gives $\alpha'=-\log_2\rho$: Anthropic's Appendix B pieces have $\rho=1/\sqrt2$, so $\alpha'=1/2$ and $\log_2 m^\ast\approx q/\bar e^{\,2}$ plus constants.
- **Lower bound** (Thm 4.13, Cor. 4.15): every grid algorithm in his rewrite-and-merge class has $e(K)\ge1/\log_2K$ ($C=1,\alpha'=1$), hence $\log_2 m^\ast\ge q/\bar e$ whatever its pieces. Inside Appendix B's piece shape, $\rho\ge2/3$ (Prop. 4.24); $\rho\ge1/2$ for all two-merge levels; below $1/2$ needs identities nobody has.

### 5c.2 Density one: yes in principle, and the class does not forbid it

| Scenario at density one, $\bar e_U=0.1321$, $q=2.03$ | $\log_2 m^\ast$ (main term, no overheads) | with Majdoddin's observed overhead factor ($417/116\approx3.6$) | status |
|---|---|---|---|
| Appendix B pieces, $\rho=1/\sqrt2$ ($\alpha'=1/2$): $q/\bar e^{\,2}$ | 116 | **417** (his proved value) | proved |
| $\rho=2/3$ (floor of Appendix B's shape) | $\approx65$ | $\approx230$ | needs new pieces of the same shape |
| $\rho=1/2$ (floor for two merges; $\alpha'=1$): $q/\bar e$ | **15.4** | $\approx55$ | needs pieces of a different shape (open problem 1) |
| his hypothetical constant levers alone (no balancing slack $7\times$, sharper tails $1.2\times$, overheads $1.4\times$) | | $417\to\approx35$–$60$ if all proved | each "hypothetical, needs proof" |

So below $2^{420}$ is not forbidden: the grid-class floor at density one is only about $2^{15}$ clauses, because the bar per edge bit there is large. The $2^{6000}$ figure in his abstract is for **general** 3-SAT, where sparsification and the parity kill shrink the bar by a factor of 400. Reaching $2^{50}$-ish at density one needs either the $\rho$ window $[1/2,1/\sqrt2)$ closed with cheap, buildable four-leg pieces (his open problem 1) or his slack and tail levers proved; these are theorems to find, not parameters to tune. And a crossover at $2^{40}$–$2^{60}$ clauses still beats nothing anyone can run: the bar algorithm itself is $2^{0.711n}$ there.

### 5c.3 ECDLP: the bar per edge bit is what kills it, and no #3-SAT progress moves that

Apply the same model to the Bochum decomposition network. Edge count per secret bit, from Bochum's constants ($<1100(d+1)$ edges per $d$-bit integer multiply, so $\approx2300b$ per modular multiply with the guessed quotient; about 12–15 modular multiplies per point addition with both branches evaluated unconditionally; one more per membership test): $s\approx3\times10^4\,mb$ edge bits. A decomposition query may cost at most $2^{(1/2-1/m)b}$ to beat rho after $r^{1/m}$ relations. Hence

$$\bar e_{\text{ECDLP}}\approx\frac{(1/2-1/m)}{3\times10^4\,m}\approx2\times10^{-6}\ \text{per edge bit}\quad(m=4),$$

about 65,000 times smaller than #3-SAT at density one and 160 times smaller than general 3-SAT. Then:

| Route | Formula | $\log_2 s^\ast$ | edges |
|---|---|---|---|
| any grid algorithm, any pieces (proved floor, $C=1,\alpha'=1$) | $q/\bar e$ | $10^6$ | $2^{10^6}$ |
| Anthropic pieces as known ($\alpha'=1/2$) | $q/\bar e^{\,2}$ | $5\times10^{11}$ | |
| OpenAI Appendix D as read by Majdoddin ($C_w=18535$) | $(C_w/\bar e)^2$ | $9\times10^{19}$ | |
| Appendix D with his unproved improvements ($C_w\approx106$) | $(C_w/\bar e)^2$ | $3\times10^{15}$ | |
| hypothetical polynomial decay, $\beta=0.1$, $C=1$ | $(q/\beta)\log_2(C/\bar e)+q$ | $\approx380$ | $2^{380}$ |
| hypothetical polynomial decay, $\beta=1$, $C=1$ | same | $\approx21$ | $\approx2\times10^6$, our actual size |

Reading: for the tensor route to touch ECDLP at the sizes we build, one would need an exact clique-sum or network contraction whose exponent per edge bit decays like $1/K$ with constant about 1, which is "contract a $K$-node clique sum in time about $2^{S/K}$". Nothing in either preprint, in Majdoddin's class, or in the polynomial-systems literature (§5b.3) has polynomial decay at all; his §4.8 says polynomial decay needs hyperedge configurations meeting three or more groups of a merge, with no known construction and no realisability test. The only ECDLP-side lever is the bar itself, i.e. edge bits per secret bit, and cutting $3\times10^4$ by a constant factor moves $\log_2 s^\ast$ by the same constant factor on a scale of $10^6$ to $10^{19}$.

**Bottom line.** Lowering the density-one #3-SAT crossover below $2^{420}$ is a legitimate open problem with a plausible target around $2^{50}$, and it is entirely about new piece families and proved slack bounds, not constants. It would not move ECDLP by a visible amount: the ECDLP floor in the same model is $2^{10^6}$ edges for the whole class and $2^{21}$ only for a hypothetical algorithm type that does not exist. The lab's position from §5 stands.

---

## 5d. Pursuing a sub-$2^{420}$ density-one #3-SAT crossover (owner request, 2026-11-21)

Model, sweep and validation: `research/tensor_contraction_ecdlp_review_20261121/threshold_model/` (`threshold_model.py`, `sweep.py`, `sweep_results.txt`, README).

### 5d.1 Instrument

I reimplemented Majdoddin's main-term threshold model from his Section 3 and the Anthropic paper's Theorem A: recursion $K_{\ell+1}=(s_\ell K_\ell)^2$, $e_{\ell+1}=(\rho+o_\zeta\zeta_\ell)e_\ell+o_K/K_\ell$ with the paper's $o_\zeta=o_K=3$; admissibility (S1) $2\zeta^2 s\ge\ln(2sK^2)$, (S2) $1/(sK)\le\zeta$; least admissible $s$ per level; Pareto dynamic programme over slack schedules; crude top $q\log_2K_L$ or the Lemma C top of his Proposition 3.9 with the (T1) size condition and the $\nu=10^{-3}$ lower-order budget. It reproduces his published columns: 33858 (exact), 28761 (28761.3), 22264 (22263.9), 5396 (5397.4), 1538 (1555, two levels at $x_0=10$), 5037 (5057), and **417 (413.2**, two levels, $x_0=11$, $q=2.39$ against his 2.41). His per-level schedule for column (X) is matched level by level once the slack grid extends below $2^{-8.5}$. I treat agreement within 1 % as validation of the model, and all numbers below are main-term values in that model, i.e. lower bounds on the crossover of the proved bound, exactly as his are.

### 5d.2 Two overheads that are slack in the Anthropic proof (proved by re-reading)

Both corrections have the same status as his columns (G), (X), (U), which "only re-read the paper's proof".

- **$o_\zeta$: 3 → 2.** (36)/(41) give $U\le(\rho+1/g)N+4cg^4\log(2\lambda)$ for the cheaper of the row and column networks, where the $N/g$ is the straight edges inside rows; Lemma 8 gives $K^2L\le U+2\zeta N+K^2$. The paper writes $(\rho+3\zeta)N$ by replacing $1/g$ with $\zeta$ via (S2). Kept exact, the per-level factor is $\rho+2\zeta+1/g$, and $1/g=1/(sK)<2^{-30}$ on every computed schedule.
- **$o_K$: 3 → 1.** The additive $3/K$ in (40) is $2N/K$ from Lemma 8's entry cost plus $N/K$ from absorbing the pieces' $2^{c\lambda}$ in (44) via $c\lambda\le N/K$. Lemma 1's entry cost is $(2^P)^{|G_\gamma|}$, with $|G_\gamma|\le 2g/K$ in the Chebyshev case (i), hence $2^{2gP/K}=2^{2N/K}$. The recursion uses case (ii), whose groupings have exactly $g/K$ nodes per group (Lemma 8's proof: "uniformly random among the groupings in which every group has exactly $g/K$ nodes"), so the entry cost is $2^{N/K}$. The absorbed term is really $c\lambda/N=c/K_{\ell+1}^2<2^{-60}$. So $e_{\ell+1}=(\rho+2\zeta_\ell+1/g_\ell)e_\ell+1/K_\ell+c/K_{\ell+1}^2$.

### 5d.3 Results at density one (bar 0.1321, tree base, Lemma C top, $x_0$ optimised)

| Case | status | $\log_2 m^\ast$ | levels | $\log_2K_L$ |
|---|---|---|---|---|
| B0: Majdoddin (U+C), $o_\zeta=o_K=3$ | proved | 413.2 (his 417) | 2 | 173.0 |
| $o_K$ 3→1 only | proved (re-reading) | 399.7 | 2 | 166.6 |
| $o_\zeta$ 3→2 only | proved (re-reading) | 398.2 | 2 | 166.7 |
| **both** | **proved (re-reading)** | **384.6** | 2 | 160.3 |
| both + $e_0=1/4$ ($\omega(1,\tfrac12,1)=2$) | conditional on two preprints | 356.7 | 2 | 147.6 |
| both + $e_0=0.2456$ (his $K=4$ one-level construction atop the tree) | conditional on the current $\omega$ curve | 341.1 | 2 | 140.9 |
| both + $o_\zeta=1$ | hypothetical (sharper balancing) | 359.2 | 2 | 147.8 |
| both + $o_\zeta=0$ | hypothetical (no balancing slack) | 183.8 | 2 | 64.6 |
| both + $\rho=2/3$ | hypothetical (new pieces, App. B shape floor) | 300.3 | 2 | 121.7 |
| both + $\rho=0.6$ | hypothetical (new pieces) | 253.0 | 2 | 100.0 |
| both + $\rho=1/2$ | hypothetical (two-merge floor) | 174.6 | 1 | 59.6 |
| B0 with bar 0.125 | sensitivity only | 608.2 | 3 | 274.1 |
| both with bar 0.125 | sensitivity only | 542.8 | 3 | 240.5 |

**Answer to the question.** Yes: on the same footing as Majdoddin's 417, the density-one main-term threshold is $2^{385}$ after two corrections that only re-read the Anthropic proof. With the matrix-multiplication claims he already uses conditionally it is $2^{341}$–$2^{357}$. Nothing else that is proved or plausibly provable this way moves it further: every remaining lever is either new piece families ($\rho$) or a balancing lemma that beats two-stage Hoeffding, and his dichotomy still applies, so these are not usable sizes in any case.

**Where the 385 bits sit.** $2\log_2K_2\approx320$ plus about 65 bits of top-level overhead. $\log_2K_2=4(x_0+\log_2s_0)+2\log_2s_1$ with $x_0=10$ and $\log_2 s_0,\log_2 s_1,\log_2 s_{\text{top}}\approx16,20,27$: roughly 70 % of the exponent is Hoeffding group-size overhead, $s\approx\ln(2sK^2)/(2\zeta^2)$, forced by small slack $\zeta$ because $\rho=1/\sqrt2$ leaves little room under the bar after two levels. That is why "no balancing slack" is worth a factor 2 and $\rho\to1/2$ a factor 2.2, and why the two are the only levers of scale.

**Bar check.** Majdoddin's density-one bar is Peng–Sheng–Xiao's exact deterministic #3-SAT counter in $O^\ast(1.4423^m)$ (IJCAI 2025), so it is counting against counting; the older Zhou–Yin–Zhou $O(1.4142^m)$ claim (AAAI 2010) is stated by Peng–Sheng–Xiao to have an analysis that "does not yield a valid runtime bound", so the 0.125 rows are a sensitivity, not a correction. The 417 is not optimistic on this account.

**Caveats.** Main-term values only; the lower-order budget $\nu\cdot 2ekm$ at $2^{385}$ still covers $C_0K^2\log_2(2Kkm)$ by a margin of about $2^{40}$ against his $C_0\le810$, but a fully explicit theorem would restate Proposition 3.2 with the corrected per-level recursion. I have not written that restatement as a proof; the two corrections are each a one-line re-reading of displays (36), (41), (44) and Lemma 1's cost paragraph, cited above, and can be checked in minutes.

---

## 5e. The two levers of scale (owner request, 2026-11-22): a balancing lemma that works, and where the piece problem actually stands

Code: `research/tensor_contraction_ecdlp_review_20261121/threshold_model/qb_model.py` (lever A pricing). Everything below is on the same main-term footing as §5d.

### 5e.1 Lever A: quantile balancing with enumerated excess (new lemma, priced)

**Where the slack comes from.** Lemma 8 of the Anthropic paper groups the $g$ nodes of a term's network into $K$ groups and pads every pair of groups to the *maximum* pair load $L_{\max}=(U+2\zeta gP)/K^2$ that Hoeffding plus a union bound over $(g+1)K$ events guarantees. That is where (S1) $2\zeta^2 s\ge\ln(2sK^2)$ comes from, and (S1) is what makes $s$ of order $2^{15}$–$2^{28}$ per level: about 70 % of the exponent in §5d. The clique exponent $e\le 0.26$ charges only $e$ per padded bit, while an *enumerated* bit costs 1; but the enumerated bits can be the typical excess rather than the worst-case one.

**Lemma Q (quantile balancing).** Let a network on $[g]$ have parameters $P,B,\tau$ and total $U$, with $c_{a,b}\le P/g$ and $K\mid g$. Put $\mu=U/K^2$ and $\sigma^2=2PU/K^3+PU/(gK^2)$. For every integer $L\ge\mu$ there is a grouping into $K$ groups of exactly $g/K$ nodes with
$$\sum_{\gamma\ne\gamma'}\bigl(Y_{\gamma\gamma'}-L\bigr)^+\;\le\;X_L:=\tfrac{K^2}{2}\Bigl(\sqrt{\sigma^2+(L-\mu)^2}-(L-\mu)\Bigr),$$
and the value of the network is computed with $K^g+2^{X_L}\,T\,2^{gP/K}\,\tau\,(g(P+B))^{O(1)}$ operations, where $T$ is the cost of one $(K,L)$-clique.

*Proof sketch.* (i) For a uniformly random equipartition, $\mathbb E Y_{\gamma\gamma'}=\mu(1+O(1/g))$, and $\operatorname{Var}Y\le\sigma^2$: expand $Y=\sum_{a\ne b}c_{ab}\,1[\chi(a)=\gamma]1[\chi(b)=\gamma']$; indicator pairs on disjoint node pairs have covariance $\le 0$ for an equipartition, pairs sharing one node have covariance $\le 1/K^3$, identical pairs $\le 1/K^2$; with $\sum_b c_{ab}=c_a\le P$, $\sum_a c_a=U$ and $c_{ab}\le P/g$ this gives $\operatorname{Var}Y\le 2PU/K^3+PU/(gK^2)$. (ii) For a mean-zero variable with variance $\sigma^2$ and $\delta\ge0$, $\mathbb E(X-\delta)^+\le(\sqrt{\sigma^2+\delta^2}-\delta)/2$ (the two-point extremal bound). (iii) By linearity some equipartition attains at most the expectation of the sum over the $K(K-1)$ pairs; the brute-force search over groupings that Lemma 1 already performs finds it. (iv) For each pair with $Y>L$, fix its $Y-L$ excess shared bits by enumeration outside the clique call (the merged tables read them as constants); pad light pairs with zero bits as the paper does. Each of the $2^{\sum(Y-L)^+}$ calls is a $(K,L)$-clique; parsimony is unaffected. Groups of exactly $g/K$ nodes give entry cost $2^{gP/K}$ (§5d.2). $\square$

**Corollary (level recursion without (S1)).** Choosing $L=\mu+d\sigma$ and optimising $d$, one level costs, in the exponent per $N=gP$,
$$e_{\ell+1}=\rho\,e_\ell+\beta(e_\ell)\sqrt{\tfrac{2\rho'}{s_\ell}+\tfrac{\rho'}{s_\ell^2}}+\tfrac{1}{K_\ell},\qquad \beta(e)=\min_{d\ge0}\Bigl[e\,d+\tfrac{\sqrt{1+d^2}-d}{2}\Bigr],\qquad \rho'=\tfrac{U}{gP}\le\rho+\tfrac1g,$$
with $K_{\ell+1}=(s_\ell K_\ell)^2$ and $s_\ell\ge1$ free. ($\beta(0.2)=0.40$, $\beta(0.13)=0.33$.) The top level (Prop. 3.9) changes the same way, with $\rho'_{\text{top}}=U/(gP)\ge(6-2/\Delta)/(2\kappa(1+\varepsilon)k)$ from the clustering lemma and (T1) unchanged.

**Pricing (main term, Lemma C top, tree base, $x_0$ optimised).**

| density | Hoeffding balancing, proved re-reading (§5d) | **quantile balancing** | schedule |
|---|---|---|---|
| 1 | 384.6 | **323.5** | $x_0=10$, two levels, $\log_2K_L=136$, $s=2^{15},2^{18}$, $s_{\text{top}}=2^{21}$ |
| 4.26 | 9669 | **7525** | $x_0=6$, seven levels, $s_{\text{top}}=2^{27}$ |

So the density-one threshold moves to $2^{324}$: with §5d, a factor $2^{93}$ below Majdoddin's $2^{417}$, all of it from re-reading the proof and one lemma about balancing. The remaining $\sqrt{s}$ dependence is now the honest variance of pair loads, not a union bound; what is left of lever A is the constant in $\sigma$ (weights in a band, his "sharper tails") and the $\beta(e)$ factor, each worth a few bits.

### 5e.2 Lever B: where pieces with $\rho<1/\sqrt2$ stand

**The problem, stated exactly (from Appendix B and Majdoddin §4.7).** A graded space $V=\bigoplus_E V_E$ with profile $D(E)=\log_2\dim V_{\le E}$ and a "local" operator calculus: strings $x,v$ name operators $a_x$ (weight $\le h$), $b_v$ (weight $h$), strings $y,u$ name states of weight $h$, and the pieces $A_s$, $s\in[-2h,2h]$, are matrix entries of $Q_s(a_x,b_v)$, a signed sum of $2h+1$ two-factor products that can be read in either order (vertex-algebra locality, Lemma 9(a)); $\sum_sQ_s=(a[-h]b)\,\mathrm{Id}$ with dual bases (Lemma 10). Reading product $i$ in one order passes through $V_{\le 2h+s}$, in the other through $V_{\le 2h-s}$. Three constraints then fix $\rho$:
1. naming: $D(h)\ge\lambda$;
2. rank: the pieces sum to the crossing, of rank $2^{2\lambda}$ in each split, so $D(4h)\ge2\lambda-\log_2|I|$;
3. cost: $4\rho\lambda=\max_s\,[D(2h+s)+D(2h-s)]$.

For $D=cE^\alpha$, (2) forces $\alpha\ge1/2$ and (3) gives $2^{\alpha-1}$ at $s=0$: the unique optimum is $\alpha=1/2$, $\rho=1/\sqrt2$, which is partitions, i.e. the Heisenberg vertex algebra. Any concave profile gives $\rho\ge2/3$ (his Prop. 4.24, equality for $D$ affine on $[h,4h]$). To go below one needs a **non-concave** profile: flat near $\lambda$ on $(h,4h)$ and jumping to $2\lambda$ at $4h$ gives $1/2$, and its pieces are the "(full, free)" and "(free, full)" pair. No local graded algebra with such a profile is known (his words and mine).

**Dead ends checked this session.**
- *Restricting the Fock space* (parts bounded by $r$, or any mode restriction) makes $D$ grow *slower* beyond $h$ while $D(h)$ is fixed, so (2) fails: $D(4h)<2D(h)$ because the restricted-partition growth $\sqrt E\,g(r/\sqrt E)$ has $g$ increasing. Enlarging the windows to compensate is punished by concavity (Prop. 4.24 with $\mu>1$, limit $3/4$). Majdoddin already excludes the fermionic Fock space (one-order pieces only).
- *The two-piece shape.* $d[x=v][y=u]=F(x,u)G(y,v)+a(x,y)b(u,v)$ asks for $\mathrm{SWAP}_N=F\otimes G+\text{rank one}$ on $(\mathbb C^N)^{\otimes2}$. For $N=2$ this is the paper's single-bit identity, $\mathrm{SWAP}=I-2P_{\text{singlet}}$. For $N\ge3$ it is impossible: a rank-one perturbation preserves every eigenvalue of multiplicity $m$ with multiplicity $\ge m-1$, so $F\otimes G$ would need $\pm1$ eigenvalues of multiplicities $N(N\pm1)/2-1$, and the entries with $x\ne v$, $y\ne u$ force the principal $(N-1)$-blocks of $G$ (and of $F$) to have rank $\le1$, which with full rank leaves only zero-diagonal tournament patterns, and those give a difference of two permutation-like matrices of rank $\ge N^2-N>1$. Alternating projections confirm: relative residual $2\times10^{-16}$ at $N=2$, $0.75$ at $N=3$, $0.87$ at $N=4$ over 20 starts.
- *Amortising the single-bit identity.* In the row split $T_1=I-aa^{\!\top}$ and in the column split $T_1=ff^{\!\top}-P$, with all terms of $T_1^{\otimes\lambda}=\sum_Q(-1)^{|Q|}(aa^{\!\top})^{\otimes Q}\otimes I^{\otimes\bar Q}$ simultaneously diagonal in each split. A piece is a class $\mathcal Q$ of subsets; its row rank is the weighted support of $Z\mapsto\sum_{Q\in\mathcal Q,Q\subseteq Z}(-2)^{|Q|}$ (weight $3^{\lambda-|Z|}$) and its column rank the weighted support of $W\mapsto\sum_{Q\in\mathcal Q,Q\supseteq W}(-1/2)^{|Q|}$ (weight $3^{|W|}$). On the middle layer $|Q|=\lambda/2$ both sums are plain counts, so no cancellation is possible and $\mathrm{rank}_{\text{row}},\mathrm{rank}_{\text{col}}\ge|\mathcal Q|\,3^{\lambda/2}$; with $p$ pieces some class has $|\mathcal Q|\ge\binom{\lambda}{\lambda/2}/p$, hence cost $\ge2\lambda+\lambda\log_23-2\log_2p-O(\log\lambda)$ and $\rho\ge(2+\log_23)/4=0.896$. That is exactly the paper's type-class value, so the simple identity cannot be amortised at all; the graded-operator route is forced.

**Where this leaves lever B.** The honest target is a local graded operator system whose profile is flat between the naming weight and the top window; equivalently, a telescoping two-order identity whose intermediate spaces have dimension $\approx2^\lambda$ for all but the extreme shifts. Vertex algebras with positive effective central charge all have $\sqrt E$ characters, so the construction must leave that world or use a grading that is not the conformal weight. I have no candidate, and I did not find one by search; this is the open problem exactly as Majdoddin states it, now with the restricted-Fock and two-piece routes closed.

### 5e.3 Status after both levers

| | $\log_2 m^\ast$, density one |
|---|---|
| Majdoddin, proved | 417 |
| + proved re-reading of overheads (§5d) | 385 |
| + Lemma Q quantile balancing (§5e.1) | **324** |
| + $\omega(1,\tfrac12,1)=2$ (conditional) | about 275 (not run separately; scales as §5d) |
| + $\rho=1/2$ pieces (do not exist as far as anyone knows) | about 150 |

Lemma Q is the one piece of new mathematics delivered; it is short, its two ingredients are a covariance bound and the two-point tail inequality, and it should be written out with the $(1+O(1/g))$ corrections before anyone quotes 324 as proved. Lever B stands where it stood, with two more doors closed.

---

## 5f. Distance to cryptographic sizes (owner question, 2026-11-22)

Driver and table: `research/tensor_contraction_ecdlp_review_20261121/threshold_model/crypto_distance_fast.py`, `crypto_distance.txt`.

**Method.** A counting route helps against a target only when its cost on the target's network, $2^{e\,U}$, is below the target's best conventional attack. So each target fixes a bar, (bits of the conventional attack per counting call) divided by $U$, the edge bits of the network to contract, and the threshold model says how large an instance must be before the route's exponent per edge bit falls under that bar. The comparison is that required size against the target's actual network size.

| Target | network edge bits $U$ (assumption) | competitor per call | bar |
|---|---|---|---|
| ECDLP-256, Bochum index calculus, $m=4$ | $6.8\times10^7$ ($\approx15$ modular multiplies per point addition with both branches, $<1100(d+1)$ edges per $d$-bit multiply, two multiplies per modular multiply) | $(1/2-1/m)\cdot256=64$ bits per decomposition query | $9.4\times10^{-7}$ |
| AES-128 key recovery as 3-CNF | $6\times10^6$ ($\approx10^6$ clauses) | $2^{128}$ key search | $2.1\times10^{-5}$ |
| Ring-LWE $n=1024$, binary secret, 20-bit noise (the LWE paper's network) | $1.13\times10^6$ ($24nr+n$, $r=23$) | a 128-bit parameter set | $1.1\times10^{-4}$ |

Encoding sizes are order-of-magnitude; the Anthropic-route threshold scales roughly as $1/\text{bar}^2$, so a factor-10 error moves it by about 100, which does not affect any conclusion below.

**Results** ($\log_2$ of the instance size at which each route starts to win; "have" is $\log_2 U$ of the target itself; coarse grid, within about 15 % of the fine grid where both were run).

| Target | have | Majdoddin as proved | + §5d + Lemma Q (best proved here) | + $\rho=1/2$ pieces (hypothetical) | OpenAI App. D (as written / his unproved improvements) | any grid algorithm, any pieces (proved floor) | hypothetical polynomial decay, $\beta=1$ |
|---|---|---|---|---|---|---|---|
| #3-SAT density 1 (reference) | – | 413 | 323 | 148 | $2\times10^{10}$ / $1.6\times10^5$ | 15 | 8 |
| #3-SAT density 4.26 (reference) | – | $1.1\times10^4$ | $7.7\times10^3$ | 780 | $3.7\times10^{11}$ / $3.1\times10^6$ | 66 | 12 |
| general 3-SAT (reference) | – | $1.0\times10^8$ | $7.6\times10^7$ | $7.7\times10^4$ | $3.3\times10^{15}$ / $2.8\times10^{10}$ | $6.3\times10^3$ | 25 |
| Ring-LWE-1024 | 20 | $8.2\times10^8$ | $7.3\times10^8$ | $2.5\times10^5$ | $2.7\times10^{16}$ / $2.2\times10^{11}$ | $1.8\times10^4$ | 28 |
| AES-128 | 22.5 | $3.3\times10^{10}$ | $2.3\times10^{10}$ | $1.2\times10^6$ | $7.5\times10^{17}$ / $6.3\times10^{12}$ | $9.4\times10^4$ | 33 |
| ECDLP-256 | 26 | $2.9\times10^{13}$ | $1.9\times10^{13}$ | $3.2\times10^7$ | $3.9\times10^{20}$ / $3.2\times10^{15}$ | $2.1\times10^6$ | 42 |

**Reading.**
1. **The distance is double-exponential, not a constant.** For ECDLP-256 the best route proved anywhere (§5d + Lemma Q on the Anthropic recursion) needs networks of $2^{1.9\times10^{13}}$ edge bits against the $2^{26}$ the attack actually has: the exponent is $7\times10^{11}$ times too large. AES-128 and Ring-LWE are $10^9$ and $3.6\times10^7$ times too large in the exponent. The two levers of this session (2^417 to 2^324 at density one) are worth about a third in the exponent at crypto bars; they do not change the scale.
2. **Even ideal pieces do not close it.** With $\rho=1/2$ (pieces nobody has) the exponent is still $10^5$–$10^6$ times too large, and Majdoddin's Theorem 4.13 floor for *every* grid algorithm with *any* pieces is $2^{2\times10^6}$ edges for ECDLP, $2^{9\times10^4}$ for AES-128, $2^{1.8\times10^4}$ for Ring-LWE. That floor is proved, so no amount of work inside the rewrite-and-merge class reaches cryptographic sizes.
3. **The OpenAI route is further away still**, because its constant is large even though its shape (polynomial in 1/bar) is better.
4. **Only one algorithm type lands near crypto sizes:** an exponent per edge bit that decays polynomially in the clique size ($e(K)\approx K^{-\beta}$ with $\beta\approx1$), which would put the thresholds at 28, 33 and 42 against actual sizes of 20, 22.5 and 26, i.e. within a factor $2^{8}$–$2^{16}$ in size. **Correction (2026-11-22):** this gap cannot be closed by better encodings, because the requirement is $\log_2U\le2\log_2(G/C)-2$, so shrinking $U$ lowers both sides. The input floor forces $C\ge1$, so this column is the absolute optimum of the approach, and it reaches a target only if $U<G^2$; ECDLP-256 and Ring-LWE-1024 fail that for information-theoretic reasons and AES-128 is borderline at best. See `research/tensor_contraction_ecdlp_review_20261121/POLYNOMIAL_DECAY.md` §2–§3. Majdoddin §4.8 shows such decay needs configurations meeting three or more groups of a merge ("hyperedge" identities); no construction exists, no realisability test exists, and nothing in the literature (§5b) gives polynomial decay at all.

**Bottom line for the lab.** The distance from today's best proved threshold to any deployed cryptographic size is about $10^{7.5}$–$10^{12}$ in the exponent, and the proved class floor alone leaves $10^{3}$–$10^{5}$. Crossing it is not a matter of constants, tuning or the levers of §5d–§5e; it needs a new algorithmic primitive of a kind whose existence is open. That is the same verdict as §0 and §5c, now with numbers per target.

---

## 6. What I did not verify

- The proof of Theorem D.1 (OpenAI Appendix D, pp. 53–64). I read §D.4.1–D.4.2 for parameters only.
- The Anthropic `proof/` tree; I did not run `lake build` or `PrintAxioms`.
- The LWE paper's edge-count constants ($325nr$, $24nr+n$, degree $23n^{3/4}r$) and Bochum's ($1100(d+1)$, $136(d+1)^{3/4}$); only the structure was checked.
- The `hashed-ecdl-tensor` recursive-contraction source (`src/contraction/recursive.rs` was not fetched). Its README and `src/contraction.rs` were read: "The recursive contraction backend follows the pinned October 25 manuscript's Appendix D.4", "No practical or asymptotic performance result is claimed", "The source constants force direct fallback below an astronomical threshold", and "a fallback is never labelled a measured subexponential speedup". The `root_u == 0` statement is taken from `workspace:docs/notes/prime-pdp-relation-leads-20261119.md` Lead 2C and matches the formula in OpenAI §D.4.2.

---

## 7. Receipts

- Reviewed PDF: `main_4.pdf`, 49 pp., 909 KB; SHA-256 `e7acaedb089cb8c852b700c2b3e3ff3e876fe0449cbf9f77470a75ae4d2c688c`.
- Bochum PDF: fetched from `raw.githubusercontent.com/bochum-crypto/subexponential-ecdl/main/subexponential-ecdl.pdf`, 523,581 bytes, SHA-256 `82e273629b437660607117d5e99f400141ed7aa78b25c6749b03651f42deaa11`, which **matches** the hash in the LWE paper's Appendix B; text extracted with `pdftotext`; repository shows one file, 3 commits, no README.
- OpenAI PDF: fetched from `raw.githubusercontent.com/openai/math/main/preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026/paper.pdf`, ~867 KB, SHA-256 `5b1612dc2eebb2cad5b4cbe62d1fed2982f01ab1908c26f70010a651487e8123`, which **matches** Appendix B; text extracted; Theorem D.1 at p. 53, §D.4.2 parameters at p. 58. The LWE paper therefore reviewed the same bytes that are public today.
- Anthropic: `eth/README.md`, `MainResults.lean`, `Solution.lean`, `Challenge.lean`, `scripts/PrintAxioms.lean`, `proof/` and `scripts/` listings read via web fetch on 2026-11-21.
- research-kb: the MCP server was not available in this session and the trunk has no CLI; prior art was taken from the workspace notes cited above. Receipt: `kb_status` not consulted.
- Lab rows: `workspace:docs/notes/rho-parity-20261120/MEASUREMENT_TABLE.md` line "successor | `ic.hashed_native_v1`"; `workspace:docs/notes/rho-parity-20261120/ecbench-hashed-native-fp20/records.jsonl` (`status: exhausted`, `error: resource limit: factor-base column policy`, `limitations: ... no sub-rho claim`).
