# Polynomially decaying clique sums: what the target is, what it would imply, what it could reach

- **Date:** 2026-11-22
- **Goal:** pinned in the workspace trunk's `docs/ACTIVE.md` ("Find a clique-sum algorithm whose
  exponent per edge bit decays polynomially in the clique size"); items G1–G6. Tracking PR
  isogeny-labs/crypto#4904 (first drafted in isogeny-labs/workspace#60).
- **Context:** `REVIEW.md` in this directory §5c–§5f (threshold
  model, Lemma Q, crypto distances). Sources: Anthropic, *A 2^{o(n)} time algorithm for 3-SAT*
  (Theorem A, Lemmas 1 and 8, Appendix B); Majdoddin, arXiv:2610.41622 (§3, §4.1, Theorem 4.13,
  §4.7–4.8).
- **Standing:** mathematics and a model computation. No algorithm with polynomial decay is
  claimed. The results are a precise statement of the target, unconditional and conditional
  limits on it, and a correction to §5f of the review note.
- **Correction (2026-11-25):** §0 item 3, §2.2 and §6 first gave the reach of polynomial decay as
  $2^{\tilde O(n^{1-\beta/2})}$ ($2^{\tilde O(\sqrt n)}$ at $\beta=1$). That figure omits Theorem A's
  $(\lambda+B)^{C_0K^2}$ overhead. Under it the reach is $2^{\tilde O(n^{2/(2+\beta)})}$
  ($2^{\tilde O(n^{2/3})}$ at $\beta=1$); $\sqrt n$ needs an overhead of $2^{o(K^2)}$. Found
  independently by two agents of the workspace P vs NP fleet
  (`workspace:docs/notes/p-vs-np-20261124/04-eps0-calculation.md`); see §2.2.

---

## 0. Summary

1. **The target is a constant-growth level.** Polynomial decay $e(K)\approx C K^{-\beta}$ is the
   same as a level that turns a $(cK)$-node clique sum into $K$-node clique sums with the
   exponent multiplied by any fixed $\rho<1$, with $c$ constant (§1). The Anthropic recursion
   has $\rho=1/\sqrt2$ but growth $K\mapsto K^2$; Majdoddin proves grids cannot do better. So the
   missing object is constant growth, not a better $\rho$.
2. **$\beta\le1$ and $C\ge1$ unconditionally** (§2.1): the tables must be read. The
   "hypothetical $\beta=1$, $C=1$" column of §5f is therefore the absolute optimum of the whole
   approach, not a moderate hypothesis.
3. **$\beta=1$ would give bounded-occurrence #3-SAT in $2^{\tilde O(n^{2/3})}$** (§2.2), far beyond
   the ETH refutation itself ($2^{n/\sqrt{\log n}}$). Generally $e\asymp K^{-\beta}$ gives
   $2^{\tilde O(n^{2/(2+\beta)})}$ under Theorem A's $(\lambda+B)^{C_0K^2}$ overhead. Only with an
   overhead of $2^{o(K^2)}$ would $\beta=1$ reach $2^{\tilde O(\sqrt n)}$, every sparse formula as
   easy as a planar one. *(Corrected 2026-11-25; the first version claimed $\sqrt n$.)*
4. **Even the optimum reaches a target only if its network has fewer than $G^2$ edge bits**,
   where $G$ is the attack budget in bits per counting call (§3). ECDLP-256 and Ring-LWE-1024 fail
   this for information-theoretic reasons (their witnesses alone are too large); AES-128 is
   borderline at best. **This corrects §5f of the review note**, which said better encodings
   could plausibly close a $2^8$–$2^{16}$ gap: the required size grows like $U^2$, so shrinking the
   encoding cannot close it.
5. **Pairwise pieces cannot give constant growth** (§4): in a level whose merges have constant
   group size $c$, a two-string configuration that saves under two merges covers only an
   $O(c^2/K)$ fraction of the edges; in Cayley-type designs, configurations whose strings have
   their endpoints joined through chains of $r$ shared groups cover all edges only if
   $r\ge\log_c K$. This makes quantitative Majdoddin's statement that polynomial decay needs pieces
   of growing arity spanning three or more groups.
6. **Literature** (§5): nothing known gives $N^{o(K^2)}$ for this object other than the Anthropic
   recursion, and nothing rules out polynomial decay beyond the rank-one, contraction-tree and
   grid classes.

---

## 1. G1. The target, exactly

Write $T(K,\lambda)=2^{e(K)K^2\lambda+o(K^2\lambda)}$ for the cost of a $(K,\lambda)$-clique sum
(Majdoddin's normalisation: brute force is $e=1/2$, and $K^2\lambda$ counts every shared bit at both
ends).

**Level lemma $\mathrm{L}(c,\rho,\kappa)$.** For every $K$ and $\lambda$, a $(cK,\lambda)$-clique sum is
computed from $(K,\lambda')$-clique sums with total cost exponent at most
$\rho\,e(K)\,(cK)^2\lambda+\kappa\,cK\lambda$.

**Claim.** $\mathrm{L}(c,\rho,\kappa)$ with constants $c\ge2$, $\rho<1$ gives
$e(K)=O(K^{-\beta})$ with $\beta=\min\{1,\log(1/\rho)/\log c\}$.

*Proof.* $e(cK)\le\rho\,e(K)+\kappa/(cK)$. Unrolling from $K_0$:
$e(c^jK_0)\le\rho^je_0+\frac{\kappa}{K_0}\sum_{i<j}\rho^{i}c^{-(j-i)}$. If $\rho c>1$ the first term
dominates and $e(K)\asymp(K/K_0)^{-\log(1/\rho)/\log c}$; if $\rho c\le1$ the second does and
$e(K)=\Theta(1/K)$ (up to a $\log K$ factor at equality). $\square$

So **any** $\rho<1$ with **constant** growth suffices. The Anthropic level has
$K_{\ell+1}=(s_\ell K_\ell)^2$: the growth is squaring, and that, not $\rho=1/\sqrt2$, is why the
decay is only $(\log K)^{-1/2}$. Majdoddin's Theorem 4.13 shows every grid level has
$e(K)\ge1/\log_2K$; his §4.8 says escaping needs configurations meeting three or more groups of a
merge.

**Thresholds as a function of $(C,\beta)$.** With bar $\bar e$ and top-level exponent $q\ge2$,
$\log_2 m^\ast=(q/\beta)\log_2(C/\bar e)$ up to lower-order terms (Majdoddin, Prop. 4.1(b)).

---

## 2. G2. What such an algorithm implies

### 2.1 Unconditional: $\beta\le1$ and $C\ge1$

Any algorithm with query access to the tables must, in the worst case, read every entry of every
table, since each entry can change the value when the other tables are nonzero. There are $K$
tables of $2^{(K-1)\lambda}$ entries, so $T(K,\lambda)\ge K\,2^{(K-1)\lambda}$ and

$$e(K)\ \ge\ \frac{K-1}{K^2}\ \approx\ \frac1K .$$

Hence $\beta\le1$, and $\beta=1$ forces $C\ge1$. The recursion's merged tables are given exactly
this way (as procedures), so the bound applies to every algorithm in the framework.

### 2.2 $e\asymp K^{-\beta}$ gives bounded-occurrence #3-SAT in $2^{\tilde O(n^{2/(2+\beta)})}$

Let $F$ be a 3-CNF on $n$ variables in which every variable occurs in at most $\Delta$ clauses.
Split the variables into $K$ groups of $n/K$ at random and assign each clause to the group of
its first variable. Node $a$ needs the values of the variables of group $b$ that occur in
clauses assigned to $a$. A variable of $b$ occurs in at most $\Delta$ clauses, so it is needed at
group $a$ with probability at most $2\Delta/K$, and
$\mathbb E\lambda_{ab}\le(n/K)(2\Delta/K)=2\Delta n/K^2$. Each variable affects at most $2\Delta$
pairs, so a Chernoff bound with bounded dependence gives
$\max_{a,b}\lambda_{ab}\le\lambda:=O(\Delta n/K^2+\log n)$ with positive probability (and a
deterministic choice by conditional expectations). Put on edge $\{a,b\}$ the values each side needs
from the other ($2\lambda$ bits), and let $W_a$ sum over the $2^{n/K}$ assignments of group $a$ the
product of [the outgoing strings agree with the assignment] and [the clauses assigned to $a$ hold,
read with the incoming strings]. The clique sum then equals $\#F$. Tables have
$2^{2(K-1)\lambda}$ entries, each computed in $2^{n/K}\mathrm{poly}(n)$ time.

With $e(K)\le CK^{-\beta}$ the exponent part of the clique sum is $2^{O(CK^{2-\beta}\lambda)}$, but
Theorem A also charges $(\lambda+B)^{C_0K^2}=2^{O(K^2\log n)}$, which at $K=\sqrt n$ alone is
$2^{\Theta(n\log n)}$. So $K$ must be chosen to balance the two. With
$K^2\lambda=O(\Delta n+K^2\log n)$ the cost is

$$\log_2T=\min_K\Bigl[O\bigl(Cn\,K^{-\beta}\bigr)+O\bigl(C_0K^2\log n\bigr)+n/K\Bigr],$$

the last term being the $2^{n/K}$ table build. The minimum is at $K^{2+\beta}\asymp n/\log n$, giving

$$T=2^{\tilde O(n^{2/(2+\beta)})},\qquad \beta=1:\ 2^{\tilde O(n^{2/3})}.$$

(Numerically, minimising this expression over $K$ fits exponents $0.676$ at $\beta=1$ and $0.806$
at $\beta=1/2$, against $2/3$ and $0.8$; `workspace:docs/notes/p-vs-np-20261124/eps0_calc.py`
gives $0.6748$ and $0.8049$ for the same model on $S$ wire bits.) Only if the overhead were
$2^{o(K^2)}$, for example polynomial in $K$, could $K$ go to $\sqrt n$ and give
$2^{\tilde O(n^{1-\beta/2})}$, i.e. $2^{\tilde O(\sqrt n)}$ at $\beta=1$: dropping the overhead from
the same minimisation fits $0.53$, falling toward $1/2$. *(The first version of this section took
$K=\sqrt n$ with the overhead omitted and stated $2^{\tilde O(\sqrt n)}$ unconditionally; corrected
2026-11-25.)*

For comparison, planar #3-SAT is $2^{O(\sqrt n)}$ by separators, and the claimed ETH refutations
give $2^{O(n\log\log n/\sqrt{\log n})}$; the reduction with $e(K)\asymp(\log K)^{-1/2}$ reproduces
the latter, which checks it. Even $2^{\tilde O(n^{2/3})}$ is a far stronger statement than the
refutation, and no standard hypothesis other than ETH excludes it.

The information bound of §3 is not affected: its cost $2^{\max(U,K^2)/K}$ is a lower bound on the
best case, and Theorem A's overhead can only raise it.

### 2.3 The $k$-clique reduction gives at most $e(K)\gtrsim\omega/(3K)$, and that hypothesis is now claimed refuted

Counting $K$-cliques in an $n$-vertex graph reduces to a $(K,\lceil\log_2n\rceil)$-clique sum:
node $a$ holds a private vertex $v_a$; edge $\{a,b\}$ with $a<b$ carries $v_a$; $W_b$ checks that
all its outgoing strings equal one value $v$ and multiplies the adjacency bits $A[x_{ab},v]$ of its
incoming strings ($W_K$ sums over $v$). The sum is $K!$ times the number of $K$-cliques, and the
tables cost $K\,n^{K-1}\mathrm{poly}$ to build. The $k$-clique conjecture (exponent
$\omega k/3$) would give $e(K)K^2\ge\omega K/3-o(K)$, i.e. $e(K)\ge\omega/(3K)$, which is weaker
than §2.1 anyway. Vassilevska Williams and Zamir (arXiv:2611.09764, 6 Nov 2026, unrefereed) claim
$n^{(\omega/3-c)k}$ for counting weighted $k$-cliques with $c>0$ independent of $k$, which would
lower this bound by a constant factor. Either way the input floor of §2.1 is the binding limit.

### 2.4 Rank-one methods are stuck at $e\ge1/4$

Evaluating the clique sum from a rank decomposition of the complete-graph tensor costs at least
its rank, and the flattening across a balanced cut of the nodes has rank
$N^{\lfloor K/2\rfloor\lceil K/2\rceil}$. So rank-based methods have $e\ge1/4$; only the merging
(partial contraction) steps of Signed Network Sums can go lower, as Majdoddin notes at $K=4$.

---

## 3. G3. Which targets the optimum could reach at all

Group a target network with $U$ edge bits (both ends) into $K$ groups. Every pair of groups gets
$L\ge1$ bits, so the clique has $K^2L\ge\max(U,K^2)$ bits, and by §2.1 the cost is at least
$2^{\max(U,K^2)/K}\ge2^{\sqrt U}$, with equality only for perfectly balanced pair loads and an
algorithm exactly at the input floor. A route that beats a conventional attack of $2^G$ per
counting call therefore needs

$$U<G^2 .$$

(The threshold model's $q\approx2$ and its overheads make this $U\lesssim G^2/4$; §5f's
"polynomial decay" column is this model.)

| Target | $G$ per call | minimal $U$ (both ends) | reachable by the optimum? |
|---|---|---|---|
| ECDLP-256, index calculus with $m$-term decompositions | $256(1/2-1/m)$, at most 85 for $m=6$ | $\ge4mb\cdot\kappa$: the witness alone is $m$ points of $2b$ bits, times an overhead $\kappa>1$ for slopes and products; $m=6$: $\ge6144\kappa$ | needs $6144\kappa<85^2=7225$, i.e. $\kappa<1.18$: **no**, since each addition must carry at least its slope and verify products |
| Ring-LWE $n=1024$, binary secret | 128 | the modular product $a\cdot s$ must be represented: $\ge2nr\ge2\cdot1024\cdot3$ even at noise width 2, $5.7\times10^5$ for the paper's network | needs $<16384$: **no** |
| AES-128, one known pair | 128 | about $9\times10^3$ (about $4.5\times10^3$ edge bits: per round 128 S-box inputs, 128 outputs, 128 round-key bits, plus the key schedule) | **borderline**: $2^{\sqrt U}\approx2^{95}$ only with perfectly balanced pair loads, which a layered cipher network does not have; with the model's overheads, no |

For ECDLP the general form is: with $U\approx4\kappa mb$ the condition $\sqrt U<b(1/2-1/m)$ becomes
$b>4\kappa m/(1/2-1/m)^2$, minimised near $m=6$ at $b>216\kappa$. So even the optimum breaks
prime-field ECDLP only for fields well beyond 256 bits (about 2000 bits at $\kappa=10$).
Asymptotically the optimum would give $2^{\tilde O(b^{2/3})}$ (balance $b/m$ against $\sqrt{mb}$),
a strong subexponential algorithm whose crossover lies past cryptographic sizes.

**Correction to the review note, §5f.** The "polynomial decay" thresholds 42, 33, 28 against
actual sizes 26, 22.5, 20 are right as computed, but the gap is not one that encodings could close:
the requirement is $\log_2U\le2\log_2(G/C)-2$, so shrinking $U$ lowers both sides. The right reading
is the table above.

---

## 4. G5. Why pairwise pieces cannot give constant growth

**Setting.** A level on $K'$ nodes uses merges $\mathcal P_1,\dots,\mathcal P_p$, partitions into groups
of constant size $c$ (so each merged network has $K'/c$ nodes; constant growth). A piece identity
rewrites a configuration of strings; it saves under merge $j$ only if strings of the configuration
share bundles (pairs of groups) of $\mathcal P_j$, since a bundle is where bits are compressed.

**Lemma R (rectangle coverage).** Let $\mathcal P_1,\mathcal P_2$ have constant group size $c$. The
crossing identity on two strings $(u,v),(u',v')$ saves under both merges only if
$u'\in G_1(u)\cap G_2(v)$ and $v'\in G_1(v)\cap G_2(u)$. If the merges are orthogonal (two nodes
share a group in at most one merge) each edge has at most one partner, and an edge has one only if
$G_1(u)$ meets $G_2(v)$, which happens for at most $c\cdot c$ of the $K'$ choices of $v$. So at most
an $O(c^2/K')$ fraction of the edges lie in a saving configuration; the rest are carried at full
cost, and the level gains nothing as $K'\to\infty$. Every edge is covered only if every group of one
merge meets every group of the other, i.e. $c\ge\sqrt{K'}$: the grid, and the squaring.

**Lemma R′ (Cayley designs, chained configurations).** Let the nodes be an abelian group $A$ of
order $K'$ and let merge $j$ be the cosets of a subgroup $H_j$ of order $c$. In the crossing
identity the two endpoints $u,v$ of a string are joined through the configuration's other nodes by
a chain of shared groups ($u\sim_1u'\sim_2v$), which forces $v-u\in H_1+H_2$. More generally, if
every string of a configuration has its endpoints joined by a chain of at most $r$ shared groups
from merges $j_1,\dots,j_r$, the configuration covers only edges whose difference lies in
$H_{j_1}+\dots+H_{j_r}$, a set of at most $c^r$ elements, and covering every difference needs
$r\ge\log_cK'$: **chained configurations of logarithmically many strings**. For configurations
whose strings' endpoints are not joined by such chains I have no coverage bound; that is the open
region, and it is where any constant-growth level would have to live.

So a constant-growth level needs either chained pieces of growing arity, which in the merged
network are multi-party factors over $\Omega(\log K)$ merged nodes, or unchained configurations
for which no saving mechanism is known. The clique-sum model is pairwise, so such a
piece must itself be written as a tensor network on the merged nodes, and its cost is a
tensor-network rank. That is the precise form of Majdoddin's "open door": find an exact identity,
over configurations of $\Theta(\log K)$ strings, whose pieces have small tensor-network rank on the
merged nodes under at least one of the merges, uniformly enough to beat the adversarial mixtures.

---

## 5. G4. Literature (sweep of 2026-11-22; abstracts or full texts as noted)

- **Christandl, Vrana, Zuiddam**, *Asymptotic tensor rank of graph tensors: beyond matrix
  multiplication*, Comput. Complexity 28 (2019), arXiv:1609.07476 (full text). For the
  complete-graph tensor $T(K_k)$: asymptotic-rank exponent per edge $\tau\le\log_7(9/2)\approx0.7729$
  for $k\ge4$ (Cor. 1.2.6), $\tau\le\omega/3$ for all $k\ge3$, $\tau$ nonincreasing in $k$; lower bound
  by max-cut flattening $\ge N^{\lfloor k^2/4\rfloor}$. Rank-one methods are therefore
  $N^{\Theta(K^2)}$ in both directions. Open Q1.3.1: is $\tau<2/3$ for some $k\ge5$.
- **Majdoddin**, arXiv:2610.41622 (full text), Prop. 2.4: input floor $N^{K-1}$; rank-one methods
  $\ge N^{\lfloor K^2/4\rfloor}$; contraction trees $\ge N^{\lceil K/3\rceil(K-\lceil K/3\rceil)}$ (via
  Austrin–Kaski–Kubjas branchwidth bounds). "No lower bound above the input floor is known for
  Signed Network Sums at any fixed $K$." At $K=4$: $N^{23/6}$ if $\omega=2$, about $N^{3.93}$ on the
  preprint curve, below the $N^4$ of trees and rank. Grid class: $e(K)\ge1/\log_2K$; two balanced
  homogeneous merges: $e(K)\ge(\log_2K)^{-3}$. Open problem 5 is the hyperedge identity.
- **Austrin, Kaski, Kubjas**, *Tensor network complexity of multilinear maps*, ITCS 2019 / ToC
  2022, arXiv:1712.09630 (abstract): $\Omega(n^{\mathrm{bw}(P)})$ in the single-contraction-tree model.
- **Abo Khamis, Hu, Suciu**, *Fast matrix multiplication meets the submodular width*, PODS 2025,
  arXiv:2412.06189 (partial): Boolean queries on binary relations; nothing for dense sum-product on
  the line graph of $K_K$, where FAQ/InsideOut widths are $\Theta(K^2)$.
- **Vassilevska Williams, Zamir**, arXiv:2611.09764 (abstract and introduction): $k$-clique and
  weighted $k$-clique counting in $n^{(\omega/3-c)k}$; a constant saving per vertex, not decay.
- **Fischer**, *The hyperclique hypothesis is false*, arXiv:2611.03880: $h$-uniform hyperclique in
  $O(n^{(1-\varepsilon_h)k})$, explicitly by Anthropic-style signed sums with per-term evaluation
  orders; constant savings. It also cites Brand–Kaski–Wang's "two-slice rank", which captures mixed
  evaluation orders beyond tensor networks, as an attempted lower-bound tool, and shows a proposed
  $\Omega(n^4)$ two-slice-rank bound is false.
- Tensor-network-rank and partial-degeneration papers (Christandl et al., arXiv:2212.14095,
  2307.07394; Christandl–Lysikov–Zuiddam 2012.14412): abstracts only; no exponents for $K_k$.

**Reading.** Nothing known gives polynomial decay, and nothing rules it out for Signed Network Sums
or unrestricted algorithms; all lower bounds above the input floor are model-restricted. The newest
work (Vassilevska Williams–Zamir, Fischer) uses the same signed-sum mechanism on related objects
and gets constant savings, consistent with the obstacle being constant growth.

---

## 6. Verdict and next discriminating step

- The target is well defined (a constant-growth level), its optimum is pinned ($\beta=1$, $C\ge1$),
  and its strength is calibrated: at $\beta=1$ it would make bounded-occurrence #3-SAT
  $2^{\tilde O(n^{2/3})}$ under Theorem A's overhead, and $2^{\tilde O(\sqrt n)}$ only with an
  overhead of $2^{o(K^2)}$ (corrected 2026-11-25).
- **Even the optimum does not reach ECDLP-256 or Ring-LWE-1024**, for information-theoretic reasons
  that no algorithm in the clique-sum framework can avoid. Finding polynomial decay would be a major
  complexity result, but it is not a route to those targets. AES-scale symmetric targets are the only
  cryptographic sizes it could conceivably touch, and only with near-ideal balance.
- The next discriminating step for the mathematics itself is in the open region of §4: the smallest
  **unchained** configurations in a Cayley design ($A=\mathbb Z_2^t$, merges by cosets of coordinate
  subgroups of order 2), asking whether any exact identity on them has pieces whose cost, summed over
  bundles and minimised over merges, beats the uncompressed count under every adversarial mixture.
  This needs a cost model for multi-group pieces first (Majdoddin's model does not define one); a
  natural candidate is the tensor-network rank of the piece on the merged nodes. Defining that model
  precisely is the prerequisite, and it is recorded as an open follow-up rather than run here.
- Because even the optimum cannot reach ECDLP-256 or Ring-LWE-1024 (§3), this line has no bearing on
  the lab's cryptanalytic targets; its value is complexity-theoretic.
