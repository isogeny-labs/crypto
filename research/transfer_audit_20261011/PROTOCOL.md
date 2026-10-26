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

(none yet)
