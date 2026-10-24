#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t1-pairs00-02
Session `ECBS1hb5804e908dc5`, status `complete`, 192 records; spec `ECS1hb703d2a1fb25` sha256 `b703d2a1fb2573aa…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 0..2: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101841, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t1-pairs00-02/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair0-crater], [icv1-fp53-t33555417-7e2a3b7a], [16], [1.0047], [0.1478], [0.156 / 2.001],
[rho-neg], [pair0-crater-twin(u=5)], [icv1-fp53-t33555417-df9f887d], [16], [0.8820], [0.0878], [0.325 / 1.382],
[rho-neg], [pair0-floor], [icv1-fp53-t33555417-ade0b980], [16], [0.9430], [0.1479], [0.133 / 2.314],
[rho-neg], [pair0-floor-twin(u=5)], [icv1-fp53-t33555417-a316c602], [16], [1.0778], [0.1500], [0.168 / 2.488],
[rho-neg], [pair1-crater], [icv1-fp53-t33568641-0d2a2a59], [16], [0.9054], [0.1149], [0.137 / 1.680],
[rho-neg], [pair1-crater-twin(u=5)], [icv1-fp53-t33568641-f5bdba39], [16], [0.8737], [0.1371], [0.139 / 2.256],
[rho-neg], [pair1-floor], [icv1-fp53-t33568641-272bd5be], [16], [1.0025], [0.1495], [0.160 / 1.905],
[rho-neg], [pair1-floor-twin(u=5)], [icv1-fp53-t33568641-0dc00973], [16], [1.2297], [0.0834], [0.804 / 1.957],
[rho-neg], [pair2-crater], [icv1-fp53-t33600281-7acbe863], [16], [0.8676], [0.1059], [0.306 / 1.592],
[rho-neg], [pair2-crater-twin(u=5)], [icv1-fp53-t33600281-881f27fd], [16], [0.9421], [0.1389], [0.317 / 2.359],
[rho-neg], [pair2-floor], [icv1-fp53-t33600281-8389a3a2], [16], [0.7610], [0.0945], [0.171 / 1.494],
[rho-neg], [pair2-floor-twin(u=5)], [icv1-fp53-t33600281-89b4b73e], [16], [1.0864], [0.1276], [0.392 / 2.084],
)