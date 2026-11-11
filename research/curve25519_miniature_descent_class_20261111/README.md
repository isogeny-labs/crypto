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
