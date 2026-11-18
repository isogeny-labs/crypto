# Resume: crypto @ research/h115-frobenius-kummer-resolvent-freeze

This is the H115-specific checkpoint note. Copy it to `.resume/RESUME.md` only
on the exact H115 branch immediately before the ordinary checkpoint push/gate.

## Goal

Determine whether one of 648 frozen low-order Frobenius--Kummer fiber-moment
predicates gives a uniform scalar digit modulo six on five prime-order toy
curves, 15 generator rows and 366 targets, followed by an independently
verified scalar-blind decoder. This is a bounded falsification experiment, not
a generic or subexponential ECDLP claim.

## Lead / hypothesis

For ordinary `E/F_p` with `r=#E(F_p)`, the exact identity
`(V-1)(pi-1)=[r]` compresses the `r^2` division torsor to the `r`-element
fiber `c0(Q)+ker(V-1)`. H115 evaluates eight X and eight Y power sums over
that fiber and 54 fixed raw/ratio/cross/mixed channels with all residue
permutations modulo two and three. The producer explicitly enumerates `r`
fiber terms per target. A uniform `r^o(1)` moment shortcut remains an open
requirement for any subexponential claim.

## Spec

- study root: `research/h115_frobenius_kummer_resolvent_20261117/`
- machine protocol SHA-256:
  `c8412d94f3e6cc928c13c93572a8796cea3a983b5fcd4e6b043a75bc5b1a2363`
- curve cohort SHA-256:
  `babd2556951ecfcaee66a5c6966364ca136bf1f62fb0a51511f391768255d295`
- producer source revision: crypto `6d8def5473`
- producer source SHA-256:
  `f0644129c8d7c0161cbddac3283f94453983d25b6f6cdeaf327b170c1fc579bd`
- independent verifier: cryptanalysis PR #1739 revision `ed206d8d`
- verifier source SHA-256:
  `e625542931d0b8b6b6b753ce19b495a03b9640c7f1cdb6ba06b3d7d9196e1a37`

## State

Protocol, primary-source screen, exact fixtures, proof map, illustrated
mechanism, Lean proof layer, native producer and independent verifier are
pushed in two draft PRs. Lean compilation, the five-curve native producer
selftest, the verifier's synthetic eight-class tamper test and its independent
unlabeled structural reconstruction pass. No `--run` invocation, candidate
packet, score, selected ID, decoder outcome, runtime or speed observation
exists.

## Next steps

1. Install an authorized checkpoint transport and run
   `scripts/checkpoint.sh push`, then `scripts/checkpoint.sh gate`.
2. Build an immutable Isolab EKS image from the bound producer source, record
   its digest and admit one Spot job with USD 1.00 / 1,800-second limits and
   zero scientific retries.
3. In that job, compile with `rustc --edition=2021 -D warnings` and run:

   ```text
   ./h115_producer --run /results/h115.jsonl --inputs \
     research/h115_frobenius_kummer_resolvent_20261117
   ```

4. Restore exact bytes into cryptanalysis revision `ed206d8d`; run
   `python3 h115_verify.py --verify h115.jsonl`, then mutate copied packets one
   class at a time and require all eight exits to be nonzero.
5. Publish raw digest, runner receipt, verifier output, tamper receipts, exact
   operation counts and claim-labeled interpretation to both draft PRs.

## Runs in flight

None. No local or remote H115 scoring process has been started.

## Stop and success conditions

Stop after the single exhaustive frozen run, on the first structural/digest/
metadata mismatch, or at the budget/deadline. Incomplete or unverifiable output
is censored, not a result. Candidate success requires the same ID exact on all
15 rows, all 366 targets decoded and independently checked, every control and
structural requirement passing, normal verifier acceptance and eight tamper
rejections. An empty 15-row intersection is a clean null result for this exact
family.
