# L06 assessment (rerun): odd-l rational Lagrangian census, genus-3 hyperelliptic

Assessor: independent assessor subagent, 2026-11-16. I did not produce the data.
Everything in this file has the standing `[pcc:unverified]`. `ASSESSMENT.md` (the earlier
"void: no lead was set up" note) is kept unchanged as history. It describes a state of
the directory that no longer holds.

## Verdict: void (instrument), as the frozen rule requires

PROTOCOL.md section 6, rule 1: "Any of G0-G7 failed: **void (instrument)**". G1 failed
on its frozen ambiguity limit: 9 of 240 gate draws were `ambiguous` (3.75%, limit 2%;
8 of 200 at p = 101, 1 of 40 at p = 257). The rule is applied as written. The producer's
`analyze.py` computes the same string, and I confirmed the G1 counts from
`raw/gate_enum_p*.jsonl`. No census number below is a result under the protocol. Each
is a measured count, reported so the next freeze can be designed with it in view.

My own judgement, which does not change the verdict: the G1 failure looks like a badly
calibrated gate, not a broken instrument. All 231 draws with a unique chi agree with
exact enumeration (0 disagreements). The ambiguous draws are curves with a large
rational 2-torsion. At p = 257 all nine census-ambiguous draws have cycle type
`[2,1,1,1,1,1,1]` and `k = 6`, so `2^5 | #J`, and several multiples of the exponent
fall in the BSGS window. Such draws cannot have prime order. The 2% limit was set at the
smallest prime, p = 101, where this effect is strongest. At the census primes the same
quantity (G8) is 0.23% at p = 257 and 0 elsewhere. The threshold is still not moved
after the data.

## 1. Integrity checks

| Check | Result |
|---|---|
| `sha256sum -c frozen.sha256` | DERIVATION.md, PROTOCOL.md, SPEC.md: OK |
| `sha256sum -c data.sha256` | all entries OK |
| `cmp replay/X raw/X` | identical for every census, gate-enum, gsp and lagr2 output and log. `summary.json` differs only in G0 (replay did not copy `test.log`) and `COMMANDS.log` differs (different runner) |
| Replay independence | weak: the replay reused the producer's binary (`l06/target/release/l06`, no rebuild). It shows determinism, not correctness |
| `raw/test.log` | 11 of 11 unit tests pass (G0) |
| `raw/exit_codes.txt` | all five census runs exit 0, none hit the 600 s timeout |
| Draw indices | contiguous `0..draws-1` at every p, each p stopped at `prime_target` with exactly 120 PRIME |

## 2. Independent audit of the instrument

Assessor code, standard library only, written without reading or importing `l06/` or
`analyze.py`: `assessor/assessor_check.py` (output `assessor/assessor_check.out.json`)
and `assessor/gsp_walk_check.py`. Seeds: `random.Random(20261116)`.

1. **Curve generation.** I re-derived `f` from SplitMix64 and `curve_seed(p, i)` for
   all 16,834 `ok` census rows: 0 mismatches.
2. **Primality.** I ran my own Miller-Rabin on every `nJ`: 0 disagreements with the
   `prime` flag. PRIME = 600 (120 per p).
3. **chi recovery, independently, on 6 PRIME curves** (two each at p = 257, 521,
   1031, sampled with seed 20261116: (257, 2560), (257, 2649), (521, 4730),
   (521, 6837), (1031, 2633), (1031, 8567)). I computed `N1` and `N2` by enumerating
   `F_p` and `F_{p^2}` and got `s1`, `s2` by Newton. Both match the record on 6 of 6.
   I then rebuilt `chi` from the recorded `s3` and recomputed `#J = chi(1)` and
   `#J^tw = chi(-1)`: they match the record on 6 of 6. A from-scratch Cantor
   implementation gives `nJ * D = 0` for 3 random nonzero classes on `J`, and
   `nJtw * D' = 0` for 3 on the twist, on 6 of 6. Because `nJ` is prime and
   `(sqrt p - 1)^6 > 2B + 1`, `nJ` is the only multiple of `ord D` in the Hasse-Weil
   interval. With `s1`, `s2` checked separately, `s3` is then fixed. So `chi` is correct
   on these curves. This is a spot check of 6 curves, not of all 600.
4. **Annihilation gate G2 really ran.** Every `ok` row carries `gate_annihilation`, and
   all are true (16,834). I could not prove from the data that the G2 seed stream
   differs from the order-search stream. My Cantor check in item 3 is the independent
   substitute on the sampled curves.
5. **Criterion as frozen.** I recomputed `fire_l` and `n_h` for all 600 PRIME curves
   by brute force over monic cubics `h` (`h h^* == chi mod l`,
   `gcd(h, h^*) = 1`, `h^*(T) = T^3 h(p/T)/h(0)`). There were 0 mismatches against the
   recorded `fire` and `n_h` at l = 3, 5 and 7. Recomputed: F_3 = 167/600,
   F_5 = 161/600, F_7 = 144/600. ALL (U block): l = 3 2274/10991, l = 5 2598/10991.
   These match RESULTS.md.
6. **GSp_6(F_l) reference.** As an independent sampler I used a random walk of 80
   random symplectic transvections, then `diag(1,1,1,m,m,m)`, Hessenberg charpoly, and
   my brute-force criterion. With 20,000 samples per cell I got these firing fractions
   (the producer's 200,000-sample figures in parentheses): (l=3, m=1) 0.1547 (0.1545);
   (3, 2) 0.2179 (0.2180); (5, 2) 0.2706 (0.2718); (7, 3) 0.2872 (0.2869). Conditioned
   on `charpoly(1) != 0`, (3, 2) gives 0.2749 (0.2734). All agree within one binomial
   standard error. A structural check: for `m = 1`, `fire == fire_nz1` in every
   producer row, as it must be, since a root 1 is then self-reciprocal.
7. **Could a bug produce the fractions?** The checks in items 1-6 cover the steps from
   curve to chi to criterion on a sample, and from criterion to fraction on the whole
   PRIME set. The PRIME rates sit near the GSp coset reference (S4: 0 of 30 cells with
   `|z| > 3`), which is what equidistribution predicts. I found no mechanism that would
   inflate them. Residual risks: chi was independently checked on 6 curves only, and
   the P stream relies on "#J odd iff f irreducible", which G5 checks on every
   `ok` row (0 failures) and which I re-derived in section 3.

## 3. Mathematics of DERIVATION.md, checked independently

- **Section 1 (`ker h(pi)` is a rational Lagrangian).** Correct. The adjoint
  `e(g(pi)x, y) = e(x, g(p pi^-1) y)`, the identity `h^*(p pi^-1) = unit * pi^-3 h(pi)`,
  the coprime splitting `V = V_h (+) V_{h^*}` with `dim V_h = 3`, and isotropy all
  check out. Characteristic `l` does not enter, so it holds for `l = 2`.
- **Claim A (no self-reciprocal factor iff fire).** Correct, including the argument
  that a self-reciprocal irreducible cubic is impossible. **Claim B** (squarefree and
  not firing gives no stable Lagrangian) is correct by the orthogonality of the primary
  components.
- **Section 4, the l = 2 statement.** Correct. With Omega the 7 roots plus infinity,
  the fixed even subsets are unions of the `k + 1` Frobenius orbits. Infinity is its
  own orbit and the rest sum to 7, so exactly half of the `2^(k+1)` unions are even.
  Modulo Omega that leaves `dim J[2](F_p) = k - 1`. So `#J` is odd iff `k = 1` iff `f`
  is irreducible. The permutation module has charpoly `(T^7 - 1)(T - 1)`, and removing
  two trivial factors gives `Phi_7 = (T^3+T+1)(T^3+T^2+1)` over F_2. With `p` odd,
  `h^*` is the reciprocal polynomial, which swaps the two cubics. A 7-cycle fixing
  infinity preserves no pair partition, because the partner of infinity would have to
  be fixed. 135 = 3*5*9, 105 = 7!!, 30 = 8!/|AGL_3(F_2)|. The A2 predictions held on
  all 1,583 ODD curves: fire_2 1583, pair-type 0, stable_other = 2. The producer's
  RESULTS.md and my reading of `summary.json` agree on this.
- The monodromy and equidistribution citation (section 3) and the theta-isogeny
  literature (section 6) remain `[pcc:unverified]`, from memory. research-kb was not
  queried by the coordinator, the executor or this assessor.

## 4. What the measured (non-verdict) numbers say

If the gate had passed, rule 4 would have read "supported" by a wide margin. The
pooled PRIME Wilson lower bounds are 0.244 / 0.234 / 0.208 at l = 3 / 5 / 7, against a
1% kill line. This is labelled as an outcome of rules 2-5 that was not reached. It is
not the verdict. In the measured data, about a quarter of prime-order genus-3
hyperelliptic Jacobians over these small `F_p` carry an `F_p`-rational
`(l,l,l)`-kernel for each odd `l <= 7`. The `l = 2` arm adds that every odd-order one,
prime order included, carries two affine-type (non-Smith) rational `(2,2,2)`-kernels.
That is a mathematical statement (section 3 above), not dependent on the void census.

## 5. Gap to rho in genus 3, and what blocks any transfer claim

- Baseline exponents for genus-3 Jacobians over `F_q`: rho `q^{3/2}`, hyperelliptic
  double-large-prime index calculus `q^{4/3}` (Gaudry-Thome-Theriault-Diem), and plane
  quartic `q^{1}` (Diem). All are cited from memory, `[pcc:unverified]`. The hoped-for
  gain is 4/3 to 1 in the exponent, reached by moving a hyperelliptic DLP to a quartic
  Jacobian.
- This lead changes no exponent and no constant. It measures the existence of a
  rational kernel, and that measurement is void. Distance to rho is unchanged: the
  hyperelliptic route is still `q^{4/3}` against `q^{3/2}` (exponent gap 1/6 in favour
  of IC), and nothing here moves any row.
- Unproven before any transfer claim:
  1. Computing the `(l,l,l)`-isogeny in genus 3. There is no Velu-type formula in
     Mumford form. The theta-coordinate method needs level-2 or level-4 structure, and
     the kernel points live over `F_{p^k}`. The observed `ext_deg` at l = 3 is 13 or 26,
     and DERIVATION section 6 gives up to 342 at l = 7. Its cost is not evaluated.
  2. The codomain type. Whether it is generically a plane-quartic Jacobian, a
     hyperelliptic Jacobian, or decomposable, for odd `l` and for the affine-type
     `l = 2` kernels, is not determined. This is the decisive question, and nothing
     here addresses it.
  3. Descent of the codomain curve to `F_p` with the correct twist (Serre obstruction).
  4. Evaluating the isogeny on divisors of `J(F_p)` and mapping them into the quartic
     model, at a cost charged per DLP. The full cost must then be compared against
     matched rho across the 0 / 0.1 / 1 GAE band of AGENTS.md section 5a, at
     cryptographic `p`, not at `p <= 4099`.

## 6. Concerns about the producer's process

- D1: the executor ran the census after G1 failed, against the task text. This is
  disclosed and harmless to the verdict, but the census data must not be cited as a
  protocol result.
- The replay reused the producer's binary, and its runner raised the timeout to 900 s.
  It reproduces byte-identical outputs, so the run is deterministic. It does not give
  an independent build.
- `raw/build.log` is a no-op incremental build (D3). The full build has no log.
- PROTOCOL/SPEC section-number mismatch (D5) is cosmetic.
- research-kb was never queried, so no literature, novelty or "not tried" statement
  may be drawn from L06.

## 7. Recommended next step

Freeze a new protocol with fresh seeds or primes, so as not to reuse data already
seen. Recalibrate G1: either run it at the census primes, or resolve ambiguity with
extra divisors or exact enumeration before counting, and keep the limit fixed before
data. Then rerun. In parallel, and more decisive for the rho gap, query research-kb
and the literature on the codomain type of `(l,l,l)` and affine-type `(2,2,2)`
quotients of hyperelliptic genus-3 Jacobians. Without a quartic codomain, a positive
census does not bear on the 4/3 to 1 transfer.
