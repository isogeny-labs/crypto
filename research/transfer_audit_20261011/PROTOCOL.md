# Transfer audit: automated subgroup-preserving map discovery and complete cost accounting

- **Lead:** Lead 5, Jacobian and Weil-descent transfers (program retrospective of 2026-10-09).
- **Protocol frozen:** before any run of the tool in this directory. Changes after the first run are
  appended under "Amendments" with the date and reason; nothing above that heading is rewritten.
- **Base revision:** `origin/main` at `71dd7db5c487f1694ee77c241f011f95f2261e59`.
- **Status of every number this study will produce:** a formula-level cost estimate, unless a row
  says `measured`. No ECDLP speedup is claimed. A verdict of `no weakness found in scope` is the
  expected outcome for every standardised curve audited here.

## 1. Question

Transferring an ECDLP into another representation (Weil/GHS descent, a hyperelliptic cover and its
Jacobian, an isogeny, a pairing into a finite field) helps only if the entire transfer-and-solve
pipeline is cheaper than the matched generic attack. The repository already holds pieces of this
(`ghs_screen`, `ghs_transport`, `jacobian_transfer`, `curve_traits`, `mov_attack`,
`hyperelliptic_index_calculus`, `sect113r1_audit`, `binary_ecc::cover_transfer`). It holds no tool
that, for one concrete curve with its generator, (a) enumerates every candidate transfer in a
declared family, (b) attaches to each candidate a subgroup-preservation certificate whose status
(`verified`, `arithmetic`, `not_evaluated`, `refuted`) is machine-derived, (c) prices the candidate
in the phases the transfer discipline requires, and (d) emits a verdict from the closed vocabulary.

**Deliverable of this study:** the `transfer-audit` binary (Rust, in this crate), its certificates on
the curves of section 3, and the table of section 9.

## 2. Hypotheses, falsification and stop conditions

**H-tool (what the study tests).** The tool, run on the frozen inputs, (i) reproduces the
repository's recorded negatives instead of rediscovering them, (ii) executes the maps that the
repository can execute and agrees with its own arithmetic certificates on every executed case, and
(iii) reports as `not_evaluated` every obligation that no code in the repository can discharge.

**H-science (expected outcome).** For every standardised curve in section 3, no candidate transfer
has an estimated total cost below the matched generic baseline, so the verdict is
`no weakness found in scope` for each curve. This is a statement about the enumerated family only
(section 5), never a statement about deployed security.

**Checks that falsify H-tool** (each is a recorded pass or fail, none is retried silently):

| id | check | pass condition |
|:--|:--|:--|
| F1 | sect113r1 GHS row | single tower `l = 1`, magic number 113, genus `2^112`, type II, Weil-capacity gate excludes genus `<= 44`; the repository test `weil_capacity_gate_excludes_genus_at_most_44` is the reference |
| F2 | sect113r1 cost cross-check | route total within 0.002 bits of `docs/curves/jacobian-transfers.json` for the same slug and route kind |
| F3 | ECC2K-130, K-163, B-163, sect113r1 prime degree | `ghs_window_is_empty(n, 2, 6)` and the list of achievable magic numbers are emitted; no proper subfield `F_(2^d)`, `1 < d < n`, is claimed |
| F4 | Koblitz trace branch | executed `Tr_(K/F_2)` kills `<G>` exactly when `gcd(r, #E(F_2)) = 1` (arithmetic prediction equals executed result on every tower tried) |
| F5 | toy Koblitz towers (n = 15, 21, 35) | for every proper divisor `l`, executed trace transport agrees with the arithmetic prediction `r | #E(F_(2^l))`; a disagreement is a tool defect |
| F6 | subfield-subgroup control | a subgroup contained in `E(F_(2^l))` survives the trace (nonzero image of the right order) and the tool labels the result "structure exists, constant factor only" |
| F7 | binary genus-3 cover | executed pullback `phi^*(G)` has exact order `r` in `Jac(H)(F_q)`, additivity and scalar-equivariance samples pass, push-pull identity `phi_* phi^* = [3]` holds on the generator |
| F8 | genus-`g` covers over prime fields | for every `g` in 2..8 the exponent `(2 - 2/g) log2 p` exceeds `0.5 log2 r` and the priced route exceeds the baseline; the tool cites `IDEA-20261009-275a14` and does not present this as new |
| F9 | measured index-calculus control (odd characteristic) | fitted exponent of total group operations against `p` is at least 1 for genus 2 and genus 3, and total operations exceed the modelled rho cost on a source subgroup of order about `p` at every size run |
| F10 | positive control | a synthetic GHS instance priced with the frozen model (`g = 4`, `l = 40`, `r` of 158 bits) is classed `formula_advantage`; if the tool cannot flag it, the classifier is broken |
| F11 | negative control | the subgroup-preservation checker rejects a map whose kernel meets `<G>` (unit test) and rejects a non-homomorphism |
| F12 | reproducibility | `transfer-audit check` regenerates every certificate byte-identically |

**Stop conditions.** The study stops at the pull request. Total compute is capped near 90
CPU-minutes including builds; every run is wrapped in `timeout`; a run that exceeds its timeout is
recorded as `INDETERMINATE` for that candidate, not retried with a larger limit.

**Abandon condition.** If the register of section 7 cannot express a candidate without inventing a
status, the candidate is recorded as `not_evaluated` and the missing capability is named in
`open_obligations`; the tool is not widened to hide the gap.

## 3. Frozen inputs

Registered curves are read from `docs/curves/registry.json` and named by ICV1 slug
(`AGENTS.md` section 11). Generators, subgroup orders and cofactors are the registry's.

| name | ICV1 slug | field | subgroup r (bits) | cofactor |
|:--|:--|:--|--:|--:|
| sect113r1 | `icv1-f2m113-tm122610772499221213-97df4ac6` | `F_(2^113)` | 112 | 2 |
| ECC2K-130 | `icv1-f2m131-tm22283658519494248867-115e0dc5` | `F_(2^131)` | 129 | 4 |
| K-163 (sect163k1) | `icv1-f2m163-tm4845466632539410776804317-e15e581e` | `F_(2^163)` | 163 | 2 |
| B-163 (sect163r2) | `icv1-f2m163-tm6224022517221266226059365-31d88d74` | `F_(2^163)` | 163 | 2 |
| P-192 (secp192r1) | `icv1-fp192-t31607402316713927207482677199-52e4af59` | `F_p`, 192 bits | 192 | 1 |
| secp256k1 | `icv1-fp256-t432420386565659656852420866390673177327-76dadd18` | `F_p`, 256 bits | 256 | 1 |

Bit lengths are read from the registry by the tool; the table above records only what is to be
checked. Toy curves are unregistered and carry no ICV1 slug; they are fully specified by field
modulus, `a`, `b`, generator and `r` in each certificate:

| name | curve | field modulus |
|:--|:--|:--|
| toy-k1-n15 | `y^2 + xy = x^3 + x^2 + 1` | `z^15 + z + 1` |
| toy-k0-n15 | `y^2 + xy = x^3 + 1` | `z^15 + z + 1` |
| toy-k1-n21 | `y^2 + xy = x^3 + x^2 + 1` | `z^21 + z^2 + 1` |
| toy-k1-n35 | `y^2 + xy = x^3 + x^2 + 1` | `z^35 + z^2 + 1` |

The tool checks irreducibility of each modulus (Rabin) and refuses the instance otherwise. For a
toy curve `#E(F_(2^n))` comes from the Lucas recurrence on the trace over `F_2` and is checked by an
annihilation test on random points; `r` is the largest prime factor of `#E`, and the generator is a
seeded point multiplied by the cofactor. All toys use seed `20261011`.

Odd-characteristic index-calculus control (F9): the repository's thread curves
`y^2 = x^5 + 3x^3 + 2x^2 + x + c` (genus 2) and `y^2 = x^7 + x^3 + cx + 1` (genus 3), via
`hyperelliptic_ic_bench::thread_instance`, `p` in {41, 61, 101, 151} (genus 2) and
{23, 31, 41, 61} (genus 3), 3 index-calculus and 9 rho trials per instance, the `fb-walk` / sparse /
gcd configuration of `examples/hyperelliptic_ic_vs_rho.rs`.

## 4. Reference (matched generic baseline) and unit

All routes are priced in `log2` of bit operations by the repository's frozen model
`jacobian_transfer::cost::CostModel::default()` (Karatsuba exponent 1.585, 10 field multiplications
per elliptic operation, measured Jacobian operation counts, double-large-prime hyperelliptic index
calculus `g! q + g^2 q^(2-2/g)`, Enge-Gaudry and smoothness-count variants). The tool records the
model constants in every certificate; it adds none of its own except the ones named in section 6.

**Baseline.** Pollard rho on `<G>`: `sqrt(pi r / 4)` group operations, each one elliptic operation
(`CostModel::log2_rho_elliptic`), divided by `sqrt(c)` where `c` is the size of the equivalence class
the curve admits:

- negation: `c = 2` for every curve;
- Koblitz-type binary curve (`a`, `b` in `F_(2^d)`, minimal `d`): negation and the `2^d`-Frobenius,
  `c = 2 n / d`, an upper bound on the speedup (the Frobenius acts on `<G>` as an eigenvalue whose
  multiplicative order divides `n / d`);
- prime field with `j = 0` and `p = 1 (mod 3)`: `c = 6`; with `j = 1728` and `p = 1 (mod 4)`:
  `c = 4`.

The model's convention (`sqrt(pi r / 4)`) is used, not the `sqrt(pi r / 2)` convention of
`isogeny_degree_search::cost::rho_reference`; the two differ by a factor `sqrt 2` and the choice is
recorded in each certificate. The cost ratio is `route / baseline`; a value above 1 means the route
is more expensive than rho. The sign convention of `jacobian_transfers` (`reference - total`) is
reported beside it so the two files can be compared.

## 5. Candidate families enumerated (exploration boundary)

For each curve the tool enumerates exactly these candidates; anything outside is "not found within
this family and boundary", never nonexistence.

**A. Weil/GHS descent (binary fields only).** One candidate per divisor `l` of `n` with `l < n`,
including `l = 1`. For each: magic number `m(b)` (Maurer-Menezes-Teske eq. (5)), GHS type and genus,
MMT Lemma 6 condition (8), descent field `F_(2^l)`, Weil capacity of `Jac(F_(2^l))`, the achievable
magic numbers of the whole tower (class-wide, isogeny-independent exclusion), and the priced route.
Executed where the code can: the trace branch (`m = 1`, `a` and `b` in `F_(2^l)`) through
`ghs_transport::trace_transport` on the actual generator and a planted known-answer target. For
`m >= 2` the smooth model and the norm-conorm point map do not exist in the repository
(`docs/GHS_TRANSPORT.md`, "Remaining construction gates"); those candidates are `arithmetic`/
`not_evaluated` and say so. Prime-field curves get one `not_applicable` row with the reason.

**B. Isogeny moves.** One candidate per curve: Frobenius discriminant, `d_K`, conductor of `Z[pi]`,
`End(E)` level information, rational `ell`-isogeny structure for `ell <= 31`
(`curve_traits::compute`), the class-wide invariants an isogeny cannot change (order, `r`,
embedding degree, twist), the subgroup-preservation argument (`gcd(deg, r) = 1`), and, for binary
curves, the GHS isogeny-extension estimate (heuristic, `jacobian_transfer::binary`). For sect113r1
the repository's own explicit degree-5 isogeny certificates (`sect113r1_audit`) are replayed and
cited as `verified`. Isogeny search cost is `modeled` or `not_evaluated`.

**C. Genus-2/3 covers.** Binary: the same-field genus-3, degree-3 cover
(`binary_ecc::cover_transfer::OrdinaryBinaryCover`), executed on the generator. Prime: the
same-field genus-2, degree-2 cover `v^2 = F(u^2 + c)` of `docs/curves/COVERS.md` (construction
condition checked, Jacobian arithmetic not available in the repository, so `arithmetic`), plus a
genus sweep `g = 2..8` of the `p^(2-2/g)` pricing. Rational 2-torsion data (number of roots of the
cubic over `F_p`; the point `(0, sqrt b)` over `F_(2^n)`) is recorded as the input to
(2,2)-correspondence (gluing) candidates; gluing itself is `not_evaluated`.

**D. Pairing transfer (MOV/Frey-Ruck).** Embedding degree `k = ord_r(q)`: exact when `r - 1` factors
completely within the factoring budget and the committed factor hints, otherwise a verified lower
bound. Cost of index calculus in `F_(q^k)` by the `L[1/3, c]` formula with `o(1)` dropped, `c` the
smallest published constant (a deliberately attacker-favourable lower bound), against the baseline.
No generic pairing is implemented in the repository (`mov_attack` ships `k = 2` supersingular only),
so the subgroup obligation is `arithmetic` (Frey-Ruck non-degeneracy) and the map is `not_evaluated`.

## 6. Cost accounting

`C_new = C_construction + C_transfer + C_destination + C_recovery + C_verification`, reported in four
columns required by the brief: **transfer** (map construction and per-point evaluation),
**relation collection**, **linear algebra**, **descent/back-mapping** (recovery and verification).
Each cell is labelled `measured`, `modeled`, `not_evaluated` or `not_applicable`, with its formula
and the constants it uses.

Constants this study adds (everything else is the frozen model):

| constant | value | use |
|:--|--:|:--|
| `LFS_C` | `(32/9)^(1/3) = 1.5262` | smallest published `L[1/3]` constant (FFS/SNFS family); an attacker-favourable lower bound for index calculus in `F_(q^k)` |
| `MILLER_MULTS_PER_BIT` | 12 | field multiplications in `F_(q^k)` per bit of `r` in one pairing evaluation; an order-of-magnitude placeholder that never decides a verdict because the discrete log in `F_(q^k)` dominates |
| `VELU_MULTS_PER_KERNEL_POINT` | 4 | field multiplications per kernel point in one Velu evaluation; placeholder, never decides a verdict because an isogeny leaves the destination solver unchanged |

Setup reuse: none is claimed. Every cost is cold, one target.

## 7. Verdict vocabulary and decision rules

Closed vocabulary: `class-weak`, `weak representative exists`, `source-transfer-weak`,
`implementation-weak`, `no weakness found in scope`. Instance errors are not verdicts: an
unverifiable curve, generator or subgroup order aborts with `INVALID_INSTANCE`. Individual checks
inside a certificate use `PASS`, `FAIL`, `NOT_RUN`, `INDETERMINATE`, `NOT_APPLICABLE`
(`audit-curve` skill).

A candidate certificate receives a weakness verdict only if all three hold:

1. its subgroup-preservation status is `verified` (executed on the actual generator) or `arithmetic`
   with every antecedent of the argument checked,
2. the destination solver and all four cost phases are priced and the total is below the baseline,
3. the map is explicit (executed or constructible by code in the repository) rather than
   existence-only.

Mapping: a pairing candidate meeting 1-3 with exact small `k` is `class-weak` (embedding degree is
an isogeny invariant); a direct explicit transfer from the source meeting 1-3 is
`source-transfer-weak`; an explicit isogeny to a representative whose attack meets 1-3 is
`weak representative exists`; an exploitable flaw in an exposed implementation is
`implementation-weak` (this tool audits transfers only and never assigns it). Everything else,
including a formula-level lead below the baseline whose map is not constructed (`lead_class` =
`formula_advantage` or `formula_marginal`), is `no weakness found in scope` with the lead and its
open obligations recorded. A lead is not a weakness.

A curve's verdict is the strongest verdict over its candidates. Absolute-size findings
(for example the repository's separate `sect113r1_audit` result that generic rho on a 112-bit
subgroup is below an 80-bit threshold) are out of scope of this tool and are cited, not recomputed.

## 8. Controls

- Positive: F10 (synthetic formula advantage), F6 (a surviving subfield subgroup), the algebraic
  `Z/15` maps, the supersingular toy pairing curve with `k = 2`, BLS12-381 with `k = 12`.
- Negative: F11 (kernel meeting `<G>`; non-homomorphism), the Koblitz trace killing `<G>`, tampered
  certificates rejected by `check`.
- Transported-data control: a planted target `Q = [d]G` (seeded `d`) is carried by every executed
  map and the relation `phi(Q) = [d] phi(G)` is verified.
- Coordinate-isomorphism control: not run; the study does not use a coordinate change, and says so.

Unit tests (cargo): subgroup-preservation certificate passes on a correct map and fails on a map
whose kernel meets `<G>`; magic number and genus on two hand-checked examples (`F_4`, `b = z`: `m = 2`,
genus 2, type II; `F_8`, `b = z`: `m = 3`, genus 3, type I); embedding degree on a pairing-friendly
toy curve and on BLS12-381; verdict and lead classification on synthetic inputs.

## 9. Outputs

`certificates/<curve>/<candidate>.json` (schema `transfer-audit/certificate/v1`, one per candidate),
`certificates/<curve>/summary.json`, `certificates/index.json` (with SHA-256 of every file),
`run-log.json` (commands, timings; excluded from the byte-reproducibility check), `RESULTS.md`
(table: curve x transfer type x verdict x cost ratio, with every statement labelled proposed,
derived, executed or measured), `cost_ratio.svg` and `pipeline.svg` generated from the certificates
by the tool, and `cost_ratio.csv`. A PDF is built only if a non-Python renderer exists in the
environment; otherwise the blocker is stated in `RESULTS.md`.

## 10. Execution

```sh
export CARGO_TARGET_DIR=/home/user/cargo-target
cargo build --release -j2 --bin transfer-audit
cargo test  --release -j2 --bin transfer-audit
timeout 900 ./target/release/transfer-audit suite --out research/transfer_audit_20261011/certificates
timeout 300 ./target/release/transfer-audit check  --certs research/transfer_audit_20261011/certificates
timeout 300 ./target/release/transfer-audit report --certs research/transfer_audit_20261011/certificates \
    --out research/transfer_audit_20261011
```

## 11. Claim boundary

Cost rows are estimates from a frozen formula model unless labelled `measured`. The toy-size
executions establish that the tool's maps and certificates are correct on the instances run; they
say nothing about deployed security. No unknown-scalar recovery on any registered curve is attempted;
planted scalars are seeded known-answer fixtures. Bounded or zero-hit statistics never prove
absence. Separate claims, kept apart in every certificate: structure exists; map executable;
subgroup preserved; destination solver advantage; complete-cost advantage.

## Amendments

### Amendment 1 (2026-10-26, before the first run of the suite)

Made while writing the tool, before any certificate was generated. Nothing above is rewritten.

1. **Map basis in the verdict rule (section 7, condition 3).** The rule reads "executed or
   constructible by code in the repository". The pairing reduction is a theorem (Frey-Ruck
   non-degeneracy of the Tate pairing) with no pairing implementation in the repository, and the
   `audit-curve` skill allows "a certified structural condition with a cited reduction and total
   cost below the threshold". The tool therefore distinguishes three map bases: `Executed`,
   `Theorem` (a cited theorem whose antecedents the tool checked; used only for the pairing, and
   only when the embedding degree is exact) and `Existence`. Condition 3 is satisfied by
   `Executed` or `Theorem`. Genus covers over prime fields, GHS with `m >= 2` and isogeny moves
   without an exhibited destination are `Existence`.
2. **Lower-bound totals.** A route with a required phase that nothing in the repository can price
   (GHS map construction, an isogeny walk to an unidentified representative, an unknown exact
   embedding degree) carries `total_is_lower_bound`. Such a total can show a route is too
   expensive; it can never support a weakness verdict.
3. **Out-of-scope certificate for prime degree.** For prime `n` the tool emits an extra candidate
   `ghs:proper_subfield` (construction status `out of scope`) holding the divisor list of `n`, the
   magic numbers achievable by any curve over the field for the tower `F_2 <= F_(2^n)`
   (`ball::achievable_magic_numbers`) and the emptiness of the window 2..=6. The candidate `ghs:l=1`
   (descent to `F_2` itself) is still enumerated and priced.
4. **Destination class of the trace route.** The trace route's destination is `E(F_(2^l))`, priced as
   rho on that curve with its own equivalence class (`2 l / d`), so the subfield-subgroup control is
   not charged against a destination weaker than the one it reaches.
5. **No measured cost row.** `measured` stays in the cost vocabulary, but no cost cell of any
   certificate is measured; the only measured quantity is the F9 control, a separate record that
   `check` excludes because its group-operation equivalents include wall-clock calibration.
6. **F2 scope.** The cross-check against `docs/curves/jacobian-transfers.json` applies to GHS rows
   with `m >= 2` and to the same-field cover rows. Trace rows (`m = 1`) are priced against the
   destination's own class and are not comparable to the repository's unaccelerated row.

### Amendment 2 (2026-10-26, after a development run in a scratch directory, before the committed run)

Recorded so the order of events is visible. The suite and the F9 control were first executed in a
scratch directory while the tool was being debugged; the committed run follows these changes.

1. **F9 was looked at before the committed run.** The scratch run of the protocol's plan (22 s)
   gave fitted exponents 0.857 (genus 2) and 0.967 (genus 3), both below the pre-registered
   threshold 1, with every index-calculus run above the modelled source rho. The threshold is not
   changed: F9(a) is reported as failed if the committed run also fails it. A longer plan
   (genus 2: p up to 401; genus 3: p up to 151) is run as a separate, labelled post-hoc record,
   `controls/hec_ic_control_extended.json`, and does not replace the F9 result.
2. **F13, end-to-end positive controls on registered curves.** Two pairing-friendly curves are
   audited by the same code: `mnt/mnt1` (embedding degree 3, an MNT example whose intended attack
   is the pairing; registry adoption status `not_asserted`) and BLS12-381 (embedding degree 12).
   Expectation fixed here, before the committed run: `mnt/mnt1` is `class-weak` under the
   conservative end of the pairing price band; BLS12-381 is `no weakness found in scope` with a
   marginal lead (the band straddles rho). The live `curve_traits::compute` of `mnt/mnt1` takes about
   six minutes in the repository's code, so its trait record is read from the committed
   `docs/curves/traits.json` (order and trace checked against the instance) and the certificate
   says so.
3. **Pairing price band.** The index-calculus price in `F_(q^k)` is a band: attacker-favourable
   (`c = (32/9)^(1/3)`, `o(1)` dropped, unweighted) to conservative (`c = (64/9)^(1/3)`, weighted by
   the field-multiplication cost). The cost table carries the conservative end, which alone may
   support a weakness verdict; the favourable end only excludes. A band that straddles the matched
   rho is a marginal lead with verdict `no weakness found in scope`. A check records whether one
   element of `F_(q^k)` fits in 1 TiB; above that the pairing cannot be evaluated and the price is
   a formal extrapolation.
4. **Lead class against the matched baseline.** The lead class uses the advantage over the matched
   rho baseline (with the curve's equivalence class), not over the repository's unaccelerated
   reference; the latter stays in the certificate as `repo_ratio_log2` for the F2 cross-check.
5. **`carried`.** Each certificate records whether the subgroup is, or may be, carried. A route that
   does not carry it (refuted, obstructed, not applicable) is priced for the record only; the report
   shows no ratio for it, because the route solves a different problem.
6. **Tool changes found by running.** ECC2K-130 has no generator in `docs/curves/registry.json`; the
   tool attaches the repository's Certicom challenge parameters after checking the field and both
   coefficients equal the registry model. The explicit sect113r1 degree-5 rows are priced as an
   identity path (same order, same class), not as a Jacobian route.


### Amendment 3 (2026-10-26, after an independent review of the merged tool, PR #2594)

Made after the committed run, in response to review findings that were checked against the code
before anything was changed. Nothing above is rewritten. The certificates are regenerated with the
tool's own `suite` command as schema `transfer-audit/certificate/v2` (tool `transfer-audit/2`); the
F9 records and `run-log.json` of the first run are kept as they were. [RESULTS.md](RESULTS.md)
section 10 lists each change with the effect on the recorded numbers.

*Note added 2026-10-27, after review of PR #2658.* "Nothing above is rewritten" held for this file but
not for the certificates: the v2 regeneration wrote over the v1 certificates, index and report files at
their original paths, which removed the first run's committed set from the tree. That conflicts with
AGENTS.md ("Do not rewrite frozen evidence"). The v1 set is restored byte for byte in
[`v1/`](v1/README.md) from commit `3f31162cd`; the files outside `v1/` remain v2.

1. **Baseline (section 4).** The model's `sqrt(pi r / 4)` is rho *with* negation (`c = 2`); section 4
   divided it by `sqrt(c)` with `c >= 2`, counting negation twice and putting every baseline 0.5 bit
   too low. The baseline is `sqrt(pi r / (2c))`: a class of `c` elements (negation included) enters as
   `sqrt(c / 2)`. The trace-route destination uses the same function. The certificate key
   `baseline.log2_total_without_automorphisms` is renamed `log2_total_negation_only`, which is what it
   was. Every baseline rises by 0.5 bit.
2. **Cover image class (section 6).** Rho on the Jacobian image was priced with negation only while
   the baseline of a Koblitz-type curve had the Frobenius class `c = 2n/d`, overstating Koblitz cover
   ratios by `0.5 log2(n/d)` bits. For the binary cover, which is defined over the subfield `F_(2^d)`
   that gives `E` its class (Frobenius commutes with the pullback), and as a best case for the genus
   sweep, rho on the image now gets the baseline's class. Prime covers keep negation only (an
   automorphism of `E` of order 3 or 4 does not lift to `v^2 = F(u^2 + c)`). The certificate records
   the total under the frozen model alone (`costs.log2_total_frozen_model`), and the F2 cross-check
   with `docs/curves/jacobian-transfers.json` compares that, so F2 stays a wiring check of the frozen
   formulas and says so.
3. **Margin (section 7, condition 2).** "The total is below the baseline" is read as "below the
   baseline by at least `CostModel::advantage_margin_bits` (8 bits)", the threshold the lead class
   already used. The prices are formulas with `o(1)` dropped ("can move it by many bits"), and the
   sign of a three-bit difference decided a weakness verdict in the first run (BN318). A price below
   the baseline by less than the margin is a `formula_marginal` lead and the verdict stays
   `no weakness found in scope`. The vocabulary is unchanged. The verdict explanation says it is
   relative to rho on `<G>`, not a statement of absolute security.
4. **Calibration (section 8, controls).** Boundary controls are added on both sides of both lines
   that decided a verdict: BN254, BN318, BLS12-638, BN350 and BN446 (the registry has no BN462).
   `transfer-audit sweep` audits every prime-field registry curve with a generator and embedding
   degree at most 64, and a unit test pins the measured set of `class-weak` curves (13 of 29).
5. **Obligation `computational_advantage` and the `evaluation` field.** `refuted` is kept only for a
   carried route priced as a route whose total (a lower bound where phases are unpriced) is not below
   the baseline. `not_applicable` when the route does not carry `<G>` or does not exist; `unknown`
   for an identity-path price (class invariants, the explicit degree-5 replay), a formula sweep or
   heuristic, an unconstructed map whose priced part is below the baseline, a price inside the margin
   and a price band that straddles the baseline. Every certificate also carries
   `evaluation: {status, reason}` with `status` one of `evaluated`, `excluded`, `not_evaluable`, so a
   reader can tell a negative result from a route the tool could not decide; this is a field, not a
   verdict label.
6. **F10 through the decision path.** The synthetic control builds a candidate and is judged by the
   same function as every certificate (`ctx::judge`), with a counterfactual that grants the map and
   the subgroup, instead of calling the lead classifier directly.
7. **Smaller corrections.** The subgroup checker no longer returns `verified` without a homomorphism
   test and treats a map evaluation error as `not_evaluated`, not `refuted`; the pairing candidate
   checks `r` does not divide `q` before computing the embedding degree; the `End(E)` check passes
   only for status `proved`; the pairing price band that straddles rho is no longer recorded as a
   refuted advantage.
8. **Not a change, a critique.** The F9(a) threshold "fitted exponent `>= 1`" was a design flaw at
   genus 2 (the model total is exactly `6q`, so any additive overhead puts the finite-range slope
   below 1). The recorded F9 result is unchanged; the note is in RESULTS.md section 6.
9. **Section 5D as written.** Amendment 2 moved the pairing verdict to the conservative end of the
   price band after a scratch run. The rule of section 5D (attacker-favourable end against the
   baseline) would have made BLS12-381 `class-weak`; RESULTS.md says so.
