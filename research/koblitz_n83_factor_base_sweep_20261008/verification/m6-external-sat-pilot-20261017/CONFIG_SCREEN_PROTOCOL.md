# Bounded CryptoMiniSat search-configuration screen

Use the exact 11,641,543-byte Gray `all_x` witness formula whose native model
passed the original export audit and whose complete-model CryptoMiniSat control
returned SAT. This input fixes the ten x blocks but leaves auxiliary circuit
variables free. Pin CryptoMiniSat 5.14.7 by SHA-256, one thread, no random-seed
override, 1,000,000 conflicts, 120-second solver wall cap, and a 150-second
outer timeout. Preserve stdout, stderr, command, input and binary hashes,
exit code, solver status, wall and peak RSS for each call.

Run the following complete two-by-two configuration panel in this order:

| Arm | Branching | Gaussian-elimination limits |
| --- | --- | --- |
| `default` | solver default | solver default |
| `wide_gauss` | solver default | `--maxmatrixrows 8000 --maxmatrixcols 8000 --maxnummatrices 20 --autodisablegauss 0 --gaussusefulcutoff 0` |
| `vsids` | `--branchstr vsids` | solver default |
| `wide_gauss_vsids` | `--branchstr vsids` | the same wide-Gauss flags |

The configuration syntax was checked on a one-variable public SAT formula
before this freeze. A `SATISFIABLE` output is accepted only if its complete
printed assignment independently satisfies all original CNF and XOR rows,
the fixed x blocks, five original-field S3 equations, and the archived
full-point witness. `INDETERMINATE` at either cap is censored; UNSAT on this
known-answer input is a producer failure. Charge four full 150-second call
envelopes, **600 seconds**, even if a call stops early. These local L0 runs
screen solver access only: they do not measure natural relation yield, rank,
or a total-runtime factor-base ordering. A positive arm requires a separate
matched-host, held-out natural-search protocol before promotion.
