# Matched norm-one-torus factor-base comparison

The exact matched-cardinality result is null: the production Prime31 Dickson
factor base does **not** make the generic five-term `2+3` query cheaper than the
equally sized ordinary small-abscissa factor base. Both arms retain 27,370
canonical three-sum states and execute the same full-scan operation vector over
the 256 common targets. [pcc:unverified]

The Rust query never materializes the two-sum set. Pair complements are made in
blocks of 1,024, batch-normalized, probed against the retained three-sum table,
and discarded. This removes the pair table's persistent memory but not its
quadratic enumeration cost.

Files:

- [`PROTOCOL.md`](PROTOCOL.md): decision rule frozen before the public seed ran;
- [`REPORT.md`](REPORT.md) and [`REPORT.pdf`](REPORT.pdf): result,
  interpretation, and reproduction commands;
- `public-precommit/manifest.json`: seed, source hashes, and fixed parameters;
- `public-precommit/results.csv`: nine counterbalanced primary/A-A rounds;
- `public-precommit/certificates.json`: transparent targets and witnesses;
- `public-precommit/verification-report.json`: independent affine replay;
- `public-precommit/artifacts.sha256`: hashes of the frozen input and outputs.

These are transparent reproducibility records, not a zero-knowledge proof or a
proof of which host executed the producer run. The independent verifier passed
all 528 hit/miss decisions and all 16 planted witnesses. [pcc:unverified]
