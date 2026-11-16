# Primary N83 fresh-trial identity-pattern panel: bounded hosted result

The frozen [`protocol.json`](protocol.json) ran at commit
`fd9121107c40b159b13497c475f848ea9a21aa96` in [GitHub Actions run
40185426122, attempt 1](https://github.com/aburan28/crypto/actions/runs/40185426122).
The Linux ARM worker was built from mathematical source commit
`83393dd87ef48f11963c582cd79b89296c4a1bea` and has SHA-256
`d7e97aaac59ab37a454d73ee0782871b6f7f266222bfe167ff9b28ec2b461aeb`.
Each of the four stratum jobs passed the positive solver control, fresh
eleven-object S3 input replay, exact four-formula export, six sequential
bounded searches, and artifact upload. All five provider jobs completed
successfully. The four independent [provider-ZIP audits](run-40185426122-attempt-1/)
return `PASS_HOSTED_STRATUM_REPLAY`.

| Stratum | Provider artifact ID | Exact ZIP SHA-256 | Export process wall (s) | Six solver process walls (s) |
| --- | ---: | --- | ---: | ---: |
| `t1_m0` | 12598203031 | `65f81231e786ba41c3741f493b8620a91df9000c249ef638cc9cb02f7e204c50` | 15.353 | 1,815.336 |
| `t1_m1` | 12597724070 | `52bc73e3f85213cb214f36bcb2b2b45af0abc970ff08a6de1bcde4bc3e28c5ce` | 13.516 | 1,808.819 |
| `t2_m0` | 12598206805 | `353b49e271361ead9be1f299053cae890a356b42a15da3e44e2942a2ac6073c6` | 14.461 | 1,810.394 |
| `t2_m1` | 12598357463 | `54447c6c3510d2980de3f2b049a7ef4953e618d910002fd843814772292c9470` | 14.656 | 1,819.804 |

All **24 solver processes** reported `INDETERMINATE` at the 300-second
solver time bound, with exit code 15. Their conflict counts ranged from
134,401 to 392,098, below the one-million-conflict cap. None reported a
SAT model, watchdog expiry, or observed OOM kill; peak RSS ranged from
747.25 to 1,447.32 MiB. The sum of their L0 process walls is
**7,254.353 seconds** across four parallel hosts. The 16 formula exports
sum to **57.985 seconds** of process wall; the separately charged worker
build was **353.906 seconds**. These are stage charges, not a complete
one-target cold index-calculus runtime. The provider run spans from the
first build-job start at 11:21:16 UTC to the final stratum-job completion at
12:11:30 UTC on 2026-10-25, including queue/setup/upload intervals.

The [precommitted independent auditor](audit_artifact.py) checked each
original provider ZIP SHA-256, artifact and job identities, pinned source
build and solver, fresh S3 manifest replay, every exported XCNF digest and
shape, and each raw solver stdout/stderr and process receipt. Its first live
positive invocation exposed a local return-value bug: an export-metadata
variable shadowed the provider metadata ID after all evidence checks. That
variable was fixed without changing the frozen workflow or provider bytes;
rerunning the auditor passed all four immutable ZIPs. The focused negative
tests still reject altered ZIPs and malformed solver status streams.

The [retained manifest](run-40185426122-attempt-1/retained-manifest.json)
records SHA-256 and byte size for all 240 provider ZIP members and maps 200
selected small receipts and compressed raw solver streams into this PR.
It omits only the eight large XCNF copies and two apt logs per stratum.
[`verify_retained.py`](verify_retained.py) rechecks those retained bytes,
the provider/job/audit bindings, all 24 solver statuses and summed process
walls; its saved [replay](run-40185426122-attempt-1/retained-replay.json)
returns `PASS_RETAINED_24_CENSORED_ARMS`. For complete byte replay, download
the exact provider ZIPs before their recorded expiry of
2027-01-23T10:35:49Z and invoke the auditor as described in
[`AUDIT.md`](AUDIT.md). The build ZIP is provider artifact `12597612377`,
SHA-256 `b530157c3040e7586ef5d1a8a9ec40a7c5bfd5586650c0c6733ece8c5e3042da`.

This panel covers exactly the two named fresh public trials, masks 0 and 1,
hash/Gray residual pools, and 5+1/4+2 splits. The hash and Gray pools have
different residual domains, so capped conflict counts do not isolate an
enumeration-order effect. The separate [complete-mask census](../full-identity-mask-census-20261025/RESULT.md)
is a necessary-condition screen of all 192 exported formulas; its coverage
ceilings are not observed success probabilities for these pinned targets.
The `a=1` root22 N83 cold screens use a different curve and subgroup from
this primary `a=0` experiment. No natural relation yield, graph closure,
rank, factor-base log table, unseen-target descent, or matched same-target
rho comparison was measured here. `selected_best_total_runtime` remains
`null`. The [AWS handoff](RESUME.md) pins the inputs and replay procedure
for the next experiment.
