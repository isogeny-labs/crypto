# Prospective prime31 algebraic experiments

Status: proposed, unrun, **[pcc:unverified]**. No spending or run receipt.
Each experiment retains the exact ordinary prime31 curve and generator in
README.md. These are informed continuations of earlier leads, not novelty claims.
Owner request: complete non-generic/index-calculus solving, Rust reuse and cold
one-target cost against the strongest matched generic reference.

## EXP-R31-A: natural-positive complete algebraic baseline

Question: can the native two-summand oracle produce useful independent rows and
recover previously unseen prime31 targets inside a registered cold budget?

- Reuse this crate's oracle, signed replay and public-input scalar recovery. [pcc:unverified]
  Add full ecbench integration before comparison. Preregister Dickson depths
  8, 9, 10; record the actual number of liftable EC points, not just fibre degree.
  Depth 2 remains a development control, not the requested complete experiment.
- Fixed public target set: generate 12 encoded points by hash-to-curve with a
  fixed domain separator and recorded subgroup check. Freeze point bytes before
  timing; hold back four points. Do not give their discrete logarithms to the solver.
  Match negation-map rho and BSGS on each point with equivalent cold setup charged.
- Separate one-target workloads; no shared collision or factor-base-log tables
  across targets in the primary reading. Include all base construction, input
  certificates, natural misses, undecided calls, F4, root extraction, rejected
  lifts, signed replay, rank work and final scalar verification.
- Controls: A/A, original unmodified algebraic baseline, planted positive,
  independently enumerated negative toy, repeated-abscissa/identity exception,
  deliberately corrupted relation, and hard resource cutoff returning undetermined.
- Primary observable: complete cold S and each arm's explicit native counters
  at w=0,0.1,1 plus same-host field calibration; secondary wall time only after L2
  admission. Report every censored/failing target. A calibration does not hide a
  rank reversal in the required band.
- Finite proposed envelope: one 4-vCPU/8-GiB Spot job, 900 s original run deadline,
  zero automatic retries, total ceiling USD 2 including build/idle/failure.
  A live balance and pool readback are prerequisites, not assumptions of this plan.
- Falsifier/stop: 256 complete calls per cell, or any resource deadline. If no
  target completes, retain the censoring and observed useful-rank yield; do not
  fit a DLP exponent or replace the workload with the toy control.
- Advancement: correct complete prime31 scalar recovery with all phases emitted;
  an improvement requires paired baseline/candidate cold intervals and independent
  replay. It is not expected or asserted that the tiny m=2 base beats rho.

## EXP-R31-B: restore the actual sparse m=8 test

Question: does a natural-positive m=8 solver cross the prior whole-call gate?

- First implement/audit six intermediate abscissae. The packed tower engine's
  four-free-variable limit blocks the current S3-chain representation; an
  existing generic exponent-vector F4 implementation is a possible correctness
  control, not an assumed competitive replacement. Never silently truncate links.
- Binary Kummer B=8/16/32 over this Fp is invalid: v2(p-1)=1.
  Use the existing nonsplit Dickson fibre B=8/16/32 (v2(p+1)=31), or explicit odd
  Kummer divisors such as 7/9/21 with the changed shapes preregistered. Keep
  actual liftable-base cardinality and scalar-order field separate.
- Compare positive natural public R queries to planted controls on the same
  curve and base, with all solver failures, every emitted signed witness and
  independent row-rank increments retained. A unit-ideal degree trace is not a
  substitute for extracting positive witnesses.
- Follow #4077's fixed accounting model: fit whole-call gamma and useful-output
  delta jointly. The optimistic m=8 gate gamma<3+delta/2 has a maximum 3.5 when
  useful output is linear; one-output gate is gamma<3. Include setup and the
  actual LA phase before treating this as an ECDLP advantage.
- Freeze small F4 matrix caps and a USD 5 total ceiling before submitting; use
  one job, 4 vCPUs/16 GiB, 1800 s including provisioning, zero automatic retries.
  Queue if admission/budget cannot be read. This proposal does not permit the
  historical 140-GB closure or a quota increase.
- Advance only with genuine useful-rank scaling, then integrate the exact
  same oracle into EXP-R31-A's cold scalar recovery. Growing-field holdouts and
  independent verification follow before any exponent claim.

## EXP-R31-C: certified preprocessing and large-prime yield

Question [pcc:unverified]: does the paired-S3 cubic preprocessing of #4228 improve useful relation
yield per fully charged invocation, and does LP combination preserve that gain?

- First compare the unmodified algebraic system and the exact certified cubic
  preprocessor on identical natural positive/negative systems, checking solution
  sets on toys and every recovered signed relation at prime31. Charge certificate
  construction, checking, extraction and failed calls. This is an application
  experiment, not a claim that the cubic construction is new.
- Only after a completed baseline, add one/double-large-prime rows from the
  same algebraic oracle. Freeze LP sets, graph reduction and rank accounting.
  Measure usable independent rows, not merely graph cycles or the 2-core size.
- Stop if positive extraction fails or complete cold cost regresses; keep the
  losing rows. Proposed maximum USD 2, 4 vCPUs/8 GiB, 900 s, zero retries after
  a fresh live budget check. No job is currently queued or running.

## Admission checklist and exact build

1. [pcc:unverified] Resolve/push this PR's exact head, protocol, input hashes and source-package
   dependencies. Unit build: `cargo build --locked --release --manifest-path
   research/prime31_ic_20261115/native/Cargo.toml`.
2. Supply authenticated supported EKS submission and checkpoint transport.
   `scripts/checkpoint.sh gate` must pass with a complete uploaded resume note;
   neither local tests nor a GitHub commit substitutes for that upload.
3. Integrate into the existing ecbench spec/phase schema and full counters; freeze
   runnable specs and PCC preregistration before any benchmark. The current
   `isolab-experiment.yml` builds ecbench, not an arbitrary example: add this
   adapter to that supported executable or use a verified custom image/command route.
4. Check live capacity, original deadline, finite remaining balance, image/source
   digest and exact public inputs. Execute/reconcile one unique submission.
5. Retain all raw records, S3 receipts, replay and independent validator outputs;
   apply scientific and dashboard gates before publishing a comparison row.

Current blocker: no runnable supported EKS/checkpoint submission in this runtime.
No performance conclusion may be drawn from the prospective limits above.
