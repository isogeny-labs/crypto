# Conditional whole-cell contract

Fix four largest first-seven table tuples, leaving first three tables with
3*3*14=126 choices and the same four kernel bits. For each remaining tuple y,
the eighth observation is v+F*K*z. Quotient by image(F*K). Each eighth accepted
tuple with the same quotient has one unique z, since rank(F*K)=4. Enumerating
only this bucket is exactly equivalent to all16 kernel choices; repeated
projected images retain every distinct accepted tuple. Compare both complete
sets in every cell, not just their counts. Original theta and old q are
recovered before the leading guard and all six other residue checks.

Every locally accepted parameter in the selected cells is retained, including
repeated roots in splitting tables. The eighth relation and all subsequent
tests are exact, so a cell with zero surviving parameters has no original
root in this necessary family and cell. Other cells, original source/U,
ghost/chart branches and full support remain unresolved. Exhausting32chosen
cells is not exhausting the full outer cover. Explicitly report its cardinality.

One worker90s,4GiB/core0; at most100000direct kernel assignment checks. Table
setup and literal field controls precede cell profiling and are charged. The
two conditional methods share setup in this structural diagnostic: their
inner timings are not isolated cold end-to-end ratios or genuine GB baselines.
Four-bit kernel elimination alone has at most16fold evaluation-count reduction
and cannot justify50x overall. Need a separate outer-cover reduction.
