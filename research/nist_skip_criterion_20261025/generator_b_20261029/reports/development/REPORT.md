# Native development checks — 29 October 2026

These are local development tests, not the requested real-curve experiment.
No performance isolation or clock is claimed. The tested source and command
bindings are in [BUILD.json](../../BUILD.json); all retained attempts belong
to the local review bundle named in the delivery record.

The final `build-controls.sh` invocation compiled the C++17 PARI producer,
the separate GMP verifier and the tiny fixture generator under
`-O2 -Wall -Wextra -Werror`. Both native self-tests, one complete synthetic
integration and nine corrupted-evidence controls passed. The fixture repeats
six tiny fields into 21 rows; its 1,127 inherited features are dummy values.
The verifier checked 588 point rows/100,548 trait cells, 19,152 cutoffs,
119,700 point differences and 28,175 dummy-feature differences. All eleven
unknown historical-point stand-ins remained NA. Both programs share OpenSSL
for SHA; their group and trait arithmetic is independently implemented.

The nine rejection controls alter a point, a trait, an unknown generator,
a recipe flag, a cutoff key, a difference, a required point row, an
inherited matrix cell and the headline summary. Each exits 1 for its intended diagnostic and writes
no admission receipt. Their data come only from the synthetic fixture.

Retained producer attempts 001–002 failed compilation; 003 compiled but its
tiny doubling control failed because PARI modular coordinates had not been
lifted to integers. Attempt 004 fixed that defect with `liftall`; 005–007
added identity/bit-width, synthetic classification, Hasse and transport-label
checks. Verifier 001–002 failed strict indentation diagnostics; 003–005 passed
the progressively strengthened synthetic checks. The final combined build also independently recomputes and compares the
headline summary; an added ninth control rejects a false summary. The build
uses the current sources, not a previously compiled binary.

The five conditional Lean proofs compiled. Axiom-auditor attempts 001 and 003
hit their 1 GiB/2 GiB limits; 002 lacked `lean` on the child process PATH.
Attempt 004 used an explicit PATH and 4 GiB cap and passed: six declarations
including the generated equation theorem, zero failing, zero compiler-trusted.
Only `propext` and `Quot.sound` occur among the reported theorem foundations.
There is no `lean4checker` fresh recheck or whole-repository gate claim.

An initial inherited-input copy had 1,100 features rather than the required
1,127. It was retained outside the repository and replaced with all 21
hash-matched final-replay files before freezing inputs or any real-cohort run.
No production cohort was executed with those stale inputs or the defective
producer. The [diagram](visual.svg) separates development from the blocked
actual comparison. No attack, historical criterion or speed result follows.
