# H114 Lean proof map and instance gate

Status: design gate completed before the first H114 Lean source edit. Lean
4.34.1 and Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` now build all
mapped declarations. The repository proof gate audits 46 declarations with
zero failures and zero trusted-native declarations; its retained receipt is
`proof-gate-v1/research_h114_multiradix_carry_cohomology_20261117/`. This
formalization deliberately does not assume or assert that one of the 11,520
measured predicates is exact.

## Six-field instance statement

1. **Structure above.** Multiplication paths quantify over Mathlib's
   `AddCommGroup A`; canonical representatives and winding equations live in
   `ℤ`; CRT and the arithmetic decoder live in `ℕ`. A concrete lifted curve is
   used only through its additive-group instance.
2. **Theorems used as stated.** Path normalization uses natural scalar-action
   distribution and associativity. Fixed representative bounds use a supplied
   divisibility witness and Presburger arithmetic. The braid theorem telescopes
   six supplied adjacent-edge equations and cancels nonzero `p`. CRT uses the
   standard natural remainder laws. Decoder reconstruction uses
   `Nat.mod_add_div`, strong induction, and the natural logarithm bound.
3. **Object as an instance.** A section is a function into `A`; its carries are
   expressions in `A`. Winding representatives are integers with interval and
   exact numerator equations. A candidate is a public function whose exactness
   remains a theorem hypothesis until supplied by evidence and, for a generic
   algorithm, by a uniform mathematical proof.
4. **Assembly probe.** The target chain is:

   ```text
   ab • (c • s30 - sab) + a • (b • sab - sa) + (a • sa - sq)
     = abc • s30 - sq;

   N23=p*w23, N25=p*w25, N35=p*w35 and canonical representatives
     => -3<=w23<=2, -5<=w25<=2, -5<=w35<=3;

   six adjacent path differences
     => w23(Q)+3w25(Q/3)+w35(Q)
        =2w35(Q/2)+w25(Q)+5w23(Q/5);

   d=(15*d2+10*d3+6*d5)%30
     => d%2=d2, d%3=d3, d%5=d5 and d is unique below 30;

   digit(m)=m%30 and inv30 • (30 • X)=X
     => inv30 • ([m]G-[digit(m)]G)=[m/30]G;

   run(30,m,log_30(m)+1)=(m,0),
   ```

   followed by an independent injective scalar-action verification endpoint.
5. **Missing components only.** Mathlib does not package this fixed
   `[2],[3],[5]` carry permutohedron, its integer braid certificate, or the
   evidence-to-oracle bridge. H114 states these generically. The native
   producer must separately prove that concrete local-ring arithmetic supplies
   every represented integer and edge equation. Lean will not manufacture the
   empirical full-score premise.
6. **Inheritance table.** Every report operation used by the proof is mapped
   below. A missing line means the formalization is incomplete.

## Statement table

| Statement | Kind | Structure | Hypotheses |
| --- | --- | --- | --- |
| `three_step_carry_path` | generic law; upstream candidate | `AddCommGroup A` | none beyond the structure |
| `path_235_telescope` through `path_532_telescope` | six frozen endpoints | `AddCommGroup A` | none beyond the structure |
| `w23_representative_range` | fixed generic law | `ℤ` | `p>0`, four canonical intervals, exact numerator equation |
| `w25_representative_range` | fixed generic law | `ℤ` | same, coefficients two and five |
| `w35_representative_range` | fixed generic law | `ℤ` | same, coefficients three and five |
| `braid_winding_coherence` | structural endpoint | `ℤ` | `p!=0` and six oriented adjacent-edge equations |
| `crt30` | executable definition | `ℕ` | none |
| `crt30_mod_two`, `crt30_mod_three`, `crt30_mod_five` | generic laws | `ℕ` | limb is in its canonical range |
| `crt30_unique_below` | endpoint | `ℕ` | candidate below 30 and the three limb congruences |
| `exact_predicate_supplies_baseThirty` | endpoint | semantic public points | exactness hypothesis |
| `agreementScore` | finite-certificate definition | finite point type | decidable equality |
| `full_agreement_score_implies_exact` | generic law | finite point type | score equals cohort cardinality |
| `full_score_supplies_baseThirty` | evidence endpoint | finite point type | verified score equals cohort cardinality |
| `radixReconstruct_eq` | generic law | `ℕ` | radix at least two; exact remainder oracle |
| `baseThirtyReconstruct_eq` | H114 endpoint | `ℕ` | exact base-30 oracle |
| `radixRoundState_log_succ_zero` | generic law | `ℕ` | radix at least two |
| `baseThirty_round_bound` | H114 endpoint | `ℕ` | none |
| `radixRun_invariant`, `radixRun_log_succ_eq` | executable decoder laws | `ℕ` | radix at least two for terminal result |
| `baseThirty_run_recovers` | H114 endpoint | `ℕ × ℕ` | none |
| `inverse_thirty_on_cyclic` | generic group law | `AddCommGroup A` | `[r]G=0`, `inv30*30=1+q*r` |
| `baseThirty_cyclic_group_step` | endpoint | `AddCommGroup A` | exact digit and cyclic inverse certificate |
| `full_score_group_trace_last_step_zero` | bounded-trace endpoint | finite cohort plus `AddCommGroup A` | full score, row bindings, order/inverse certificate |
| `verified_scalar_unique` | independent-check endpoint | arbitrary scalar and point types | injective action on admitted domain |
| `h114_conditional_end_to_end_certificate` | final conditional theorem | finite cohort, group, scalar domain | all visible evidence/oracle/binding premises |

## Inheritance table

| Report operation or law | Lean object | Inheritance line |
| --- | --- | --- |
| lifted addition and subtraction | `+`, `-` on `A` | `[AddCommGroup A]` |
| multiplication by 2, 3, 5 and 30 | `n • x` | natural scalar action from the group instance |
| carry expansion | `nsmul_add`, `nsmul_sub`, `nsmul_nsmul` | Mathlib group lemmas |
| canonical integer representatives | inequalities in `ℤ` | standard ordered-ring instance |
| exact winding quotient | `N=p*w` | explicit native-verifier handoff |
| six edge differences | six equalities in `ℤ` | explicit native-verifier handoff |
| CRT digit | `Nat.mod` and fixed coefficients | standard natural arithmetic |
| native exact-ID score | filtered `Finset.univ` cardinality | full cardinality forces every record to pass |
| scalar digit and quotient | `m%30`, `m/30` | natural remainder/division |
| logarithmic terminal bound | `Nat.log 30 m + 1` | standard power/log and division lemmas |
| inverse multiplication on `<G>` | `inverse_thirty_on_cyclic` | derived from order and Bezout certificate |
| final EC equality | injectivity of `fun k => k • G` | explicit admitted-domain hypothesis |

## Proof chains

### Three-step path and six endpoints

For arbitrary natural multipliers `a,b,c`:

```text
(a*b) • (c • s30 - sab) + a • (b • sab - sa) + (a • sa - sq)
= (a*b*c) • s30 - sq.
```

Each of `235`, `253`, `325`, `352`, `523`, and `532` is a direct
specialization with its own intermediate `sab` and `sa`. Since all products
normalize to 30, every endpoint is `[30]s(Q/30)-s(Q)`. This proves path
coherence in the abstract group without coordinates or a curve equation.

### Fixed representative ranges

For `w_ab` the concrete handoff is

```text
a*x + y - b*z - t = p*w_ab,
0 <= x,y,z,t < p, p > 0.
```

For each frozen coefficient pair, linear bounds give

```text
-(b+1)*p < p*w_ab < (a+1)*p,
```

and integer discreteness gives `-b <= w_ab <= a`. H114 exposes the three
specializations required by the protocol rather than hiding them in an
unproved generic division lemma.

### Braid/hexagon certificate

Orient adjacent differences as `K_left-K_right`:

```text
K235-K325 = p*w23(Q)
K325-K352 = 3*p*w25(Q/3)
K352-K532 = p*w35(Q)

K235-K253 = 2*p*w35(Q/2)
K253-K523 = p*w25(Q)
K523-K532 = 5*p*w23(Q/5).
```

Both triples telescope to `K235-K532`. Cancelling nonzero `p` proves exactly

```text
w23(Q)+3*w25(Q/3)+w35(Q)
  = 2*w35(Q/2)+w25(Q)+5*w23(Q/5).
```

This theorem proves that a verifier accepting the six oriented edge equations
must accept a zero braid residual. It does not predict a scalar digit.

### CRT digit

For canonical limbs `d2<2`, `d3<3`, `d5<5`, set

```text
crt30(d2,d3,d5) = (15*d2 + 10*d3 + 6*d5) % 30.
```

The three coefficients are idempotent for moduli 2, 3 and 5. The residue
theorems recover each limb. If `x<30` has the same three residues, fixed-modulus
arithmetic proves `x=crt30(d2,d3,d5)`. This is uniqueness of recombination,
not evidence that the winding limbs equal a scalar's residues.

### Finite evidence handoff

```text
good = {q in cohort | candidate(q)=scalar(q)%30}
good subset cohort
|good| = |cohort|
=> good = cohort
=> candidate(q)=scalar(q)%30 for every scored q.
```

The theorem binds the score, observable, and scalar label from one record. It
does not extend exactness from the seven frozen curves to generic prime fields.

### Conditional base-30 decoder

```text
inv30 • ([m]G - [digit(m)]G)
= inv30 • ([m]G - [m%30]G)                 [exact predicate]
= inv30 • (30 • ([m/30]G))                 [remainder decomposition]
= [m/30]G.                                  [cyclic inverse certificate]
```

The inverse certificate is derived on `<G>` from `[r]G=0` and
`inv30*30=1+q*r`; no inverse action on the whole ambient group is assumed.
The generic arithmetic run maintains

```text
recovered + 30^rounds * remaining = m
```

and terminates at `rounds=Nat.log 30 m+1` with `(m,0)`. Thus an exact,
polynomial-time public digit predicate would yield a polynomial-time—and hence
subexponential—conditional ECDLP algorithm. H114's experiment and Lean proof
do not discharge the exact-oracle premise.

### Final independent verification

The final theorem packages separately:

```text
radixRun(30,m,log_30(m)+1) = (m,0);
the last required quotient is zero;
the corresponding verified group update is the identity;
the independently checked scalar equals the admitted true scalar.
```

The full-score/oracle premise remains an explicit argument. No theorem name or
wrapper may conceal it behind an unconditional `AlgorithmCorrect` claim.

