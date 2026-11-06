#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.5pt)

= Reduced-round neural distinguishers under crypto-ml-v1 (2026-11-02)

*Study:* `research/neural_distinguishers_20261102` · *protocol frozen:* `d7e1cc432` · *run:* task cards
`CARD-neural-distinguishers-20261102-1` (interrupted, 33/35 arms) and `-2` (recovery of cell aes-r4, completed;
receipt `b8902ba6…`) at commit `f0993f828` · *rows:* `results/rows.jsonl` (sha256 `ab49ea47…`) · *summary:*
`results/summary.json` (`2405a91a…`) · *host:* session container, 4 vCPU, CPU only, \$0.

== What was built

`docs/crypto-ml-v1.md` defined the lab's ML-cryptanalysis contract with no implementation. This study implements it
end to end in native Rust (no Python): Speck32/64 and AES-128 generators checked against published vectors (Speck
paper Appendix C; FIPS-197), Gohr's real-vs-random task with exactly balanced classes and one fresh key per sample
(key-disjoint splits by construction), a 128-128 MLP and logistic regression trained from scratch, a classical
difference-table naive-Bayes baseline, and per-cell positive, negative (labels permuted in train and test) and A/A
controls, with a Bonferroni-corrected decision rule over 21 tests at α = 0.01.

== Results (measured; held-out accuracy, n_test = 131 072 per cell, 95% Wilson intervals)

#table(
  columns: 6,
  [*cell*], [*MLP*], [*logistic regression*], [*difference-table NB*], [*neg. control*], [*A/A Δ*],
  [Speck r3 (pos. ctrl)], [0.9993 [0.9991, 0.9994]], [0.9551], [0.9656], [0.5000], [+0.0001],
  [Speck r5], [*0.8775* [0.8758, 0.8793]], [0.7517], [0.7502], [0.5018], [−0.0001],
  [Speck r6], [*0.6938* [0.6913, 0.6963]], [0.5836], [0.5750], [0.5004], [−0.0010],
  [Speck r7], [*0.5190* [0.5163, 0.5217]], [0.5008 (n.s.)], [0.5001 (n.s.)], [0.5022], [+0.0012],
  [AES r2 (pos. ctrl)], [1.0000], [1.0000], [1.0000], [0.4996], [+0.0000],
  [AES r3], [0.4998 (n.s.)], [0.5046 (n.s.)], [*0.5221* [0.5194, 0.5248]], [0.4988], [+0.0011],
  [AES r4], [0.5016 (n.s.)], [0.5015 (n.s.)], [0.5016 (n.s.)], [0.4989], [−0.0012],
)

"n.s." = the Bonferroni-corrected Wilson lower bound is ≤ 0.5: _no distinguisher found at this size_. Every other
entry _distinguishes_ under the frozen rule. All seven cells are valid: positive controls detected (> 0.9), every
negative control's 95% interval contains 0.5.

#figure(image("results/fig_accuracy.svg", width: 100%),
  caption: [Held-out accuracy by round count, one panel per cipher, three models, 95% Wilson bars; dashed line =
  chance. Recorded values from `results/summary.json`; cards -1/-2.])

== Reading the results (comparative claims, at the protocol's scope)

- *Speck32/64, 5–7 rounds.* The MLP distinguishes at 5, 6 and 7 rounds and its 95% interval lies above both
  classical baselines at every Speck round count. At 7 rounds only the MLP distinguishes (corrected lower bound
  0.5142). Advantage over chance: 0.378 / 0.194 / 0.019 at 5 / 6 / 7 rounds, against an A/A seed spread
  ≤ 0.0012. For orientation only (not reproduced, not used by the rule): Gohr's ResNet with 10#super[7] samples
  reports 0.929 / 0.788 / 0.616; this MLP with 2#super[20] samples reaches 0.878 / 0.694 / 0.519.
- *AES-128, 3 rounds.* The classical difference-table baseline distinguishes (0.5221, lower bound 0.5172) while
  the bit-level MLP and logistic regression do not. A small MLP on raw bits is therefore *not* uniformly stronger
  than a classical statistic: here the byte-level difference distribution carries signal the MLP did not learn at
  this size.
- *AES-128, 4 rounds.* No model distinguishes at this size.

== Execution record

Card -1 ran all cells in order and was killed by the agent harness's 30-minute limit on background shell
commands (not by a card timeout) after 33 of 35 arms; no receipt was written. Card -2 re-ran only cell aes-r4 with
the same frozen code and seeds and appended its 5 rows. Determinism check: the three aes-r4 arms present in both
runs agree exactly on `correct` (65 746, 65 588, 65 737). The summary uses the last row per (cell, model, arm). The
experiment-director's assessment (`cards/ASSESSMENT-20261102.md`) classifies card -2 _completed_ (all digests
recomputed and matching) and card -1 _interrupted_. The relay check (`task_card.py check-relay`) is
*inconclusive*: it reports one expected receipt line missing although that line is present verbatim in the tool's
own output, and the report file it was run on was assembled from the tool output rather than the executor's
message, so it does not independently verify the relay.

== Scope and limits

Distinguisher accuracy is neither key recovery nor a full-round result. The difference-table baseline is weaker
than Gohr's full Markov DDT distinguisher, which is not implemented, so "beats the classical baselines" means
these baselines. One architecture, one sample size and one epoch budget per cell; a larger model or more data may
distinguish AES r3/r4 or improve Speck r7. Training times are recorded per row but are not a performance claim.
The EKS route of AGENTS.md §9a was unavailable (no AWS identity); everything ran in the session container. No
independent `review-run` record exists yet.

== Next technical steps

1. Add Gohr's Markov DDT baseline for Speck r5–r7 (the stronger classical reference).
2. AES r3: a byte-level model (embedding per byte difference) to test whether the MLP's miss is representational.
3. Reuse this training/evaluation harness for the ECDLP dataset models
   (`docs/ml-dataset/training-protocol-20261101.md`) on development data only.
