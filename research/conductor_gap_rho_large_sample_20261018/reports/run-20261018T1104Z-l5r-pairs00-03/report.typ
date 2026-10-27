#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l5r-pairs00-03
Session `ECBS1h848777e5f858`, status `complete`, 192 records; spec `ECS1h1bbba5157ca7` sha256 `1bbba5157ca752b6…`; binary sha256 `ee0af723cf6c1064…`, git commit `20dba5f4c750ae4888415cf92cf395a2c94f6be0`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 0..3 of cm_orbits: crater vs floor vs crater twins, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101835, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l5r-pairs00-03/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair0-crater], [icv1-fp53-t33555417-7e2a3b7a], [16], [1.1938], [0.1530], [0.137 / 2.440],
[rho-neg], [pair0-crater-twin(u=3)], [icv1-fp53-t33555417-b31145d8], [16], [0.8292], [0.1091], [0.112 / 1.536],
[rho-neg], [pair0-floor], [icv1-fp53-t33555417-ade0b980], [16], [0.7869], [0.1101], [0.114 / 1.726],
[rho-neg], [pair1-crater], [icv1-fp53-t33568641-0d2a2a59], [16], [0.8474], [0.1098], [0.115 / 1.736],
[rho-neg], [pair1-crater-twin(u=3)], [icv1-fp53-t33568641-498ed63c], [16], [0.8512], [0.1419], [0.066 / 1.928],
[rho-neg], [pair1-floor], [icv1-fp53-t33568641-272bd5be], [16], [0.9350], [0.1230], [0.227 / 2.163],
[rho-neg], [pair2-crater], [icv1-fp53-t33600281-7acbe863], [16], [0.8873], [0.0994], [0.404 / 1.689],
[rho-neg], [pair2-crater-twin(u=3)], [icv1-fp53-t33600281-aad96b3d], [16], [0.8838], [0.1088], [0.390 / 1.958],
[rho-neg], [pair2-floor], [icv1-fp53-t33600281-8389a3a2], [16], [0.9810], [0.1090], [0.407 / 1.795],
[rho-neg], [pair3-crater], [icv1-fp53-t33623877-90323303], [16], [0.9800], [0.0903], [0.321 / 1.622],
[rho-neg], [pair3-crater-twin(u=3)], [icv1-fp53-t33623877-ee2584c0], [16], [0.9659], [0.1293], [0.051 / 1.771],
[rho-neg], [pair3-floor], [icv1-fp53-t33623877-87bc6afd], [16], [0.9442], [0.0720], [0.458 / 1.462],
)