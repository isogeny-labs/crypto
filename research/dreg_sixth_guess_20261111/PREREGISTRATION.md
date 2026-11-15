# Preregistration: a sixth guess on the four saturated S5 branches of (3, 25, 9) draw 1

Frozen before any run of this study. Follow-up to `research/dreg_guess_scaling_20261106`, which found that
S5 (five guesses) holds the MutantXL closure at degree 4 on draws 4, 5 and 7 but not on draw 1. All four
failing branches, b28–b31, share the corner x₃[8] = e₁[0] = e₁[1] = 1 (`--fix 26=1,27=1,28=1`), and each
saturates at closure rank 188,151. **Class:** stage diagnostic (exact algebra). It produces no `S`, no rho
ratio and no row.

## Question

Does one more guess, chosen by the S5 construction rule ("the next invariant and the next e₁ coordinate"),
make every corner branch refute at degree 4? If it does, k\* ≤ 6 on draw 1, and k\* ≤ 6 holds at ℓ = 9 on all
4 registered draws.

## Arms (draw 1 of `research/dreg_guess_scaling_20261106/draws/m3-25-9.jsonl` only; 17 runs; `pod/commands.txt`)

- **N, identity control.** S5.d1.b28 re-run with this build. It must equal the committed
  `runs/pod/S5.d1.b28.jsonl` on every scan field except `secs`: saturated at degree 4, unrefuted, closure rank
  188,151. If it does not, the study is void.
- **I, next invariant.** For each b ∈ {28..31} and f ∈ {0, 1}: S5's guesses plus `--sum 15=6+f` (s[6] of s = x₁ + x₂).
  That is 8 runs.
- **E, next e₁ coordinate.** For each b ∈ {28..31} and g ∈ {0, 1}: S5's guesses plus `--fix 29=g` (e₁[2]). That is 8 runs.

Every run uses `dreg_linear_guess --engine pe --batch 64 --to 4 --max-cols 230000 --check`, timeout 7,200 s,
`RAYON_NUM_THREADS=2`, with F4 caps of 5·10⁷. These are the settings of the parent study.

## Decision rule (fixed now)

- An arm **suffices** iff all 8 of its runs refute at degree 4 (`"degree":4, … "closure_refuted":true`).
- If either arm suffices, the reading is **"k\* ≤ 6 on draw 1"**, and with the parent study, "k\* ≤ 6 at ℓ = 9
  on 4 of 4 draws".
- If neither arm suffices, the reading is **"neither registered sixth guess suffices"**. That is about these
  two guesses only.
- A capped, killed or missing run is unmeasured. Its arm then cannot suffice.

## Predictions (before the runs)

- I suffices: 0.55.
- E suffices: 0.45.
- At least one arm suffices: 0.7.

The reason for these numbers: at ℓ = 8, invariant guesses beat raw ones (7 of 12 against 1 of 12), and the
failing corner already fixes two e₁ coordinates.

## Host

One RunPod CPU pod (cpu3g, 64 vCPU, 256 GB, about $2.56/h). It uses the upload launcher of
`research/koblitz_ic_leads_build_20261024/pod/upload_launch.py`, which hash-checks the binary, the draw file,
`commands.txt` and `job.sh`. All 17 runs execute at once. The lifetime is capped at 3 h (at most about $8;
the cost cap is $15). It is not isolab-eks, because that image carries only `ecbench` (parent study, Amendment 1).
Outcomes are exact algebra, so wall times are descriptive only.
