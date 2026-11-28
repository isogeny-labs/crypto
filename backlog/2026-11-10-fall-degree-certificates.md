# Certify last-fall-degree instances at n = 33–51

Priority: high (queue item Q4 of `research/ic_track_ruling_20261110/DECISIONS.md`).
Funding: EKS Spot, memory-heavy class.

**Why.** The programme's strongest theory result is that the last fall degree of
the binary Weil descent of S₃ is certified ≥ 5 at n = 33–45, while the first fall
degree is 2. That is the treatise's Chapter 6 and the paper candidate
(`research/paper_opportunities_20261029/REPORT.md`, gates G1–G2). Only the
certificates' hashes are committed, and there are few instances per n. Whether
the last fall degree reaches 6 is the open question.

**State.** Runs E13 (2026-11-08) and E14 (2026-11-09) of
`research/fall_degree_bounds_20261008/` did not complete:

- E13a: "0 of 4 draws measured" at 13 GiB (`RESULTS.md:568`);
- E14: "Not completed. RunPod terminated the pod" (`RESULTS.md:595`).

**Next.**

1. Rerun on isolab-eks with checkpoint/resume (CRIU/EBS contract) at the memory
   the E13 log shows was exceeded, plus headroom.
2. Raise the instance count per n at n = 33–51.
3. Publish the closure certificates, not only their hashes (G1).
4. Run the independent checker (`verify/cert_check.py` in
   `crypto-autoresearcher:analysis/semaev-m2-last-fall-degree/`) on every new
   certificate.

Use the frozen protocol of the parent study; a change of estimator is a new study.
