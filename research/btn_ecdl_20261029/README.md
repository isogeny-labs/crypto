# Boolean-tensor-network relation finding (Akshima et al., v0.2) — prototype

Prototype of the relation-finding step of "A Subexponential Time Algorithm for
Elliptic Curve Discrete Logarithm" (Bochum/MPI, 2026-10-29), which is
**conditional on an unreviewed OpenAI tensor-contraction algorithm** and
asymptotic only. The authors publish no code (their repository holds the PDF
only). This directory encodes "A1 + A2 = R, A_i in a hashed factor base" on the
Koblitz curve `y^2 + xy = x^3 + 1` as a network of local predicates, counts
solutions with a plain bucket-elimination contractor (a stand-in for the
claimed algorithm), samples a witness bit by bit (paper Lemma 1), and runs the
index-calculus linear algebra.

No speed-up, no ECC2K-130 discrete logarithm, no leaderboard or benchmark row.

| File | Role |
|---|---|
| `btn.py` | GF(2^n), curve, constraint network, multiplier/squarer gadgets, contractor |
| `relation.py` | point-addition, curve-membership and factor-base predicates |
| `test_enc.py` | network count == brute-force count of decompositions |
| `run_toy.py` | full m=2 index calculus on GF(2^5), r = 11 |
| `ecc2k130_check.py` | the same encoding on the ECC2K-130 field and curve |
| `width.py` | treewidth estimate (min-fill) of the relation network |
| `test_enc7.py` | the n = 7 count check with cutset conditioning |
| `gpu_contract.py` | device contractor: CuPy on a GPU, NumPy fallback, cutset as a batch axis |

## Verified results (local, seeded)

* `python3 test_enc.py 5`: for 3 random R on GF(2^5), the contraction count
  equals the brute-force number of ordered pairs (A1, A2), x1 != x2, with
  A1 + A2 = R (e.g. 42 = 42, 40 = 40).
* `python3 test_enc7.py 1` (`test_enc7.log`): GF(2^7), R = (40, 59): network
  count 112 equals brute-force count 112. Plain bucket elimination needs
  tables of 2^34 entries here (min-fill width 34 vs 23 at n = 5), so the count
  conditions on the 7 bits of x1 (128 contractions of width about 25); 336 s.
  Dropping the P2 curve-membership predicate as redundant is wrong: it gives
  237 solutions instead of 112 (P2 need not lie on the curve), so both points
  are checked.
* `python3 run_toy.py 1` (`toy_seed1.log`): GF(2^5), group order 44 = 4·11,
  subgroup r = 11, |F1| = |F2| = 8, N = 16. 17 relations, each checked with
  reference curve arithmetic; the kernel mod 11 recovered the secret
  ξ = 3 and `Q == [ξ]P` holds. Contraction took about 340 s, so this checks the
  construction, not speed.
* `python3 ecc2k130_check.py`: curve `y^2+xy=x^3+1` over GF(2^131),
  f = x^131+x^13+x^2+x+1 (the ECC2K-130 curve). The addition + two curve-membership
  network has 105,062 variables, 104,801 predicates, 524,008 edges after
  replacing each variable by an equality tree, maximum predicate arity 5.
  A witness derived from reference arithmetic on random curve points satisfies
  all predicates (0 violated); flipping one bit of the output violates 1.
  Factor-base predicates are omitted at this size (a 131-bit table).

## GPU contractor (`gpu_contract.py`)

Compiles the elimination order and einsum strings once, turns the cutset bits
into a batch axis (all 2^k conditioned contractions run as one batched einsum
per step, chunked to `--mem-gb`), and keeps tables as float64 on the device
(cuBLAS DGEMM). Counts stay exact below 2^53, which the code checks.

* `--selftest --brute` (`gpu_selftest.log`, NumPy backend): three n = 5 targets,
  unbatched, batched over a 3-bit cut, `btn.contract` and brute force all agree
  (40, 40, 42).
* `--n 7 --cut 7 --R 40 59 --brute --mem-gb 4 --backend numpy` (`gpu_n7.log`):
  count 112 = brute force 112, width 26 after the cut, 539 s on this CPU
  container (the looped `test_enc7.py` took 336 s; batching costs memory
  traffic on a CPU).
* **Not run on a GPU.** This container has no GPU, CuPy or PyTorch. The CuPy
  path is untested and no GPU timing is claimed. Run it with
  `pip install cupy-cuda12x` and `python3 gpu_contract.py --backend cupy --selftest --brute`
  first; repo rules route measured runs through isolab-eks.

## Running on RunPod (`runpod/`)

`runpod/launch.py` ships this directory to a RunPod GPU pod through the repo's
existing contract (`ecc2k130/runpod_job.py`: CUDA 13.3 image, source and
results through presigned S3 URLs, pod terminated in `finally`) and runs
`runpod/gpujob.sh`:

1. installs CuPy, records the device in `env.json`;
2. `gpu_contract.py --backend cupy --selftest --brute` (stops if it fails);
3. the n = 7 count for R = (40, 59) against brute force, timed;
4. `run_toy.py <seed> --n 7 --backend gpu --cut 7`: end-to-end toy ECDL on
   GF(2^7), r = 29, one record per seed in `toy_n7_seed<s>.json`.

```
export RUNPOD_API_KEY=...            # and AWS credentials for the S3 bucket
scripts/checkpoint.sh init           # resume checkpoint; hooks refuse launchers without it
python3 research/btn_ecdl_20261029/runpod/launch.py --list-gpus
python3 research/btn_ecdl_20261029/runpod/launch.py --out /tmp/btn-runpod-1 --gpu-match L40S   # one seed fits the 330-min job; raise --job-minutes for more
python3 research/btn_ecdl_20261029/runpod/launch.py --dry-run --out /tmp/btn-dry   # no credentials needed
```

Verified here: `--dry-run` builds the archive and start command, and the
unpacked archive passes the self-test on the NumPy backend. `run_toy.py 1
--backend numpy` (the same code path as `--backend gpu`) recovers ξ = 3 at
n = 5 in 32 s. Not verified here: anything on a GPU or on RunPod (no GPU, no
RunPod key and no AWS credentials in this container). Step 4 may take hours: a
single n = 7 count took 539 s on this CPU, and a relation needs about 29 counts.

## What this does not show

* The contraction is exponential (min-fill width 23 at n = 5, 34 at n = 7;
  n = 7 only fits with cutset conditioning, 336 s per count, so a full n = 7
  index-calculus run (about 25 relations) was not attempted). Only a working Theorem D.1 algorithm would
  change that, and the paper states it saves nothing below about 2^(2^17) edges.
* The multiplier chain here has O(n^2) edges. The paper's O(n)-edge Fourier
  multiplier is not implemented, and its integer construction does not carry
  over to GF(2)[x] without a characteristic-2 variant. The paper itself treats
  prime fields; the ECC2K-130 run is an encoding check on the binary curve.
* ECC2K-130 is not attacked. Its prime-order subgroup ℓ is 130 bits.
