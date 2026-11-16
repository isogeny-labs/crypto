#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t8-pairs21-23
Session `ECBS1ha33acc0192cc`, status `complete`, 192 records; spec `ECS1h8c20f1e39b8d` sha256 `8c20f1e39b8d30ec…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 21..23: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101848, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t8-pairs21-23/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair21-crater], [icv1-fp53-t34058157-767b2a00], [16], [1.1875], [0.1392], [0.229 / 2.667],
[rho-neg], [pair21-crater-twin(u=5)], [icv1-fp53-t34058157-45e3e1da], [16], [0.7984], [0.1067], [0.160 / 1.481],
[rho-neg], [pair21-floor], [icv1-fp53-t34058157-da7474d1], [16], [1.1331], [0.1603], [0.176 / 2.394],
[rho-neg], [pair21-floor-twin(u=5)], [icv1-fp53-t34058157-81f18da7], [16], [1.2106], [0.1266], [0.255 / 2.318],
[rho-neg], [pair22-crater], [icv1-fp53-t34093521-383bf2fa], [16], [1.0949], [0.1480], [0.066 / 2.060],
[rho-neg], [pair22-crater-twin(u=5)], [icv1-fp53-t34093521-d38745e7], [16], [1.0044], [0.1102], [0.367 / 1.834],
[rho-neg], [pair22-floor], [icv1-fp53-t34093521-8e2561e6], [16], [1.0934], [0.1266], [0.100 / 1.866],
[rho-neg], [pair22-floor-twin(u=5)], [icv1-fp53-t34093521-d54fc311], [16], [0.9814], [0.1400], [0.128 / 2.239],
[rho-neg], [pair23-crater], [icv1-fp53-t34106297-c2a974f9], [16], [1.1282], [0.1347], [0.316 / 2.257],
[rho-neg], [pair23-crater-twin(u=5)], [icv1-fp53-t34106297-15465236], [16], [1.0696], [0.1406], [0.096 / 2.009],
[rho-neg], [pair23-floor], [icv1-fp53-t34106297-9acec86d], [16], [1.0730], [0.1229], [0.334 / 2.058],
[rho-neg], [pair23-floor-twin(u=5)], [icv1-fp53-t34106297-ffb5628a], [16], [0.9191], [0.1305], [0.145 / 2.054],
)