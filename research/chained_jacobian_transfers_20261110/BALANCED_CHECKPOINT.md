# Balanced genus2 development checkpoint

Producer commit: cryptanalysis28ba661017cdba4b132c2dcbc1615a3fd92a2a48.
Source `experiments/chained-transfer-reference/src/balanced.rs` SHA-256
`08c872be3afc9ab78d575a757660cf788d6dc083a6d9610e66f9b31fc94d5fb8`;
shared cubic norm helper SHA-256
`5d0416f3574777bb1a95bb687e2560872f9e15d03d1031973d91cdaa8ab731da`.
Upstream native runtime remains crypto75a737b8, with the same seven input
digests. Claims remain unsigned/unverified. [pcc:unverified]

The full written algorithm, primary sources, editable diagram and inspected
two-page PDF are in the producer's
[BALANCED.md](https://github.com/isogeny-labs/cryptanalysis/blob/28ba661017cdba4b132c2dcbc1615a3fd92a2a48/experiments/chained-transfer-reference/BALANCED.md)
and [PDF](https://github.com/isogeny-labs/cryptanalysis/blob/28ba661017cdba4b132c2dcbc1615a3fd92a2a48/experiments/chained-transfer-reference/balanced-report.pdf).
The construction is Galbraith corrected ch.10§10.4.2 balanced arithmetic,
not a new canonical-representative theorem.

The bounded prototype accepts monic smooth split sextics at checked primes
5..101, representing finite support plus infinity counts relative to
K=∞++∞−. Complete10,404-class original and translated models match the
existing odd-chart engine on paired operations. The primary F37 example,
1,211 fixed associativity triples, malformed tags and infinity controls pass.
The original full-class first norms also match the native oracle.
[pcc:unverified]

The quadratic cover of the existing E109/F101 source needs no rational
Weierstrass point. All109 source scalar/norm[2]/inverse55 controls pass;
the Jacobian has11,663 rational classes, norm image109 and equal fibers107.
Independent elliptic quotient orders109/107 agree with the class count.
Full consumer check passes native25/reference37 tests, formatting/strict
Clippy/guards; one existing benchmark remains ignored. Receipt
balanced-pinned-01.txt and producer PCC-8b7ca0 remain unsigned.
[pcc:unverified]

Six Lean bookkeeping/expansion lemmas compile here, gate6 declarations,
claim1cadd0. They do not formalize finite fields, valuations, Picard groups or
the primary uniqueness theorem. Total written35 lemmas; gates22+8+7+12+6
declarations. The first unknown-tactic failure and corrected pass are retained.
[pcc:unverified]

This narrows the catalog representation gap only as a bounded development
prototype. Production arithmetic and the1,940-row catalog remain unchanged.
Extension fields, large primes, nonmonic/inert models, normalized targets and
catalog-wide replay remain open. No actual non-split Richelot map, benchmark,
prime-field descent or cost advantage is claimed. Next CT13 classifies and
constructs Frobenius-stable kernels/codomains on predeclared E19/F17, with
actual map descent still required. Frozen-ledger/S3/budget/KB/vault blockers
remain as in CHECKPOINT.md. [pcc:unverified]
