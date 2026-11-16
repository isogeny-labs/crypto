# Index calculus on $E(\mathbb F_p)$ in the XL/Macaulay model: lower bounds, their exact conditions, and where they stop

**Standing.** Everything in this note is `[pcc:unverified]`.

- **Proofs:** desk proofs, written here. Twelve supporting statements are machine-checked in Lean 4.34.1 / Mathlib (§6).
- **Numbers:** local toy computations (python-flint, numpy, msolve 0.9) on a shared 4-core VM.
- **Not done:** no pcc claim filed, no `bench.record/v1`, nothing run on EKS, nothing committed.
- **Prior art:** research-kb and the lead atlas were not queried. The literature statements below come from the repo's own notes, as in the closure ledger.
- **Location:** code in `code/`, outputs in `out/`, Lean in `lean/ICCounting/`. All paths are relative to `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/finish/theorem/`.

**Not claimed.** This note does not claim that no algorithm beats Pollard rho on $E(\mathbb F_p)$. That is an open problem: generic-group lower bounds do not apply to index calculus. The results below hold inside one precisely defined restricted model, RAIC (§1), and each result states which hypotheses it uses.

---

## 0. Results at a glance

| # | Statement | Status | Exponent of the cost lower bound (rho: $1/2$) |
|---|---|---|---|
| Thm A | arity $m=2$, any factor base, any solver | **proved** in RAIC (uses A2, A3 and attempt cost $\ge1$) | $\ge 1/2$: $\mathbb E[\text{cost}]\ge c\sqrt n$ |
| Thm B | base with **no auxiliary variables** (univariate $f$), or a **graph** presentation $x=\varphi(t)$, $f_T(t)=0$; XL solver at any degree | **proved** in RAIC | $1$: $\mathbb E[\text{cost}]\ge(2^{-m}-o(1))\,n$ (graph: $(2(e+1))^{-m}n$) |
| Thm C | base with **auxiliary variables** (towers, Boolean bits, any complete intersection), XL solver | **conditional** on a typical-degree hypothesis $\mathrm G(\theta)$ about the *exact* XL degree (Lemma 4) | $\ge 1/2$ whenever the typical weak XL degree stays above $\Delta+\tau_m\rho$ ($\rho$ = Macaulay maximum; $\tau_m\le 0.110$ for binary towers and bits). Under the random-function model the exponent is $1$. |
| Gap | F4 / polynomial-calculus solvers on auxiliary-variable presentations | **not covered** | msolve refutes tower systems at F4 degree $4$–$5$ ($m=2$) and $12$ ($m=3$), whereas the exact XL degree is $8$–$22$ and $18$–$21$ (§5.5) |

The new ingredient is an exact formula for the XL/Macaulay solving degree (Lemma 4). For every presentation with a radical base ideal, the least degree at which XL refutes, eliminates or learns anything is the summation polynomial's degree plus an interpolation degree on the factor-base grid. Every exponent statement then reduces to the size of that interpolation degree:

- **No auxiliary variables:** the XL degree is at least $\deg f\ge|\mathcal B_x|$, proved by a three-line degree argument (Lemma 5, in Lean).
- **Auxiliary variables:** the interpolation degree can a priori be anything, so a hypothesis is needed. On every toy tower and Boolean-bit instance it sat within one degree of the random-function value, i.e. near the Macaulay maximum.

**What the theorem settles.**
- Inside the XL model it closes ledger rows A5 (fixed-degree root bases), A6-APS (vanishing-polynomial presentation of an interval base) and the parametrised cases of A4, with no heuristic.
- It does **not** close the one live item, auxiliary-variable (PKM-type) presentations. For those the binding solver is F4, not XL, and F4 escapes the model.

---

## 1. The model RAIC (restricted algebraic index calculus)

### 1.1 Group and relations
- **Curve.** $p$ is prime and $E:\ y^2=x^3+ax+b$ is a curve over $\mathbb F_p$ with $G=E(\mathbb F_p)$ of prime order $n$, so $n=p+O(\sqrt p)$.
- **Problem.** Given $P,Q\in G$, find $\lambda$ with $Q=\lambda P$.
- **Factor base.** Fix an arity $m\ge 2$ and a factor base $\mathcal B\subseteq G\setminus\{O\}$ closed under negation.
- **Relations.** A relation is a target $R=aP+bQ$, with $(a,b)$ uniform, together with $P_1,\dots,P_m\in\mathcal B$ such that $R=P_1+\dots+P_m$.
- **Decomposable targets.** $N_{\rm dec}$ denotes the number of $R\in G$ that admit such a decomposition.

**Summation polynomial** (Semaev 2004; literature, re-checked numerically in §5.1).
- $S_{m+1}(x_1,\dots,x_{m+1})$ is symmetric, of degree $2^{m-1}$ in each variable.
- For $x_i\in\bar{\mathbb F}_p$: $S_{m+1}=0$ iff there are $y_i$ with $(x_i,y_i)\in E$ and $\sum\pm(x_i,y_i)=O$.

Write $S_R:=S_{m+1}(x_1,\dots,x_m,x(R))$ and $\Delta:=\deg S_R$ (total degree in $x_1..x_m$; measured $\Delta=4,12,32$ for $m=2,3,4$ at random $x(R)$).

**Fact 0 (non-degeneracy).** For every $(x_2,\dots,x_m)\in\bar{\mathbb F}_p^{m-1}$, $S_R(X,x_2,\dots,x_m)$ is a nonzero polynomial in $X$ with at most $2^{m-1}$ roots. By symmetry the same holds in every variable.

*Proof.* A root $X$ is the abscissa of a point $\pm R\pm P_2\pm\dots\pm P_m$ with $x(P_i)=x_i$, so there are at most $2^{m-1}$ roots. A polynomial with finitely many roots over the infinite field $\bar{\mathbb F}_p$ is nonzero. $\square$

### 1.2 Factor-base presentations
A **presentation** is a tuple $\mathfrak B=(h_1,\dots,h_r)\subset\mathbb F_p[x,z_1,\dots,z_s]$, where $s\ge0$ counts the auxiliary variables.
- $J_1=(h_1,\dots,h_r)$ is assumed **radical**, with finite zero set $W\subset\bar{\mathbb F}_p^{1+s}$.
- $\varphi$ is the largest fibre size of $\pi_x:W\to\bar{\mathbb F}_p$.
- $\mathcal B_x:=\pi_x(W(\mathbb F_p))\cap x(G\setminus O)$ and $\mathcal B:=x^{-1}(\mathcal B_x)$.

So $|\mathcal B|\le 2|\mathcal B_x|\le 2|W|$.

| family | generators | $W$ | covers ledger rows |
|---|---|---|---|
| (U) univariate | $f(x)$, $\deg f=d$, squarefree | roots of $f$ | A5 (Kummer coset, random split, low height), A6-APS (interval $\prod_{v<2^s}(x-v)$) |
| (Γ) graph | $x\,\mathrm{den}(t)-\mathrm{num}(t)$, $f_T(t)$; $e=\max\deg$, $|T|=d_T$ | $\{(\varphi(r),r):r\in T\}$ | A4 parametrised images (Möbius, Lattès, Vélu, power maps of an algebraic $T$) |
| (T) binary tower (PKM chain) | $x^2-t_1,\ t_j^2-t_{j+1},\ t_{k-1}^2-c$ | $g\mu_{2^k}$, $|W|=2^k$ | A4 algebraic $\mu_{2^k}$, the PKM Kummer chain |
| (β) Boolean bits | $x-\sum_j2^jb_j,\ b_j^2-b_j$ | $\{0,\dots,2^s-1\}$ | A6 Boolean-bit Macaulay |

**H-basis property.** $\mathfrak B$ is an *H-basis* if $J_1\cap k[\cdot]_{\le D}=\sum_j h_j\,k[\cdot]_{\le D-\deg h_j}$ for all $D$.
- **(U), (T), (β) are H-bases.** Each is a Gröbner basis for a degree-compatible order, because its leading monomials $x^d$; $x^2,t_j^2$; $x,b_j^2$ are pairwise coprime (Buchberger's first criterion). A degree-order Gröbner basis has standard representations.
- **(Γ) is in general not an H-basis.** Degree falls occur. Only inequalities are claimed for it.

### 1.3 The decomposition system
- **Variables.** $m$ blocks $(x_i,z^{(i)})$, so $N=m(1+s)$ variables.
- **Equations.** $F_R=\{S_R\}\cup\{h_j^{(i)}\}$.
- **Base ideal.** $J=\sum_iJ_1^{(i)}$ is radical, with $V(J)=W^m$ (a tensor product of reduced finite algebras over a perfect field is reduced).
- **Grid zeros.** $V_R:=\{w\in W^m:S_R(x(w))=0\}$.
- **Eliminant.** $g_R:=\prod_{\xi\in\pi_{x_1}(V_R)}(x_1-\xi)$.
- **Degenerate targets.** $R$ is *degenerate* if $\pi_{x_1}(V_R)=\pi_x(W)$, so that $g_R\in J_1$.

**Degree filtration and interpolation degree.**
- $L_t$ is the space of functions on $W^m$ that are restrictions of polynomials of total degree $\le t$, and $H(t)=\dim L_t$ (the affine Hilbert function).
- For $V\subseteq W^m$ and a function $h$ on $W^m\setminus V$:
$$\delta(h;V):=\min\{t:\ \exists u\in L_t,\ u=h\ \text{on }W^m\setminus V\},\qquad \delta(0;V):=-\infty .$$
- $\rho:=\min\{t:H(t)=|W|^m\}$ is the regularity index; $\delta\le\rho$ always.

### 1.4 Solver: XL at degree $D$
- **Literal XL.** $\mathrm{Mac}_D(F_R)=\mathrm{span}\{u f: f\in F_R,\ u \text{ monomial},\ \deg(uf)\le D\}$.
- **Strong XL$^J$** (free reduction modulo the base): $\mathrm{Mac}^J_D:=S_R\cdot k[\cdot]_{\le D-\Delta}+J_{\le D}\supseteq\mathrm{Mac}_D$, where $J_{\le D}:=J\cap k[\cdot]_{\le D}$.

**Success notions at degree $D$.**
- *refutation*: $1\in\mathrm{Mac}_D$;
- *elimination*: $g_R\in\mathrm{Mac}_D$;
- *weak success*: $\mathrm{Mac}_D$ contains a block-1 polynomial $u(x_1,z^{(1)})\notin J_1$.

Weak success is the least any XL-based procedure needs before it has learned anything about $x_1$. The least such degrees are $D_{\rm ref},D_{\rm elim},D_{\rm weak}\in\mathbb N\cup\{\infty\}$, and $D_{\rm weak}\le D_{\rm ref}$; for non-degenerate $R$ also $D_{\rm weak}\le D_{\rm elim}$.

### 1.5 Axioms (the model's assumptions; nothing else is assumed)
- **A1 (cost).** An XL step at degree $D$ costs at least the rank of the space it computes: $\operatorname{rank}\mathrm{Mac}_D$ for literal XL, $\dim(\mathrm{Mac}^J_D/J_{\le D})$ for XL$^J$. An attempt that stops at degree $D$ costs at least that.
- **A2 (attempts).**
  - Attempt $i$ uses an independent uniform target $R_i$.
  - It runs XL degree by degree. It may stop at any degree, depending on the history and on what it has observed so far in this attempt: the ranks of the computed spaces and the success/refutation flags.
  - It can yield a relation only if $R_i$ is decomposable and weak success happens at or before its stopping degree.
  - Because stopping may react to ranks, a rank deficit is itself information. Its first degree is the *detection degree*
$$D_{\rm det}(R):=\min\{D:\dim(\mathrm{Mac}^J_D/J_{\le D})<H(D-\Delta)\}=\Delta+\delta_{\rm det}(R),\qquad \delta_{\rm det}(R)=\min\{t:L_t\cap\mathcal F_{V_R}\ne0\},$$
    where $\mathcal F_{V}$ is the space of functions supported on $V$. Then $\delta_{\rm det}=\infty$ iff $V_R=\emptyset$ (Lemma 6(ii)).
- **A3 (relations).**
  - Classical index calculus: the algorithm needs $h\ge c|\mathcal B|$ relations ($c\in(0,1]$; $c=1/2$ with the $\pm$ fold).
  - Theorem B needs only $h\ge1$.
  - Linear algebra costs $\ge0$; no $|\mathcal B|^2$ assumption is used.

**Outside the model:**
- F4/F5 and polynomial-calculus solvers (degree falls);
- explicit-table and MITM decompositions (ledger B1–B13, already floored at $\ge1.6\sqrt h\times$ rho);
- collision hybrids that stop at the first dependency (B3);
- lifting (A8);
- Weil descent over $\mathbb F_{p^k}$ (A1–A3).

---

## 2. Lemmas (all proved; Lean marks machine-checked statements)

**Lemma 1 (K1).**
- (U): $|\mathcal B_x|\le d$ and $|\mathcal B|\le 2d$.
- (Γ): $|\mathcal B_x|\le d_T$.
- In general $|\mathcal B|\le2|\mathcal B_x|\le 2|W|$.

*Proof.* A nonzero polynomial over a field has at most $\deg f$ roots. An abscissa has at most two ordinates, because $Y^2-c$ has degree 2. $\square$
**Lean:** `k1_root_bound`, `base_abscissas_le`, `fiber_card_le_two`, `points_card_le`, `k1_points`.

**Lemma 2 (counting).**
(a) $N_{\rm dec}\le\binom{|\mathcal B|+m-1}{m}\le|\mathcal B|^m$.
(b) $|V_R|\le 2^{m-1}\varphi\,|W|^{m-1}$ for every target.
(c) $\sum_R|V_R|\le 2^m|W|^m$, hence $\#\{R:|V_R|\ge L\}\le 2^m|W|^m/L$.
(d) There are at most $2^m\varphi|W|^{m-1}$ degenerate targets.

*Proof.*
- (a) A decomposable target is the sum of a multiset of size $m$ from $\mathcal B$.
- (b) Fix $w_1,\dots,w_{m-1}$. Fact 0 leaves at most $2^{m-1}$ values of $x_m$, each with at most $\varphi$ preimages.
- (c) For fixed $w$, $S_{m+1}(x(w),X)$ is nonzero in $X$ (Fact 0 in the last variable) with at most $2^{m-1}$ roots, and each root abscissa gives at most 2 targets.
- (d) A degenerate target has $|V_R|\ge|\pi_x(W)|\ge|W|/\varphi$; apply (c). $\square$

**Lean:** `sums_card_le` (the $|\mathcal B|^m$ form).

**Lemma 3 (surplus and yield invariance).**
- A complete-intersection presentation ($r=1+s$) gives $N+1$ equations in $N$ unknowns, a surplus of exactly 1 for every $s$.
- The set of $\mathbb F_p$-rational decompositions of $R$, and $N_{\rm dec}$, depend only on $\mathcal B$, not on the presentation.

*Consequence.* Auxiliary unknowns cannot change the yield, $\mathbb E_R[\#\text{decompositions}]\le|\mathcal B|^m/n$. They can change only the *cost of finding* decompositions. This answers the ledger's request for "an argument that auxiliary unknowns cannot raise the net surplus above 1". $\square$
**Lean:** `surplus_eq_one`.

**Lemma 4 (exact XL degree = interpolation degree).** Let $J$ be radical, let $f\in\{1,\ g_R,\ u\}$ (with $u$ an admissible block-1 polynomial vanishing on the block-1 projection of $V_R$), and let $D\ge\deg f$.
1. If $f\in\mathrm{Mac}^J_D$ (in particular if $f\in\mathrm{Mac}_D$), then $D\ge\Delta+\delta(f/S_R;V_R)$.
2. Conversely, if $D\ge\Delta+\delta(f/S_R;V_R)$, then $f\in\mathrm{Mac}^J_D$; if $\mathfrak B$ is an H-basis, then $f\in\mathrm{Mac}_D$.

Hence, exactly for XL$^J$, exactly for XL on H-bases, and as lower bounds for XL in general:
$$D_{\rm ref}=\Delta+\delta(1/S_R),\quad D_{\rm elim}=\max\big(\deg g_R,\ \Delta+\delta(g_R/S_R;V_R)\big),\quad D_{\rm weak}=\min_u\max\big(\deg u,\ \Delta+\delta(u/S_R;V_R)\big).$$

*Proof.*
- (1) Write $f=vS_R+j$ with $j\in J$ and $\deg(vS_R)\le D$. Restricted to $W^m$, $f=vS_R$, so $v=f/S_R$ off $V_R$. Since $\deg(vS_R)=\deg v+\Delta$ in a domain, $\delta(f/S_R;V_R)\le D-\Delta$.
- (2) Take $v$ of degree $\le D-\Delta$ agreeing with $f/S_R$ off $V_R$. Then $f-vS_R$ vanishes off $V_R$. It also vanishes on $V_R$, where $S_R=0$ and $f=0$ by the choice of $f$. So $f-vS_R\in I(W^m)=J$ (radical), with degree $\le D$. With an H-basis, $J_{\le D}\subseteq\mathrm{Mac}_D$.
- All linear systems involved are defined over $\mathbb F_p$, so degrees over $\bar{\mathbb F}_p$ and over $\mathbb F_p$ agree (Galois descent for linear systems). $\square$

**Lemma 5 (base-blindness).**
- (U): $D_{\rm ref},D_{\rm elim},D_{\rm weak}\ge d$.
- (Γ): $D_{\rm ref},D_{\rm weak}\ge d_T/(e+1)$.
- (T), (β): no such bound. Every generator has degree $\le2$, and this is exactly why auxiliary variables escape K1.

*Proof (U).* Every nonzero element of $J=(f(x_1),\dots,f(x_m))$ has degree $\ge d$, because the generators form a Gröbner basis with leading terms $x_i^d$. For $D<d$ this gives $\mathrm{Mac}^J_D=S_R\cdot k[\cdot]_{\le D-\Delta}$. Every nonzero element $vS_R$ has $\deg_{x_i}(vS_R)\ge\deg_{x_i}S_R\ge1$ for every $i$. ($S_R(\cdot,x_2,\dots)$ is nonzero by Fact 0 and has the root $x(R-P_2-\dots-P_m)$, hence is non-constant.) So it is neither a constant nor a block-1 polynomial. $\square$

*Proof (Γ).* Let $q\in J_1$ with $\deg q<d_T/(e+1)$, let $C=\{x\,\mathrm{den}(t)=\mathrm{num}(t)\}$ (irreducible, $\gcd(\mathrm{num},\mathrm{den})=1$), and assume $T$ avoids the poles of $\varphi$.
- With $a=\deg_xq$, $Q(t):=\mathrm{den}(t)^{a}\,q(\varphi(t),t)$ is a polynomial of degree $\le ea+\deg q\le(e+1)\deg q<d_T$ vanishing on $T$, so $Q\equiv0$.
- Hence $q$ vanishes on $C$, so $q\in(x\,\mathrm{den}-\mathrm{num})$.
- So for $D<d_T/(e+1)$, $\mathrm{Mac}^J_D$ lies in the ideal generated by $S_R$ and the $m$ graph equations.
- That ideal's variety is nonempty (no refutation).
- Its block-1 projection contains a cofinite subset of $C$. Given $t_1$, put $x_1=\varphi(t_1)$, choose generic $t_3,\dots,t_m$, take a root $x_2$ of the nonzero polynomial $S_R(x_1,\cdot,x_3,\dots)$ (Fact 0), and lift it through the surjective map $\varphi$. The set of $t_1$ admitting no completion is constructible and proper, hence finite.
- A block-1 polynomial in the ideal therefore vanishes on $C\supseteq W$, so it lies in $J_1$. $\square$

**Lean (U), literal-XL form:** `refutation_needs_degree`, `elimination_needs_degree`. If $1$, or a nonzero $g$ free of $x_j$, equals $u_0S+\sum u_if_i$ with every $\deg(u_if_i)\le D<d=\deg f_i$, then all $u_i=0$, so $S$ is constant, or free of $x_j$.

**Lemma 6 (rank).**
- (i) For $D\ge\Delta$: $\operatorname{rank}\mathrm{Mac}_D\ge\binom{N+D-\Delta}{N}$, because multiplication by $S_R\ne0$ is injective.
- (ii) $H(D-\Delta)\ge\dim(\mathrm{Mac}^J_D/J_{\le D})\ge H(D-\Delta)-|V_R|$. The image of $S_R\,k[\cdot]_{\le D-\Delta}$ in $k[W^m]$ is $S_R\cdot L_{D-\Delta}$, and the kernel of multiplication by $S_R$ consists of the functions supported on $V_R$. $\square$

**Lemma 7 (Hilbert function of presentations).**
- For (U), (T), (β), and for any complete-intersection H-basis whose top forms are a regular sequence with degrees $e_1..e_r$:
$$\textstyle\sum_t\big(H(t)-H(t-1)\big)z^t=\prod_l(1+z+\dots+z^{e_l-1})^m ,$$
so $|W|=\prod e_l$ and $\rho=m\sum(e_l-1)$:
  - (U): $(1+\dots+z^{d-1})^m$, $\rho=m(d-1)$;
  - (T), (β): $(1+z)^{mk}$, $\rho=mk$ (with $k=s$ for (β)).
- The coefficient sequence is palindromic, so $H(\lfloor\rho/2\rfloor)\ge|W|^m/2$.

*Proof.*
- For the four families, the standard monomials are $x^a$ with $a<d$, resp. the products of distinct chain/bit variables, of degree = digit sum.
- In general, the associated graded ring is the graded complete intersection of the top forms.
- A product of palindromic polynomials is palindromic. For a palindromic $c_j=c_{\rho-j}$: $\sum_{j>\rho/2}c_j\le\sum_{j\le\rho/2}c_j$. $\square$

**Lemma 8 (random functions; a heuristic yardstick, not used as an assumption).** If $h$ is uniform in $k[W^m]$, then
$$\Pr[\delta(h)\le t]=p^{H(t)-|W|^m},\qquad \Pr[\delta(h;V)\le t]\le p^{H(t)+|V|-|W|^m}.$$
*Proof.* $L_t$, resp. $L_t+\{\text{functions supported on }V\}$, is a subspace of dimension at most $H(t)$, resp. $H(t)+|V|$. $\square$

**Generic predictions** (dimension counts used to read the data):
- $\delta(1/S_R)=\rho$;
- $\delta(g_R/S_R;V_R)=\min\{t:\#\{a:\mathrm{wt}(a)>t\}\le|V_R|\}$;
- $\delta_{\rm weak}=\min\{t:\#\{a:\mathrm{wt}(a)>t\}<|\pi_xW|-|\pi_{x_1}V_R|+|V_R|\}$.

**Lemma 9 (Wald ratio and master inequality).**
- *Ratio bound.* Suppose, under A2–A3, each attempt has a type, hi or lo, fixed by the history before the attempt (the predictable decision of Theorems B and C), and:
  - type-hi attempts cost $\ge C_{\rm hi}$ in expectation and succeed with probability $\le\pi_{\rm hi}$;
  - type-lo attempts cost $\ge1$ and succeed with probability $\le\pi_{\rm lo}$.

  Then
  $$\mathbb E[\text{total}]\ \ge\ h\cdot\min\big(C_{\rm hi}/\pi_{\rm hi},\ 1/\pi_{\rm lo}\big).$$
  *Proof.* $M_k=\sum_{i\le k}(\text{cost}_i-\lambda\,\text{succ}_i)$ is a submartingale for $\lambda=\min(\cdot,\cdot)$, since each conditional increment is $\ge0$ whichever type was chosen. Apply optional stopping at $T\wedge k$ with monotone convergence; $T$ is the time of the $h$-th success. $\square$
- *Master inequality.* Suppose $h\ge c|\mathcal B|$, $\pi_{\rm hi}\le\min(1,|\mathcal B|^m/n)$, and $C_{\rm hi}\ge\kappa|\mathcal B|^{e}$ with $e\ge m/2-1$, $|\mathcal B|\ge1$, and $1/\pi_{\rm lo}\ge C_{\rm hi}/\pi_{\rm hi}$. Then
$$\mathbb E[\text{total}]\ge c\kappa\sqrt n .$$
  *Proof.* $\big(\max(B,\,n/B^{m-1})\big)^2B^{2e}\ge n$ for $B\ge1$, $2e+2\ge m$. Case $B^m\ge n$: $B^{2+2e}\ge B^m\ge n$. Case $B^m<n$: $(n/B^{m-1})^2B^{2e}\ge n\iff nB^{2e}\ge B^{2m-2}$, which holds since $B^{m+2e}\ge B^{2m-2}$. $\square$

  **Lean:** `master_sq` (rational $B,n$; natural $m$, $E_2=2e$), `amgm_core`; the median step of Lemma 7 is `palindromic_half`.

---

## 3. Theorems

**Theorem A ($m=2$; any base, any solver).** Under A2–A3 with attempt cost $\ge1$:
$$\mathbb E[\text{total}]\ \ge\ c\sqrt n .$$
*Proof.* $\pi\le N_{\rm dec}/n\le|\mathcal B|^2/n$ (Lemma 2a), so Lemma 9 gives $\mathbb E\ge c|\mathcal B|\max(1,n/|\mathcal B|^2)$. Then `master_sq` with $m=2$, $e=0$. $\square$

At arity 2 index calculus can at best match rho's exponent. This holds for any per-attempt decomposition procedure, however fast, so at the level of exponents it subsumes the ledger's arity-2 explicit-table floor.

**Theorem B (no auxiliary variables, or graph presentations; XL at any degree; unconditional in RAIC).** For (U) with $\deg f=d$, and every schedule, with literal XL:
$$\mathbb E[\text{total}]\ \ge\ h\cdot\max\!\Big(1,\frac{n}{N_{\rm dec}}\Big)\binom{m+d-\Delta}{m}\ \ \ge\ \ n\cdot\frac{\binom{m+d-\Delta}{m}}{\binom{2d+m-1}{m}}\ =\ \big(2^{-m}-o_d(1)\big)\,n .$$
For XL$^J$ the same holds with $\binom{m+d-\Delta}{m}-2^{m-1}d^{m-1}$ in place of the binomial (Lemmas 2b, 6(ii); on targets with $V_R=\emptyset$ no subtraction is needed). For (Γ), replace $d$ by $\lceil d_T/(e+1)\rceil$ in the binomial, and use $\binom{2d_T+m-1}{m}$ in the denominator. The limit is then $(2(e+1))^{-m}n$.

*Proof.*
- Below degree $d$ every target shows the same observations: $\mathrm{Mac}_D=S_R\cdot k[\cdot]_{\le D-\Delta}$ has rank $\binom{m+D-\Delta}{m}$, and there is no success or refutation (Lemma 5). So whether an attempt reaches degree $d$ does not depend on its target.
- Attempts that stop below $d$ never succeed ($\pi_{\rm lo}=0$).
- Every other attempt reaches degree $d$, and by Lemma 6(i) costs $\ge\binom{m+d-\Delta}{m}$.
- $N_{\rm dec}\le\binom{2d+m-1}{m}$ (Lemmas 1, 2a).
- Lemma 9 with $h\ge1$ gives the bound. $\square$

**Constants** (`out/thresholds.json`). These are the XL$^J$ form, with the $2^{m-1}d^{m-1}$ subtraction, at worst case $|\mathcal B|=2d$; the literal-XL constants are larger. $\mathbb E[\text{total}]/n\ge$

| $d$ | $m=2$ | $m=3$ | $m=4$ |
|---|---|---|---|
| $2^8$ | 0.2408 | 0.0986 | vacuous |
| $2^{12}$ | 0.2494 | 0.1233 | 0.0577 |
| $2^{16}$ | 0.2500 | 0.1249 | 0.0622 |
| $2^{20}$ | 0.2500 | 0.1250 | 0.0625 |

The $m=4$, $d=2^8$ cell is vacuous with the XL$^J$ subtraction; with literal XL, or on targets with $V_R=\emptyset$, no subtraction is needed. Exponent: **1**, against rho's 1/2, for every $m$ and $d$.

**Theorem C (auxiliary variables; XL$^J$; conditional).** Let $\mathfrak B$ be an H-basis complete intersection, and fix a threshold $\theta$. Write $H:=H(\theta-\Delta)$.
- Call $R$ **$\theta$-good** if $D_{\rm weak}(R)\ge\theta$, $D_{\rm det}(R)\ge\theta$ and $|V_R|\le H/2$.
- $\varepsilon_1(\theta):=\Pr_R[R\ \text{not }\theta\text{-good}]$.
- $\nu(\theta):=\#\{R\ \text{decomposable, not }\theta\text{-good},\ D_{\rm weak}(R)<\infty\}$.

  Degenerate targets have $D_{\rm weak}=\infty$: every univariate consequence lies in $J_1$, so arity-$m$ XL elimination never solves them. They include every target with a lower-arity decomposition $R=Q_1+\dots+Q_{m-2}$, padded by $P+(-P)$ for every $P$. On the toy grids every degenerate target had this pattern (§5.4, item 4). Harvesting them is lower-arity index calculus, which the same theorems bound at that arity.

By Lemma 2c, the $|V_R|$ part of $\varepsilon_1$ is at most $2^{m+1}|W|^m/(nH)$. Then for every schedule
$$\mathbb E[\text{total}]\ \ge\ h\cdot\min\Big(\big(1-\varepsilon_1\big)\tfrac H2\cdot\max\!\big(1,\tfrac{n}{N_{\rm dec}}\big),\ \ \frac{n}{\nu(\theta)}\Big).$$
*Proof.*
- Every $\theta$-good target shows the same observations below degree $\theta$: ranks $H(D-\Delta)$ and no success. So at each attempt the decision "continue to degree $\theta$" is the same for all good targets. Call it $\alpha_i\in\{0,1\}$; it is fixed by the history.
- Apply Lemma 9 with type hi $\Leftrightarrow\alpha_i=1$:
  - if $\alpha_i=0$, the attempt can succeed only on decomposable non-good targets that XL can solve at all, so $\pi_{\rm lo}\le\nu/n$;
  - if $\alpha_i=1$, $\pi_{\rm hi}\le\min(1,N_{\rm dec}/n)$, and each good target costs $\ge H-|V_R|\ge H/2$ (Lemmas 4, 6(ii)), so $C_{\rm hi}\ge(1-\varepsilon_1)H/2$. $\square$

**Hypothesis $\mathrm G(\theta)$** (the only unproved input):
- (i) $\varepsilon_1(\theta)\le1/2$;
- (ii) $\nu(\theta)\le\frac4H\min(N_{\rm dec},n)$.

That is, decomposable targets that are not $\theta$-good (cheaply solvable, or cheaply detectable) are rarer, by a factor $H/4$, than decomposable targets.

**Proven partial cases of $\mathrm G$.**
- (U), (Γ): below the witness degree the ranks are target-independent; this is Theorem B.
- (β): a nonzero multilinear polynomial of degree $t$ in $N$ Boolean variables has $\ge2^{N-t}$ nonzeros (classical Reed–Muller distance), so $\delta_{\rm det}(R)\ge ms-\log_2|V_R|$.
- (T): no proven floor on $\delta_{\rm det}$. The conjectured analogue, $\delta_{\rm det}\ge mk-\log_2|V_R|$, held on all 370 tower targets of §5.4 item 4, with minimum margin 0.

**Corollary C1 (sufficient condition for $\ge\sqrt n$).** Under $\mathrm G(\theta)$ and $h\ge c|\mathcal B|$: if
$$H(\theta-\Delta)\ \ge\ 4\kappa\,|\mathcal B|^{m/2-1},$$
then $\mathbb E[\text{total}]\ \ge\ c|\mathcal B|\cdot\kappa|\mathcal B|^{m/2-1}\max(1,n/|\mathcal B|^m)\ \ge\ c\kappa\sqrt n$.
*Proof.* Both terms of the minimum are $\ge\frac H4\max(1,n/N_{\rm dec})$; then use $N_{\rm dec}\le|\mathcal B|^m$ and `master_sq` with $e=m/2-1$. $\square$

**Corollary C2 (binary towers and Boolean bits).**
- Here $H(t)=\sum_{j\le t}\binom{mk}{j}=2^{mk\,h_2(t/mk)-O(\log k)}$ for $t<mk/2$, and $|\mathcal B|\le2^{k+1}$.
- The size condition of C1 holds, for large $k$, as soon as $\theta-\Delta\ge\tau\,mk$ with $h_2(\tau)>\tfrac12-\tfrac1m$, that is $\tau>\tau_m$.

| $m$ | 3 | 4 | 5 | 6 | 8 | 10 | 20 | $\infty$ |
|---|---|---|---|---|---|---|---|---|
| $\tau_m$ | 0.0246 | 0.0417 | 0.0532 | 0.0615 | 0.0725 | 0.0794 | 0.0941 | 0.1097 |

At finite $k$, the least $t/(mk)$ with $H(t)\ge2^{k(m/2-1)}$ is 0.029 ($m=3$, $k=128$), 0.045 ($m=4$), 0.064 ($m=6$) and 0.074 ($m=8$) (`out/thresholds.json`).

Read contrapositively: an XL-type tower or bit attack can reach rho's exponent only if one of two things happens. Either a constant fraction of targets has weak XL degree below $\Delta+\tau_m mk$, which is **3–11 % of the way from $\Delta$ to the Macaulay maximum $\Delta+mk$**; or low-degree decomposable targets are common, which violates $\mathrm G$(ii).

**Corollary C3 (random-function model).** If the Semaev functions behaved like random functions (Lemma 8), $\mathrm G(\theta)$ would hold with $\theta-\Delta=\rho-o(\rho)$ and $H=(1-o(1))|W|^m$. The per-attempt cost would then be about brute force over $W^m$, and $\mathbb E[\text{total}]=\Omega(n)$: exponent 1. The test in §5.4 shows that this model is right for the strong degree but only approximately right for the weak degree.

---

## 4. What is proved and what is assumed

| item | status |
|---|---|
| Lemmas 1–9, Theorems A, B, the inequality of Theorem C, Corollaries C1–C2 as implications | proved (desk); Lean for the parts of Lemmas 1, 2a, 3, 5(U), 7 (median step), 9 listed in §6 |
| Semaev's characterisation and degree $2^{m-1}$ per variable | literature (Semaev 2004), checked numerically, not re-proved |
| A1 cost = rank of the XL space; A2 uniform independent targets, stopping driven only by ranks and success flags; A3 $h\ge c|\mathcal B|$ (Thm A, C) or $h\ge1$ (Thm B) | model axioms |
| H-basis property for (U), (T), (β) | proved (coprime leading terms) |
| $\mathrm G(\theta)$: (i) at most half the targets are not $\theta$-good (weak or detection degree $<\theta$, or more than $H/2$ grid zeros); (ii) non-good decomposable targets number $\le\frac4H\min(N_{\rm dec},n)$ | **unproved** for towers; for bits the detection part is proved; toy evidence only (§5.4); (ii) is beyond what sampling can check |
| first-fall-degree or semi-regularity heuristics | **not used anywhere**. The XL degree is computed exactly (Lemma 4). The first-fall degree is an F4/PC notion, and §5.5 shows F4 sits far below the XL degree. |

---

## 5. Numerical checks (every lemma, on toy instances)

Commands (from `code/`):
```
python3 check_L4_xl_degree.py {uni2,kummer2,uni3,tower2,graph2,bits2} 1
python3 check_L4_weak.py {uni2,tower2,uni3}
python3 check_weak_d32.py
python3 check_counting.py > ../out/counting.json
python3 check_G_hypothesis.py {uni,tower,bits} m size 786433 ntar ndec 5 1   (grids: run_G_all.sh, run_G2_all.sh, run_G2_rest.sh, run_G_bits.sh, run_G_bits2.sh)
python3 check_det.py {uni,tower,bits} m size 786433 ndec 1                      (grid: run_det2.sh)
python3 msolve_tower.py k seed ;  python3 msolve_tower_m.py k seed 3
python3 thresholds.py ; python3 summarize_L4.py ../out ; python3 summarize_G.py ../out
```

### 5.1 Semaev polynomials
| $m$ | vanish on true sums | nonzero at random tuples | $\Delta$ | degree per variable |
|---|---|---|---|---|
| 2 ($S_3$) | 200/200 | 200/200 | 4 | 2 |
| 3 ($S_4$, by resultant) | 200/200 | 200/200 | 12 | 4 |
| 4 ($S_5$, 131,005 generic terms) | 100/100 | 100/100 | 32 | 8 |

Toy prime $p=12289$.

### 5.1b Lemmas 1, 2, 7, 9 by exhaustion (`code/check_counting.py`, `out/counting.json`)

| check | instances | result |
|---|---|---|
| Lemma 1 (K1): $|Z(f)|\le\deg f$, $|\mathcal B|\le2|\mathcal B_x|\le2\deg f$ | 200 random $f$ over $\mathbb F_{12289}$ (split, random, Kummer; $\deg\le48$) | all hold; max $|\mathcal B|/(2\deg f)=1.0$ |
| Lemma 2a $N_{\rm dec}\le\binom{|\mathcal B|+m-1}{m}$ | 10 bases (uni $d=4,6,8$; tower $k=2,3$; $m=2,3$), prime-order curves over $\mathbb F_{1153}$, all targets enumerated | all hold (e.g. 92 vs 120; 51 vs 55) |
| Lemma 2b $\max|V_R|\le2^{m-1}\varphi|W|^{m-1}$ | same, every target abscissa | all hold (max 22 vs 256) |
| Lemma 2c $\sum_R|V_R|\le2^m|W|^m$ | same | all hold (max 520 vs 4,096) |
| Lemma 2d degenerate targets $\le2^m\varphi|W|^{m-1}$ | same | all hold (max 8 vs 512) |
| Lemma 7: $H(t)$ by evaluation rank equals the series, palindromic, $H(\lfloor\rho/2\rfloor)\ge|W|^m/2$ | uni $d=5$ ($m=2$), $d=4$ ($m=3$); tower $k=3$ ($m=2$), $k=2$ ($m=3$); bits $s=3$ ($m=2$); every $t$ | all equal and palindromic; median holds |
| Lemma 9 (Wald): mean attempts for $h=50$ relations $=hn/N_{\rm dec}$ | $n=1213$, $N_{\rm dec}=737$, 400 runs | 82.58 measured vs 82.29 predicted (s.e. 0.37) |

### 5.2 Lemma 4 (exact XL degree), Lemma 5 (base-blindness), Lemma 6 (rank), Lemma 1, Lemma 7
- **Method.** The literal Macaulay matrix was built over $\mathbb F_{12289}$. For each target the check confirms $f\notin\mathrm{Mac}_{\rm pred-1}$ and $f\in\mathrm{Mac}_{\rm pred}$, where pred $=\max(\deg f,\Delta+\delta)$ and $\delta$ comes from an independent evaluation-matrix interpolation.
- **Results** (`out/L4_summary.json`):

| family ($m$) | targets | $D_{\rm XL}=$ pred exactly | Lemma 5 $D\ge d$ | Lemma 6 (i) and (ii) | $D$ range |
|---|---|---|---|---|---|
| uni ($m=2$, $d=2..8$) | 42 | 42/42 | 42/42 | 42/42 | 2–18 |
| kummer ($m=2$, $d=4,8,16$) | 15 | 15/15 | 15/15 | 15/15 | 9–34 |
| uni ($m=3$, $d=2..4$) | 12 | 12/12 | 12/12 | 12/12 | 2–21 |
| tower ($m=2$, $k=1..3$, $S$ in $x$ and reduced) | 23 | 23/23 | n/a | 23/23 | 2–8 |
| bits ($m=2$, $s=1,2$; the $s=3$ run was killed for memory) | 10 | 10/10 | n/a | 10/10 | 2–8 |
| graph $x=t^2+t$ ($m=2$, $d_T=2..5$; not an H-basis) | 15 | pred $\le D_{\rm XL}$: 15/15 (strict by 1–3) | $D\ge d_T$: 15/15 | 15/15 | 6–11 |

- Non-decomposable targets give $D_{\rm ref}=\Delta+\rho$, the Macaulay bound: $2d+2$ for uni $m=2$, $3d+9$ for $m=3$, $\Delta+2k$ for towers. This reproduces the rootfam lab's measured $D_{\rm ref}$.
- **Degenerate targets** ($\delta=-\infty$, $g_R\in J$) occur only on tiny bases. They were found by this check: the first run's formula predicted $\Delta$ instead of $\deg g_R$ for one $d=2$ target, which forced the $\delta(0)=-\infty$ convention in Lemma 4.

### 5.3 Weak degree (Lemma 4, weak form)
Three independent computations of $D_{\rm weak}$ were compared: the literal Macaulay intersection with block-1 polynomials, evaluation-matrix ranks, and the fast coefficient-domain routine.
- uni2: 25/25 targets agree in all three.
- tower2: 7/7 agree.
- uni3 ($d=3,4,5$): 9/9 non-degenerate targets agree in all three. The 3 degenerate targets are reported as "no weak success up to the cap" by all three.
- At $d=16$ and $d=32$ (uni, $m=2$) the literal Macaulay matrix confirms the fast value: 4/4. Weak success occurs at $D=29$ and $D=57$ and not one degree earlier.

### 5.4 Hypothesis test for $\mathrm G$ (preset `out/PRESET_G.txt`, sha256 `5e95fe30…`; amendments 1–3 written before the respective data)
$p=786433$, one curve per instance.
- Targets per instance: 300/200/100 non-decomposable and 40/30/20 decomposable, for $m=2/3/4$.
- Families: uni and tower are preset; bits is exploratory (amendment 2). The bits $m=2$, $s=10$ instance (grid $2^{20}$) was dropped as too slow before any of its output was read.
- Source: `out/G_summary.json`, `out/G_table.md`.

| family | $m$ | size | grid | $\rho$ | non-dec: $\delta=\rho$ | dec (degenerate) | non-degenerate dec: $\delta\ge$ generic | min $\delta_{\rm weak}/\rho$ | min (weak − generic) | min $H(\delta_{\rm weak})/|W|^m$ | controls rand / planted |
|---|---|---|---|---|---|---|---|---|---|---|---|
| bits | 2 | 4 | 256 | 8 | 300/300 | 40 (0) | 40/40 | 0.75 | 0 | 0.957 | 5/5 / 3/3 |
| bits | 2 | 6 | 4096 | 12 | 300/300 | 40 (0) | 40/40 | 0.833 | 0 | 0.9963 | 5/5 / 3/3 |
| bits | 2 | 8 | 65536 | 16 | 300/300 | 40 (0) | 40/40 | 0.812 | 0 | 0.9978 | 5/5 / 3/3 |
| bits | 3 | 3 | 512 | 9 | 200/200 | 30 (14) | 16/16 | 0.778 | -1 | 0.9688 | 5/5 / 3/3 |
| bits | 3 | 4 | 4096 | 12 | 200/200 | 30 (8) | 22/22 | 0.833 | 0 | 0.9954 | 5/5 / 3/3 |
| bits | 3 | 5 | 32768 | 15 | 200/200 | 30 (8) | 22/22 | 0.867 | 0 | 0.9993 | 5/5 / 3/3 |
| bits | 4 | 2 | 256 | 8 | 100/100 | 20 (12) | 8/8 | 0.875 | 0 | 0.9492 | 5/5 / 3/3 |
| bits | 4 | 3 | 4096 | 12 | 100/100 | 20 (9) | 11/11 | 0.833 | -1 | 0.991 | 5/5 / 3/3 |
| bits | 4 | 4 | 65536 | 16 | 100/100 | 20 (6) | 14/14 | 0.875 | -1 | 0.9994 | 5/5 / 3/3 |
| tower | 2 | 2 | 16 | 4 | 300/300 | 40 (0) | 40/40 | 0.75 | 0 | 0.8125 | 5/5 / 3/3 |
| tower | 2 | 3 | 64 | 6 | 300/300 | 40 (0) | 40/40 | 0.667 | 0 | 0.8594 | 5/5 / 3/3 |
| tower | 2 | 4 | 256 | 8 | 300/300 | 40 (0) | 40/40 | 0.75 | 0 | 0.957 | 5/5 / 3/3 |
| tower | 2 | 5 | 1024 | 10 | 300/300 | 40 (0) | 40/40 | 0.8 | 0 | 0.9873 | 5/5 / 3/3 |
| tower | 2 | 6 | 4096 | 12 | 300/300 | 40 (0) | 40/40 | 0.833 | 0 | 0.9963 | 5/5 / 3/3 |
| tower | 2 | 7 | 16384 | 14 | 300/300 | 40 (0) | 40/40 | 0.786 | 0 | 0.9934 | 5/5 / 3/3 |
| tower | 2 | 8 | 65536 | 16 | 300/300 | 40 (0) | 40/40 | 0.812 | 0 | 0.9978 | 5/5 / 3/3 |
| tower | 3 | 2 | 64 | 6 | 200/200 | 30 (22) | 8/8 | 0.833 | 0 | 0.8906 | 5/5 / 3/3 |
| tower | 3 | 3 | 512 | 9 | 200/200 | 30 (16) | 14/14 | 0.778 | -1 | 0.9688 | 5/5 / 3/3 |
| tower | 3 | 4 | 4096 | 12 | 200/200 | 30 (14) | 16/16 | 0.833 | 0 | 0.9954 | 5/5 / 3/3 |
| tower | 3 | 5 | 32768 | 15 | 200/200 | 30 (2) | 28/28 | 0.867 | 0 | 0.9993 | 5/5 / 3/3 |
| tower | 4 | 2 | 256 | 8 | 100/100 | 20 (16) | 4/4 | 0.875 | 0 | 0.9727 | 5/5 / 3/3 |
| tower | 4 | 3 | 4096 | 12 | 100/100 | 20 (10) | 10/10 | 0.833 | -1 | 0.991 | 5/5 / 3/3 |
| tower | 4 | 4 | 65536 | 16 | 100/100 | 20 (12) | 8/8 | 0.875 | -1 | 0.9994 | 5/5 / 3/3 |
| uni | 2 | 8 | 64 | 14 | 300/300 | 40 (0) | 40/40 | 0.786 | 0 | 0.875 | 5/5 / 3/3 |
| uni | 2 | 16 | 256 | 30 | 300/300 | 40 (0) | 40/40 | 0.833 | 0 | 0.9336 | 5/5 / 3/3 |
| uni | 2 | 32 | 1024 | 62 | 300/300 | 40 (0) | 40/40 | 0.855 | -2 | 0.9541 | 5/5 / 3/3 |
| uni | 2 | 64 | 4096 | 126 | 300/300 | 40 (0) | 40/40 | 0.865 | -7 | 0.9622 | 5/5 / 3/3 |
| uni | 3 | 4 | 64 | 9 | 200/200 | 30 (4) | 26/26 | 0.778 | -1 | 0.8906 | 5/5 / 3/3 |
| uni | 3 | 8 | 512 | 21 | 200/200 | 30 (5) | 25/25 | 0.857 | -1 | 0.9688 | 5/5 / 3/3 |
| uni | 3 | 16 | 4096 | 45 | 200/200 | 30 (2) | 28/28 | 0.911 | -1 | 0.9937 | 5/5 / 3/3 |
| uni | 4 | 3 | 81 | 8 | 100/100 | 20 (14) | 6/6 | 0.875 | 0 | 0.9136 | 5/5 / 3/3 |
| uni | 4 | 4 | 256 | 12 | 100/100 | 20 (10) | 10/10 | 0.833 | -1 | 0.9492 | 5/5 / 3/3 |
| uni | 4 | 6 | 1296 | 20 | 100/100 | 20 (8) | 12/12 | 0.9 | 0 | 0.9776 | 5/5 / 3/3 |

**Totals.**
- 33 instances.
- Non-decomposable targets: 7,100, every one with $\delta=\rho$.
- Decomposable targets: 1,040, of which 192 are degenerate. The other 848/848 have $\delta\ge$ generic.
- Pass 2 reproduces pass 1's strong $\delta$ on all 5,350 shared rows (0 mismatches).

**Readings against the frozen rule.**
1. **Strong $\delta$, CONSISTENT.**
   - Every non-decomposable target in every family and arity had $\delta=\rho$ exactly. This is the random-function prediction (Lemma 8).
   - Every non-degenerate decomposable target had $\delta\ge$ its generic prediction.
   - Negative controls (random functions): $\delta=\rho$ in all of them. Positive controls (planted degree $t_0$): $\delta=t_0$ in all of them.
2. **Weak $\delta$, mixed.**
   - **Towers and Boolean bits: CONSISTENT** (deviation $\ge-1$ in every instance).
   - **Univariate grids: REFUTED as a random-function prediction** at $m=2$, $d\ge32$. Every target sits $2$ ($d=32$) and $7$ ($d=64$) degrees below the random prediction (literal-XL confirmed at $d=32$).
   - Semaev functions therefore have structure that lets XL learn a univariate constraint somewhat early. The gap $\rho-\delta_{\rm weak}$ was 3, 5, 9, 17 for $d=8,16,32,64$, roughly $d/4$, against $\approx\sqrt{2d}$ for random functions.
   - This does **not** affect Theorem B, which is unconditional.
   - It shows that $\mathrm G$ cannot be taken from the random-function model wholesale. Even so, $\delta_{\rm weak}/\rho\ge0.667$ in every instance ($\ge0.75$ on grids of $\ge256$ points), far above $\tau_m\le0.11$.
   - The rank $H(\delta_{\rm weak})$ stayed $\ge0.81$ of $|W|^m$ ($\ge0.93$ on grids of $\ge256$ points), which is brute-force size.
3. **Limit of the evidence.**
   - Sampling cannot verify clause (ii) of $\mathrm G$, which asks for a rate $\le4/H$; $H$ is $10^2$–$10^5$ here and samples are 20–40 decomposable targets per instance.
   - Rare low-degree decomposable targets that a cheap, early-stopping algorithm could harvest are not excluded by the data.

4. **Detection degree $\delta_{\rm det}$ (amendment 3, `out/PRESET_G_amend3.txt`, written before the data; fresh decomposable targets, `out/G_det_summary.json`).**
   - **Sample.** 860 decomposable targets in 31 family/arity/size cells: the main grid without tower $m=4$, $k=4$ and bits $m=4$, $s=4$. 184 were degenerate and 676 non-degenerate.
   - **Frozen rule: REFUTED** as a random-function prediction. Some targets sit $\ge3$ degrees below the generic value.
   - **Where the deviations sit.**
     - Most are in **degenerate** targets: down to $-10$ (uni $m=3$, $d=16$).
     - These are the lower-arity targets. Every degenerate target in the main test had $|V_R|\ge|W|$, and $|V_R|=3|W|-2$ at $m=3$: the pattern $\{(x,x,c)\}$ and its permutations, from $R=Q_c$ padded by $P+(-P)$.
     - Non-degenerate decomposable targets had $|V_R|\le m!$.
     - A function supported on that pattern has degree $\approx\frac{m-1}{m}\rho$: $[x_1{=}x_2]\,f'(x_1)$ has degree $d-1$ on a univariate grid. Hence $\delta_{\rm det}/\rho=2/3$ at $m=3$.
   - **Non-degenerate targets.**
     - Deviation counts: $0$ for 494, $-1$ for 175, $-2$ for 1, $-3$ for 6. The $-3$ targets are all uni $m=4$, $d=6$; the $-2$ target is bits $m=4$, $s=3$.
     - Minimum $\delta_{\rm det}/\rho$ was $0.667$.
     - Towers: deviation $\ge-1$ in every instance.
   - **Bits.** The proved Reed–Muller floor $\delta_{\rm det}\ge ms-\log_2|V_R|$ held on every bits target.
   - **Consequence for Theorem C.** Degenerate targets are detectable early but not solvable by arity-$m$ XL elimination, so they do not enter $\nu$. Filtering them by detection and solving them at lower arity is lower-arity index calculus, which the theorems cover at that arity. By rank alone (an estimate, not a proof) it is also costlier than running the lower-arity solver directly: about $|W|^{m-1}$ per attempt against $|W|^{m-2}$.

### 5.5 XL versus F4 (outside the model; why Theorem C does not close PKM)
- **System.** The same tower decomposition system was given to msolve 0.9 (F4, grevlex, one thread). The exact XL refutation degree comes from Lemma 4.
- **Table** (`out/msolve_tower*.jsonl`):

| $m$ | $k$ | $N$ | XL degree (Lemma 4) | XL columns $\binom{N+D}{N}$ | F4 max pair degree | F4 largest matrix | time |
|---|---|---|---|---|---|---|---|
| 2 | 2 | 4 | 8 | 495 | 4 | 44×37 | 0.01 s |
| 2 | 3 | 6 | 10 | 8,008 | 4 | 147×126 | 0.01 s |
| 2 | 4 | 8 | 12 | 125,970 | 4 | 357×278 | 0.01 s |
| 2 | 5 | 10 | 14 | 1,961,256 | 4 | 892×600 | 0.03 s |
| 2 | 6 | 12 | 16 | 30,421,755 | 5 | 2,803×2,064 | 0.11 s |
| 2 | 7 | 14 | 18 | 471,435,600 | 5 | 6,354×4,333 | 0.9 s |
| 2 | 8 | 16 | 20 | 7,307,872,110 | 5 | 11,340×8,385 | 11.8 s |
| 2 | 9 | 18 | 22 | 113,380,261,800 | 5 | 23,644×14,284 | 76.8 s |
| 3 | 2 | 6 | 18 | 134,596 | 12 | 306×369 | 0.03 s |
| 3 | 3 | 9 | 21 | 14,307,150 | 12 | 1,795×2,065 | 0.04 s |

- **F4 refutes at essentially the input degree** ($\Delta$ or $\Delta+1$). Its degree falls make the XL degree irrelevant: the polynomial-calculus degree here is $\approx\Delta$, while the Nullstellensatz/XL degree is $\Delta+\rho$.
- **F4 matrix widths still grow**: 37, 126, 278, 600, 2,064, 4,333, 8,385, 14,284 columns for $k=2..9$, a factor of 1.7–3.4 per level. That is consistent both with growth linear in $|W|=2^k$ and with the polylogarithmic bound $\binom{2k+5}{5}$ on monomials of degree $\le5$; the ladder is too short to separate them.
- If F4's degree stayed bounded at larger $m$ and its matrices stayed polylogarithmic, the per-attempt cost would be $|\mathcal B|^{o(1)}$, and index calculus with $m\ge3$ would beat rho. This is the PKM hope, and the XL model can neither confirm nor exclude it.
- The repo's F4 runs on PKM towers record a slowly growing degree at $m=3$ (`research/pkm_symmetric_tower_20261026`: $D=4,5,5,6$ at $N=6..15$; repo numbers, not re-run).
- **So:** the lower bound of Theorem C, and hypothesis $\mathrm G$, say nothing about F4. Whether F4's per-attempt cost on PKM presentations grows at least like $|\mathcal B|^{m/2-1}$ is exactly the open ledger item. It is being measured by the sibling `finish/pkm-msolve` run; no numbers from it are used here.

---

## 6. Lean formalisation

`lean/ICCounting/ICCounting.lean` (sha256 `b9f7c49e3f0a897b1db6f6ecbb226ae3201dc8b93f63df66492f209653787178`). Checked with Lean 4.34.1 against a Mathlib checkout at tag `v4.34.1`.
- **Toolchain.** No Lean toolchain was present, so the release tarball was fetched into the scratchpad. The Mathlib cache was fetched for the imported modules only.
- **Command.**
  ```
  cd lean/mathlib4 && lake env lean ../ICCounting/ICCounting.lean
  ```
- **Output** (`lean/ICCounting/check.log`): exit 0, no `sorry`. Every theorem depends only on `propext`, `Classical.choice`, `Quot.sound`; `surplus_eq_one` uses only `propext`, `Quot.sound`.

| Lean theorem | statement | model lemma |
|---|---|---|
| `k1_root_bound` | nonzero $f$ vanishing on a finset $Z$ ⇒ $|Z|\le\mathrm{natDegree}\,f$ | Lemma 1 (K1) |
| `base_abscissas_le` | $|\{x: f(x)=0\wedge\text{onCurve}(x)\}|\le\deg f$ | Lemma 1 |
| `fiber_card_le_two` | $|\{y:y^2=c\}|\le2$ | Lemma 1 |
| `points_card_le`, `k1_points` | points over $\mathcal B_x$: $\le2|\mathcal B_x|\le2\deg f$ | Lemma 1 |
| `sums_card_le` | $|\{\sum_i b_i: b\in\mathcal B^m\}|\le|\mathcal B|^m$ | Lemma 2a |
| `surplus_eq_one` | $(m(1+s)+1)-m(1+s)=1$ | Lemma 3 |
| `refutation_needs_degree` | Macaulay certificate of $1$ with all base multiples of degree $<d$ ⇒ $\deg S=0$ | Lemma 5 (U) |
| `elimination_needs_degree` | certificate of a nonzero $g$ free of $x_j$, base multiples of degree $<d$ ⇒ $S$ free of $x_j$ | Lemma 5 (U) |
| `amgm_core` | $(n/b+b)^2\ge4n$ | Theorem A |
| `master_sq` | $B\ge1,\ m\le E_2+2\Rightarrow n\le\max(B,n/B^{m-1})^2B^{E_2}$ | Lemma 9 |
| `palindromic_half` | $c_j=c_{R-j}\Rightarrow\sum_{j\le R}c_j\le2\sum_{j\le R/2}c_j$ | Lemma 7 (median step) |

**Not formalised:**
- Lemma 4 (the interpolation characterisation);
- Lemma 6, Lemma 7 apart from its median step, Lemma 8;
- Wald's identity;
- the cost axioms;
- Semaev's theorem.

---

## 7. Verdict and gaps

**Verdict.**
- **Proved.** In the restricted XL model, prime-field index calculus cannot beat rho's exponent at arity 2 (Theorem A, any base). At any arity it cannot do better than exponent 1 with a factor base cut out without auxiliary variables or as a rational-map image (Theorem B, unconditional). This closes ledger rows A5, A6-APS and the parametrised A4 cases inside the model.
- **Conditional.** For auxiliary-variable presentations (A4 $\mu_{2^k}$ towers, PKM chains, A6 Boolean bits), the same conclusion holds whenever hypothesis $\mathrm G(\theta)$ holds at a degree $\theta-\Delta\ge\tau_m\rho$, a few percent of the Macaulay maximum (Corollary C2: $\tau_m\le0.110$).
  - In every sampled instance the weak and detection degrees of solvable targets satisfied this with a large margin: $\delta_{\rm weak}/\rho\ge0.667$ and $\delta_{\rm det}/\rho\ge0.667$.
  - Under the random-function model the exponent would be 1.
- **Does not settle PKM.** The theorem does not settle the PKM question. F4 refutes the same tower systems at degree $\approx\Delta$ (§5.5), so the XL model is not the binding model there.

**Gaps.**
1. **F4/PC is outside the model.** A lower bound for F4 needs polynomial-calculus degree *and size* bounds, for example a size–degree trade-off in the style of Impagliazzo–Pudlák–Sgall. None is known for Semaev systems. Measured F4 degrees on towers are bounded by $\Delta+1$ at these sizes.
2. **$\mathrm G(\theta)$ is unproved.**
   - It is a non-vanishing statement for high-weight coefficients of $1/S_R$ (resp. $u/S_R$) in the standard-monomial basis of $W^m$: character-sum-type quantities for towers, Walsh–Möbius coefficients for bits.
   - The random-function model fails quantitatively for the weak degree on univariate grids, and for the detection degree of lower-arity (degenerate) targets and a few $m=4$ targets (§5.4). So $\mathrm G$ needs a Semaev-specific proof, not a heuristic.
   - For bits the detection part of $\mathrm G$ is proved (Reed–Muller distance). For towers the analogous floor $\delta_{\rm det}\ge mk-\log_2|V_R|$ is open.
   - Clause (ii), on rare low-degree decomposable targets, cannot be checked by sampling.
3. **Model axioms.**
   - A1 charges XL for the full rank of its space. Sparse or structured elimination that never materialises the space is not covered.
   - A3 ($h\ge c|\mathcal B|$) excludes first-dependency and collision variants; the ledger's B3 floor covers those separately.
   - A2 assumes uniform targets, and lets stopping react only to ranks and success flags. An attempt that inspected the computed spaces in other ways, for instance by evaluating $S_R$ on the grid, would be doing brute-force search, which is outside XL.
4. **Scope.** The model excludes lifting (A8), Weil descent over $\mathbb F_{p^k}$ (A1–A3), explicit-table oracles (B-family), and non-Semaev relation search.
5. **Toy scale.**
   - Grids up to $65{,}536$ points.
   - $m\le4$ for $\mathrm G$; $m\le3$ for the literal-XL checks.
   - $m\ge6$ (the ledger's PKM question) is not reached.
   - $p\in\{12289,786433\}$.
   - Lemma 4 is proved for all sizes; only $\mathrm G$ depends on scale.
6. **Unconditional ECDLP hardness** against all algorithms is not claimed and remains open.
