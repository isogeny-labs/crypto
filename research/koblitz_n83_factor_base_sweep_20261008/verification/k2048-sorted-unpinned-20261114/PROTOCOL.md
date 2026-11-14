# N83 a=0 K2048 all-summand ordering gate — frozen 2026-11-14 UTC

## Question and exact input

Test whether all-summand x ordering improves native-XOR search on the
retained K2048 five-summand circuit. The [unrestricted constructed-target
control](../k2048-unpinned-positive-20261114/RESULT.md) has a checked
solution but reached 1,000,001 CryptoMiniSat conflicts without a model.
This gate changes the circuit by adding four lexicographic comparisons;
the curve, base, target, shared-root domain, solver binary, seed, wide-XOR
policy and resource cap stay matched.

The curve is `icv1-f2m83-tm6151469093347-debefd74`, with field modulus
`z^83+z^45+z^2+z+1`, subgroup order
`2417851639230796216685689`, and cofactor 4. The retained K2048 base
has 339,968 usable points and 2,048 signed-Frobenius columns. The five
constructed summands are the first representatives of columns 0 through
4. A separate arithmetic replay sorted them by unsigned x as columns
`[1,3,4,0,2]`; their four proper partial sums are affine and the full
sum is the same public target
`(985527933806849610456454,7623568253424067907890851)`.
This validates a witness in the zero-identity chain pattern for the
constructed point. The public natural arm uses sealed fixture 1 from the
same corpus as the matched D18/K2048 solver pair.

`ChainedS3Encoding::order_all_summands` already enforces nondecreasing
unsigned x on a shared domain. Its documented permutation argument covers
all identity patterns together; this gate constructs only pattern 0, so
the natural arm tests that particular encoding and target. The constructed
point has a checked pattern-0 witness. Treat solver outcome as a diagnostic
until original-formula, group and relation-row replay succeeds.

Sources checked: the research KB's N83 and PDP solver summaries, the
retained K2048 original-XCNF protocols and raw receipts, and current
`binary_semaev_chain_sat.rs` source. Earlier K2048 first-pair ordering at
1,000 conflicts and earlier vmtf/VSIDS studies are separate arms; neither
measured this exact all-summand-ordered constructed formula.

## Frozen sequence

1. Extend the source-bound K2048 exporter with distinct
   `positive-sorted-pinned`, `positive-sorted-unpinned`, and
   `natural-sorted` modes. Each uses the same retained archive and exact
   five-summand shared-root S3 circuit. Apply `order_all_summands(200000)`
   after the orbit domain. For the positive modes, construct the original
   public target and reorder the five actual points by unsigned x before
   computing chain partials. The pinned mode explicitly fixes the five
   summand, three intermediate and redundant target x blocks; the unpinned
   mode adds no witness pins. The natural mode uses fixture 1. Export to
   fresh files only from a clean committed source.
2. Run a bounded pinned-positive solver control. Independently replay its
   complete model against every original CNF/XOR row, all 339,968 archive
   rows, subgroup, point sum and signed-Frobenius relation row. A failure
   stops this sequence.
3. Export the unpinned sorted positive formula. Before its solver call,
   independently test the pinned-positive model against every row of that
   original formula, check equal target and sorted witness receipt, and
   require zero explicit witness pins. Freeze the exact source, formula,
   positive replay and solver hashes in a committed lock.
4. Run the sorted unpinned positive with the same CryptoMiniSat 5.14.7
   binary, one thread, seed 20261114, VSIDS, 32 Gaussian matrices,
   `--autodisablegauss 0 --gaussusefulcutoff 0 --maxmatrixcols 25000`,
   1,000,000-conflict and 600-second solver limits, plus a 660-second
   process-group watchdog. Sample host load and swapouts three times 15
   seconds apart, requiring load at most 24, unchanged swapouts and at
   least 2 GiB free scratch. Retain all terminal statuses and raw output.
5. If and only if the sorted positive reaches a model that independently
   replays, export and run the sorted natural fixture-1 formula under a
   separately committed lock and the same resource gate. Independently
   replay any returned model against the original formula, group and
   relation row. Retain attempts without checked rows and rank as such.

The outcome guides a full ordinary-relation producer and later complete
same-target IC/rho comparison for both N83 curves. No stage timing here
replaces the full online or cold phase account.
