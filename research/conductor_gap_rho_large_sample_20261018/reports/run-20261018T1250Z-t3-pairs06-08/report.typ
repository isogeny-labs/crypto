#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t3-pairs06-08
Session `ECBS1hf203898d7cbc`, status `complete`, 192 records; spec `ECS1h8235395859ed` sha256 `8235395859edde2c…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 6..8: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101843, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t3-pairs06-08/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair6-crater], [icv1-fp53-t33756437-6aab8717], [16], [1.0832], [0.1320], [0.293 / 2.119],
[rho-neg], [pair6-crater-twin(u=5)], [icv1-fp53-t33756437-e110131e], [16], [0.9805], [0.1249], [0.168 / 1.966],
[rho-neg], [pair6-floor], [icv1-fp53-t33756437-405cd167], [16], [0.9802], [0.1455], [0.244 / 2.184],
[rho-neg], [pair6-floor-twin(u=5)], [icv1-fp53-t33756437-13031df7], [16], [0.9517], [0.1240], [0.143 / 2.084],
[rho-neg], [pair7-crater], [icv1-fp53-t33785081-1ffbd22f], [16], [0.8904], [0.1267], [0.066 / 2.041],
[rho-neg], [pair7-crater-twin(u=5)], [icv1-fp53-t33785081-6c0e92e7], [16], [1.0851], [0.1307], [0.134 / 2.474],
[rho-neg], [pair7-floor], [icv1-fp53-t33785081-aac0c67d], [16], [0.9703], [0.1204], [0.124 / 1.879],
[rho-neg], [pair7-floor-twin(u=5)], [icv1-fp53-t33785081-48de0bd1], [16], [0.9534], [0.1162], [0.118 / 1.722],
[rho-neg], [pair8-crater], [icv1-fp53-t33857097-f2d97cf9], [16], [0.7917], [0.1120], [0.249 / 1.706],
[rho-neg], [pair8-crater-twin(u=5)], [icv1-fp53-t33857097-f9c8d507], [16], [0.9411], [0.1454], [0.238 / 2.622],
[rho-neg], [pair8-floor], [icv1-fp53-t33857097-0874dbcd], [16], [0.8212], [0.1278], [0.092 / 2.059],
[rho-neg], [pair8-floor-twin(u=5)], [icv1-fp53-t33857097-9680835f], [16], [0.8467], [0.1426], [0.027 / 2.031],
)