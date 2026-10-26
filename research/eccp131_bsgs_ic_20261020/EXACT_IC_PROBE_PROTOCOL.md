# ECCp-131 exact two-sum admission probe

**Frozen before the first exact-curve probe.** This is a bounded diagnostic
of the current small-x, two-summand method. The acceptance criterion for the
requested challenge remains a scalar for the published point, independently
checked by `[x]G = Q` after natural relation collection, useful rank, and
target descent on this exact group.

## Inputs and method

- Curve and subgroup: Certicom ECCp-131 as constructed by
  `CurveParams::eccp131`, with the published `Q` in `ic`'s named profile.
- Base: the first 512 on-curve abscissae starting at `x=0`, one deterministic
  square root per abscissa, both signs admitted by the relation oracle. The
  command records the ordered base digest and exact distinct pair-sum count.
- Setup candidates: 128 independent seeded nonzero scalars `a`, each checking
  `[a]G` against all signed two-point sums of that base.
- Target candidates: 128, starting with `Q` and then seeded nonzero `a` in
  `Q+[a]G`. The same fixed base is used. Seed: `131`.
- Every pair-sum hit is reconstructed with actual signed points and checked
  as a group identity before its row enters the modular rank calculation.
  The target scalar is emitted only when the augmented system determines its
  column and an independent point multiplication verifies `[x]G=Q`.
- Positive control: release unit test
  `exact_ic_probe_recovers_a_fresh_target_from_natural_rows` on a generated
  small prime-order curve, with its own generator and a planted scalar. It
  does not enter the ECCp-131 relation corpus.

## Command, limits, and artifacts

Run the release `ic` binary from this branch once, after the control passes:

```sh
RAYON_NUM_THREADS=1 gtimeout 600 /usr/bin/time -l \
  /private/tmp/codex-eccp131-build-crypto/release/ic prime \
  --curve eccp-131 --size 512 --trials 0 --controls 8 \
  --setup-candidates 128 --target-candidates 128 --seed 131 \
  --json --out research/eccp131_bsgs_ic_20261020/ic-prime-eccp131-b512-seed131.json
```

The command runs with one Rayon thread and a 600-second wall watchdog. Retain
its exit status, standard streams, `/usr/bin/time -l` resource output, source
commit, binary hash, and raw JSON under this directory. The exact pair-sum
enumeration is finite (`512²` keys before deduplication). Do not extend the
candidate count after inspecting its results; a later probe needs a new
protocol and distinct artifacts.

## Interpretation

- Natural checked rows and positive independent rank would justify a new
  predeclared relation/rank/descent protocol at a larger base or with a new
  oracle. A verified scalar would satisfy the point-specific correctness
  check; a speed claim would additionally require matched references.
- Zero rows or target hits bounds only this base, seed, and candidate set.
  The independently derived upper bound `2B²/(n−1)` characterizes uniform
  candidates for the entire two-sum base, without relying on this sample.
- Timeout, invalid witness, or failed control is an implementation or resource
  result. It does not decide the curve-level ECDLP.
