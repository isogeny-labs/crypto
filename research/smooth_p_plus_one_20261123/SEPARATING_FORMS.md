# Conservative RUR acceptance and fixed separating forms

2026-11-24. New qualification after the two retained failing invocations.
This is a local development protocol, not a cost benchmark. [pcc:unverified]

## Evidence motivating the change

The first cofactor-msolve F127 panel returned nine correct rows on its first
target, then seven versus ten on (116,69), and exited 1. A separately declared
diagnostic returned eight versus ten. That captured RUR reports quotient
dimension ten but has a degree-nine eliminant and an automatically added
random linear form. All records remain unchanged; eleven original planned
queries were not executed. [pcc:unverified]

A single-valued parametrization using a degree-nine polynomial cannot encode
ten distinct points. A finite RUR marker is therefore insufficient for this
adapter's completeness contract. Before extraction, require the actual
eliminant degree to equal the solver's reported quotient dimension. Reject
missing or inconsistent metadata. This is conservative: a complete reduced
description of a nonradical ideal may also be rejected. It is a necessary
acceptance check, not a signed Gröbner-basis certificate. [pcc:unverified]

## Fixed, bounded parameter choices

For the cofactor adapter, append a variable A and the equation

\[
A=b+jc+j^2d,\qquad j=0,1,\ldots,15,
\]

where native \((b,c,d)\) are the existing parser's `(t0,a1,a0)`.
This adds a uniquely determined coordinate and preserves the original
solution set. It does not change the original cofactor equations. There is
no assertion that one of these sixteen forms always separates a given set.

Pass `-c 0` to disable msolve's automatic variable-order/random-linear-form
genericity repair. The installed solver's separate two-point polynomial
controls show its precise contract: nonseparating input can return exit zero,
an empty output file, and the explicit genericity-failure message; adding
`A=x` returns a degree-two parametrization. Retain those fixture inputs,
stdout, stderr, RUR files and exits. [pcc:unverified]

Advance to the next fixed form only after that exact exit-zero/empty-output
genericity message, or an eliminant/quotient-dimension mismatch. An unrelated
solver failure, invalid RUR, failed signed replay or deadline stops the query.
The original equation preparation, all form attempts, extraction, replay and
record assembly share one absolute three-second deadline. No attempt gets a
fresh budget. A partial set is never combined with another attempt or accepted.

Each attempt records its form, linear equation, status, reason and RUR byte
count. Store the shared exact input prefix once; each actual input is that
prefix, then `,\n`, the recorded linear equation, then `\n`. Raw RUR capture
has a 262,144-byte cumulative cap per query, with explicit omissions thereafter.
The full-base controls below require all RURs needed for accepted results to
remain available; an omission stays a replay limitation.

## Frozen panels and gates

First run the unit suite, including rejection of the retained partial RUR,
the parameter-binding checks, and a fake solver that emits one known
genericity failure then an unrelated error. The latter must stop after two
attempts. Keep the earlier original-library/public-example regression receipts;
their source has not changed since the preceding qualified export.

Freeze a new executable and exact source before these calls:

1. F127 depth four: `cofactor-msolve` and `refined`, all four existing targets.
2. F8191 depth four: `cofactor-msolve`, all four existing targets.
3. F8191 depth five: `cofactor-msolve` and `refined`, all five existing targets.

The new Refined calls are regression checks for the shared conservative RUR
guard; they do not replace earlier Refined evidence. Each panel uses one
worker, 2 GiB whole-process address space, and a 60-second outer timeout with
five-second kill grace. The per-query budget remains three seconds and the
construction cap remains 20,000 terms. There are no panel retries or cap
increases. Compare complete signed sets with the full direct reference for
the cofactor arm and its regular-chart restriction for Refined. Preserve every
incomplete outcome and stop the process on any mismatch, after logging it.

If these gates succeed, the cofactor representation has a local exact-output
control through this external backend. No relative timing, native-F4 success,
full scalar recovery or admitted cold-cost advantage follows. The original
native allocation refusals and two external mismatches remain part of the
record. Independent signed verification and full-cost comparison remain open.
