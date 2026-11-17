# Timed head-to-head: multi-target IC vs multi-target rho in wall time (protocol, frozen before data)

Follows `../mt_fieldops_v5_20261112/` (PR #3968, claim PCC-20261112-22e5d2 [pcc:unverified]): in counted
operations the IC vs rho order flips with the price h of a hash-table or native operation (IC dearer at h = 0,
cheaper at h = 1), so the counted study cannot say which is faster on a machine. This study times both, in this
harness's implementation, and measures the operation prices.

## Questions

1. In wall time on one core of this host, is the fastest multi-target IC (ic5, ic5ks, ic5w, ic5rw) faster or slower
   per configuration than the fastest multi-target rho (rho2xks x 4 kappa, rhodpu x 6), and by how much?
2. What does one hash-table operation and one native operation cost here, in GAE (one affine addition), and which
   part of the section 5a band (h = 0, 0.1, 1) does that correspond to?

## Execution route and host (deviation from the lab default, recorded before data)

The default route for timed x86 measurements is Iso Lab on EKS (`isolab-eks`). This session's AWS identity is a
placeholder that AWS rejects (InvalidClientTokenId; no AWS network secret on the environment), so the run is on the
session's own host: a 4-vCPU cloud VM (Intel Xeon @ 2.10 GHz, 4 vCPU, no hardware PMU), not an isolated L2 node.
Every second here is therefore a diagnostic, single-host measurement, not lab-admitted (no native L2 gate); the
controls below are the study's own. A rerun on EKS L2 with the same `run.py` and binary is the follow-up.

## Instrument (one binary: `examples/mt_fieldops.rs` at the commit that adds this file)

Changes since v5 (no counted operation changed): an `opcost` arm (timed microbenchmark: field mul/sqr/inv, one
affine addition, x-pair, step index, canonicalisation + hash, std HashMap lookup/insert at 2^10..2^24 entries with
the harness's key/value shape, and a fast multiplicative hasher as a sensitivity); a printed `ms_arm` (wall time
before the shared verification pass) next to `ms` (total).

## Design

- Configurations: the v5 curves `../mt_fieldops_v5_20261112/instances/W{24,28,32}-{0..3}.json` (4 per size; already
  validated, not tuned on), seed 43 (new), T in {1024, 4096, 16384}: 36 configurations.
- Run keys per configuration: the v5 grid, 17 keys (ic5, ic5ks, ic5w, ic5rw at mu = 1, k = 4; rho2xks kappa
  0.05/0.1/0.25/0.5; rhodpu kappa 0/0.25 x d offset -1/0/1; bsgs2 mf 0.7/1.0/1.4).
- Each key runs 3 times (repeats). Executions are sequential, one process at a time, pinned to one CPU
  (`taskset -c 2`); within each round of repeats the order of all keys is a fixed-seed shuffle (seed 20261112),
  rounds in sequence, so repeats of a key are spread over the session (interleaved A/A). Load average is recorded
  per execution.
- `opcost` runs once per size on curve 0 before and once after the head-to-head.
- Every target of every run must verify; failures are reported, not dropped.

## Analysis (`analyze.py`)

- Time of a key = median over its repeats; primary metric `ms_arm` (solve only); secondary `ms` (incl. verify).
- Per configuration: icbest = fastest IC key, rhobest = fastest rho key, bsgsbest = fastest bsgs2 key; ratio
  icbest/rhobest. Per cell (size x T, 4 configurations): median ratio with a bootstrap 95% interval; **faster** if
  the upper bound < 1, **slower** if the lower bound > 1, otherwise **unranked**.
- Selection-bias control: rho has 10 keys and the IC 4, so best-of favours rho. Each side's key is also fixed per
  cell on repeat 0 and scored on repeats 1 and 2 (`fixed`).
- A/A: per configuration the rho best key's repeat-0/repeat-1 ratio (should straddle 1), and the median and 95th
  percentile of |log(repeat / key median)| over all keys.
- opcost: prices in GAE (padd = 1 GAE), I/M, native op and lookup prices by table size.

## Predictions (from one smoke configuration, 24 b, T = 1024, and three opcost runs; not data)

- opcost: I/M about 23-28; a native operation about 0.005-0.01 GAE (near the loop baseline); a std-hasher lookup
  0.05-0.1 GAE at 2^10-2^16 entries rising to 0.4-0.6 GAE at 2^22-2^24; the fast hasher 2-5x cheaper.
- Head-to-head: the IC faster than rho in most cells (ratio about 0.8-0.95), most clearly at large tables (32 b,
  large T) where lookups miss cache; slower than BSGS everywhere.

## Limits known in advance

One host, a cloud VM with possible steal time and no PMU; one implementation (std HashMap with SipHash, counting
cells in the field layer); 4 curves per size and one seed; short runs at 24 bits are noisy (A/A reported); the
result is about this implementation and host, not about the methods in general.
