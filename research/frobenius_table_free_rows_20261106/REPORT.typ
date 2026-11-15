#set page(margin: 2cm)
#set text(size: 10pt)
= Free relations from the Frobenius pair table's build-time collisions: none
2026-11-06 · probe `free_rows_probe` (`src/cryptanalysis/ic_framework/plugins.rs`) · frozen lazy-orbit panel, eight Koblitz cells

*Question.* Do the dropped inserts of `FrobeniusPairTable`'s build give target-free relations among the column logarithms? At rank c − 1 the walk would need about 2 hits instead of about c + 1.

*Answer.* No. Every one of the 15 to 3,870 collisions per cell is a symmetry of the table: its row is identically zero, and the rank is 0 on every cell. These are exact counts, reported as unverified assertions.

#figure(image("fig_free_rows.svg", width: 100%), caption: [Build-time collisions, non-trivial rows (0 everywhere, drawn at 0.5) and the rank needed, per cell.])

*Why.* The folded table has about reps × |F| entries, far below the square root of the number of signed Frobenius classes, about r/2n. Genuine collisions are therefore expected below 1 per cell, and about 8 at n41. The collisions that occur are pair sums met again through a Frobenius or sign image.

*Not claimed.* No method change, no benchmark, no leaderboard row.
