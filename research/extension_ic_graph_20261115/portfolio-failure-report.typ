#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
= Runpod portfolio33: capture failed before workers
16 November 2026 · Failed attempt preserved · Native Rust transport correction

The fixture generator completed. Each of the six IC/graph/rho worker starts
returned `AlreadyExists` before entering scientific worker code. No scalar
comparison or cost result was produced. The stdout redirection created the
same file that the Rust worker opens with `create_new`. [pcc:unverified]

#grid(columns:(130pt,20pt,130pt,20pt,130pt),align:center,
 rect(inset:8pt,radius:3pt,fill:rgb("e8f2f6"))[Shell opens raw path],
 [→],rect(inset:8pt,radius:3pt,fill:rgb("f9e5df"))[Rust create-new refuses],
 [→],rect(inset:8pt,radius:3pt,fill:rgb("ece8f2"))[Error and empty raw retained],
)

#table(columns:(250pt,180pt),align:left,inset:6pt,
 [Recorded event],[Outcome],
 [Source published before sole dispatch],[948afcaba (full SHA in custody)],
 [Public fixture],[p1009, seed331],
 [Fixture / worker starts],[0 / six exit1],
 [Flow records],[14, retained exactly],
 [Whole-process wall],[1.842s, practicality only],
 [Portable pre/preworker/post hashes],[Pass after collection],
 [Owned pod deletion / later lookup],[204 / 404],
 [Scientific result / parity],[None],
)

The original stderr reports the exact create-new refusal. Original empty
raws, argv, flow, exit statuses, source and binary custody hashes are
preserved. The container also lacked `ps`, invalidating the intended
compiler preflight; the error is retained. Artifacts were collected and
checked before pod3lmv488eiypxct was deleted at16:43:31UTC. [pcc:unverified]

Amendment29 separates `mode.summary.jsonl` stdout from the worker-owned
`mode.raw.json`, requires the preflight command and adds `procps` during
bootstrap. A Rust transport-only stand-in uses the same create-new contract:
the frozen old wrapper fails for the exact collision, while all six corrected
capture paths pass. The stand-in performs no curve computation or scientific
measurement. Fresh trial34 uses seed337 with the same configurations,
mathematical checks and budgets. No failed33raw was replaced.
[pcc:unverified]

PCC-20261116-b60806 has unverified standing. Exact original bundle, returned
archive, copied source/binaries and host facts remain at
#raw("/workspace/ic-runpod-runs/runpod-ic-graph-rho-portfolio-20261116-33").
The Git packet's `local-custody.sha256` binds those files; this is not an
independent signed execution verdict. [pcc:unverified]
