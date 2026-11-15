# Inverse construction handoff

## What was done
New sparse shared clone /Volumes/SSD990-2/hec-inverse-work at08db2132ac,
originGitHub. Original homeworktree and dirtyresume untouched. Built private
msolve0.11against matching FLINT3.5;71tests pass. Generator built, not executed.

## Decisions made
Exact ordered-triple inverse equations, public alignedKummer input only.
Separate known-point control and corrupted-input control. Freeze/checkpoint
before G1. Actual solver/parametrization verifier require separate sourcefreeze.
No group enumeration or supplied target scalar/divisor.

## Open threads
Complete fresh workspace resume checkpoint/gate; run G1once. Then collector,
parametrization verifier, bounded positive/target inverse solves. Goal remains
full10xgenus2+; construction/LA/genus2/4also open.

## Errors
Installedmsolve missingFLINT24; archivedmsolve missingneogb3; LLVM missingz3.
First autogen ran in wrongdirectory, corrected before build. Rootinclude/lib
paths selected FLINT3.6whileopt resolves3.5; private rebuild selected3.5both.
No global library edits. Pre-freeze generator delimiter typo corrected.
No scientific worker executed or rerun during setup.

## G1passed
Algorithm6221010218c0/headc3268afa412f/checkpoint214444 preceded one exit0
execution, emptystderr.12control residuals, directtensor expansion and corruption
control pass. Generatedpositive/target systems87/88KiB. No inverse solver run.
Next exactRURsubstitution verifier and adjacentKummer sign recovery.

## Solver/verifier preparation
Built bounded collector and independent modular-parametrization verifier, not
executed. Exact emittedpolynomial substitution and knowncontrol point required.
Positivecell first; actualtarget only after passing control. NextKummerpublic
input added for non-oracle sign recovery. Class reconstruction still open.
