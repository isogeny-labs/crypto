# Owner decisions, 2026-10-29

These were recorded in the session that produced this program (claude.ai/code session_01TCbLH7wSrGRZ55EP1YTyqw). The owner's answers are quoted in substance. Where the owner delegated a decision, it is marked "delegated", and merging the PR that carries this file confirms it.

| # | Question | Decision | Source |
|:--|:--|:--|:--|
| D-EKS | Compute for the published reference arm (`reference_arm/`, derived estimate about 49 CPU-hours, range 15-150, EKS Spot) | **Approved** | owner |
| D2 | May the Table B incumbent be ranked against the published algebraic reference? | **Yes** | owner |
| D1 | Does the small-abscissa base, with membership polynomial F_V = prod(x - c), count as an "algebraically defined" factor base? | **Yes, tentatively.** V is the zero set of F_V. So the semaev-naive/msolve arm is the counted algebraic reference. Note that deg F_V = \|V\|, so this base gives a Gröbner solver no low-degree structure; that is why the PKM tower lane matters. | owner ("I think so") |
| R1 | How is rootfind-subtract classified (options A, B, C in `ruling/README.md`)? | **Option C, two counted tracks, delegated.** The *generic track* holds table, pair-table and root-finding decomposition (Table B, rootfind-*, mitm, subtract, the 2026-10-29 20-bit gain). Its improvements count and ratchet, labelled "generic: bounded by sqrt(r) (Shoup / Corrigan-Gibbs-Kogan); cannot beat rho". The *algebraic track* holds relations from a solver that receives the factor-base constraint inside the system. The prime-field algebraic baseline is empty until the reference arm runs. **Amended per D2:** the memo's Option C text kept the tracks unranked against each other; the owner's explicit D2 answer takes precedence. Arms from the two tracks may be ranked against each other on the same instances, in the same unit and across the §5a band. Each track still ratchets only its own baseline: a generic-track gain is never an algebraic-track improvement, and the reverse holds too. | delegated ("use judgment") |
| D3 | Pinned, vendored msolve counter patch | **Allowed.** Its sha256 is bound into the candidate identity. Without it, msolve's work could be priced only in a proxy unit or by wall time. | delegated (no answer given) |
| D-PREFETCH | Run the hash-table prefetch measurement (`hashtable_prefetch/`): the paired L2 session on isolab-eks, fp20-fp32, incumbent vs `layout=lines` with an A/A arm. D-EKS did not cover it. | **Approved** (owner, 2026-10-29). The sub-choices in O10 (incumbent oracle, stacking on #2783/#2806, default if it wins, holdout cell) are not decided. Until the owner rules, the run uses the draft protocol's defaults: incumbent `mitm-prime-batch`, and it waits for #2783/#2806 to merge. The patch must compile and pass `cargo test` and clippy before any session. | owner |

## Why root-finding is generic, not Gröbner

At m = 2, rootfind-subtract fixes x_1 = c for each c in V (exhaustive guessing). It solves the remaining quadratic S_3(c, X, x_R) via its roots x(R -/+ P_c), and tests each root against a hash index of V. It returns the same solution set a Gröbner solve of <S_3, F_V(x_1), F_V(x_2)> would. Its cost, however, never depends on F_V, and the oracle runs identically on a random V. It is therefore a generic algorithm, subject to the sqrt(r) bound. See `ruling/README.md`.

## Not yet done

- AGENTS.md still carries the 2026-10-24 wording. The Option C text proposed in `ruling/README.md` §5 is the change the owner confirms by merging.
- EKS execution needs a session with the isolab-eks route (an AWS identity). The reference-arm code must also be implemented and built first, which needs several GB of disk. Neither is available in the session that wrote this file.
