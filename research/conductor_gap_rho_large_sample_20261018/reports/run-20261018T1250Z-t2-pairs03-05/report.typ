#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t2-pairs03-05
Session `ECBS1hac93fc74d4e5`, status `complete`, 192 records; spec `ECS1h4b1ddeaa0c58` sha256 `4b1ddeaa0c5879cf…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 3..5: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101842, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t2-pairs03-05/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair3-crater], [icv1-fp53-t33623877-90323303], [16], [0.8761], [0.1344], [0.180 / 1.901],
[rho-neg], [pair3-crater-twin(u=5)], [icv1-fp53-t33623877-5a16adae], [16], [0.9770], [0.1519], [0.213 / 1.943],
[rho-neg], [pair3-floor], [icv1-fp53-t33623877-87bc6afd], [16], [0.9405], [0.0992], [0.443 / 1.966],
[rho-neg], [pair3-floor-twin(u=5)], [icv1-fp53-t33623877-715b30f6], [16], [0.9261], [0.1301], [0.153 / 2.127],
[rho-neg], [pair4-crater], [icv1-fp53-t33684497-ecd1e0df], [16], [0.9922], [0.1472], [0.137 / 2.160],
[rho-neg], [pair4-crater-twin(u=5)], [icv1-fp53-t33684497-484e9f13], [16], [1.1414], [0.1207], [0.252 / 2.328],
[rho-neg], [pair4-floor], [icv1-fp53-t33684497-3523039e], [16], [0.6895], [0.0833], [0.113 / 1.173],
[rho-neg], [pair4-floor-twin(u=5)], [icv1-fp53-t33684497-582bd297], [16], [1.1494], [0.1568], [0.207 / 2.471],
[rho-neg], [pair5-crater], [icv1-fp53-t33705281-dcfbcce7], [16], [0.8512], [0.1088], [0.187 / 1.616],
[rho-neg], [pair5-crater-twin(u=5)], [icv1-fp53-t33705281-d97b5dec], [16], [1.0296], [0.1110], [0.234 / 2.090],
[rho-neg], [pair5-floor], [icv1-fp53-t33705281-298d41bc], [16], [1.0725], [0.1207], [0.329 / 1.961],
[rho-neg], [pair5-floor-twin(u=5)], [icv1-fp53-t33705281-db3dec79], [16], [1.0906], [0.1237], [0.199 / 1.727],
)