# Independent review of the R17 production mul5 integration

Status: arithmetic and the public q=223 deterministic control passed at the
scope reviewed. The integration is not eligible for modeled comparison or
promotion because its analyzer contract is incomplete. All observations are
producer-side `[pcc:unverified]`.

This review did not edit `strong_rho.rs` or `main.rs`. The exact audited hashes,
commands, exit statuses, and reduced outputs are retained in
[`r17_production_review.txt`](../receipts/r17_production_review.txt).

## Disposition by obligation

| Obligation | Disposition | Evidence or defect |
|:--|:--|:--|
| Evaluation/interpolation formula | passes deterministic review | Production operations implement the frozen values at `0,1,-1,2,∞`, interpolation, and `z^3=c` reduction; 20,000 production pairs and the separate exhaustive oracle agreed. |
| Exact-division preconditions | passes for dispatch, with coverage gap | `mul_raw` selects mul5 only when `q%2!=0 && q%3!=0`; otherwise it falls back to Karatsuba. Production tests do not exhaust the fallback domain. |
| Overflow bounds | passes under the existing `q<2^31` constructor bound | Every canonical sum is below `2^32`; `x+2q` in `fdiv3` is below `3*2^31`; all fit in `u64`. |
| Default kernel compatibility | semantic q=223 control passes; byte compatibility changes | `FastF3::from_instance`, `StrongOpts::for_instance`, and unsuffixed CLI arms retain mul6. The default record now adds `field_kernel` and zero `fq_small_div` fields, so exact historical JSON bytes/source hashes change. |
| Per-call field counters | passes deterministic review | Each production mul5 call records five `fq_mul`, four `fq_small_div`, one `f3_mul`, and no field inverse. The q=223 phase delta is one `fq_mul` per `f3_mul`. |
| Analyzer authority and pricing | **blocking defect** | `analyze.py` and `test_analyze.py` contain no `fq_small_div`; `strong_components` accepts records that omit it, `Components` cannot retain it, and modeled cost silently assigns it zero. |
| CLI arm contract | **defect** | `-mul5` is accepted, but the required explicit `-mul6` arm is absent and `strong-b8-mul6` panics as an invalid width. Kernel state is carried as a Boolean rather than the specified enum. |
| Full correctness matrix | partial | Production tests cover 20,000 q=223 pairs and one width-8 planted trajectory. They do not satisfy the frozen exhaustive prime/c/boundary/algebra-law/multiwidth/DP-sequence matrix. |
| Count-free candidate | deterministic control passes | `fast-b8-mul5` returns the same q=223 aggregate trajectory and every field counter remains zero. Optimized-IR removal for this new kernel was not independently checked here. |
| Constant-time behavior | no constant-time contract established | `fdiv2` branches on parity and `fdiv3` takes one to three residue-dependent loop iterations; the kernel selector is also a runtime branch in every cubic multiplication. |

## Formula and domain audit

The production path computes:

```text
v0, v1, vm1, v2, vinf
even = (v1+vm1)/2
d2   = even-v0-vinf
odd  = (v1-vm1)/2
at2  = (v2-v0-4*d2-16*vinf)/2
d3   = (at2-odd)/3
d1   = odd-d3
```

and returns `(v0+c*d3, d1+c*vinf, d2)`. Multiplication by 4 and 16 is routed
through the existing small-constant addition-chain helper, so those operations
do not masquerade as general `fq_mul` entries. The formula matches the frozen
R1 contract `[pcc:unverified]`.

`fdiv2` and `fdiv3` receive canonical residues because every input is produced
by `fa` or `fs`. The outer dispatcher checks invertibility of two and three
before entering mul5. A direct caller cannot bypass it because `mul5_raw` is
private. Under the constructor's existing `1<q<2^31` condition, none of the
integer lifts can overflow `u64` `[pcc:unverified]`.

## Deterministic production control

The release selftest exited zero and reported equality between production
mul5, production Karatsuba, and the frozen field on 20,000 q=223 pairs. A
separate q=223 run compared:

```text
strong-b8
strong-b8-mul5
fast-b8
fast-b8-mul5
```

The planted scalar, success flag, steps, aggregate DP count, restarts,
lookahead count, cycles, cycle additions, and selected native counts were
identical. In each setup/walk/verify phase, the mul5 arm removed exactly one
`fq_mul` for every `f3_mul` and recorded four `fq_small_div` for every
`f3_mul`; both fast arms emitted zero field counters `[pcc:unverified]`.

The command emits development clocks and calibration records. They were
excluded from the review predicate and support no timing statement.

## Blocking analyzer defect

The producer exports `setup|walk|verify.fq_small_div`, but the consumer still
constructs each phase only from `fq_mul` and `fq_inv`. Consequently:

1. a mul5 record with the new work present is modeled as though it were free;
2. a record omitting `fq_small_div` is still accepted;
3. no test requires authoritative small-division fields; and
4. the frozen instruction to export this work as separately unpriced rather
   than silently zero-priced is not met.

This blocks any cost, ratio, or leaderboard use of the candidate. The repair
must add a separately retained small-division component, require the key in
every authoritative strong-rho phase, reject it for non-authoritative modeled
records, and make its price/sensitivity explicit. Folding it into `fq_add` or
silently choosing zero would change the frozen accounting contract
`[pcc:unverified]`.

## CLI and compatibility defects

The unsuffixed default remains Karatsuba and `-mul5` is unambiguous. However,
the frozen R1 interface requires explicit `-mul6` and `-mul5` diagnostic arms.
There is no `-mul6` suffix handler; the command `strong-b8-mul6` exits 101 with
“invalid batch width.” Contradictory combinations are rejected only as failed
width parsing, rather than by an explicit kernel-suffix validator. The Boolean
`use_mul5` also drifts from the specified `F3MulKernel` enum. None of these
changes corrupts the tested default, but they leave the required comparison
surface incomplete `[pcc:unverified]`.

Existing default consumers must also account for the additive JSON schema
change: even mul6 records now carry `field_kernel:"karatsuba6"` and three
zero-valued `fq_small_div` fields. The q=223 mathematical and legacy field
counters matched the archived control, but byte-for-byte replay must bind the
new source/schema rather than reuse an old hash `[pcc:unverified]`.

## Correctness coverage still owed

Before deterministic acceptance, production—not only the standalone formula
prototype—still needs:

- exhaustive equality across tractable primes through 257 and multiple `c`,
  including reducible quotient rings;
- explicit boundary operands and zero/identity/commutativity/distributivity/
  square/inverse laws;
- mul5 planted-log controls at widths 1, 4, 8, 16, and 32;
- an exact DP-sequence or committed trajectory digest, not only aggregate DP
  and native counters;
- the preregistered `fq_add` delta/replay check; and
- optimized-IR confirmation that the new fast mul5 path removes all field
  counter mutations.

The earlier standalone R17 harness supplies an independent algebraic
falsifier over small and boundary moduli, but it does not replace tests that
exercise the integrated production functions `[pcc:unverified]`.

## Constant-time and code-generation note

The exact division helpers are input-dependent: `/2` selects on parity and
`/3` searches one to three lifts. The current research comparator has no
documented constant-time requirement, so this is not classified as a
correctness failure. It does mean constant-time behavior must not be claimed,
and the variable work plus the per-multiplication runtime kernel branch must
be included in isolated code-generation and wall-time review before promotion
`[pcc:unverified]`.

## Required disposition

Keep mul5 selectable and keep mul6 as default. Repair the analyzer and CLI,
complete the integrated correctness matrix, and rerun the deterministic
composition gate before requesting isolated timing. Until then the production
integration is implemented but unverified for promotion; no leaderboard row
is eligible.
