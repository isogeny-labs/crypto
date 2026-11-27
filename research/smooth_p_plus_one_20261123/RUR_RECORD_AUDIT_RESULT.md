# Saved parametrizations fail different semantic checks

2026-11-25 · Offline native validation, `rur_record_audit_v1` · PR #5142

Both saved F127 cofactor parametrizations pass the existing degree/dimension
guard. The older output satisfies all eight original equations at every one
of its 13 rational roots, but the recorded fixed form at only one root. The
newer output satisfies neither check at any root. This local, unsigned result
is pinned by **PCC-20261125-30d751**. [pcc:unverified]

| Retained output | Rational roots | Equation-valid roots | Form-valid roots | Both |
|:--|--:|--:|--:|--:|
| Older, factorwise panel | 13 | 13 | 1 | 1 |
| Newer, liftable panel | 13 | 0 | 0 | 0 |

![Recorded counts, one saved output per panel; all F127 eliminant roots checked. This is a finite census, with no sampling interval or timing comparison. Source: evidence/rur_record_audit_v1/audit.jsonl.](rur-record-residuals.pdf){width=97%}

## Exact scope and observed counterexamples

The fixture is \(E/\mathbb F_{127}:y^2=x^3+x+7\), of order 109, with target
\((41,95)\), depth 4 and the unchanged cofactor comparator. The original
serialized equations and fixed form are byte-identical between records;
their RUR bytes differ. Each declares quotient dimension 13 and eliminant
degree 13. The fixed form is \(A-t_0=0\). Coordinates use the documented
negative numerator divided by denominator, with the root as the final
coordinate. All recorded denominators at these roots are 1.

At parameter 10, the old decoded assignment, in order
\((t_0,a_1,a_0,w_0,w_1,w_2,w_3,A)\), is
\((113,91,12,99,123,12,124,10)\). All eight equation residuals are 0,
but the form residual is 24. Only parameter 87 satisfies that output's form.
The new assignment at 10 is \((12,62,67,12,119,85,12,10)\); its equation
residuals are \((4,36,126,71,126,13,86,68)\), and its form residual is 125.
Every root and residual is preserved in the raw audit. [pcc:unverified]

The protocol predicted that the old output would satisfy both checks; that
prediction failed. Its previously checked 13 signed rows remain a distinct
observation: a correct extracted answer did not certify the full recorded
parametrization. For the newer record, failure is already present in the
decoded coordinates, before signed-relation filtering. The audit does not
identify a solver, input transport or parsing implementation as the cause.

\newpage

## Evidence, execution and next check

The [frozen protocol](RUR_RECORD_AUDIT.md),
[Rust example](native/examples/rur-audit.rs) and four synthetic tests were
published before the actual audit. Source and before-call commits:

```text
source:   b05d4d85b0943feb1276fa3c61fd0769c50ded9b
identity: c7ffa2f548eb88c0d69ca9a82c374b9bfdc5321e
```

The separate before-call identity pins eight files, including both original
inputs and the executable. All eight pins remained unchanged.
The [reproduction guide](RUR_RECORD_AUDIT_REPRODUCE.md) gives exact commands.

| Requirement | Evidence and status |
|:--|:--|
| Preserve both original outputs and stopped panel | Complete locally; original JSONLs unchanged, missing suffix remains missing |
| Qualify the audit before actual inputs | Four synthetic tests and release build pass; PCC-20261125-a53126, unsigned |
| Evaluate every recorded coordinate and equation | Two records, all 26 rational roots, eight equations plus fixed form per root |
| Retain malformed or invalid outcomes | Zero parsing failures; invalid residuals emitted explicitly; audit exit 0, empty stderr |
| Change extraction acceptance or solver | Not attempted in this component |
| Identify implementation cause and qualify correction | Open; trace exact input bytes and invocation next |

Flow: two planned inputs, two attempted, two parsed and audited; no omitted
record, retry or solver call. The wrapper ran once with one worker, a 1 GiB
address-space cap and 30 seconds plus five seconds termination grace on the
shared Linux development host recorded in `postcall_identity.json`. No
performance isolation or wall-time inference is claimed. Exit 0 means the
descriptive audit completed, not that the parametrizations passed.

Synthetic controls include a two-root positive, a coordinate mutation that
preserves the degree guard, malformed degree metadata and repeated/unknown
input monomials. The auditor is separate from extraction but producer-local;
neither this result nor its claims have a signed independent verdict. It
checks rational roots of these retained outputs, not arbitrary inputs or
general algebraic-closure completeness. No new identity theorem is asserted.

The installed msolve 0.7.5 tutorial and
[upstream documentation](https://msolve.lip6.fr/downloads/msolve-tutorial.pdf)
specify the decoding convention. The
[Sage parser](https://github.com/sagemath/sage/blob/develop/src/sage/rings/polynomial/msolve.py)
uses the same convention and a small-characteristic degree guard; the
[msolve README](https://github.com/algebraic-solving/msolve/blob/master/README.md)
requires unique monomials in each input polynomial. Exact source blob IDs
and the unavailable KB queries are retained in the frozen protocol. These
references guide this audit; they do not establish the failure's cause.

Next, inspect exact solver input/argument handling against the recorded form,
then separately qualify any semantic acceptance check. No replacement solver
run, changed convention or correction is silently substituted here. The
[liftable-fiber result](LIFTABLE_FIBERS_RESULT.md), its retained comparator
mismatch and all prior comparisons remain historical evidence. Study figures,
the index-calculus scoreboard and progress timeline gain no performance
datum from this diagnostic and remain unchanged.

Git publication retains the data while the supported artifact upload/replay
for eleven earlier raw files remains unfinished. This diagnostic adds no
file above the artifact threshold. The private stream has 711 queued records;
no Cloudflare receiver is configured and no external delivery has occurred.
Canonical experiment export access remains unavailable; the accompanying
development-flow record is explicitly not a canonical export. This component
does not supply a full-cost recovery comparison.
