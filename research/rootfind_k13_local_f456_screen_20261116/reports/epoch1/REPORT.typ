#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)

= K13 local screen: first epoch custody verdict

*Status: invalid for timing inference.* Session `ECBS1h124a5b50bd9a` completed the frozen `ECS1h326f1843d687` plan on two public K13 targets: 48/48 recorded answers verified. The independent `ecbench verify --replay-all` command reproduced all 32 measured deterministic executions with zero problems.

The executable SHA-256 was `dba933709f466966dc13f069b9bf0e91def033284f8b1cb779b25c8c9b3448c1`. Its path was under the admission worktree. ecbench records Git state from the worktree containing the executable, so the session named commit `557f28efa880...` instead of the repaired source branch's `fb5fd632771b...`. This source stamp is insufficient for the timing protocol. The observed wall times are excluded from comparisons.

#image("source-custody.svg", width: 100%)

The raw session, 48 work-pricing rows, and replay receipt are retained. The planned repeat will copy identical executable bytes under the repaired source worktree and keep the same frozen targets. It is a custody repeat, not an independent target holdout. See `INVALID_EPOCH_1.md` and `../../audits/epoch1/replay-all.json` for the full verdict and receipt.
