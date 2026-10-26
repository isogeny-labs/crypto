# `bitop/v1`: one unit of work for every ECDLP method

Every ECDLP cost in this repository, in `aburan28/cryptanalysis` and in cairn's
bound objectives is charged in one versioned accounting convention: the
**bit operation** (`bitop/v1`). Its primitive weights are fixed
gate-equivalent formulas. They define a reproducible accounting unit; they are
not physical gate counts, circuit lower bounds, or estimates of a particular
implementation's synthesized circuit. A method's cost is

```text
work = Σ count(primitive) · gates(primitive, width)
```

an exact integer under this convention, determined by the recorded counts,
widths, and these formulas. No host, compiler or run enters the pricing. The
implementation is `src/cryptanalysis/bitops.rs`.
`docs/units/bitop-v1.vectors.json` freezes the formula weights for every
primitive at a spread of widths. Any other implementation must reproduce every
row exactly. The implementation and vector field named `gates` is retained for
compatibility and means the convention's gate-equivalent weight.

## Why one unit

Before `bitop/v1` the repositories charged work in at least six units:

- group-addition equivalents, with non-group work converted by nanosecond
  ratios from one host;
- solver work priced by this host's wall time, or not at all;
- `full_prices.json` ratios;
- reference picoseconds (`cryptanalysis` ic-bench);
- Callgrind instructions;
- wall-clock online speedups.

You cannot divide a figure in one of these by a figure in another. An
exponent fitted in one is not an exponent in another.

The worst consequence was silent: work with no conversion ratio was charged
zero. Examples:

- linear algebra on any curve without a row in `docs/ic/calibration.json`;
- Buchberger, SAT and PDP solver work.

Every index-calculus total was therefore a lower bound, and the ratio of two
lower bounds bounds nothing.

## The conventions

Throughout, `b` is the bit length of `p`, `n` the degree of `F_{2^n}`, and
`w` the bit length of the subgroup order `r`.

### Integers

| primitive | model units | derivation |
|---|---|---|
| `w`-bit addition | `5w` | ripple carry, a full adder is 2 XOR + 2 AND + 1 OR |
| `w × w` product | `w² + 5w(w−1)` | `w²` partial-product ANDs, `w − 1` row additions |

### Prime field `F_p` (Montgomery representation)

| primitive | model units |
|---|---|
| `field_add` | `2·5b + 3b = 13b`: an addition, a conditional subtraction of `p`, and a `b`-bit select |
| `field_mul` | `2·(b² + 5b(b−1)) + 13b`: the product, the reduction's product, and the final subtraction |
| `field_sqr` | as `field_mul` |
| `field_inv` | `2b · 13b`: `2b` steps of the binary extended Euclid, each a subtraction, a comparison and a select |
| `field_sqrt` | `2b · field_mul`: exponentiation (`b` squarings, `b/2` products) plus a `b/2`-product allowance for the Tonelli–Shanks loop |
| `legendre` | as `field_inv` (the Jacobi algorithm is a gcd) |

### Binary field `F_{2^n}` (polynomial basis)

Reduction is by a pentanomial, i.e. four low terms. A trinomial is cheaper;
the convention takes the dearer modulus so that no curve is flattered by its
own.

| primitive | model units |
|---|---|
| `field_add` | `n` XORs |
| `field_mul` | `n² + (n−1)² + 4(n−1)`: schoolbook carry-less product plus reduction |
| `field_sqr` | `4(n−1)`: spreading the bits is wiring, then the reduction |
| `field_inv` | `2n · 5n`: polynomial extended Euclid, `2n` steps of a shift-XOR pair and a select |
| `as_solve` | `n² + n(n−1)`: half-trace as a precomputed `n × n` matrix over `F_2` times a vector |
| `canon` | `as_solve + 2n²`: normal-basis conversion, then the least of `n` rotations |

### `F_{p⁴}` (`ecbench::fp4`)

| primitive | model units |
|---|---|
| `field_mul`, `field_sqr` | `16 · field_mul(F_p)` (schoolbook) |
| `group_add`, `group_double` | `97 · field_mul(F_p)` (`ecbench::fp4`'s `C_ADD`, the extension inversion through the norm) |

### Group operations (affine, as every counted group in `ecbench` computes them)

| curve | addition | doubling |
|---|---|---|
| `y² = x³ + ax + b` over `F_p` | `I + 2M + S + 6A` | `I + 2M + 2S + 8A` |
| `y² + xy = x³ + ax² + b` over `F_{2^n}` | `I + 2M + S + 9A` | `I + 2M + 2S + 5A` |

A scalar multiplication counted as such (rather than as the additions and
doublings it performed) is charged `1.5·w` additions. That is the convention
of the strong rho reference, which is the only method that counts one this
way.

### Arithmetic modulo `r` and bit-packed work

| primitive | model units |
|---|---|
| `modr_mul` | `field_mul` of `F_p` at `w` bits |
| `modr_add` | `field_add` of `F_p` at `w` bits |
| `modr_inv` | `field_inv` of `F_p` at `w` bits (binary extended Euclid) |
| a multiply-subtract of elimination (`row_op`) | `modr_mul + modr_add` |
| `word_op` | 64: one 64-bit XOR, AND or compare of a bit-packed algorithm (F4 and Macaulay rows, Gray-code enumeration) |
| `hash_probe` | `4κ` with `κ` the bits of a field element: a fold-XOR hash, a key comparison and a select. Keys are field elements, already uniform, so no multiplier is needed. A table's real cost is memory, which is reported beside the unit and never folded in |

## What the unit leaves out, and how that is caught

The convention deliberately leaves out several things, which are reported
beside it and never folded into the model-unit total:

- **Memory.** Table sizes and peak resident memory.
- **Control.** Branches, loop overhead and allocation.
- **Uncounted work.** Anything a method does that no counter sees.

The **instruction audit** is a diagnostic for missing counters, not proof that
all work is counted or that the formula weights match an implementation's
circuit. `ecbench audit-ir` replays a record under Callgrind and compares each
phase's instructions per model unit with the reference walk's ratio. A phase
whose ratio is far above the reference walk's may be doing work its counters
miss, such as F4 symbolic preprocessing when only elimination XORs are
counted. A record that fails the audit is not admitted as a bound; passing it
does not validate the gate-equivalent formulas as physical gate counts.

Work that was uncounted when sessions were committed is counted in a phase's
`work_native` map, never in `native`, so committed records keep replaying
exactly. The code that counts an item of a family's uncounted list also writes
the marker `counted:<item>`; a record that carries the marker leaves the item
off its `known_uncounted` list, and a record without it (every record written
before the counter existed) keeps it.

An algebraic engine whose native unit counts one kind of work (the
elimination's word XORs, say) reports its whole run beside it, in
`SolverStats.extra`: `bitop_complete = 1` and the `bitop_word_ops`,
`bitop_hash_probes` and `bitop_field_*` counts of every primitive it
performed. The F4 family (`f4-f2`, `matrix-f4`, `matrix-f5`, `inherited-f4`,
`f6-ic`, and the `pdp3-*`, `pdp-split` and symmetrised oracles around them)
gathers those counts in `src/cryptanalysis/bitop_tally.rs`, one unit per
inner-loop iteration that touches a matrix word or a monomial mask: the
Macaulay and symbolic-preprocessing builds, monomial sorts (`k·⌈log₂ k⌉`
comparisons), hashing and column indexes, packing, pivot searches and lead
scans, read-back, the splitting tree's substitutions and scans, and the
`F_{2^n}` operations of the summation-polynomial build and of each lift.
Across calls the flag holds only while every call set it.

A record whose counters include one with no `bitop/v1` rule has no total. Its
`work.complete` is `false` and the unpriced counters are named. An unknown
cost is never written as zero.

### Self-check work (policy, 2026-10-21)

Every harness charges self-checks the same way, so their totals compare:

- **The domain is checked once, outside counted work.** Curve parameters,
  the generator and the target are validated when the instance is built. In
  ecbench this happens in `workload.rs` on scratch counters that are thrown
  away; it is the same for every arm.
- **The solution check is algorithm work, charged per target.** A method
  that verifies its own answer (rho's `[d]G = Q`, index calculus's `verify`
  phase) pays for it, because a real solver must.
- **Re-checking operands inside the arithmetic is not algorithm work.** Code
  that re-validates every input and output of a group operation (the
  hyperelliptic `checked_*` arithmetic re-runs `HyperellipticCurveP::validate`
  and `MumfordDivisorP::validate` on each call, 39–59% of all counted gates
  in `research/hec_bitop_20261020`) is a defensive check of the
  implementation, which a solver does not need. It is counted, so it can be
  seen, but outside the headline: a harness reports it as a separate
  validation subset, and the cost it compares is the total minus that
  subset (`algorithm_bitops`).

A harness that cannot separate operand re-checks from the algorithm's own
work says so and reports its total as including them.

## Relation to the unpriced-work band (AGENTS.md §5a)

§5a requires a comparison in group-addition equivalents to do two things:

- list each arm's unpriced counters;
- rank the arms only where the order holds with that work weighted at 0, 0.1
  and 1 GAE per operation.

A comparison in `bitop/v1` between two complete arms has no unpriced counters
for either side, so the band has nothing to weigh. The listing is
`work.unpriced` and `work.known_uncounted`, both empty. That leaves two
conditions for ranking in the unit:

- both arms' records are complete;
- both arms pass the instruction audit.

When either arm is incomplete, its `bitop/v1` total is a lower bound, as
§5a says of any partial price. Such an arm has no `bitop/v1` bound
(`bound fit --unit bitop/v1` refuses it), and its group-addition comparison
still goes through the band. For the same reason `--unit` and `--weight`
exclude each other.

## Changing the convention

A committed figure must reprice to the same integer forever, so these formulas
and their weights are frozen. A different convention gets a new name:

- `bitop/v2` for Karatsuba products;
- a trinomial-aware reduction;
- a memory-cost model.

Every record keeps its primitive counts, not only its total, so any later
version re-prices the whole history without re-running it.
