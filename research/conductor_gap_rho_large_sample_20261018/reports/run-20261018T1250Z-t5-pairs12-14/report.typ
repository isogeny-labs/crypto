#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t5-pairs12-14
Session `ECBS1h49d14c6fd0f0`, status `complete`, 192 records; spec `ECS1h1cf87d894169` sha256 `1cf87d894169780e…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 12..14: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101845, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t5-pairs12-14/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair12-crater], [icv1-fp53-t33958457-7a874e74], [16], [0.7619], [0.1011], [0.182 / 1.745],
[rho-neg], [pair12-crater-twin(u=5)], [icv1-fp53-t33958457-6860e9a3], [16], [1.0900], [0.1567], [0.282 / 2.174],
[rho-neg], [pair12-floor], [icv1-fp53-t33958457-98c5c4ea], [16], [0.7738], [0.1035], [0.178 / 1.586],
[rho-neg], [pair12-floor-twin(u=5)], [icv1-fp53-t33958457-451cbad3], [16], [0.7807], [0.1237], [0.118 / 1.807],
[rho-neg], [pair13-crater], [icv1-fp53-t33970721-ad200cc4], [16], [1.0573], [0.1397], [0.361 / 2.382],
[rho-neg], [pair13-crater-twin(u=5)], [icv1-fp53-t33970721-225ab591], [16], [0.8467], [0.1036], [0.246 / 1.511],
[rho-neg], [pair13-floor], [icv1-fp53-t33970721-364fe217], [16], [0.7478], [0.1249], [0.291 / 1.882],
[rho-neg], [pair13-floor-twin(u=5)], [icv1-fp53-t33970721-9578eb6d], [16], [0.8611], [0.1217], [0.219 / 1.997],
[rho-neg], [pair14-crater], [icv1-fp53-t33982257-15905736], [16], [1.0029], [0.0895], [0.358 / 1.637],
[rho-neg], [pair14-crater-twin(u=5)], [icv1-fp53-t33982257-bbf1e6dd], [16], [0.9348], [0.1181], [0.041 / 1.824],
[rho-neg], [pair14-floor], [icv1-fp53-t33982257-f6869d8a], [16], [1.0253], [0.1572], [0.100 / 2.224],
[rho-neg], [pair14-floor-twin(u=5)], [icv1-fp53-t33982257-d7ca5205], [16], [0.7644], [0.1182], [0.137 / 1.733],
)