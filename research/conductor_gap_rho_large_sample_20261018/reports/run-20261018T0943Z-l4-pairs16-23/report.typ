#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l4-pairs16-23
Session `ECBS1h9e9d5e792405`, status `complete`, 256 records; spec `ECS1h64158436c7bf` sha256 `64158436c7bf1d19…`; binary sha256 `ee0af723cf6c1064…`, git commit `53661c8a41fc9b46f4e32e73a77f9549a2f00605`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 16..23 of cm_orbits: crater vs floor, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 16 curves × 16 `planted_shared` targets, target seed 2026101824, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 256}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l4-pairs16-23/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair16-crater], [icv1-fp53-t34024341-e3c0702b], [16], [1.0949], [0.1689], [0.156 / 2.473],
[rho-neg], [pair16-floor], [icv1-fp53-t34024341-0eea4a4a], [16], [1.0710], [0.1649], [0.110 / 2.251],
[rho-neg], [pair17-crater], [icv1-fp53-t34040357-35c8b067], [16], [1.1482], [0.1275], [0.424 / 2.322],
[rho-neg], [pair17-floor], [icv1-fp53-t34040357-f237e2be], [16], [0.8680], [0.1344], [0.016 / 2.002],
[rho-neg], [pair18-crater], [icv1-fp53-t34042037-e54d747f], [16], [0.7724], [0.1156], [0.274 / 1.683],
[rho-neg], [pair18-floor], [icv1-fp53-t34042037-202608c9], [16], [1.0102], [0.1226], [0.319 / 2.039],
[rho-neg], [pair19-crater], [icv1-fp53-t34049261-62ff07a5], [16], [0.9035], [0.0909], [0.431 / 1.723],
[rho-neg], [pair19-floor], [icv1-fp53-t34049261-aad1a4df], [16], [0.8869], [0.1310], [0.142 / 1.848],
[rho-neg], [pair20-crater], [icv1-fp53-t34050177-eee03cfb], [16], [1.0314], [0.1770], [0.076 / 2.429],
[rho-neg], [pair20-floor], [icv1-fp53-t34050177-1fef7712], [16], [0.8059], [0.1162], [0.133 / 1.520],
[rho-neg], [pair21-crater], [icv1-fp53-t34058157-767b2a00], [16], [1.2008], [0.1788], [0.121 / 2.558],
[rho-neg], [pair21-floor], [icv1-fp53-t34058157-da7474d1], [16], [0.9348], [0.1267], [0.247 / 2.179],
[rho-neg], [pair22-crater], [icv1-fp53-t34093521-383bf2fa], [16], [1.0109], [0.1326], [0.324 / 1.950],
[rho-neg], [pair22-floor], [icv1-fp53-t34093521-8e2561e6], [16], [0.8125], [0.1443], [0.181 / 2.443],
[rho-neg], [pair23-crater], [icv1-fp53-t34106297-c2a974f9], [16], [1.0593], [0.1129], [0.258 / 1.632],
[rho-neg], [pair23-floor], [icv1-fp53-t34106297-9acec86d], [16], [1.0937], [0.1342], [0.459 / 2.333],
)