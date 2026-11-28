# Direct-baseline reproduction receipt

Before any R or P solver row, the new instrument and the pinned predecessor
binary each ran the complete fresh-seed `d=7` stage baseline: four random and
four planted targets, seed `2026112503`, 120-second cap, degree ceiling 24.

All 8/8 pairs matched exactly on arm, order, target kind/index/seed, field and
subgroup, target scalar and affine point, planted roots/signs, variable and
equation counts, input terms and degree, construction-operation count, planted
checks, verdict, counted F4 work, reached and productive degrees, matrix
extrema, steps, censoring, timeout, and degree-ceiling status.

Descriptive build/F4 time, process peak RSS, schema labels, and instrument-only
metadata were not required to match. Raw outputs from both binaries are
retained under `d-reproduction/{new,old}`.
