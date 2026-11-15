# N83 a=0 K2048 unpinned constructed-target search — frozen 2026-11-14 UTC

## Question and contribution

Test whether the native-XOR five-summand K2048 solver can find a relation
when a decomposition is guaranteed to exist but its five summand and three
intermediate x-coordinate blocks are withheld. The public target x remains
fixed in the circuit. This separates solver access from the existence of a
decomposition on the exact primary N83 `a=0` curve. It is a bounded
solver-stage diagnostic; its output is kept separate from natural relation
yield and complete one-target runtime.

The curve is `icv1-f2m83-tm6151469093347-debefd74`, with polynomial-basis
modulus `z^83 + z^45 + z^2 + z + 1`, subgroup order
`2417851639230796216685689`, and cofactor 4. The retained K2048 base has
339,968 subgroup-usable points and 2,048 signed-Frobenius columns. Its
compressed object, panel manifest, generic replay, upload receipt and public
corpus must match the five SHA-256 values already embedded in
`primary_adapter.rs` before export. The constructed target is the group sum
of the first representative in columns 0 through 4, point
`(985527933806849610456454,7623568253424067907890851)`. The exporter
passes only that public point and the retained factor-base domain to the
solver; private witness coordinates and column indices remain in a validation-only
receipt.

The preceding [K2048 positive gate](../k2048-m5-xcnf-20261114/RESULT.md)
returned SAT with all nine x blocks pinned, and its independent replay
checked every original CNF/XOR row and the group relation. Its positive XCNF
SHA-256 is
`f46a900495ec1a3594add973ee641de21a318a8fda6c9bfa67bd19057326d44c`;
the archived model and transcript are retained in the same run root. The
current [same-public-point pair](../a0-matched-public-solver-20261114/RESULT.md)
reached one million conflicts without a model on each natural arm. This new
gate asks whether K2048's unassisted search also censors on a target with a
known valid witness. It does not reuse that witness as a SAT assumption.

Sources checked (KB): revision `a340f0566afc4abb83dd81cd3eef5b01`,
refresh 2026-11-14T03:49:08Z;
`source:7e900ab87238ad44d277757ec0a68a79` points to the N83 campaign
summary in `papers/ecdlp-index-calculus-program/sections/koblitz.tex`.
The K2048 positive/natural protocols and exporter source above supply the
exact local comparison. An exact search of the retained K2048 protocols
found no constructed unpinned receipt; that search alone is not an absence
claim.

## Frozen stages

1. Extend the existing source-bound exporter with a distinct
   `positive-unpinned` mode. It must construct exactly the prior positive
   target, build the same five-summand chained-S3 circuit and shared-root
   domain, and omit all 747 explicit witness x pins, including the redundant
   target x pin. The target x stays constrained by the public input. Retain
   the witness only in the export
   receipt. Require a clean committed source checkout and write-once output.
2. Before solver search, independently evaluate the archived positive SAT
   assignment against every clause and XOR row of the new original XCNF.
   Check its target against the archived positive receipt and its shape
   against the public natural formula. A failed check stops the run.
3. Freeze a lock with source commit, exporter binary SHA-256, exact object
   and formula hashes, CryptoMiniSat 5.14.7 binary SHA-256, control replay,
   command, host gate and output path. Commit the lock before solver launch.
4. On the local Darwin arm64 host, use the same K2048 wide-XOR policy as the
   matched pair: one thread, seed `20261114`, VSIDS,
   `--maxnummatrices 32 --autodisablegauss 0 --gaussusefulcutoff 0
   --maxmatrixcols 25000`, one-million-conflict and 600-second solver limits,
   and a 660-second independent process-group watchdog. Sample load and
   swapouts three times at 15-second spacing. Require load at most 24,
   unchanged swapouts and at least 2 GiB free scratch.
5. Retain exact stdout/stderr, command, host samples, wait4 CPU/RSS, conflict
   count, process wall and terminal status. If SAT, independently check every
   original XCNF row and reconstructed group sum before counting a checked
   relation. Preserve `INDETERMINATE`, timeout, OOM, host rejection and
   producer failure distinctly. A success here is a constructed-control
   solver result, not an ordinary relation-yield estimate.

The next scientific gate remains checked ordinary relations and novel rank
on a common public corpus, followed by complete same-target IC/rho
measurement in an admitted performance tier for both N83 curves.
