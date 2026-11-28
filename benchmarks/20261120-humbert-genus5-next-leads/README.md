# Genus-5 Humbert cover: next leads and a matched rho (2026-11-20)

Study: `research/humbert_genus5_cover_20261021/next_leads_20261120` (PROTOCOL.md frozen before data;
REPORT.typ / report.pdf). Bench record: `benchmarks/records/20261120T2130Z-humbert-genus5-next-leads.json`.

The campaign was 150 runs on toy fields. All IC and matched-rho runs solved; the 7 failures are the
old scalar rho stopping at its 2e8-step cap. SETUP_FAST makes setup 0.06× and t_ic 0.65–0.79×,
with counted fields identical on 30/30 runs. dlpf at alpha 0.25–0.35 is 0.83× of alpha 0.5 at
q = 32003. The matched 4-thread rho runs at about 14× the scalar rho's steps per second. Against
it, the best IC arm takes 1.68, 1.19 and 0.77 of rho's expected time at q = 16001, 24001 and
32003. Against the old scalar rho that ratio was 0.04–0.09. L0, unverified assertion
[pcc:unverified]. Not a leaderboard or README row: there is no VALIDATION.json (validation gate).
