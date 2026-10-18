#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t4-pairs09-11
Session `ECBS1h322f62266589`, status `complete`, 192 records; spec `ECS1hf3396776418d` sha256 `f3396776418dc8e4…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 9..11: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101844, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t4-pairs09-11/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair10-crater], [icv1-fp53-t33917801-d0ac2d7c], [16], [1.1349], [0.1354], [0.449 / 2.586],
[rho-neg], [pair10-crater-twin(u=5)], [icv1-fp53-t33917801-84641058], [16], [0.9003], [0.0996], [0.197 / 1.635],
[rho-neg], [pair10-floor], [icv1-fp53-t33917801-08a3e7f3], [16], [0.9899], [0.1003], [0.226 / 1.748],
[rho-neg], [pair10-floor-twin(u=5)], [icv1-fp53-t33917801-d81b9f4b], [16], [0.9561], [0.0911], [0.333 / 1.701],
[rho-neg], [pair11-crater], [icv1-fp53-t33938381-e2744f8f], [16], [1.1472], [0.1202], [0.301 / 1.800],
[rho-neg], [pair11-crater-twin(u=5)], [icv1-fp53-t33938381-b598355a], [16], [0.8942], [0.0859], [0.321 / 1.433],
[rho-neg], [pair11-floor], [icv1-fp53-t33938381-1e2228d2], [16], [0.9613], [0.1239], [0.252 / 2.158],
[rho-neg], [pair11-floor-twin(u=5)], [icv1-fp53-t33938381-cfc759ee], [16], [0.7964], [0.1414], [0.058 / 2.018],
[rho-neg], [pair9-crater], [icv1-fp53-t33913601-631b71ee], [16], [0.9501], [0.1176], [0.268 / 1.794],
[rho-neg], [pair9-crater-twin(u=5)], [icv1-fp53-t33913601-d697860c], [16], [0.9770], [0.1226], [0.407 / 2.072],
[rho-neg], [pair9-floor], [icv1-fp53-t33913601-523b5d72], [16], [1.0698], [0.1163], [0.360 / 2.083],
[rho-neg], [pair9-floor-twin(u=5)], [icv1-fp53-t33913601-b20a246d], [16], [1.0294], [0.1027], [0.377 / 1.691],
)