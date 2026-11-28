# smoke, not data

Smoke test of the L4 instrument (2026-11-22, Phase 1). Nothing here is a result and none of it may be
quoted as one. Commands (all single core, total CPU well under 60 s):

    python3 controls.py --k 16 --n 2000 --n3 200000 --tag smoke --out smoke --predict
    python3 controls.py --k 20 --n 2000 --n3 200000 --tag smoke --out smoke --predict
    python3 controls.py --k 9  --n 500  --n3 50000  --tag smoke --out smoke --predict
    python3 run_l4.py exhaustive --k 9  --tag smoke --out smoke          # toy curve, p = 1021, |Z| = 3
    python3 run_l4.py exhaustive --k 16 --rows 400 --tag smoke --out smoke   # 400 of 65437 rows (4 s)
    python3 run_l4.py sample --k 16 --n 200000 --tag smoke --out smoke
    python3 run_l4.py sample --k 20 --n 200000 --tag smoke --out smoke
    python3 run_l4.py sample --k 20 --variant given --n 200000 --tag smoke --out smoke
    python3 run_l4.py sample --k 9 --n 100000 --tag smoke --out smoke
    python3 analyze_l4.py --dir smoke --tag smoke --ks 9,16,20 --label "(smoke, not data)"

What the smoke shows about the instrument (not about the claim): the toy curve k = 9 (not a family
member; p = 1021, three rational 2-torsion x) was run exhaustively and all analysis checks, including
the exact phi_t prediction of the Z-rooted pair counts, ran end to end; the positive controls and the
wrong-A control returned the expected values at k = 9, 16, 20; throughput is about 6.5e6 pairs/s
(400 exhaustive rows of k = 16 in 4.1 s).
