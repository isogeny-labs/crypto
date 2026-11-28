# 2026-11-23: genus 2/3 sibling divisors (root-finding follow-up)

## 21:15Z: sibling-divisor component study published

### What was done
- A workflow worker built `research/rootfind_genus23_20261123/`. It contains:
  - a Python port of `genus_ladder.rs` Cantor with exact counted F_p operations;
  - a frozen protocol plus an amendment;
  - a sibling-walk study over the lab curves at g=2 (p=65537 and p=524309) and g=3 (p=65537).
- An independent red-team review found three blocking items, all confirmed:
  - The distance to rho was priced with a band of the study's own making.
  - The re-priced figures were labelled "lab".
  - P5 was scored at only one weight.
- A fix-up pass rewrote the rho projection with `genus_ladder_analyze` `gae_w` (§5a). It also fixed the `rep_analyze.py` per_curve overwrite and regenerated the tables, figure and report.pdf.

### Decisions made
- Headline `[pcc:unverified]`:
  - The sibling cuts Cantor cost per candidate to 0.73 (g=2) and 0.78 (g=3).
  - Under §5a GAE the arm total moves by at most 1.7% at w≥0.1.
  - Distance to rho at w=1: about 12× (g=3) and about 98× (g=2).
  - By the frozen rule the study is recorded as refuted on P2.
- No leaderboard or benchmarks row.

### Open threads
- Add an `IcExt::sibling` flag in `genus_ladder.rs` and run an end-to-end certified solve.
- Measure per-host `w_cal`.

### Errors
- First-draft rho pricing was wrong; it was caught by the red team and corrected. No Rust build: the disk was about 750 MB free.
