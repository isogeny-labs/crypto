# Lean certificate for the height-4,096 public-point lift search

`XedniFullLift4096.no_canonical_lift_height4096` is a checked Lean 4 theorem for the measured public point `(744,244)` on `E₉/F₁₀₀₉`: no coprime canonical rational coordinates `x=a/b²`, `y=c/b³` with `|a|≤4096`, `1≤b≤4096` satisfy the exact curve equation and both coordinate reductions. The proof checks the complete finite modular sieve in Lean and then proves that any such rational-coordinate lift would contradict a sieve entry. The prior height-64 theorem is retained as a compact hand-checkable case.

## Mathematical proof

The public x-coordinate forces `a ≡ 744b² (mod 1009)`. For each positive `b≤4096`, let `r` be the least residue of `744b²`. The bounds force `a=r+1009j` for some `j=-5..4`; [`CandidateListProof.lean`](formal/CandidateListProof.lean) proves that coverage. [`Sieve4096.lean`](formal/Sieve4096.lean) checks every `b=1..4096` and each of those ten representatives: after the `a` bound and coprimality filter, the curve right side `C=a³+9ab⁴+b⁶` is negative or has no square root modulo one of the listed primes `3..97`. This computation is proved by Lean `decide` and kernel checked; it does not rely on unverified native evaluation.

[`SquareModulo.lean`](formal/SquareModulo.lean) proves that if `c²=C` over the integers, then `C` has a square root residue modulo every positive modulus. [`FullLift4096.lean`](formal/FullLift4096.lean) applies that structural lemma to the sieve result. The public y-coordinate congruence is retained in the theorem statement; the x congruence and curve equation already exclude every candidate. The theorem preserves the exact field prime, curve coefficient, target, height cap, coprimality, and exceptional non-square/negative cases.

The target is the reduced `3Q` fixture on `E₉: y²=x³+9x+1`, `Q=(0,1)`, from the prior [small-scalar ladder](../xedni_small_scalar_ladder_20261120/README.md). [`protocol.json`](protocol.json) pins the source fixture and [`protocol-4096.json`](protocol-4096.json) pins the extended certificate. An independent [`audit_4096.py`](audit_4096.py) scans all 33,558,528 `(a,b)` box positions directly, filters x congruence, and verifies every resulting candidate with a separately computed modular-square table. Its completed [`audit-4096.json`](audit-4096.json) records 33,127 x-compatible pairs: 13,014 noncoprime, 319 with negative curve right side, and 19,794 with a modular nonsquare witness. No pair remains unclassified. The earlier [`audit.py`](audit.py) and [`audit.json`](audit.json) retain the four-candidate height-64 check.

## Proof map and status

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Enumerate x-compatible representatives | `XedniCandidateList.x_candidate_mem` | `a∈[-4096,4096]`, `a≡744b² mod 1009`, arbitrary natural `b` | Checked |
| Every square has a modular root | `XedniSquareModulo.square_has_modular_root` | Integer `c²=C`, positive modulus `m` | Checked |
| Exhaustive finite sieve | `XedniSieve4096.check4096` | All `b=1..4096`, ten x representatives, exact bounds and coprimality, negative or modular nonsquare test | Checked by kernel reduction |
| Full bounded coordinate obstruction | `XedniFullLift4096.no_canonical_lift_height4096` | Coprime canonical `a,b`, exact `E₉` equation, both public coordinate congruences, height 4096 | Checked |
| Compact base instance | `XedniBoundedLift64.no_canonical_lift_height64` | Same curve and target, height 64 | Checked |
| Rational-point representation bridge | No Lean declaration yet | Every rational point on this integral curve has `x=a/b²,y=c/b³` in reduced canonical form, with good-reduction semantics | Open |

The finite theorem has no assumed search conclusion, custom axiom, `sorry`, or `admit`. The next proof step is the separate bridge from rational points and reduction to these canonical coordinates; the Lean statement records the representation hypotheses explicitly.

## Rebuild

[`formal/lean-toolchain`](formal/lean-toolchain) pins `leanprover/lean4:v4.35.0-rc4`; [`formal/lake-manifest.json`](formal/lake-manifest.json) pins Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` and the dependency tree. The build checks both pins against the existing cached project and compiles all modules in dependency order:

```sh
research/exotic_ecdlp_20261021/xedni_lift_lean_certificate_20261120/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_lift_lean_certificate_20261120/audit_4096.py
```

[`formal/build-output.txt`](formal/build-output.txt) retains `#print axioms` for the checked declarations. The full obstruction reports only `propext`, `Classical.choice`, and `Quot.sound`. `lean4checker --fresh` was unavailable on this host; the pin-checked Lean build and kernel axiom audit are the retained proof checks.
