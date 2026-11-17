#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: t6-pairs15-17
Session `ECBS1h191ff34b0f26`, status `complete`, 192 records; spec `ECS1hbcf435eaad36` sha256 `bcf435eaad362e63…`; binary sha256 `ee0af723cf6c1064…`, git commit `5bd27ac67fe62a0c963b0f9586969a5a139a2ee4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 15..17: crater, crater twin, floor, floor twin (u=5), 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101846, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/t6-pairs15-17/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair15-crater], [icv1-fp53-t34001297-62a8f2e9], [16], [0.8776], [0.1276], [0.004 / 1.741],
[rho-neg], [pair15-crater-twin(u=5)], [icv1-fp53-t34001297-6ffefd71], [16], [0.9539], [0.1475], [0.202 / 2.318],
[rho-neg], [pair15-floor], [icv1-fp53-t34001297-62508329], [16], [1.1459], [0.1540], [0.263 / 2.194],
[rho-neg], [pair15-floor-twin(u=5)], [icv1-fp53-t34001297-6a3ae236], [16], [1.0796], [0.1433], [0.272 / 2.475],
[rho-neg], [pair16-crater], [icv1-fp53-t34024341-e3c0702b], [16], [1.0623], [0.1133], [0.321 / 2.046],
[rho-neg], [pair16-crater-twin(u=5)], [icv1-fp53-t34024341-bcad485a], [16], [0.8866], [0.1561], [0.154 / 2.566],
[rho-neg], [pair16-floor], [icv1-fp53-t34024341-0eea4a4a], [16], [0.7610], [0.1223], [0.227 / 1.870],
[rho-neg], [pair16-floor-twin(u=5)], [icv1-fp53-t34024341-9c10679e], [16], [1.0229], [0.1228], [0.257 / 1.790],
[rho-neg], [pair17-crater], [icv1-fp53-t34040357-35c8b067], [16], [0.8953], [0.0942], [0.358 / 1.598],
[rho-neg], [pair17-crater-twin(u=5)], [icv1-fp53-t34040357-5a22db31], [16], [1.0501], [0.1335], [0.310 / 2.120],
[rho-neg], [pair17-floor], [icv1-fp53-t34040357-f237e2be], [16], [0.8677], [0.1275], [0.067 / 2.228],
[rho-neg], [pair17-floor-twin(u=5)], [icv1-fp53-t34040357-9b464e0f], [16], [1.0262], [0.1350], [0.376 / 2.532],
)