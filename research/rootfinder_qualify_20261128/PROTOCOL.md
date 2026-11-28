# Rootfinder qualification protocol (post-#5283)

Policy: matched native vs fastest open external univariate root finder
(FLINT `nmod_poly_factor`), plus a cold F4/PKM pipeline cell. Magma is the
preferred closed backend when a licence is available; absent Magma is named,
not silently replaced as “fastest”.

Standing: every timing on a non-isolated host is `[pcc:unverified]` L0
diagnostic. Claim-grade numbers require Iso Lab L2 admission (§9a), a
checkpointed `RESUME.md`, and a `bench.record/v1` row.

## Frozen inputs

| Field | Value |
| --- | --- |
| Native path | `glv_invariant_base::poly_roots_fp_counted` (shared by F4 Auto / #5283) |
| External open | FLINT 3.x `nmod_poly_factor`, helper `flint_helper/root_nmod` |
| External closed | Magma `Roots` / `Factorisation` when `magma` is on PATH |
| Primes | 17, 31, 101, 1009, 65537, 2^31−1 |
| Degrees | 2..6 (p31: deg ≤ 3 on L0; full 2..6 on Iso Lab) |
| Seeds | 1..7 (ChaCha/StdRng stream id = `seed ^ p ^ (degree<<32)`) |
| Polys | monic split ∏(x−r_i) with distinct random roots |
| Correctness | native roots = FLINT roots = planted roots |

## Commands

```sh
gcc -O2 -o research/rootfinder_qualify_20261128/flint_helper/root_nmod \
  research/rootfinder_qualify_20261128/flint_helper/root_nmod.c \
  -lflint -lgmp -lmpfr -lm

cargo run --release --example rootfinder_external_qualify -- \
  --flint research/rootfinder_qualify_20261128/flint_helper/root_nmod \
  --out research/rootfinder_qualify_20261128/raw/l0-panel.jsonl
```

Iso Lab (when AWS identity validates):

1. `scripts/checkpoint.sh init` and fill `.resume/RESUME.md`.
2. Submit the same binary under isolab-eks `cpu-spot`, isolation L2.
3. Close a `bench.record/v1` with wall_s and count metrics; regenerate progress.
4. Re-run Magma on the same coeff lines if Magma is present on the image.

## Stop / success

- Root-set disagreement on any cell: fail closed; do not report a ratio.
- L0 median `native_ns / flint_ns` is recorded only as diagnostic.
- Claim-grade: L2 receipt + Magma-or-named-absent + PKM counter refresh on
  the corrected #5283 path.

## Blockers recorded 2026-11-27 (America/Los_Angeles)

Injected `Awskeyid` / `Awssecret` return `InvalidClientTokenId` from
`sts:GetCallerIdentity`. No Iso Lab job was launched.
