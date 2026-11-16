# L06 derivation: rational Lagrangians of J[l] from a split h*h^* of chi mod l

Coordinator (rerun), 2026-11-16. Standing of everything here: `[pcc:unverified]`.
This file checks, line by line, the two notes left by the first coordinator. They
were treated as unverified claims. Verdicts: note (1) is correct, and it can be made
sharper (section 2). Note (2) is correct (section 4), and the lab's own code had
already reached the same picture (section 5).

## 0. Setting and notation

- `p` an odd prime, `H: y^2 = f(x)` with `f` squarefree of degree 7 over `F_p`. `J`
  is its Jacobian, dimension 3, principally polarised.
- `pi` is the `p`-power Frobenius endomorphism of `J`. `chi(T)` is its characteristic
  polynomial, degree 6:
  `chi(T) = T^6 - s1 T^5 + s2 T^4 - s3 T^3 + p s2 T^2 - p^2 s1 T + p^3`.
  `#J(F_p) = chi(1)`, and the quadratic twist has order `chi(-1)`.
- `l` a prime with `l != p`. `V = J[l]` is a 6-dimensional `F_l`-space. The
  characteristic polynomial of `pi` on `V` is `chi mod l` (Tate module).
- `e: V x V -> mu_l` is the Weil pairing of the principal polarisation. It is
  bilinear, alternating, nondegenerate, and `e(pi x, pi y) = e(x, y)^p`.
- For monic `h` of degree `d` with `h(0)` a unit mod `l`, define
  `h^*(T) = T^d h(p/T) / h(0)` (monic, degree `d`). An irreducible `g` with
  `g^* = g` is called self-reciprocal. `chi` itself satisfies
  `T^6 chi(p/T) = p^3 chi(T)`, so `chi^* = chi`.
- "Criterion fires at `l`": there is a monic cubic `h` over `F_l` with
  `chi = h h^* (mod l)` and `gcd(h, h^*) = 1`.

## 1. Note (1), checked: ker h(pi) is a rational Lagrangian

Step 1: `h(0)` is a unit. `h^*` has constant term `p^3 h_3 / h(0) = p^3 / h(0)` (h monic),
so `h(0) h^*(0) = p^3 = chi(0)`, a unit mod `l` because `l != p`. So `h(0)` is a unit,
`h^*` is well defined, and `pi` is invertible on `V`.

Step 2: the adjoint of `pi`. `e(pi x, y) = e(pi x, pi pi^{-1} y) = e(x, pi^{-1} y)^p
= e(x, p pi^{-1} y)`. So for any polynomial `g`, `e(g(pi) x, y) = e(x, g(p pi^{-1}) y)`.

Step 3: `V = V_h (+) V_{h^*}` with `V_h = ker h(pi)`. By Cayley-Hamilton `chi(pi) = 0`
on `V`, and `h, h^*` are coprime, so `V` is the direct sum of `ker h(pi)` and
`ker h^*(pi)`. The characteristic polynomial of `pi` on `V_h` has only irreducible
factors of `h`, the one on `V_{h^*}` only those of `h^*`, their product is `h h^*`, and
`h, h^*` are coprime. So the characteristic polynomial on `V_h` is exactly `h`, and
`dim V_h = 3`. (This holds without `h` being squarefree.)

Step 4: isotropy. From the definition, `h^*(p/T) = (p/T)^3 h(T) / h(0)`, i.e.
`h^*(p pi^{-1}) = p^3 h(0)^{-1} pi^{-3} h(pi)`. For `x, y in V_h`:
`e(h^*(pi) x, y) = e(x, h^*(p pi^{-1}) y) = e(x, unit * pi^{-3} h(pi) y) = e(x, 0) = 1`.
`h^*(pi)` is invertible on `V_h` (its characteristic polynomial there is `h`, coprime to
`h^*`), so `h^*(pi) V_h = V_h` and `e(V_h, V_h) = 1`.

Step 5: `V_h` is `pi`-stable (`pi` commutes with `h(pi)`), isotropic, and of dimension
3 = half of 6. So it is a maximal isotropic subgroup of `J[l]` stable under Frobenius,
that is, an `F_p`-rational Lagrangian subgroup scheme, and `J -> J / V_h` is an
`F_p`-rational `(l,l,l)`-isogeny onto a principally polarised abelian threefold.

Nothing in steps 1-5 uses `l` odd. The argument holds for `l = 2` too (`e_2` is
alternating), which matters for section 4.

Compatibility with prime order: the criterion is a statement about `chi mod l`, and
`#J = chi(1)` prime only says `chi(1) != 0 mod l`, i.e. 1 is not a root of `chi mod l`.
Nothing forbids the criterion then. Note that `J[l](F_p) = 0` in that case, so the
points of `V_h` live only over an extension `F_{p^k}`, `k > 1` (section 6).

## 2. Sharpening (checked here, not in the notes): a polynomial test, exact when chi mod l is squarefree

Factor `chi mod l` into monic irreducibles. Because `chi^* = chi`, the operation `*`
permutes its irreducible factors, preserving multiplicities. Factors come as
self-reciprocal `g = g^*`, or as pairs `{g, g^*}` with `g != g^*` and equal multiplicity.

Claim A (equivalence of tests). The criterion fires iff `chi mod l` has no
self-reciprocal irreducible factor.
- If `q = q^*` divides `chi = h h^*`, then `q | h` or `q | h^*`. Applying `*` (which is
  multiplicative up to units and fixes `q`), `q` divides both. Then `h, h^*` are not
  coprime. So a firing criterion excludes self-reciprocal factors.
- Conversely, if every factor lies in a pair `{g, g^*}` with multiplicity `a`, put
  `h = prod over pairs of g^a` (one representative per pair). Then `h^* = prod (g^*)^a`,
  `h h^* = chi`, they are coprime, and `deg h = 6/2 = 3` automatically.
- Self-reciprocal factors of odd degree: only linear `T - c` with `c^2 = p mod l`.
  A self-reciprocal irreducible cubic is impossible: the involution `r -> p/r` on its 3
  roots has a fixed root with `r^2 = p`, so `r` lies in `F_{l^2}`, contradicting degree 3.
- Number of valid `h`: `2^(number of pairs)`. Distinct `h` give distinct kernels
  (the characteristic polynomial of `pi` on `V_h` is `h`). So the curve has at least
  `2^(pairs)` rational Lagrangians of this kind.

Claim B (exactness for squarefree `chi mod l`). If `chi mod l` is squarefree and has a
self-reciprocal factor, there is no Frobenius-stable Lagrangian at all.
- Squarefree characteristic polynomial: `pi` is semisimple and each primary component
  `V_g = ker g(pi)` is an irreducible `F_l[pi]`-module of dimension `deg g`. Every
  `pi`-stable subspace is a direct sum of some `V_g`.
- Orthogonality: for `x in V_g`, `y in V_{g'}`, Step 2 with `g'(p/T) = g'(0) T^{-d'}
  g'^*(T)` gives `1 = e(x, g'(pi) y) = e(unit * pi^{-d'} g'^*(pi) x, y)`. If
  `g'^* != g`, then `g'^*(pi)` is invertible on `V_g`, so `V_g` is orthogonal to `V_{g'}`.
- So a self-reciprocal `V_q` is orthogonal to every other component. It is not
  isotropic: otherwise it would lie in the radical of `e`, which is zero. A stable
  isotropic `L = sum of V_g over a set S` can contain no self-reciprocal `V_q`, and at
  most one member of each pair. Its dimension is at most
  `(6 - total degree of self-reciprocal factors) / 2 < 3`.

So the census can label every curve and every `l` in one of three exact classes:
`fire` (rational Lagrangian exists), `none` (`chi mod l` squarefree, does not fire: no
rational Lagrangian exists), `undetermined` (not squarefree, does not fire: the
criterion is silent; other stable Lagrangians can exist, e.g. inside `V_q` with `q`
self-reciprocal of multiplicity 2). The note's "sufficient, not necessary" is right,
and Claim B shows exactly where necessity fails.

Gap flagged: Claim B uses only linear algebra. I did not prove anything about the
non-squarefree class; the census reports its size and makes no statement about it.

## 3. Reference density: what "random Frobenius" means for odd l

For fixed odd `l` and the family of genus-3 hyperelliptic curves, the mod-`l` image of
monodromy is believed to be all of `GSp_6(F_l)` (big monodromy for hyperelliptic
families, odd `l`: J.-K. Yu; Achter-Pries) `[pcc:unverified]`, cited from memory and not
checked in research-kb (section 7). Then, as `p` grows with `l` fixed, Frobenius classes
equidistribute in the coset of multiplier `p mod l`, and the expected firing rate is
the share of elements of that coset whose characteristic polynomial has no
self-reciprocal factor. Conditioning on `#J` prime is conditioning on `chi(1) != 0 mod l`
(and on events at other primes, heuristically independent). The census computes both
reference numbers by uniform sampling of the coset (SPEC.md section 7). It is an
estimate by sampling, not an enumeration. With `p <= 4099` and `|GSp_6(F_7)|` about
`7^22`, equidistribution is not guaranteed at these sizes (a confounder, PROTOCOL.md).

Hand check of the sampler logic in dimension 2 (genus 1, `GL_2` with `det = m`), used as
a gate: the criterion (deg h = 1) fires iff the eigenvalues are `a != m/a`, both in
`F_l`. Classes with distinct rational eigenvalues have size `l(l+1)`, and there are
`(l - 1 - s)/2` unordered pairs, `s = #{a : a^2 = m}`. The coset has `l(l^2 - 1)`
elements. So the rate is `(l - 1 - s) / (2(l - 1))`. Examples: `l = 3, m = 1`: 0;
`l = 3, m = 2`: 1/2; `l = 5, m = 1`: 1/4; `l = 5, m = 2`: 1/2; `l = 7, m = 1`: 1/3;
`l = 7, m = 3`: 1/2.

## 4. Note (2), checked: the l = 2 arm

Claim: if `#J` is odd then `f` is irreducible and `chi mod 2 = Phi_7 = (T^3+T+1)(T^3+T^2+1)`.

- `J[2]` for odd degree: let `Omega` be the 7 roots of `f` plus the point at infinity.
  `J[2]` is the space of even-size subsets of `Omega` modulo `{empty, Omega}`, with
  pairing `|A cap B| mod 2`, and Frobenius acts through its permutation `sigma` of
  `Omega` (fixing infinity).
- `J[2](F_p)` has dimension `k - 1`, `k` = number of irreducible factors of `f`. The lab
  checked `v_2(#J) >= k - 1` on 7,701 curves (`research/smith_near_prime_20261114`,
  `[pcc:unverified]`). So `#J` odd iff `k = 1` iff `f` irreducible, `sigma` = a 7-cycle
  plus the fixed point at infinity.
- As an `F_2[sigma]`-module, the permutation module is `F_2[x]/(x^7 - 1) (+) F_2`.
  Passing to even subsets removes one trivial summand and passing to the quotient by
  `Omega` removes the other. Over `F_2`, `x^7 - 1 = (x + 1)(x^3 + x + 1)(x^3 + x^2 + 1)`.
  So `J[2] = F_2[x]/(Phi_7)` and `chi mod 2 = Phi_7`, as claimed. In general
  `chi mod 2 = prod over cycles c of sigma of (T^len(c) - 1) / (T - 1)^2` (gate G5).
- With `p` odd, `h = T^3 + T + 1` has `h^* = T^3 h(1/T) / h(0) = T^3 + T^2 + 1`. The two
  are coprime and swapped. The criterion fires, and by section 1 (valid for `l = 2`)
  `ker h(pi)` and `ker h^*(pi)` are two distinct rational Lagrangians of `J[2]`.

Which kind are they? Of the 135 maximal isotropic subgroups of `J[2]` in genus 3, 105 are
of pair type (from the `8!/(2^4 4!) = 105` partitions of `Omega` into four pairs; the
kernels of Smith's construction), and 30 are not. The 30 correspond to the affine
`F_2^3` structures on the 8 points (`8!/|AGL_3(F_2)| = 40320/1344 = 30`): the 14 affine
planes, with `empty` and `Omega`, form `RM(1,3)`, the [8,4] extended Hamming code, whose
image mod `Omega` is a 3-dimensional isotropic subgroup. A 7-cycle fixing infinity
preserves no pair partition: the partner of infinity would have to be fixed. So the
kernels found are not of pair type. They are the extended cyclic Hamming codes
generated by `x^3 + x + 1` and `x^3 + x^2 + 1`, which are affine type. (Counts 135/105/30
are checked as gate G6. The Hamming-code identification follows from the module
computation above and is not separately gated.)

Consequence, stated carefully: every genus-3 hyperelliptic Jacobian (odd-degree model)
with odd `#J`, prime order included, has at least two `F_p`-rational
`(2,2,2)`-kernels. Smith's closure statement remains true **for pair type**, which is
what Smith's trigonal construction computes. What is not known to this coordinator:
an explicit, efficient construction for the affine-type `(2,2,2)` quotient, and whether
its codomain is generically a plane-quartic Jacobian, a hyperelliptic Jacobian, or
decomposable. I recall (unverified, not checked in research-kb) that in the
Recillas/Donagi trigonal picture the non-pair Lagrangians are related to the
hyperelliptic locus differently from the pair ones. That has to be looked up before
anyone writes "the affine-type quotient is a plane quartic". The census measures only
existence. The main session's proposed control ("l = 2 should show no usable kernel
when #J is prime") is wrong as stated. The corrected arm tests the criterion against
the pair-type test and the explicit stable-Lagrangian count (PROTOCOL.md arm A2).

## 5. Prior art in this repository (10-minute grep, done)

- `src/cryptanalysis/smith_kernel_coverage.rs` already enumerates the 135 Lagrangians of
  `J[2]`, separates pair type from "the other 30", and checks `chi mod 2 =
  charpoly(Frobenius | J[2])`. It states the necessary condition `chi mod 2 = g g^*`. So
  the `l = 2` picture of section 4 is consistent with existing lab code. This lead adds
  the sufficiency direction for coprime `g, g^*` (section 1) and the odd-`l` census.
- `research/smith_near_prime_20261114`: `k = 1` (f irreducible) never passes Smith's
  conditions (0 of 1,110). `research/smith_even_census_20261115`: the same closure for
  the even-degree model. Both are about pair type at `l = 2`. Neither looks at odd `l`.
- `grep` found no odd-`l` Lagrangian census in `research/` or `src/`. That is a grep of
  this checkout, not a literature or "not tried" claim.

## 6. What computing the codomain would need (not implemented)

1. Kernel generators. `V_h` is defined over `F_{p^k}` with `k` = order of `pi` on `V_h`.
   That order divides the order of `T` in `(F_l[T]/(h))^*`, with equality when `h` is
   squarefree. The census records this `k` (SPEC.md `ext_deg`). For prime-order `J`,
   `k > 1` always. Points: draw `R in J(F_{p^k})` (order `chi_k(1)` from `chi`), multiply
   by the `l`-free part of `#J(F_{p^k})`, then project to `V_h` with a polynomial in `pi`
   that is the identity on `V_h` and zero on `V_{h^*}` (CRT idempotent mod `h h^*`), and
   reduce to `l`-torsion. Cost: Cantor arithmetic over `F_{p^k}` with `k` up to
   `l^3 - 1` (342 at `l = 7`).
2. The isogeny. There is no Velu-type formula in Mumford coordinates for
   `(l,l,l)`-isogenies in genus 3. The general tool known to me is isogeny computation
   in theta coordinates (Lubicz-Robert; Cosset-Robert in genus 2), which needs a level-2
   or level-4 theta structure (rational only over a further extension, through
   `J[4]`/`J[2]`), costs a polynomial in `l^g` operations, and returns theta constants
   of the codomain. `[pcc:unverified]`, from memory.
3. Back to a curve. From the codomain theta constants: decide Jacobian versus product
   (vanishing even theta constants), hyperelliptic versus plane quartic (exactly one
   vanishing even theta constant for hyperelliptic), then build the quartic (Weber or
   Riemann formulas via an Aronhold set), and descend to `F_p` with the right twist
   (Serre's obstruction). Each step is a known construction over `C` or `Q-bar`. Their
   cost and descent behaviour over `F_p` for these `k` is not established here.
4. A map on divisors (to transfer a DLP), not just the codomain: evaluating the isogeny
   on points of `J(F_p)`. The theta method gives that in theta coordinates, so it needs
   the coordinate change from step 3 as well.

Until steps 1-4 have an evaluated cost, a positive census says only "rational odd-`l`
kernels exist on prime-order Jacobians at rate X", not "the Smith route reopens".

## 7. Gaps and things not done

- Research-kb was not queried. `scripts/research-kb search ...` returned verbatim:
  `research-kb: no executable found; set RESEARCH_KB_BIN or put research-kb on PATH`.
  No MCP `research-kb` tool was available to this subagent. So the literature statements
  in sections 3, 4 and 6 are from memory, labelled `[pcc:unverified]`, and no novelty
  or "not tried" claim is made.
- The monodromy and equidistribution assumption (section 3) is what makes the sampled
  `GSp_6` rate a fair reference. The census tests it empirically at small `p`. It is not
  proved here.
- The census computes `chi` by exact point counts only on a gate subsample. On the
  census curves it uses `N_1` plus exact group orders of `J` and its twist (SPEC.md
  section 4), because counting `F_{p^3}`-points costs `p^3` per curve, infeasible for
  thousands of curves at `p ~ 2^12` inside the limits. The brief allowed choosing the
  method. Gate G1 ties the two methods together.
