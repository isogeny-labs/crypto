#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t7-pairs18-20
Session `ECBS1h868599a25248`, status `complete`, 192 records; spec `ECS1h27f24d121c1d` sha256 `27f24d121c1dbcfa…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 18..20: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101847, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t7-pairs18-20/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair18-crater], [icv1-fp53-t34042037-e54d747f], [16], [1.0758], [0.0996], [0.240 / 1.799],
[rho-neg], [pair18-crater-twin(u=5)], [icv1-fp53-t34042037-2ba7ca48], [16], [0.9771], [0.1507], [0.344 / 2.319],
[rho-neg], [pair18-floor], [icv1-fp53-t34042037-202608c9], [16], [0.8542], [0.1151], [0.203 / 1.688],
[rho-neg], [pair18-floor-twin(u=5)], [icv1-fp53-t34042037-0f161b8b], [16], [1.1805], [0.1565], [0.104 / 2.759],
[rho-neg], [pair19-crater], [icv1-fp53-t34049261-62ff07a5], [16], [0.9304], [0.1796], [0.183 / 3.059],
[rho-neg], [pair19-crater-twin(u=5)], [icv1-fp53-t34049261-e28fb75b], [16], [0.9626], [0.0761], [0.575 / 1.597],
[rho-neg], [pair19-floor], [icv1-fp53-t34049261-aad1a4df], [16], [0.9910], [0.1160], [0.342 / 1.917],
[rho-neg], [pair19-floor-twin(u=5)], [icv1-fp53-t34049261-cb3673dd], [16], [0.9245], [0.1071], [0.152 / 1.852],
[rho-neg], [pair20-crater], [icv1-fp53-t34050177-eee03cfb], [16], [1.2584], [0.1521], [0.307 / 2.203],
[rho-neg], [pair20-crater-twin(u=5)], [icv1-fp53-t34050177-213056d7], [16], [0.8512], [0.1360], [0.170 / 2.538],
[rho-neg], [pair20-floor], [icv1-fp53-t34050177-1fef7712], [16], [0.8581], [0.1083], [0.156 / 1.792],
[rho-neg], [pair20-floor-twin(u=5)], [icv1-fp53-t34050177-e0a3cfea], [16], [1.1293], [0.1420], [0.239 / 2.038],
)