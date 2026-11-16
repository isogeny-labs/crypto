COMPILE RESULT: BLOCKED. No card was written, so there is no card path and no lint output. A card with unfilled values cannot pass `tools/task_card.py lint`, and I do not invent values for the mechanical tier. I did not run `lint` or `prompt`.

Verified from commands in /home/user/crypto (branch claude/fastest-grobner-base-algorithm-u74ipw, HEAD 66d8635ef275b135cafbac9f3911319823130428):
- /home/user/crypto/research/groebner_solver_shootout_20261116/ holds only PROTOCOL.md, analyze.py, grid.tsv, preflight/ and protocol.json. It has no request JSON, no image.json, no job script or driver, and no cards/ directory.
- `git status --porcelain` shows the study, groebner_compare/semaev.py, external_worker.py, semaev_verifier.py and test_semaev.py as untracked. runner.py and certificate.py are modified. HEAD does not contain the study or the harness.
- The dispatch supplied no validated EKS request, tested image identity, deployment selection or exact client timeout. AGENTS.md §9a and the director's rules require all four from the main session.

**Specification errors, each a blocker:**

1. SE-1: No request file exists.
   - The only route for these solvers is `isolab.eks.command-request/v1`, submitted with `taskq/deploy/eks-karpenter/remote/launch-remote-command`.
   - `launch-remote` and `validate-request` handle `isolab.eks.request/v1`, not command requests.
   - There is therefore no validator step I can name for a command request. I did not look for a validator that accepts one.
   - The request, one per cell/shard (15 cells, at most 3000 s per Job, 2 vCPU), must give a digest-pinned image, argv, cpu, memory, seconds, isolation profile, job_name and `wait_seconds` ≤ 3000.
   - The main session must supply these files so I can bind them by SHA-256.
2. SE-2: P6, the solver image, is missing.
   - No image.json exists with the `isolab/tasks@sha256:…` digest, base digest, source revision, and msolve (c2722288…) and cryptominisat5 (00db7e59…) binary hashes.
   - The only image.json in the repo is the arity one: `research/arity_completion_20261104/launch/image.json`, digest `sha256:dd524955…` (source 4fa25efc…). It is an unrelated image, so I did not use it.
   - The image identity check also needs a tested-image receipt.
3. SE-3: P1–P5, the in-Job driver, are missing.
   - A script has to run the natural, planted and warm-up targets in the frozen Williams order and enforce the 300 CPU-s / 8 GiB budget per call.
   - It also has to run the oracle (P2), the rows.jsonl emitter (P3), the tamper check (P4) and t_GA plus matched rho into cells.json (P5).
   - It must upload results to `ISOLAB_RESULTS_URI`, since command Jobs only keep what they upload under `runs/<job>/<pod-uid>/`.
   - None of these exist yet; the argv for the cells is therefore unspecifiable.
4. SE-4: The source commit is missing.
   - The card's `commit` field must be a commit that contains the study and groebner_compare. Nothing is committed or pushed.
   - The harness SHA-256s in protocol.json are pinned to uncommitted files, and `freeze.commit` is 66d8635ef2, which lacks them.
   - The source-identity check in the Job needs the pushed commit.
5. SE-5: The benchrec record path is not freezable.
   - `tools/benchrec.py init --slug … --problem …` writes `benchmarks/records/<YYYYMMDD>T<HHMM>Z-<slug>.json`, named from the run time. A card cannot name the file that later `run`, `add-raw`, `metric` and `close` steps need.
   - The main session should run `init` first, under authorization, and give me the record path. I can then compile `add-raw`, `upload` and `close` steps.
   - The `metric` argv values (name, value, unit) are analysis outputs. They stay with the reasoning tier.
   - `benchrec close --status` is also a reasoning-tier decision (measured, invalid or censored), not the executor's.
   - The `bench-records` check needs the record, S3 raw hashes and the progress report, all in the same PR.
6. SE-6: The client timeout is missing.
   - `launch-remote-command` allows `wait_seconds` plus 1500 s of polling, with the CodeBuild timeout capped at 100 minutes. Which `wait_seconds` and `timeout_seconds` to freeze is unspecified.
   - Per-Job status reads (`launch-remote-command` with `isolab.eks.command-status/v1` requests) and a `delete-command` request are also needed. I will compile them once the requests exist.
7. SE-7: Run gates are not met.
   - `scripts/checkpoint.sh gate` fails (no upload credential), and `.resume/RESUME.md` was not checkpointed to S3. The hooks refuse run launchers until it passes.
   - There is no owner-approved budget (USD 15 ceiling, 24 vCPU).
   - There is no evidence of cluster read access for `scripts/isolab_pool_check.sh`. Without it the card cannot claim the pool is free, and the card notes would say so.
   - The protocol itself says "Running this protocol needs owner authorization", and P1–P7 must all exist before launch.
8. SE-8: Several instructions can't be put in a card as given.
   - "Williams-balanced interleaving" and the "planted calls in a cell stop after 3 censored" rule are in the protocol, but their executable form belongs in the SE-3 driver.
   - The card can only run that driver unchanged.
   - Magma, trace-f4 and julia-f4-warm stay UNAVAILABLE, as in the protocol.

**Facts I verified for later binding** (current file SHA-256; they may change after the commit):
- launch-remote-command: 7e1daae8dc70f8fcb813d9315daa73df3e18e7e5f72e2d47f34f9a3c85637416
- run-command: 1cdaa7447893fd5c2dc337d3903f8981107df1de6d77ca18cd8191cc8449ced2
- status-command: 435c9a2f35cace854534accf300402365a4f99f15babb0bbd187aa3e1ff92809
- current-run-request-buildspec.yml: 35504896fd8e9eae15690bda0ddd11c2a453ec1c463b8cbeb3c1224f286f335e
- scripts/isolab_pool_check.sh: 887146b97a21c909808364cdbc23ef31fa0b40f4d05b8221e9d35ee7728c1d7f
- protocol.json file hash: 9ab6434739c01b7d5919d147c8a4dacf08256e7054f128057b183bed5ee9341a. This differs from the canonical protocol_sha256 116b33f7…, which the freeze block computes. Bind the file hash and cite the canonical one.
- analyze.py: 0b0eee393c01ee4b26d90013c0f007fcfecd533883d0fc7d62858be90e477929 (matches the frozen value)
- grid.tsv: 666b262e93e071f94189d6742c3d5e65175696f65275f3f6a5c1b79d0574db3e (matches the protocol)

Format guide: `research/arity_completion_20261104/cards/CARD-arity-launch-koblitz-a2.yaml` is an existing frozen_command card using `launch-remote-command`. It has source-identity, image-identity and budget-readback steps, with binds on image.json and its job scripts.

**To unblock, the main session needs to provide:**
- the committed and pushed source SHA;
- request JSONs and the image.json for the new image (the P6 receipt);
- the Job driver script (SE-3);
- an open benchrec record path;
- the frozen `wait_seconds` and client timeouts;
- owner authorization with a budget.

I will then recompile the card. Lessons outcome recorded: O-a344bdc9d065 (scope experiment-director, result blocked).