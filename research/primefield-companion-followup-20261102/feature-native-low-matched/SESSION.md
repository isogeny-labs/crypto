# Isolated low-product instrument — T-1544

## What was done
Read previous truncated-product derivation, native receipt, saved accounting hooks
and matched runner. Prior literature is FLINT nmod_poly/mul.html from the preceding
construction; truncated multiplication remains an established method.

## Decisions
Use preserved instrumented monic as baseline/A-A, new truncated monic as candidate.
This isolates truncation. Existing product helper hook covers all new products;
existing schoolbook update hook covers both loops. Add hooks for the two shifted
assembly additions. Preserve old protocols and binaries. Bounded software checks
will validate triangular leaf hooks and full recovery equality, not cost reduction.

## Open threads
Generate/reverse-verify, build runner/tests, execute software contracts, bind a
separate draft and publish. Preflight/admission remain before comparative execution.

## Errors
No comparison or cloud job at construction start.
