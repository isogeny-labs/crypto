# Instruction audit and reproduction check of the cover row's two arms

2026-11-07. Machine-readable summary: [`AUDIT_IR.json`](AUDIT_IR.json)
(`ic-acct.audit-ir/v1`). Rows:

- reproduction of committed rows: [`audit/reproduction.jsonl`](audit/reproduction.jsonl);
- Callgrind audits: [`audit/rows/`](audit/rows/).

Figure: [`audit/figures/ir_per_gate.svg`](audit/figures/ir_per_gate.svg)
(PNG beside it, drawn by `audit/figures/plots.py` from those files only).

This is the audit `docs/units/BITOPS.md` requires before a count is admitted
as a `bitop/v1` bound. README.md recorded that none had been run. It is
**not** a `VALIDATION.json`, and it admits no leaderboard or `benchmarks/`
row.

## How the arms were audited

Both arms are custom examples (`examples/jv_cover_ic_vs_rho_bitops.rs`), not
ecbench sessions, so `ecbench audit-ir` cannot replay them. This branch adds
the same bracketing to the example:

- `--callgrind 1` dumps a Callgrind interval at each arm's start and end
  (`jv_arm_begin` / `jv_arm_end`);
- `jv_bitops::set_callgrind_phases` also dumps at every phase change
  (`jv_phase_<phase>`);
- `examples/jv_cover_audit_ir.rs` sums the parts by phase, divides by the
  row's gates and checks the re-executed counts against a reference row.

The hook is off by default, and a counted run without it is unchanged.

## 1. Reproduction of the committed rows

| arm | rows re-executed | code | reproduce total | reproduce every phase's primitive counts |
|:--|--:|:--|:--|:--|
| IC, `p = 1009` and `1511`, all seeds and moves | 16 of 16 | study commit `b4eb521af1` | **16/16** | **16/16** |
| IC, `p = 1009` (seed 4/moves 64, seed 1/moves 8) | 2 | `main` `a2be660914` | 0/2 | 0/2: only `descent.field_{add,mul,inv}` differ |
| rho (R3 `p = 1009` s1 m8 r0; R0 `p = 1009` s4 m8 r0; R3 `p = 1511` s4 m64 r0) | 3 of 48 | `main` `a2be660914` | **3/3** | **3/3** |

- **IC at its own commit.** At the commit that committed the study, every IC
  row reproduces exactly: the total and all 9 phases × 11 primitives. The A/A
  row (`raw/ic_aa_1511.jsonl`) is the same instance as `ic_1511` seed 1,
  moves 8, and reproduces with it.
- **IC on `main`.** The descent's field counts fall:
  - `field_add` 4,114,807,962 → 3,295,309,128 (seed 4, moves 64);
  - `field_add` 1,812,071,254 → 1,451,949,809 (seed 1, moves 8).

  The totals fall 13.0% (8,423,064,020,276 → 7,328,187,104,406) and 8.7%
  (5,548,562,308,194 → 5,067,415,823,734). Every other phase reproduces. The
  descent code changed after the study, so the committed counts are
  reproducible at their commit, not at `main`. Candidate commits are
  `30be3440dd` (arity-4 decompositions), `960f2f4e4a` (four-large-prime path)
  and `73582ce306` (sieve leads); the change was not bisected.
- **Rho on `main`.** Rho reproduces exactly. The other 45 rho rows were not
  re-executed: each takes 2–20 minutes on this host.

## 2. Instructions per counted gate (Callgrind)

The committed rows are out of Callgrind's reach here. They are 5·10¹²–1.5·10¹³
gates for IC and 7·10¹³–2·10¹⁴ for rho, 2–5 minutes each natively, so hours to
days each under Callgrind. The audit therefore ran on the code of `main` plus
this branch, at the protocol's pilot size `p = 503` (seed 1, moves 8; rho R3,
run 0) and at `p = 251` for rho. These are the same code paths as the study at
smaller `p`, not the study's rows.

- **Reproduction under Callgrind.** Every audited run reproduces the counts
  of a native run of the same row exactly.
- **Determinism.** Two native runs of each `p = 251` row agree exactly.

valgrind-3.22.0, `--tool=callgrind --cache-sim=no --branch-sim=no --dump-instr=no`.

| arm | p | bitop/v1 | Ir (charged phases) | Ir / kbitop | counted operations | Ir / counted operation |
|:--|--:|--:|--:|--:|--:|--:|
| IC | 503 | 8.448·10¹² | 1.486·10¹² | **175.8** | 2.548·10¹⁰ | 58.3 |
| rho R3 | 503 | 7.078·10¹² | 6.915·10¹⁰ | **9.77** | 1.296·10¹⁰ | 5.34 |
| rho R3 | 251 | 2.141·10¹² | 2.604·10¹⁰ | **12.16** | 4.874·10⁹ | 5.34 |

"Ir (charged phases)" leaves out:

- the instance's construction inside the route (`jv_phase_instance`,
  1.451·10¹¹ Ir), which the tally also excludes;
- the harness (1.8·10⁷ Ir).

The IC arm's phases at `p = 503`:

| phase | bitop/v1 | Ir | Ir / kbitop | Ir / counted operation |
|:--|--:|--:|--:|--:|
| relations (sieve) | 6.770·10¹² | 1.435·10¹² | 212.0 | 66.0 |
| descent | 1.604·10¹² | 5.017·10¹⁰ | 31.3 | 13.4 |
| linear algebra | 7.264·10¹⁰ | 3.06·10⁸ | 4.2 | 69.2 |
| walk | 7.69·10⁸ | 8.41·10⁷ | 109.3 | 65.7 |
| model | 5.10·10⁸ | 5.65·10⁷ | 111.0 | 68.0 |
| factor base | 2.58·10⁸ | 2.43·10⁷ | 94.0 | 68.2 |
| verification | 6.79·10⁷ | 7.66·10⁶ | 112.7 | 65.3 |
| transport | 2.39·10⁶ | 2.65·10⁵ | 110.8 | 65.2 |

### Control: the same run with the counting hooks idle

`--count 0` runs the identical route with `jv_bitops` disabled. Each hook
then costs one relaxed load, and the phase dumps are kept. The gates are
taken from the counted twin row (`--gates-from-reference 1`). The control
verified on the challenge curve with the same legacy product count
(5,906,575,690), so it ran the same route.

| | counting on | counting off (control) | rho R3 p = 503 |
|:--|--:|--:|--:|
| Ir, charged phases | 1.486·10¹² | 6.649·10¹¹ | 6.915·10¹⁰ |
| Ir / kbitop | 175.8 | **78.7** | **9.77** |
| relations (sieve) Ir / kbitop | 212.0 | 91.0 | |
| descent Ir / kbitop | 31.3 | 30.5 | |
| walk, model, transport, verification Ir / kbitop | 109–113 | 41–44 | |
| linear algebra Ir / kbitop | 4.2 | 1.6 | |

- **What the hooks cost.** 8.21·10¹¹ Ir, 55% of the counted run's
  instructions, about 32 Ir per hooked operation.
- **What remains.** Without them, the IC arm still spends **8.1× rho's
  instructions per counted gate** at `p = 503`.
  - The sieve is at 91.0 Ir/kbitop, 9.3× rho: 28 Ir per counted operation
    against rho's 5.3.
  - The curve-arithmetic phases (walk, model, transport, verification) sit at
    about 4× rho, though they run the same kind of `F_p` arithmetic.

### Verdict: the IC arm is not admitted

- **Over the threshold, as counted.** The IC arm spends 18× rho's
  instructions per counted gate at `p = 503` (175.8 against 9.77). This is far
  over the 2× the `bitop_unit_20261016` panel admitted.
- **Part of it is the instrument.**
  - Every IC phase whose primitives are counted by a per-operation hook sits
    at 65–69 Ir per counted operation, whatever its code: walk, model,
    transport, verification, sieve and linear algebra.
  - The descent, whose F4 engine charges its operations in batches through
    `jv_bitops::charge`, sits at 13.4.
  - Rho, which counts natively and prices afterwards, sits at 5.3.
  - A constant cost per operation across unrelated code is the signature of
    the counting hooks' own instructions (the thread-local tally on every
    `mm`, `am`, `sm` and `inv_mod`). The control measured them at 55% of the
    counted run.
- **The control removes the hooks' share and the gap stays.** With the hooks
  idle (above), the IC arm is still 8.1× rho, over the 2× bar, so the IC
  counts stay unadmitted on the audit's own terms.
- **What the remaining gap might be.** Either:
  - work the counters miss, most of it in the sieve (91 Ir/kbitop, 28 Ir per
    counted operation); or
  - instructions per primitive that the convention leaves out on purpose
    (control, allocation, `%` reductions in generic code), which would also
    explain the curve-arithmetic phases at 4× rho.

  Telling them apart needs a symbolised profile. The release binary carries
  no symbols, so Callgrind attributed every part to `clone`. A
  `debug = "line-tables-only"` build and `callgrind_annotate` on the sieve's
  parts would do it; that is not done here.
- **Rho shows no sign of uncounted work.** It is the reference whose ratio
  sets the bar: 9.8–12.2 Ir/kbitop at `p = 251` and `503`, about 5.3 Ir per
  counted operation. The 3 committed rows re-executed reproduce exactly, but
  45 of its 48 committed rows were not re-executed, so its admission rests
  on that sample.
