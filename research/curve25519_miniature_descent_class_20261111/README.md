# Does the k = 20 Curve25519 miniature's factor base share one 2-descent class? No.

2026-11-11. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen at `3ce1458662` before any data.
Instrument: [`descent_class.py`](descent_class.py), an independent stdlib-Python
re-construction of the miniature (not the Rust instrument). All statements are
[pcc:unverified]; no claim is registered, and nothing here is a cost result.

## Answer

The premise behind the "2× filter on the 8 cofactor lifts" in
[`research/tracezero_toy_20261106/README.md`](../tracezero_toy_20261106/README.md) fails for the
"bits" base. The base points fall in **both** descent classes, so no lift is excluded.

## Registered cells (`descent_class.out.txt`, `results_registered.json`)

- Miniature rebuilt by the family rule: p = 1048573, A = 926, #E = 1049624 = 8·131203
  (ℓ = 131203 prime). One rational 2-torsion abscissa (x = 0); E[8] cyclic of order 8.
- delta(P) = x_M(P) mod squares is a homomorphism: 0 violations in 2,000 random pairs.
  delta is −1 on a generator of E[8] and +1 on the order-ℓ subgroup, as it must be.
- Base classes ("bits": every Weierstrass abscissa x_W ∈ [0, 2^t) on the curve, class of
  x_W − A/3):

| t | base abscissae | delta = +1 | delta = −1 | smaller class |
|---|---|---|---|---|
| 5 | 11 | 4 | 7 | 36% |
| 6 | 25 | 10 | 15 | 40% |

  Under the frozen rule (both classes ≥ 10%) the premise **fails**.
- The hit check (check 4) is underpowered at these sizes: 0 hits at t = 5 and 3 at t = 6 over
  200 targets × 8 lifts, none violating the delta identity.

## Supplementary run, not registered (`descent_class_t9.out.txt`, `results_supplementary_t9.json`)

Added only because the registered cells give too few hits to exercise check 4. Same script,
t = 9: 233 base abscissae, 124 in class +1 and 109 in class −1 (53% / 47%). 175 hits over 200
targets × 8 lifts: 91 with delta(R+K) = +1 and 84 with delta(R+K) = −1, **0 violations of
delta(R+K) = delta(P_i)·delta(P_j)**. Pair sums by class: 54,747 and 54,064.

## What follows

- Each target has 8 lifts R + K, four with delta = +1 and four with −1. Because the base has
  both classes, every lift can decompose. A hit in either class is equally likely, as the t = 9
  counts show. **No lift can be skipped.**
- What delta does give is a restriction on candidate pairs: for a lift with delta = +1 only pairs
  with equal classes can work, for delta = −1 only mixed pairs. With the class fractions above
  that keeps about half of the pairs (0.50 at t = 9; 0.46 to 0.54 at t = 5, 6).
  It does not change which relations exist. Whether it saves F4 work (for instance by splitting
  the base into two sub-bases) is not measured here and needs the Rust instrument and a frozen
  spec priced under AGENTS.md §5a.
- Not tested: the "isogeny" base (the tower's output set is produced by Rust code not
  re-implemented here), "digits", "square-bits" and "edwards-bits". Their class mix is unknown.
- Correction to the earlier note: the sentence "if every factor-base point had a fixed delta
  class, at most half of the 8 attempts could succeed" is true but its condition does not hold
  for the "bits" base. The expected gain from the filter on lifts is therefore 1×, not 2×, for
  that base.

## Not done

No figure or PDF (matplotlib is not installed here), no benchmark record (no timing was taken),
no Conductor task and no research-kb query (neither available). Sources read:
`repo:src/bin/curve25519_family.rs` (`build_miniature_at`, base forms),
`repo:research/curve25519_cofactor_algebraic_20261102/PROTOCOL.md`.

## The "isogeny" base (PROTOCOL_isogeny.md, frozen at `d1f5fafcf9` before its data)

Result: **the premise fails for the isogeny base too.** The point set V = x(R' + ⟨Q⟩) was produced
by the real `isogeny_tower(p, t, seed, ..)` from `src/cryptanalysis/ic_framework/prime_tower.rs`
through a scratch program ([`towerdump/main.rs`](towerdump/main.rs), built against this
repository's library with `Cargo.toml.txt` renamed; it is not part of the workspace). Dumps are
in `tower_dumps/`. [`classify_isogeny_base.py`](classify_isogeny_base.py) keeps the V-abscissae
that lie on the miniature (p = 1048573, A = 926) and takes delta = chi(x_W − A/3)
(`isogeny_classes.out.txt`, `isogeny_classes.json`).

| registered cell (seed 0) | on-curve abscissae | delta = +1 | delta = −1 | smaller class |
|---|---|---|---|---|
| isogeny t = 4 | 9 | 6 | 3 | 33% |
| isogeny t = 5 | 18 | 7 | 11 | 39% |
| isogeny t = 6 | 25 | 14 | 11 | 44% |

- Under the frozen rule (both classes ≥ 10% in the registered cells) the premise fails.
- Supplementary, unregistered: t = 9, seed 0: 121 / 135. Seeds 1 to 20 at t = 5 pool to 153 / 166
  (48% in class +1) and at t = 6 to 311 / 308 (50%); per-seed minority fractions range from 21%
  to 50%. The class mix does not depend on the seed in any way I can see.
- Why: V is a coset's x-coordinates on a searched auxiliary curve E', then read as Weierstrass
  abscissae on the miniature. It has no relation to the miniature's 2-torsion, so
  chi(x_W − A/3) behaves like a fair coin on it. This matches the "bits" base.

Both bases tested on the miniature therefore span both descent classes: no cofactor lift can be
skipped, and the only remaining use of delta is class-pruning candidate pairs (about half per lift,
effect on F4 cost unmeasured). "digits", "square-bits" and "edwards-bits" remain untested.
No figure or PDF (matplotlib is not installed here), no cost claim, no matched rho.
Everything here is [pcc:unverified].

## The remaining bases (PROTOCOL_other_bases.md, frozen at `a579737e0f` before its data)

[`classify_other_bases.py`](classify_other_bases.py), output `other_bases.out.txt`. Same miniature,
delta = chi(x_M); "too few" means fewer than 8 on-curve abscissae, which cannot count either way.

| base | cell | on-curve | class +1 | class −1 | verdict |
|---|---|---|---|---|---|
| square-bits | lo = 0, t = 4 / 5 / 6 | 11 / 20 / 35 | all | 0 | **fixed class (+1)** |
| square-bits | lo = 984071, t = 4; lo = 414271, t = 5 | 14; 22 | all | 0 | **fixed class (+1)** |
| edwards-bits | t = 4 / 5 / 6 | 7 / 14 / 29 | 3 / 9 / 15 | 4 / 5 / 14 | too few / mixed / mixed |
| t0-bits | lo = 0, t = 4 / 5 / 6 | 6 / 14 / 32 | 4 / 10 / 18 | 2 / 4 / 14 | too few / mixed / mixed |
| digits | `0:1;2;4;8;16` | 11 | 4 | 7 | mixed |
| digits | `113971:289589;761384;273167;963514;602728` | 22 | 19 | 3 | mixed (14% minority) |
| digits | `51134:293482;1015139;374525;914144;700453;505371` | 43 | 36 | 7 | mixed (16% minority) |

- **square-bits is the fixed-class base**, by construction (x_M = w²). Supplementary t = 9, lo = 0:
  510 base points; 197 hits over 200 targets × 8 lifts, **all on lifts with delta(R+K) = +1 and none
  on the four lifts with delta = −1**. So for this base exactly half of the 8 lifts are provably dead.
  I read the oracle only partly; the code I saw skips lifts whose abscissa was already tried and
  I did not find a class-based skip. That is a reading of the code, not an established absence.
- The two selected digit sets lean toward class +1 (86% and 84%). They were chosen for coverage on
  the even cosets, and the parent study notes that frame selection "already puts the coverage on
  coset 0"; class +1 is the even half of the group. So selection may already capture much of the
  effect, without an explicit class filter. This is consistent with, not a test of, that note.
- edwards-bits and t0-bits are mixed where the cell is large enough. In t0-bits z and 1/z share a class
  (translation by the 2-torsion point multiplies the class by B = 1), so classes come in pairs.

## Correction: this mechanism is not new to the repository

My earlier notes presented the 2-isogeny descent class as a finding. It is lever T2 of
[`research/curve25519_torsion_leads_20261103`](../curve25519_torsion_leads_20261103/README.md): the
`square-bits` base (x = w² + A/3) was built so that every base point lies in 2E(F_p), tested on five
miniatures with 0 odd-coset points, and measured in the F4 engine: coverage about 2×, but F4
mul-adds per relation 1.28× (t = 4) to 1.71× (t = 5) higher, "T2 is closed in its bit-encoded F4
form". My first search for the idea listed a 2026-11-03 session log that I did not open; I should
have. What my notes add is small:

- a derivation of why the class is constant on a line through the 2-torsion abscissa, and the
  trace-zero toy fields where it explains the b = 0 deficit;
- a check that the other repository bases are mixed, so the lift filter applies to `square-bits` only;
- the value delta(P) = chi(x_M(P)) is one Legendre symbol per base point, whereas the parent study
  prices frame selection with [r]P per column. Whether a one-bit class label is enough for its
  selection (its labels are full cosets in Z/8) is not tested here, and no saving is claimed.

Correction to a detail in the toy-test and miniature scripts: the class of (0,0) on y² = x(x²+Ax+B) is
B mod squares, which is +1 here; `descent_class.py` used chi(A²−4) at x = 0, a point no base
contains, so no result changes.

No cost claim, no matched rho, no figure or PDF (matplotlib missing). All of it [pcc:unverified].

**Update 2026-11-14:** the open points above were measured in
[`research/curve25519_descent_skip_20261114`](../curve25519_descent_skip_20261114/README.md). For `square-bits` the lift
oracle already tries only the 4 class +1 lifts, because `cofactor_fiber` builds the torsion group from the base points'
`[r]P` and every base point is in 2E(F_p); an explicit skip fired 0 times. On the `bits` base the oracle tries all 8
lifts. A one-bit class label reaches 91.9% of the full coset label for frame selection. Lifts cost 8× (`bits`) and 4×
(`square-bits`) per target and lowered hits per counted mul-add on `square-bits` (rate ratio 0.69, post hoc p = 0.0045).
