# R07: current relations do not yet produce an NFS matrix

The corrected native filter **completed with exit 0**, discarding one
composite-factor row, appending 78,556 free relations, and removing 152,650
duplicates. It retained **4,518,304 unique relations**, including free
relations, before singleton removal. Four singleton passes left **92
relations and no tracked large ideals**. No matrix prerequisite or factor
was produced; the native controller requested more relations.

The [native filter log](native-filter.log), [summary](summary.json), and
[verification](verification.json) give the exact scopes and counts. Ideals
in that singleton graph are those above the native cutoff 100000. A raw
relation count is not a completed matrix or factorization.

The original R06 file is unchanged. The filter used an independent copy,
whose first 4,592,399 lines retain the original SHA-256. Its file grew
because the native filter appended free relations. Thus the receipt's
`input_relation_file_unchanged:false` is an observed change to the copy,
not damage to the predecessor checkpoint. Continue lattice collection from
R06's original state, not the filter-modified copy.

The first attempt misspelled the native option and printed help before
filtering, despite returning zero. Its command, script, output, hashes and
input dependency remain retained. The [explicit correction](PROTOCOL-correction.md)
uses `-minrels 1` only to dispatch this diagnostic before the ordinary
collection gate. The filter's elimination and arithmetic conditions were
unchanged. The corrected attempt entered native filtering and completed.
The warning about a non-prime factor describes the one row that was skipped;
it did not terminate the filter.

Read the [frozen protocol](PROTOCOL.md),
[report](reports/run-20261025T0512Z/REPORT.md), and
[inspected PDF](reports/run-20261025T0512Z/report.pdf).
The full raw archive is local pending artifact-store setup. It contains
both attempts, actual executed source, native states and input hashes;
the parse-only attempt uses an explicit symlink to the original R06 input.
That input is also recoverable as the verified prefix of the corrected
attempt's archived relation file. No remote object or upload is claimed.

No extra lattice sieving, linear algebra, square root, paid machine or
remote task was launched by R07. The arithmetic result remains eight of
nine integers complete, with the same 444-bit cofactor unresolved.
