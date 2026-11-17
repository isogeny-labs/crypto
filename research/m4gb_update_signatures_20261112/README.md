# M4GB divisibility signatures in `update()` (2026-11-12)

Owner's ask (2026-11-12): "Speed it all up", after the Gröbner performance
levers note ([`docs/ic/GROEBNER_PERF_LEVERS_20261111.md`](../../docs/ic/GROEBNER_PERF_LEVERS_20261111.md))
and the persistent-server study
([`../m4gb_persistent_server_20261111/`](../m4gb_persistent_server_20261111/)).
This study measures the M4GB side. The in-house engines are a separate
change.

## Where the time went (Callgrind, before the change)

Captured 24-variable descent system (unsatisfiable, the common case), one
thread, patches 0001–0003, `-O3 -march=x86-64-v3` (Valgrind cannot decode
AVX-512), 2.15 billion instructions `[pcc:unverified]`:

| part | share |
|:--|--:|
| `update()` (Gebauer–Möller pair update), inclusive | 70 % |
| `divides` on integer-coded monomials | 25 % |
| decoding an integer monomial one variable at a time (`const_iterator::operator++`) | 16 % |
| GF(2) row additions (`add_to`) | 3.4 % |

M4GB stores monomials as their rank in degrevlex order. Every divisibility
test against a leading monomial or a pair's lcm decodes that rank variable by
variable, and `update()` runs such tests over every critical pair and every
basis element for every new basis element. Bit-packing the GF(2) rows, lever
4 of the note, would touch 3.4 % of this profile; it is not the lever here.

## What changed

Patch [`0004-divisibility-signatures-in-update.patch`](../../third_party/m4gb-patches/0004-divisibility-signatures-in-update.patch)
(switch `-DM4GB_SUPPORT_MASK=0` restores 0001–0003):

1. **Signature of a monomial**: `m1` has bit `v` set when variable `v`
   occurs, `m2` when it occurs with exponent 2; `small` says every exponent
   is at most 2 and `N <= 64`. Over GF(2) with the field equations every
   monomial is small.
2. **Exact divisibility from signatures**: `a | b` is
   `(a.m1 & ~b.m1) == 0 && (a.m2 & ~b.m2) == 0` when both are small (exponents
   in {0, 1, 2}); otherwise `m1` is only a necessary condition (valid for any
   `N`, bits fold modulo 64) and the exponent walk decides.
3. **Signatures of integer monomials from a cache**: a thread-local
   direct-mapped table of 16,384 entries keyed by the rank.
4. **lcm from signatures**: `set_lcm` writes the union of the masks when both
   operands are small, without decoding either.
5. **In `update(p)`**: the new leading monomial's signature is computed once;
   each critical pair stores its lcm's signature at construction; the pair
   pruning test, the "lcm(pLM, p_i) != lcm" tests and the minimal-lcm scan use
   the stored signatures.

Every result is decided exactly; nothing is approximated. Instruction counts
on the two captured 24-variable systems (Callgrind, same build flags)
`[pcc:unverified]`:

| step | unsatisfiable system | satisfiable system |
|:--|--:|--:|
| 0001–0003 | 2.15 G | 3.26 G |
| + `m1` prefilter, cached | 1.62 G | n/m |
| + exact two-mask test | 1.41 G | 2.03 G |
| + lcm from masks, signature of pLM once | 1.28 G | 1.80 G |
| + plain thread-local table | 1.24 G | 1.76 G |
| + signatures stored in critical pairs (0004) | 1.01 G (0.47×) | 1.39 G (0.43×) |

After 0004 no function is above 7.5 % of the profile (`to_index` 7.3 %,
`update` 6.6 %, `add_to` 6.4 %, map iteration 5.1 %).

Correctness before the run: on all 15 GF(2), GF(31) and GF(65521) inputs at
hand (captured descent systems and the kernel study's inputs) the patched
build and the switched-off build give the same exit code and byte-identical
output files; the one 30-variable input timed out in both.

## Protocol

1. `capture.sh` (with the 0001–0003 binaries): `ic descent --solver m4gb-f2
   --solver fes-f2` on every target of 15 cells (Koblitz K and random R;
   m = 2 at 12 to 24 unknowns, m = 3 at 12 unknowns), 6 seeded targets each,
   through wrapper binaries that copy every input file: 90 systems.
2. `ab_corpus.py` under `tools/benchrec.py run`: three arms through `--serve`
   (one process per arm and binary width, warmed): `old` (0001–0003), `new`
   (0001–0004), `oldAA` (a second server of the old build, A/A control).
   Every system × 5 repeats, the arm order rotating with the repeat and the
   system; wall from request to reply; reply word and output SHA-256
   compared across arms.
3. `summarize.py`, `make_report.sh`.

Binaries: `scripts/m4gb/build.sh n16_gf2 n20_gf2 n24_gf2 n28_gf2 n32_gf2`
from the pinned commit, once with patches 0001–0003 and once with 0001–0004.
Local container, one thread, not isolated; a stage diagnostic, no
leaderboard row.

## Pre-stated expectation and falsifier

Expectation: the paired ratio new/old falls with the number of unknowns,
about 0.75× at 16 to 20 unknowns and about 0.6× at 24, near 0.9× on the
degree-6 m = 3 systems (their time is less in `update()`), with the A/A
ratio within a few percent of 1 on every cell and no disagreement.
Falsifier: any disagreement in reply or output bytes; or a cell at 20 or
more unknowns whose median paired ratio is not below the A/A band.

## Results `[pcc:unverified]`

Run 2026-11-12 05:18 to 05:40 Z, record
`benchmarks/records/20261112T0517Z-m4gb-update-signatures.json` (measured;
raw `corpus_ab.json`). Full table `RESULTS.md`, graph `results.svg/png`,
`REPORT.pdf`. 90 systems × 5 repeats × 3 arms.

| unknowns (m) | cells | `old` ms (median of cell medians) | `new` ms | paired `new`/`old`, cell medians | systems `new` below `old` | A/A `oldAA`/`old`, cell medians |
|--:|:--|--:|--:|--:|--:|--:|
| 12 (2) | K, R | 3.5 / 3.4 | 3.7 / 3.1 | 0.97 / 0.91 | 8 / 12 | 0.99 / 1.02 |
| 14 (2) | K, R | 8.7 / 15.4 | 6.0 / 10.1 | 0.75 / 0.66 | 11 / 12 | 0.97 / 0.93 |
| 16 (2) | K, R | 15.5 / 29.3 | 10.5 / 18.6 | 0.68 / 0.64 | 12 / 12 | 1.00 / 1.10 |
| 18 (2) | K, R | 32.0 / 36.4 | 20.6 / 27.7 | 0.65 / 0.73 | 12 / 12 | 1.02 / 1.11 |
| 20 (2) | K, R | 41.2 / 52.0 | 31.6 / 37.3 | 0.72 / 0.74 | 12 / 12 | 0.98 / 1.01 |
| 22 (2) | R | 231.5 | 129.8 | 0.57 | 6 / 6 | 0.96 |
| 24 (2) | K, R | 430.3 / 334.3 | 235.5 / 203.3 | 0.61 / 0.64 | 12 / 12 | 1.03 / 1.01 |
| 12 (3), degree 6 | K, R | 7,000 / 7,199 | 6,231 / 6,494 | 0.88 / 0.89 | 12 / 12 | 1.00 / 1.01 |

- **0 disagreements** in reply word or output bytes over 1,350 paired
  requests.
- **From 14 unknowns up the patched build is 0.57 to 0.75× of the old one
  per call**, below the old build on 65 of the 66 systems from 14 to 24
  unknowns, while the A/A control's cell medians stay at 0.93 to 1.11×. At
  22 and 24 unknowns the ratio is 0.57 to 0.64×, matching the 0.43 to 0.47×
  instruction ratio of the two profiled systems less the parts of the solve
  that the patch does not touch.
- **At 12 unknowns there is no effect** (0.91 and 0.97×, inside the A/A
  spread): the solve is about 3 ms and `update()` is a small share of it.
- **The degree-6 systems are 0.88 to 0.89×** on all 12 systems; their time
  is mostly outside `update()`.

The pre-stated expectation held, slightly better than predicted from 14 to
20 unknowns (0.64 to 0.75× against an expected 0.75×). The falsifier did not
fire: no disagreement, and every cell from 20 unknowns up has a paired
median of 0.57 to 0.74×, below its A/A band. Local container, one thread,
not isolated; a stage diagnostic, no leaderboard row. The ranking against
the in-house `inherited-f4` engine is not changed by this patch alone
(`inherited-f4` was 6 to 28× below M4GB per call in the persistent-server
study).

## Sources checked

KB (`research-kb`) unavailable in this session. M4GB `src/m4gb.hpp`
(`update()`, `crit_pair_t`), `lib/monomial_base.hpp`
(`monomial_static_t::divides`, `set_lcm`), `lib/monomial_degrevlex.hpp`
(integer codec, `const_iterator`); `research/m4gb_persistent_server_20261111/README.md`;
`docs/ic/GROEBNER_PERF_LEVERS_20261111.md`.
