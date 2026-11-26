# CT343 — actual Prime31 divisor-character qualification

2026-11-23 UTC, [pcc:unverified]. Existing draft #5066; frozen starting head e1c245d665e4e913420119d46886516d73af8bfe. [Preregistered bounded protocol](https://github.com/isogeny-labs/crypto/pull/5066#issuecomment-7117383208). No timed experiment, no solver-performance claim.

## Mathematical scope

For the unchanged deliberately bielliptic six-root model at p=2147483647, use CT339's odd chart z=1/(x-1), eta=y*z^3 and F(z)=product(1-j*z), j=1..5. Put c=leading coefficient=-120. For a reduced divisor (U,V), deg U=n, a=1/j, evaluate the squareclass of (-c)^n U(a) if U(a) is nonzero. For a simple ramified support a, write U=(z-a)U0 and use c^3 F'(a)(-c)^(n-1) U0(a). In the monic model X=c*z, this is the product of the local derivative at the branch point and the remaining differences. The factored check instead multiplies c(z_i-a) in the quadratic extension over the geometric support, replacing the zero factor at a by product of the four nonzero branch differences. The product descends to the base field; compare its Legendre bit with the coefficient evaluator.

These expressions reuse the preceding CT343 certificate and Stoll 2001 (doi:10.4064/aa98-3-4; bibliographic record revisited), CT343_TWIST_RANK.md and CT343_CHARACTER_CERTIFICATE.md. Full group injectivity is conditional on the previously stated geometric kernel and descent homomorphism premises, not established by sampling. Only the original model is tested here; twist arithmetic remains untested.

## Bounded result

| Check | Passed |
|---|---:|
| Cohort entries (230 distinct) | 232 |
| Coefficient vs factored squareclasses | 1160 |
| Kernel-coset shift and quotient-image equality pairs | 928 |
| Doubling has zero signature | 64 |
| Sampled addition homomorphism | 64 |

Cohort categories: identity 1, infinity 2, ramified degree-one 5, affine degree-one 64, nonsplit quadratic 46 (32 generated directly from extension-field conjugate support, others via addition), repeated 64, split 50. The two leading character coordinates distinguish the four tested kernel points exactly: 00,10,01,11. All tested kernel points are annihilated by the quotient and by multiplication by 2. The shift checks verify both signature translation and unchanged quotient image for each of four kernel elements per cohort entry. These are exact arithmetic self-checks, not independent proof of the Cantor core, native Rust replay, or a complete DLP trial.

## Reproduction

Use Python 3 and the unchanged CT339 source under name ct339.py (SHA256 c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695). Run:

    python3 ct343_prime31_divisors.py > local.json
    python3 -O ct343_prime31_divisors.py | cmp - local.json
    cmp local.json CT343_PRIME31_DIVISORS.json

Run both modes with --bad-branch and --bad-pair and require nonzero exit. Both fail in both modes, with kernel signatures and pair rank errors respectively. Seed 34331, search cap 4096 each cohort; no timings. Source SHA256 ada42694476bd5720b0a07acebffaa27dacada156cf64d361049968012b071e2; raw SHA256 8a13a44426c69a81843e4fa057edf94cab54e5fe51739f198010e3cecf798090. Source Git blob cf81c2b6d486feba4da75db927a7dff36d9955c3; raw 53ab7c6d4c6dff44ab9be1302bfa8193e63250b5. Both remote blobs match the executed local identities. Source commit eac1cd83119b39e991767a2fdd2b65f82ee199ed, raw commit cea63bd45a6f08940bf6b740739d2aec02541d4f.

## Handoff

### What was done
Read AGENTS, ACTIVE, latest CT343 report/log and twist report, root #3877 (merged), queue #3904, #4215, #4178, cryptanalysis #1523; targeted autoresearcher CT343 search empty, not evidence of absence. Batched independent connector reads. Preregistered bounded arithmetic before execution; validated two formulas, seven support categories, four kernel signatures and both mutation modes. No native agent slots, Rust, Conductor or KB in runtime; daily session log bounded read returned empty. Combined CI status at frozen head empty, not a passing CI gate. One coordinator performed all work, no parallel elapsed-time gain inferred.

### Decisions
Stay on existing draft #5066; additive public arithmetic evidence only. No production edits, no timed runs, no CI rerun/merge/spend. Existing 695/701 norm-point fixture and C169/C13a/C13b/N2_actual/D measurement reservations untouched. No novel transfer asserted.

### Open threads
Separate native Cantor implementation and independent signed PCC; full twist divisor validation; physically priced character operations and complete matched cold comparison remain open. The original norm-reuse owner retains priority on authenticated fixtures. Shared daily log and repository PDF packaging require supported worktree access. Do not mark ready/merge or claim performance from these counts.

### Errors
Expected mutants reject; no unexpected arithmetic failures. Shared daily log could not be fetched by line range; preserve it and use task-local handoff. No native independent verifier available. [pcc:unverified]
