# Lean certificate for one bounded public-point lift

`XedniBoundedLift64.no_canonical_lift_height64` is a checked Lean 4 theorem: the public point `(744,244)` on `E₉/F₁₀₀₉` has no canonical rational lift with `x=a/b²`, `y=c/b³`, `|a|≤64`, and `1≤b≤64`. The theorem uses the exact integral curve equation and both public-coordinate reduction congruences. It is a complete certificate for this finite height bound and this target.

## Mathematical proof

Reducing `x=a/b²` to public x-coordinate 744 gives `a ≡ 744b² (mod 1009)`. The bounds leave exactly four integer pairs: `(-51,2)`, `(-55,20)`, `(61,32)`, and `(-11,33)`. The middle and last pairs have noncoprime numerator and denominator, so they are not canonical coordinates. For `(-51,2)`, the curve equation requires `c²=-139931`, which is impossible. For `(61,32)`, the curve equation requires `c²=1649637029 ≡ 2 (mod 3)`, while an integer square modulo 3 is 0 or 1. Thus every canonical candidate in the height-64 box is excluded. The public y-coordinate congruence is retained in the theorem statement; x and the curve equation already give the contradiction.

The target is the reduced `3Q` fixture on `E₉: y²=x³+9x+1`, `Q=(0,1)`, from the prior [small-scalar ladder](../xedni_small_scalar_ladder_20261120/README.md). [`protocol.json`](protocol.json) pins that fixture and its source SHA-256 digests. [`audit.py`](audit.py) independently enumerates the four x-compatible pairs and checks their exact integer curve values; [`audit.json`](audit.json) records `PASS`.

## Proof map and status

| Result | Lean declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Enumerate x-compatible pairs | `XedniBoundedLift64.x_candidates` | `a∈[-64,64]`, `b∈[1,64]`, `a≡744b² mod 1009` | Checked |
| Integer square residue obstruction | `XedniBoundedLift64.not_square_mod_three` | Every integer square modulo 3 differs from 2 | Checked |
| Bounded public-point lift obstruction | `XedniBoundedLift64.no_canonical_lift_height64` | Coprime canonical `a,b`, exact `E₉` equation, both coordinate congruences, height 64 | Checked |
| Measured height-4,096 exclusion | No Lean declaration yet | Same public target, height 4,096 | Open; prior exhaustive Python audit |
| Rational-point bridge | No Lean declaration yet | Every rational point on the integral model has canonical `x=a/b²,y=c/b³` with the required reduction semantics | Open |

The central Lean statement has no assumed search conclusion or custom axiom. Its representation hypotheses are explicit. It does not substitute the height-64 certificate for the prior measured height-4,096 result; that larger finite certificate and the rational-point bridge remain separate obligations.

## Rebuild

[`formal/lean-toolchain`](formal/lean-toolchain) pins `leanprover/lean4:v4.35.0-rc4`; [`formal/lake-manifest.json`](formal/lake-manifest.json) pins Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` and the dependency tree. The build uses the existing cached, pinned project and compares both pin files before invoking Lean:

```sh
research/exotic_ecdlp_20261021/xedni_lift_lean_certificate_20261120/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_lift_lean_certificate_20261120/audit.py
```

[`formal/build-output.txt`](formal/build-output.txt) retains `#print axioms` for all three declarations. The main theorem reports only `propext`, `Classical.choice`, and `Quot.sound`; the build stderr is empty. `lean4checker --fresh` was unavailable on this host, so the pin-checked Lean build and kernel axiom audit are the recorded proof checks.

The next certificate step is a tractable modular-sieve encoding of the 20,000-plus height-4,096 x-compatible candidates. The preliminary integer sieve found that every coprime candidate has either negative curve right side or a nonsquare residue modulo a prime at most 97; this observation is not yet a Lean theorem and needs its own frozen certificate and independent replay before promotion.
