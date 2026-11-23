# Progressive residual batches

Start each query with one prefix residual, then double each batch up to the
configured maximum. Prefix positions are visited exactly once in their original
order. The first matching suffix still wins and an absent query still scans all
prefixes. Batch1 is unchanged. Geometric growth trades more inversions on late
or absent matches for less unused arithmetic on early matches; it is not assumed
to improve complete runtime. Standard batch-inversion cost tradeoffs were
reviewed in Bernstein–Lange precomputation arithmetic in earlier iterations.

All 29 release tests and strict Clippy pass. Existing exhaustive all-target
checks cover batch caps1,2,7,256, resets, absent results and exact first witnesses;
full archived certificates still match. The new frozen panel will retain every
existing control and add the immediately preceding candidate as arm y, so the
schedule's effect is observed within one panel rather than across hosts.
The optimized baseline2bd4588fba60 remains the goal denominator.
