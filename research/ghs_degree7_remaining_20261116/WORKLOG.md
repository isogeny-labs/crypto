# Degree-seven remaining-shapes worklog

## 2026-11-15 22:41 America/Los_Angeles

### What was done

Read the newest dated shared session logs, prior degree-seven sparse,
linear-square and quadratic-square protocols, the trace-`+22` 24-cell result,
and Howe's original Theorem 4.17. Queried the shared research KB (revision
`9141e386254340a9bdba3036f2bb706a`, refreshed 14 November 2026) for
degree-seven cubic-square covers, trace-`+22` GHS, and Howe's bound. Wrote the
frozen protocol and cubic-square Sage source in this directory. No counted run
had started at this entry.

### Decisions made

The linear branch factor cannot be an odd pole of an even-degree denominator
in the normalized degree-seven case. The remaining denominator classes are
`B=H²` with monic cubic `H`, `B=Q_iQ_j L_1²` with linear `L_1`, and
`B=Q_iQ_jQ_k`; the first uses three quadratic equations in three field variables
and is the next executable screen.

### Open threads

Commit the frozen source, run both source-orbit representatives, audit any
solutions and failures, then move to the other two denominator classes.

### Errors

Conductor holds `docs/session-logs/2026-11-15.md` under T-6448, so this scoped
worklog records this task's decisions until that journal can be reconciled.

## 2026-11-15 22:44 America/Los_Angeles

### What was done

Committed frozen protocol and source at `1417546d4915`. First b=68 run took
4.78 process CPU seconds. It completed 232 zero-dimensional systems with 355
allowed cubic parameters, all failing the square identity, but 11 systems had
positive-dimensional ideals and were censored. Added an exact finite-field
recovery for those systems; source has not yet been rerun at this entry.

### Decisions made

The partial receipt cannot support a zero-hit statement. Re-run both source
orbits after committing the positive-dimensional recovery.

### Open threads

Complete and audit b=68 and b=84; then address other denominator classes.

### Errors

Initial solver logic treated positive-dimensional ideals as censored. The
amended bounded enumeration resolves them or records a solver error.

## 2026-11-15 22:50 America/Los_Angeles

### What was done

Started b=68 and b=84 finite-field recovery runs at source `79da85c9c7e2`.
Both remained active after several minutes. A process sample of b=68 showed
the cost concentrated in repeated NTL polynomial gcds in the univariate
fallback. Sent SIGINT to both processes; both exited 130 without a final JSON
receipt. Changed the fallback to take degree-at-most-two roots from the first
nonzero polynomial and verify all remaining equations.

### Decisions made

The two interrupted runs are incomplete evidence, and the edited source must
be committed before restarting both targets.

### Open threads

Commit, rerun, audit all cells and exact identities; continue the other shapes.

### Errors

Univariate gcd was needlessly expensive. Direct root filtering is exact here
because every specialized equation is quadratic or lower in `w`.

## 2026-11-15 22:56 America/Los_Angeles

### What was done

Stopped both second-amendment runs without a final JSON receipt after the
finite-field fallback expanded many parameter triples; process sampling showed
the candidate-list construction. A separate exact diagnostic checked all 243
b=68 coefficient ideals: 232 had dimension zero, ten dimension one, and one
dimension three. In each dimension-one ideal, the reduced Gröbner basis has
a monic relation `t³+c w³+d=0` with coefficients in `K`. In the dimension-three
case the CRT branch residue `h` is constant, so `H²h` has degree six.

### Decisions made

Use inverse Frobenius to solve `t` from `w` and then roots of quadratic
equations in `s` for every `w∈K`. This is exact for the checked relation
shape and bounded by 243 univariate systems per exceptional assignment.

### Open threads

Commit third amendment; finish b=68 and b=84, independently audit receipts.

### Errors

The direct finite-field fallback was exact but not computationally bounded
enough in the exceptional ideals. The interrupted files contain no results.

## 2026-11-15 22:58 America/Los_Angeles

### What was done

Committed the Frobenius reduction at `0c362e2ba465`. Both counted v3 runs
completed. b=68: 243 systems, 232 zero-dimensional, ten Frobenius-reduced,
one constant assignment, 2,785 parameter solutions, 355 degree-seven
coprime candidates, zero full identities, zero censored, 7.636379 CPU s.
b=84: corresponding counts 243, 232, ten, one, 2,774, 340, zero, zero,
7.837593 CPU s. The source SHA256 matches between both receipts. Wrote the
bounded result note.

### Decisions made

This closes the cubic-square denominator class for the fixed carrier and
the two target representatives. Continue to two quadratic branch poles plus
a linear square, then three quadratic branch poles.

### Open threads

Independent elimination replay; remaining two degree-seven pole classes;
explicit trace-+22 transfer if any map is found.

### Errors

None in the final v3 runs. Earlier interrupted receipts remain incomplete.

## 2026-11-15 23:00 America/Los_Angeles

### What was done

Committed final cubic-square receipts and bounded result at `10c907330308`.
Derived the two-pole linear-square identity by cancelling `S=Q_iQ_j` and
`H²=(x-c)²` from the cover equation. Wrote its frozen protocol and Sage
screen. The exact candidate count is 7,906,140 per target representative
after the necessary nonzero-square leading-coefficient filter.

### Decisions made

Use ten necessary square evaluations to keep full polynomial checks small;
all survivors still receive the complete identity check. Freeze and commit
this source before any counted run.

### Open threads

Run b=68 and b=84, audit all coverage counters, then address the three-pole
class.

### Errors

None in this preparation.

## 2026-11-15 23:03 America/Los_Angeles

### What was done

Committed the two-pole linear-square protocol and source at `7804e49e1678`.
Both counted runs completed. Each checked 7,906,140 logical candidates and
65,340 denominators. b=68 had 11,076 point-filter survivors and zero full
square identities in 27.232 CPU s; b=84 had 11,195 and zero in 27.648 CPU s.
The two receipts have the same source SHA256. Wrote the bounded result note.

### Decisions made

This closes the second of the three previously unsearched normalized
degree-seven denominator classes on the fixed carrier. The last class is
`B=Q_iQ_jQ_k`; its naive candidate count is 643,043,610 per source model
after the square-leading-coefficient filter. Use exact bitset intersections
of necessary point conditions to avoid per-candidate polynomial arithmetic.

### Open threads

Freeze and run the three-pole screen, then independently audit all three new
shape results and assess the fixed-carrier degree-seven bound.

### Errors

None in the two counted runs.

## 2026-11-15 23:05 America/Los_Angeles

### What was done

Committed the two-pole receipts and result at `143a03d63fe8`. Derived
the final branch-only denominator identity `M C²=A³+A²B+bB³`, with
`deg M=5` and `A=(Bh mod M)+M(λx²+μx+ν)`. Wrote and froze a three-pole
protocol and source. Its 643,043,610 logical candidates per source model
are represented by intersections of exact 243-bit sets for all `ν` values.

### Decisions made

The bitset representation is exact: at each evaluation point, translate the
allowed affine values of `ν` by `λz²+μz`; intersect sixteen necessary
conditions; then perform a full polynomial identity check on survivors.
Commit source before counted execution.

### Open threads

Run and audit both source representatives. If zero, combine all denominator
classes into a fixed-carrier degree-seven lower bound with stated scope.

### Errors

None in preparation.

## 2026-11-15 23:08 America/Los_Angeles

### What was done

Committed three-pole protocol/source at `652290a2d155`. Both counted runs
completed. Each covered 643,043,610 logical candidates, 90 assignments,
and 1,440 direct bitset controls. b=68 had 14,151 point-filter survivors,
zero exact squares, 22.407 CPU s; b=84 had 13,980, zero, 19.561 CPU s.
Both receipts bind to the same source SHA256. Wrote the result note.

### Decisions made

The three newly screened denominator classes are closed for the fixed
trace-`+22` carrier and two source representatives. Do not claim a complete
degree-seven exclusion yet: sparse and repeated-linear cases were screened
for trace `+26`, not this trace.

### Open threads

Adapt the already published sparse and linear-square screens to trace `+22`;
then combine all classes and audit the fixed-carrier bound.

### Errors

None in the three-pole counted runs.

## 2026-11-15 23:09 America/Los_Angeles

### What was done

Committed three-pole receipts and result at `ec293d780656`. Copied and
parameter-adapted the existing exact sparse and one-variable linear-square
sources from trace `+26` to this fixed trace-`+22` carrier. Wrote a frozen
completion protocol. No counted completion run had begun at this entry.

### Decisions made

Reuse the established CRT and coefficient-elimination algorithms, changing
only the exact source and carrier equations. This completes the shapes
needed before a full normalized degree-seven claim.

### Open threads

Commit source and run four cells (sparse and linear for masks 68 and 84),
then audit the denominator partition and combine receipts.

### Errors

None in source preparation.

## 2026-11-15 23:12 America/Los_Angeles

### What was done

Committed sparse and linear-square completion sources at `6b863a236f75`.
All four counted runs finished. Sparse arms had the expected branch
assignment counts and zero hits (39.039/39.537 CPU s). Linear arms had the
expected logical assignment counts, including all-zero polynomial fallbacks,
and zero hits (1.901/1.837 CPU s). Wrote the bounded result note.

### Decisions made

The fixed trace-`+22` carrier now has receipts for every expected normalized
degree-seven denominator class. Audit the mathematical partition and every
receipt before drawing a complete fixed-carrier conclusion. This is not
an attack-cost result or a vulnerable-family claim.

### Open threads

Write the denominator-partition theorem, Lean arithmetic checks, receipt
manifest/audit, and prior-work scope check. If the complete exclusion holds,
move to a different carrier or degree in Howe's 4–26 window.

### Errors

None in these four counted runs.

## 2026-11-15 23:17 America/Los_Angeles

### What was done

Ran `audit_receipts.py`: all twelve source-bound receipts across six
denominator arms passed SHA256 and coverage-counter checks, including zero
censored cubic cells. Wrote the fixed-carrier degree-seven theorem with
normalization and valuation-parity proof. `lean` checked the new
`DegreeSevenShapeArithmetic.lean` arithmetic file and the existing
`Trace22Orbit.lean` source-orbit file without errors.

### Decisions made

The complete degree-seven exclusion is limited to the fixed carrier and
ten exact source equations obtained by Frobenius. Howe's degree ceiling 26
still leaves degrees 4,5,6,8–26; its map is abstract here. The computational
elimination is structurally audited but lacks independent algebra-system
reimplementation. This is a bounded obstruction, not a vulnerability.

### Open threads

Commit theorem and audit, then classify the twelve recorded trace-`+22`
carriers under base-field isomorphism to see whether one or more additional
geometry classes remain to be searched. Continue new-family construction.

### Errors

None in audit or Lean checks.

## 2026-11-15 23:20 America/Los_Angeles

### What was done

Committed the fixed-carrier theorem, receipt audit, and Lean arithmetic at
`84f27681f158`. An exploratory exact PGL2(F3) enumeration of the twelve
recorded carrier polynomials found two six-element components, represented
by masks 32280 and 35349. Wrote a source-bound orbit certificate and a
generator that changes exactly one carrier literal in five frozen search
arms, plus the second-carrier protocol. No counted second-carrier run had
begun at this entry.

### Decisions made

Screen one representative of the second explicit PGL2 orbit. The earlier
quadratic-square screen already covers it, so the five remaining arms suffice
for all normalized degree-seven denominators on the twelve recorded models.

### Open threads

Generate and commit second-carrier sources, run orbit certificate and ten
search cells, audit hits and coverage, update bound/novelty status.

### Errors

None in preparation.

## 2026-11-15 23:21 America/Los_Angeles

### What was done

The first generator version created sparse and linear copies, then stopped
because the cubic source spaces its polynomial expression differently.
Changed the generator to find the unique `f=` assignment, assert the exact
old expression after removing spaces, and retain its original spacing.
Reran it successfully: all five second-carrier sources now exist.

### Decisions made

The stopped generator produced no counted search data. Commit the corrected
generator, orbit source, and five copies before execution.

### Open threads

Run orbit certificate and ten search cells; audit.

### Errors

Initial exact-string replacement failed on cubic source formatting. The
unique-assignment assertion now handles both spacing styles.

## 2026-11-15 23:22 America/Los_Angeles

### What was done

Committed the second-carrier protocol and source copies at `ecd9cbd26c16`.
The first source-bound orbit-certificate execution verified its assertions
but failed while serializing Sage integers to JSON. Added `default=int` to
the JSON encoder. No counted second-carrier cover search had begun.

### Decisions made

Commit the serialization amendment, then rerun the certificate and ten cells.

### Open threads

Complete second-carrier runs and audit maps or exclusions.

### Errors

Sage `Integer` is not directly JSON serializable by Python's encoder; the
explicit integer conversion fixes only receipt serialization.

## 2026-11-15 23:22 America/Los_Angeles — timestamp correction

### What was done

Compared worklog headings against exact Git commit times and the local `date`
command. Corrected several later headings that had been estimated ahead of
the clock; all entries now appear in chronological order. The work details
and committed source/receipt evidence were unchanged.

### Decisions made

Use the local clock, rather than an elapsed-time estimate, for further log
headings.

### Open threads

Rerun the corrected orbit certificate; then second-carrier searches.

### Errors

Earlier worklog headings were ahead of local time.

## 2026-11-15 23:27 America/Los_Angeles

### What was done

Committed corrected orbit source at `cb26c558000c`, reran the certificate,
and ran ten second-carrier cells at source `ecd9cbd26c16`. All five arms for
both source masks completed with zero exact hits; cubic cells had zero
censoring. The prior quadratic-square census covers the same representative.
`audit_second_carrier.py` checked source hashes and counters for all twelve
second-carrier receipts and independently recomputed twelve PGL2(F3)
coordinate substitutions using pure-Python polynomial arithmetic. Wrote
the twelve-carrier bounded result.

### Decisions made

The finite census's twelve carrier equations and ten source equations have
no degree-seven map in the normalized class. Do not extend this to every
genus-five carrier or call it a vulnerability. Next inspect degree eight,
whose odd denominator degree forces the rational branch pole.

### Open threads

Commit these receipts and audits; then freeze the degree-eight lead after
checking its direct-cover prior work.

### Errors

None in the counted second-carrier runs or independent coordinate replay.
