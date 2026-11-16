# Protocol: lifts arms on the screened isogeny seeds (frozen before the arms run)

2026-11-15. Extends `curve25519_isogeny_seed_screen_20261115` (PR #4233): that study ran the blind arm for the 8 highest (top) and
8 lowest (bottom) on-curve-count seeds among candidates 21 to 60 (`groups.json`, committed before any engine run). Here: the same 16 seeds
at `lifts=1`. Same instrument and conditions: `curve25519_family f4cost 20 isogeny@SEED 5 3 20000 2026111403 8000 2 full 1 30000000 0 0`,
binary sha256 `ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`, k = 20 miniature (p = 1048573, A = 926), t = 5, G = 3,
m = 2, 20,000 targets, target seed 2026111403. All results [pcc:unverified]. 16 arms. No rho arm.

Reference (unscreened, PR #4213): pooled `lifts=1` rate over seeds 1 to 20 = 2.864 hits per 1e9 mul-adds [2.727, 3.007]
(1620 hits / 565.5e9); lifts cost per target is 1.4e6 mul-adds, about 8x the blind arm's 1.77e5.

## Fitted model (fixed now, seeds 0 to 20 `lifts=1` arms only)
Poisson rate hits = c x (mul-adds in 1e9) x base_points^k, maximum likelihood over k in steps of 0.05: **k = 2.05, c = 0.0022857**.
(The blind arms of the same seeds gave k = 1.65, whose upper end the screen study then exceeded; this protocol fixes the lifts exponent before the lifts screen data.)

## Hypotheses (directional, fixed now)
- H17 (screening separates under lifts): top-group pooled `lifts=1` hits per 1e9 mul-adds exceed the bottom group's, one-sided exact conditional binomial p < 0.05.
- H18 (screened top beats unscreened under lifts): top-group rate has its Poisson 95% lower bound above 2.864.
- H19 (lifts remain neutral on screened seeds): pooled over the 16 seeds, the rate ratio `lifts=1` / blind (blind arms from the screen study) has a two-sided exact conditional binomial p of at least 0.05. If it falls below 0.05, neutrality is not general.
- H20 (the lifts model transfers): for each group, observed lifts hits lie inside the exact Poisson 95% count interval of the model's expected count (c x sum of exposure x base_points^2.05).
- H21 (cost shape): lifts mul-adds per target differ by less than 10% between the 16 arms' largest and smallest. (Seeds 0 to 20 spanned 1.4121e6 to 1.4160e6.)
Decision: supported only if the stated bound holds; otherwise not supported, reported as such. Five hypotheses, no correction.

## Descriptive
Per-arm hits, rates, group ratio top/bottom, Spearman of on-curve count vs arm rate over the 16 arms, the lifts/blind rate ratio per group,
and the model's observed/expected per arm. Cost accounting: lifts costs 8x per target, so rates per mul-add (not per target) are the comparison, as in PR #4157.

## Out of scope
Other t, k, G, other candidate seeds, a rho arm, GAE-band pricing of seed selection, a mechanism test. No Conductor task, isolab run or S3
checkpoint (deterministic counted mul-adds; the AWS route is absent here).
