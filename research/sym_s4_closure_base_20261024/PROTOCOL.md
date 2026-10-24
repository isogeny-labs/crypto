# Pre-registration: the symmetrised S₄ solver on the Frobenius-closure base (replication of a negative)

Registered 2026-10-24 by the commit that adds this file, before the oracle
exists and before any run. Owner request 2026-10-24: preregister the
structured Semaev solver, and the symmetrised solver "anyway" next to the
sparse closure solver
([`../structured_closure_20261024/PROTOCOL.md`](../structured_closure_20261024/PROTOCOL.md)).

Labels: *recorded*, *derived*, *predicted*.

## 0. What is already known (*recorded*)

- The symmetrised `S₄` is built and tested: `src/cryptanalysis/koblitz_sym_s4.rs`.
  - `S₄` written in `e₁, e₂, e₃` is quadratic after Weil descent.
  - `e₁, e₂, e₃` lie in `W₁, W₂, W₃` of dimensions `1, …, k(ℓ−1)+1` for a
    geometric base `V = θ·⟨1, g, …, g^{ℓ−1}⟩`.
  - `decompose_by_e3` is exact, at cost `2^{dim W₃}·poly(n)`.
  - `decompose_pair_linear` solves `m = 2` with one linear solve in `(s, p)`.
- End to end on the **standard** subspace, the symmetrised arms cost
  987×–769,221× strong rho at n = 17, 23, 31. The x-frame F4 arm cost
  1,180×–15,846×, and the pair tables cost 9×–131×
  (`research/koblitz_symmetrised_e2e_20260927/RESULTS.md`).
- Against the pair table directly, `decompose_by_e3` costs 553×–34,901×
  more, in `F_{2^n}` multiplications per target. That is a sealed negative
  (`research/pdp_shape_replay_20261015/EXPERIMENTS.md` L3c, `examples/e3_vs_table.rs`).
- On the Frobenius closure `∪ σʲ(V)` (round 2,
  `research/pdp_split_slots_20261012`), the x-frame F4 arm solves 2.0–3.2×
  fewer systems than on V. It costs 0.35–0.54× at the pinned XOR ratio, and
  it stays 36× (n = 11) to 25,257× (n = 17, ℓ = 5) above strong rho.

## 1. The question

The closure was never tried with the symmetrised solver.
`σʲ(V) = ⟨1, g_j, …, g_j^{ℓ−1}⟩` with `g_j = z^{2^j}` is geometric, so every
slot's subspace admits the symmetric encoding. But a τ-slot tuple puts the
summands in possibly different subspaces, and symmetry only holds among
summands that share a slot.

**Q.** On the closure base, with the solver chosen per slot class, does the
symmetrised solver still lose to the pair table on the same base at every
§5a weight? And how does it compare with the round-2 F4 arm?

It is a replication with one changed factor, the base. A confirmed negative
is the expected outcome and is recorded as such.

## 2. The oracle to build (`pdp-sym`)

It is native Rust, in `src/bin/ecbench/pdp_sym_oracle.rs`, registered next to
`pdp-split`. The oracle string is `pdp-sym:m=3,slots=all,max_free=24`. For each
query target and each slot tuple `(0, j₂ ≤ j₃)` of `pdp_split::slot_tuples`,
it solves by class:

| slot class | solver | per-system cost (*derived*) |
|---|---|---|
| `(0, 0, 0)` | `decompose_by_e3` on `V₀` (all three summands symmetric) | `2^{3ℓ−2}` eliminations of `n × (3ℓ−1)` |
| two equal, `(0, 0, j)` or `(0, j, j)` | enumerate the odd summand `x ∈ V_j`, lift it, form both `x(R ∓ P)` with one charged group addition each, then `decompose_pair_linear` on the pair's slot | `2^ℓ` eliminations of `n × (3ℓ−1)` |
| all distinct | no symmetry, so the round-2 path: `pdp-split` F4, plan `3`, guess 4 | as round 2 |

Every candidate is lifted and verified as `pdp-split` does, and lift
additions are charged as group additions. The native work is
`F_{2^n}` multiplications (`split_words_counted`) and elimination word XORs.
It is recorded as unpriced solver cost (`counters.sym_field_mul_uncharged`,
`counters.sym_word_xor_uncharged`), so the §5a band prices it.

Tests that land with the oracle:

- On n = 11 and n = 13 with ℓ = 4, for every slot tuple of 32 random
  targets, the decompositions `pdp-sym` returns equal the exhaustive
  enumeration (`pdp_split::exhaustive_decompositions_bases`), in both
  directions. Spurious `(e₁, e₂, e₃)` roots whose cubic does not split in `V`
  are rejected by `split_in` and counted.
- The `tests/ecbench.rs` verification test is extended to `pdp-sym`.

## 3. Sessions

There is one ecbench session per curve. They are the round-2 curves and
dimensions:

| curve | ℓ |
|---|--:|
| `icv1-f2m11-t67-05f5aa36` | 4 |
| `icv1-f2m13-t181-515ee569` | 4 |
| `icv1-f2m17-tm101-00378d4e` | 5 |
| `icv1-f2m19-tm797-9c54981b` | 4 |

Each session has 8 public targets, with fresh seeds fixed in the spec files
committed with the oracle, before any run. Arms, interleaved (`order:
alternate`), L0:

| arm | method |
|---|---|
| `rho-strong` (reference) | `rho.signed_frobenius_strong` (lanes 32, dp_bits 4, step_cap_factor 2000) |
| `rho-frob` | `rho.signed_frobenius` |
| `clo-sym` (candidate) | `ic.pipeline`, `koblitz-closure:dimension=ℓ`, `pdp-sym:m=3,slots=all,max_free=24` |
| `clo-unsplit-g4` | `ic.pipeline`, `koblitz-closure:dimension=ℓ`, `pdp-split:m=3,plan=3,guess=4,slots=all,max_free=24` (round 2's best F4 arm) |
| `clo-mitm3` | `ic.pipeline`, `koblitz-closure:dimension=ℓ`, `mitm-frobenius:m=3` (the pair table, fully charged) |

The `timeout_seconds` is 7200 per execution. An n = 19 run that cannot
finish inside it is recorded as censored, as in round 2.

## 4. Analysis, fixed now

Analysis is native, with `pdp_split_analyze --ecbench` extended to read the
`sym_*` counters.

- **Per arm:** verified count, recorded S, unpriced native ops, and the §5a
  band S at w = 0, 0.1 and 1 GAE per unpriced op, for every arm, rho
  included. Report the median and per-workload values.
- **Pairs ranked, only where the order holds at every weight:**
  `clo-sym` against `clo-mitm3`, against `clo-unsplit-g4`, and against both
  rho arms.
- **Class shares:** the fraction of systems solved by each slot class, and
  the fraction of the `clo-sym` native cost each class carries.
- **Distance remaining** to strong rho for every arm, per the owner's
  success criterion, with no pass/fail gate.

## 5. Decision rule

- If `clo-mitm3` is cheaper than `clo-sym` at every weight on every curve,
  the sealed negative is **confirmed on the closure base**. The atlas lead
  `LEAD-ICE-boolean-f4-f5-f6-engines` gets the evidence row, and nothing
  reopens.
- If `clo-sym` is cheaper than `clo-mitm3` at every weight on at least two
  curves, the negative is **reopened on the closure base**. The rows go
  through the validation gate before any leaderboard or benchmarks entry.
- Anything else is **unresolved in the band**, with the curves named.

## 6. Predictions (before the runs)

1. Every recorded run verifies, and the exhaustive-agreement tests pass
   (0.9).
2. `clo-mitm3` is cheaper than `clo-sym` at w = 0.1 and w = 1 on every curve,
   by at least 30× at w = 1. At w = 0 the order is unresolved, because
   unpriced work is free there (0.8). So the negative is confirmed or
   unresolved, never reopened (0.85).
3. `clo-sym` ÷ `clo-unsplit-g4` at w = 1 lies between 0.3 and 3 on every curve
   (0.6). The standard-base sym/x ratio was 0.76–4.0 (*recorded*).
4. The two-equal class carries most of `clo-sym`'s cost on every curve
   (0.6). It is the most frequent class with a `2^ℓ` enumeration.
5. No arm's band S lies below strong rho at every weight (0.95).

## 7. What was run before this protocol

Nothing on this oracle, which does not exist yet. Read: the studies in §0,
`koblitz_sym_s4.rs`, `examples/e3_vs_table.rs`, and round 2's README and
ANALYSIS.

## 8. Evidence and reporting

The run directory holds:

- `sessions/<curve>/` and `audits/` (`ecbench verify --replay-all`);
- `ANALYSIS.json`/`.md`, `README.md` against §5;
- a report, graphs (band S per arm and curve; class shares) and a PDF, per
  the `research-visuals` skill.

Because these are performance measurements, the PDF and graphs are also
copied to `benchmarks/<date>-sym-s4-closure-base/`, with a
`benchmarks/README.md` row. That row passes through the validation gate
(`VALIDATION.json`: replay certificate, validator report, matched rho with
the §5a band), or renders as unvalidated.
