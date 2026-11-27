# Canonical fixed forms and complete-equation residue checks

2026-11-25. Software qualification protocol; no new msolve panel or timing run.

## Scope and sources

Preserve the [parser diagnosis](MSOLVE_INPUT_AUDIT_RESULT.md), both saved RURs,
all previous panels and their missing suffixes. Correct both fixed-form
callers (cofactor comparator and full-cutoff/factorwise/liftable variants),
without changing their charts, three-second query deadline, 16-form family,
term/capture limits, relation replay or degree-equality rule. Refined and
single-system methods do not acquire this new fixed-form policy implicitly.

Sources checked (KB): status and searches for "rational univariate
representation verify polynomial substitution quotient algebra denominator"
and "finite field RUR semantic residual completeness guard" fail with wrapper
exit127; no KB MCP is loaded. Reuse the version-pinned msolve/Sage output
contracts in RUR_RECORD_AUDIT.md and MSOLVE_INPUT_AUDIT.md, and read the actual
callers, transport, extractor and native polynomial representation. This is
an implementation correction and standard polynomial-residue validation,
not a new index-calculus mechanism or a formal theorem.

## Frozen acceptance design

Share one serializer for \(A-t_0-ja_1-j^2a_0\), for j=0,...,15 and the two
existing prime fixtures127 and8191. Normalize coefficients and omit every
zero monomial. The actual recorded form equals the string sent to the solver.

After retaining raw solver output, each caller invokes one shared guard
before signed extraction. Validate canonical input tokens and RUR shape,
characteristic, distinct variable names, degrees, scaling and numerator
count. Require the existing eliminant-degree/quotient-dimension equality;
keep its original nonseparating error prefix so existing form advancement
remains unchanged. Require a monic nonconstant square-free eliminant Q and
a denominator D with gcd(D,Q)=1. Reject unsupported or malformed shapes.

Decode coordinates as -numerator/D, with the last variable equal to T.
In the ring Fp[T]/(Q), use numerator T*D for that last coordinate. For every
serialized equation of total degree d, substitute the numerator polynomials
and multiply each monomial by D^(d - its degree). Require the resulting
remainder to be zero. Check every coordinate, including auxiliary cofactors,
and the exact fixed form. The denominator-unit check makes this the standard
cleared-denominator residual test at all roots of Q. It does not establish
that a solver-supplied quotient dimension is correct or that no component
was omitted. The existing full-answer reference comparisons remain needed.

Keep the current `[-1]` empty-result contract, explicitly recorded as
`solver_empty_unchecked`, with zero equations checked. Do not relabel an
empty marker as a parametrization certificate or an independently established
UNSAT result. The positive guard reports equation/variable/degree counts and
scalar-operation count. Record its error or receipt in the attempt.

Use the caller's existing absolute deadline for parsing, polynomial operations
and acceptance. Fixed guard caps:262144 bytes per input/output,64 variables,
2048 eliminant degree,20000 input terms,16 total monomial degree and8000000
scalar multiply/add budget units. Count polynomial multiplication/reduction
work and check deadlines at bounded inner-loop boundaries. Cap or deadline
failure is incomplete, not an accepted prefix. Semantic failure stops the
attempt; it does not create a new form retry. Retain the bad output.

The directly attested `src/full_cutoff.rs` remains unchanged. Compile a new
`full_cutoff_current.rs` through an explicit path in main.rs. Preserve
`src/production.rs` and the pinned system/liftable source. The mutable
production_current.rs and main.rs may integrate the shared guard. Update
the source-layout note; preserve all old claim and ledger bytes.

## Qualification and publication

Run native synthetic tests for all fixed forms on both fixture primes;
positive residues, nonconstant unit denominator, invalid coordinate/form,
nonunit denominator, repeated eliminant, malformed names/degrees/scaling,
unknown/duplicate/zero input monomials, expired deadline and operation cap.
Use the saved old/new RURs as regression fixtures: the old output with its
effective parsed form should pass, the same output with intended canonical
form0 should fail the form, and the new output should fail an original
equation even with its effective form. These tests reuse recorded data and
invoke no scientific solver.

Stub-solver tests exercise both caller paths, exact zero-free input, retention
of rejected raw output, one-attempt semantic failure and explicit empty-marker
status. Run the relevant feature/default native suites and release build;
keep any failed test/build output before a corrected rerun. No full repository
test is repeated solely for unchanged dependencies; existing root-build limits
remain recorded. Verify historical source claims after integration.

Publish exact source/build/test commands and identities, unsigned claims,
source-linked report, graph/diagram and inspected PDF, plus reproduction and
checksums. This component adds no scientific solver panel or cost comparison.
A corrected-policy panel requires separate freezing and the normal execution,
checkpoint and resource gates; no retry replaces a prior failed panel.
