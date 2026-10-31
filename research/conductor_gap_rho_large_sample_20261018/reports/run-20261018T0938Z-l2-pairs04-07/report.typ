#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l2-pairs04-07
Session `ECBS1h22da8004014e`, status `complete`, 192 records; spec `ECS1h877682e54175` sha256 `877682e541755942…`; binary sha256 `ee0af723cf6c1064…`, git commit `d290558b2657bd320ea2a9964a91238924619763`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 4..7 of cm_orbits: crater vs floor vs crater twins, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101822, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l2-pairs04-07/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair4-crater], [icv1-fp53-t33684497-ecd1e0df], [16], [0.8473], [0.1289], [0.217 / 1.990],
[rho-neg], [pair4-crater-twin(u=3)], [icv1-fp53-t33684497-434b27f3], [16], [1.0001], [0.1233], [0.313 / 2.117],
[rho-neg], [pair4-floor], [icv1-fp53-t33684497-3523039e], [16], [0.9016], [0.1301], [0.261 / 1.641],
[rho-neg], [pair5-crater], [icv1-fp53-t33705281-dcfbcce7], [16], [0.9839], [0.0770], [0.427 / 1.593],
[rho-neg], [pair5-crater-twin(u=3)], [icv1-fp53-t33705281-769fbfc4], [16], [1.0544], [0.1728], [0.206 / 2.789],
[rho-neg], [pair5-floor], [icv1-fp53-t33705281-298d41bc], [16], [0.7182], [0.1072], [0.091 / 1.459],
[rho-neg], [pair6-crater], [icv1-fp53-t33756437-6aab8717], [16], [0.9597], [0.0887], [0.587 / 1.771],
[rho-neg], [pair6-crater-twin(u=3)], [icv1-fp53-t33756437-601c2640], [16], [0.9473], [0.1097], [0.209 / 1.714],
[rho-neg], [pair6-floor], [icv1-fp53-t33756437-405cd167], [16], [1.0068], [0.0987], [0.215 / 1.696],
[rho-neg], [pair7-crater], [icv1-fp53-t33785081-1ffbd22f], [16], [0.9392], [0.1432], [0.107 / 2.495],
[rho-neg], [pair7-crater-twin(u=3)], [icv1-fp53-t33785081-48525048], [16], [0.7422], [0.1036], [0.215 / 1.654],
[rho-neg], [pair7-floor], [icv1-fp53-t33785081-aac0c67d], [16], [0.8317], [0.1297], [0.306 / 2.492],
)