#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l3-pairs08-15
Session `ECBS1hdf6f37886907`, status `complete`, 256 records; spec `ECS1hd840e09aa8c2` sha256 `d840e09aa8c2b6ce…`; binary sha256 `ee0af723cf6c1064…`, git commit `d290558b2657bd320ea2a9964a91238924619763`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 8..15 of cm_orbits: crater vs floor, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 16 curves × 16 `planted_shared` targets, target seed 2026101823, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 256}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l3-pairs08-15/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair10-crater], [icv1-fp53-t33917801-d0ac2d7c], [16], [1.0283], [0.1345], [0.066 / 2.090],
[rho-neg], [pair10-floor], [icv1-fp53-t33917801-08a3e7f3], [16], [1.0389], [0.1206], [0.190 / 1.723],
[rho-neg], [pair11-crater], [icv1-fp53-t33938381-e2744f8f], [16], [1.0881], [0.1485], [0.120 / 2.236],
[rho-neg], [pair11-floor], [icv1-fp53-t33938381-1e2228d2], [16], [0.9146], [0.1047], [0.149 / 1.584],
[rho-neg], [pair12-crater], [icv1-fp53-t33958457-7a874e74], [16], [0.9961], [0.1178], [0.321 / 1.720],
[rho-neg], [pair12-floor], [icv1-fp53-t33958457-98c5c4ea], [16], [0.6663], [0.1002], [0.041 / 1.440],
[rho-neg], [pair13-crater], [icv1-fp53-t33970721-ad200cc4], [16], [1.2165], [0.1514], [0.265 / 2.025],
[rho-neg], [pair13-floor], [icv1-fp53-t33970721-364fe217], [16], [0.8586], [0.1246], [0.212 / 1.839],
[rho-neg], [pair14-crater], [icv1-fp53-t33982257-15905736], [16], [0.9402], [0.1299], [0.105 / 1.750],
[rho-neg], [pair14-floor], [icv1-fp53-t33982257-f6869d8a], [16], [0.7590], [0.1022], [0.174 / 1.618],
[rho-neg], [pair15-crater], [icv1-fp53-t34001297-62a8f2e9], [16], [1.0989], [0.1267], [0.298 / 1.997],
[rho-neg], [pair15-floor], [icv1-fp53-t34001297-62508329], [16], [0.9196], [0.1133], [0.211 / 1.829],
[rho-neg], [pair8-crater], [icv1-fp53-t33857097-f2d97cf9], [16], [0.9327], [0.1569], [0.193 / 2.485],
[rho-neg], [pair8-floor], [icv1-fp53-t33857097-0874dbcd], [16], [0.8284], [0.1099], [0.311 / 1.718],
[rho-neg], [pair9-crater], [icv1-fp53-t33913601-631b71ee], [16], [0.9627], [0.1297], [0.221 / 2.291],
[rho-neg], [pair9-floor], [icv1-fp53-t33913601-523b5d72], [16], [0.9504], [0.1215], [0.241 / 2.307],
)