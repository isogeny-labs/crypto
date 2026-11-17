#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l7r-pairs08-15
Session `ECBS1h360f692db80f`, status `complete`, 256 records; spec `ECS1h29aaf2a6b267` sha256 `29aaf2a6b2676c28…`; binary sha256 `ee0af723cf6c1064…`, git commit `20dba5f4c750ae4888415cf92cf395a2c94f6be0`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 8..15 of cm_orbits: crater vs floor, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 16 curves × 16 `planted_shared` targets, target seed 2026101837, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 256}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l7r-pairs08-15/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair10-crater], [icv1-fp53-t33917801-d0ac2d7c], [16], [0.7779], [0.1052], [0.255 / 1.777],
[rho-neg], [pair10-floor], [icv1-fp53-t33917801-08a3e7f3], [16], [0.6634], [0.0887], [0.219 / 1.513],
[rho-neg], [pair11-crater], [icv1-fp53-t33938381-e2744f8f], [16], [0.7028], [0.0993], [0.064 / 1.476],
[rho-neg], [pair11-floor], [icv1-fp53-t33938381-1e2228d2], [16], [0.8058], [0.0961], [0.062 / 1.290],
[rho-neg], [pair12-crater], [icv1-fp53-t33958457-7a874e74], [16], [0.8224], [0.1570], [0.150 / 2.573],
[rho-neg], [pair12-floor], [icv1-fp53-t33958457-98c5c4ea], [16], [1.1107], [0.1398], [0.174 / 2.143],
[rho-neg], [pair13-crater], [icv1-fp53-t33970721-ad200cc4], [16], [0.8975], [0.1296], [0.144 / 2.140],
[rho-neg], [pair13-floor], [icv1-fp53-t33970721-364fe217], [16], [1.0803], [0.1324], [0.360 / 1.991],
[rho-neg], [pair14-crater], [icv1-fp53-t33982257-15905736], [16], [1.2006], [0.1268], [0.269 / 2.084],
[rho-neg], [pair14-floor], [icv1-fp53-t33982257-f6869d8a], [16], [0.8091], [0.0792], [0.408 / 1.426],
[rho-neg], [pair15-crater], [icv1-fp53-t34001297-62a8f2e9], [16], [0.9938], [0.1198], [0.157 / 1.765],
[rho-neg], [pair15-floor], [icv1-fp53-t34001297-62508329], [16], [1.0344], [0.1297], [0.511 / 2.308],
[rho-neg], [pair8-crater], [icv1-fp53-t33857097-f2d97cf9], [16], [1.0047], [0.1312], [0.076 / 2.096],
[rho-neg], [pair8-floor], [icv1-fp53-t33857097-0874dbcd], [16], [0.9912], [0.1147], [0.292 / 1.724],
[rho-neg], [pair9-crater], [icv1-fp53-t33913601-631b71ee], [16], [1.0984], [0.1462], [0.158 / 2.081],
[rho-neg], [pair9-floor], [icv1-fp53-t33913601-523b5d72], [16], [0.7499], [0.1076], [0.155 / 1.691],
)