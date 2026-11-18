# R3 supplementary toy crossover test (2026-11-18)

Status: preregistered before running, exploratory sensitivity only; not native or signed PCC evidence.

Hypothesis: median half/full cold EC additions >1 at B=24 and <1 at B=40. A failed solve or the opposite sign at either B falsifies this toy hypothesis.

Frozen engine: R2 run.py and independent audit.py, E(F_1009): y²=x³+4x+10, order 1069, same per-arm stream, full/75%/50%/25% + A/A. B=24 has 8 seeds starting 0x1c2026111860; B=40 has 8 starting 0x1c2026111870. No tuning after preregistration.

Budget: 16 seeded cases, 80 arms, <60 seconds CPU, <10 MB files; zero cloud spend. Every relation row, dense rank, reconstructed target and matched same-target rho independently audited. Record setup, table, collection, matrix wall time/dimensions, modular LA and descent separately. Compare EC adds and proxies GAE+w*Fr for w=0,0.1,1. Diagnostics are unisolated local Python, not native cold wall measurements.

Prior art and closure: Semaev 2004, Shoup 1997, research/ic_mega_pass_20261031/brief/BRIEF.md, research/prime_ic_collection_frontier_20261117/README.md, R2 PR #4763. Classic generic pair-table engineering; no algebraic novelty or rho-beating claim.

Constraints: official /workspace/isogeny-workspace absent, GitHub DNS failure for official checkout, Conductor coordination tools and KB unavailable. This follow-up is preserved on existing R2 PR; native 24/28-bit experiment remains queued.
