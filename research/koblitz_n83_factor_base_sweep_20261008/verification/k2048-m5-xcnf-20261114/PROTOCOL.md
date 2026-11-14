# N83 a=0 K2048 five-summand XCNF gate — frozen 2026-11-14 UTC

## Question and inputs

Test whether the retained K2048 signed-Frobenius base admits a checked
five-summand relation on the same public N83 `a=0` point used by the full-D18
four-summand gate. The base has 339,968 distinct subgroup points and 2,048
folded columns. Its nominal unordered five-multiset count divided by the
subgroup order is 15.652801629828787. This count is a capacity screen, not
an observed yield or solver success rate. The comparison changes both base
and arity, so any later effect cannot be assigned to either alone.

The frozen S3 object is
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz`.
Its 10,313,647 downloaded bytes have SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`
and BLAKE3
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`.
The retained manifest, generic replay and storage receipt have SHA-256 values
`68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8`,
`5ad374ef0042b2ebd6562746b7ca67e46c6b0f4d46c4ba25abae83fd2dd411e2`,
and `f1139c1dd61beac0e1d9e80a1d02e983f98c7b1abc6887f1816f013d4693bf4c`.
The public corpus SHA-256 is
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`;
fixture zero is `(1834913121178197242856238,8797964190732846213981377)`.
The local input copy is under
`/Volumes/SSD990-2/tmp/n83-a0-k2048-m5-20261114`.

## Export and positive gate

Add one source-bound five-summand chained-S3 exporter using the retained
Frobenius-orbit root domain, identity mask zero, and no first-pair ordering.
Require exact archive and receipt checks before constructing either formula.
The constructed positive uses five distinct stored orbit representatives;
all five summand x blocks, three intermediate x blocks and the target x are
pinned to the independently checked group sum. The natural formula uses the
same core circuit and root domain with those witness pins absent. Export
write-once XCNF and a receipt with source, binary, input, shape and file
hashes. Use the exact committed source and clean checkout for each export.

The positive must return SAT under CryptoMiniSat 5.14.7 with one thread,
seed `20261114`, full Gaussian matrices (`--maxnummatrices 32
--autodisablegauss 0 --gaussusefulcutoff 0`) and VSIDS branching. Require
independent replay of every original CNF and XOR row, then characteristic-two
curve, subgroup, group-sum and signed-Frobenius relation-row replay. Only
that result releases the public natural query.

### 11:15 UTC bounded-control amendment

The constructed positive also uses a 1,000,000-conflict and 600-second
CryptoMiniSat limit with a 660-second process-group watchdog. The source-bound
positive export from commit `54753787383afe3a2a0fe7e7773dc47941e50025`
has 111,478 variables, 1,141,221 CNF clauses, 4,980 XOR rows, and 747
positive x pins. Its XCNF SHA-256 is
`f46a900495ec1a3594add973ee641de21a318a8fda6c9bfa67bd19057326d44c`.
This amendment was recorded before the external solver run.

### 11:19 UTC natural-formula lock

The constructed positive returned SAT and independently passed all original
CNF/XOR constraints, the group sum and relation row. Its replay SHA-256 is
`d648542575b7aa2bd8dbbeebc32727531449ecf38f5d8a87c8f00d56b13e0296`.
The unpinned public formula was then exported from clean commit
`9c993d2f2b2b4835dca4b480045ea337d66f472f`, with the same compiled
exporter binary. It has 111,478 variables, 1,140,557 CNF clauses and 4,980
XOR rows. Its XCNF SHA-256 is
`02d07716333e073f7eb40a7646ae3652b86b2129cb4d226f5dcc611d6bda7fff`.
The exact solver lock below binds the one public query before search.

### 11:30 UTC provenance correction

The first positive and natural external-solver calls were orchestrated and
replayed by Python scripts before the repository's native-implementation rule
was noticed. Their frozen raw bytes and terminal statuses remain historical
evidence. A Rust runner and independent Rust replay, followed by a separately
locked native repeat, are required before extending the comparison or using
the recorded process walls for performance claims. The earlier protocol,
locks and raw measurements are retained unchanged.

### 11:35 UTC native-repeat protocol

Use the retained positive and natural XCNF bytes without regenerating their
circuits. First, independently verify the existing positive model with the
new Rust XCNF/curve/row replay as a verifier sanity check. Then run the same
constructed positive through the Rust host/process gate with the exact
CryptoMiniSat binary, flags, 1,000,000-conflict and 600-second solver limits,
660-second process-group guard, and three 15-second load/swap samples. Replay
the new model in Rust against every original CNF/XOR constraint, archived
factor-base point, subgroup and relation row. Only that native positive
releases one native public natural query on the same retained formula and
public point. Preserve every run, host rejection and terminal state. Freeze
the native source/binary/formula hashes in a new committed lock before each
solver run. The native repeat remains an L0 diagnostic; any wall-time factor-
base ranking requires a separate calibrated Tier 1 or Tier 0 comparison and
complete end-to-end IC/rho evidence.

### 11:38 UTC native positive lock

The new Rust verifier checked the historical positive model against all
1,141,221 CNF clauses, 4,980 XOR rows, 339,968 archived rows, subgroup and
relation row; its independent receipt SHA-256 is
`6f5defdfb297e6d069e893869f49c9058b8343d1f669c559c76d32b1a0be1a3b`.
The one native positive repeat is bound by `NATIVE_POSITIVE_LOCK.json` to
source commit `d75033d50260210c8f094d00acfbccc1ad35d395`, runner source
and binary hashes, the exact previously exported positive XCNF, the same
CryptoMiniSat binary and frozen flags, and one new raw output path. This lock
precedes the native positive solver call.

### 11:40 UTC native natural lock

The native positive repeat returned SAT in 839 conflicts and its Rust replay
passed every original XCNF row, archived K2048 group lift and relation row.
The native replay SHA-256 is
`03432e7b5e1d9af74cd7d7111ee2289506b50434bdf9dc58919438a7c0f2737d`.
`NATIVE_NATURAL_LOCK.json` binds that positive gate, the same native runner
source/binary and solver, the exact previously exported public-natural XCNF,
frozen flags and one new raw output path before the native natural call.

## Bounded search and interpretation

The first public natural query uses the same solver flags, at most 1,000,000
conflicts or 600 solver seconds, plus a 660-second process-group guard. Admit
each local L0 run only with three 15-second load samples no higher than 24,
unchanged swapouts and at least 2 GiB free scratch. Preserve complete solver
stdout/stderr, command, wall/CPU/RSS, host samples, formula and model hashes,
exit code, and every cap or failure in the outside-repository raw root.

A natural SAT assignment counts as one relation only after original-XCNF and
independent group/row replay. An `INDETERMINATE`, cap, UNSAT without a checked
certificate, memory abort, or host rejection remains its exact terminal
status. One query cannot estimate relation yield or rank. Subsequent common
query/rank, full-width logarithm, unseen-target descent, complete cold cost
and same-point rho gates are required for factor-base selection on this curve.

## Sources checked

- Repository: `SOLVER_GATES.md`, the K2048 orbit capacity and matched
  K1182/K2048 result, and the full-D18 ordered XCNF result. The prior K2048
  m5 orbit model built with 1,140,557 clauses and one native query capped at
  5,000 conflicts; this protocol tests external XOR solving on exact retained
  input and a common public point.
- KB revision `a340f0566afc4abb83dd81cd3eef5b01`, refreshed
  `2026-11-14T03:49:08Z`: `source:007a1d17ad595712f30908a1c41cf26b`
  (`crypto-autoresearcher:inputs/SATIC-TRIMOSKA-2019/paper_fulltext.md`)
  supplies the SAT/Semaev and symmetry context; it does not predict this
  N83 cell. `source:ad4d05b2a1e4c2678babfc4b27b3bb90`
  (`crypto:papers/ecdlp-index-calculus-program/sections/catalogue.tex`)
  indexes prior N83 arity studies. The KB predates today's merged D18 arm,
  so its current result is read from the repository.
