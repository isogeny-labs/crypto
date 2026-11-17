# L11: Semaev arm A at n=35/37 (PARTIAL, interrupted)
Toy GF(2), N1 generator and gf2close d=4. No security claims. PROTOCOL.md was written before the runs.
- n=35: 1 UNSAT A draw (s=2) gives **one=0**, low4=3885 = n(3N-5)+8n = 3605+280, dim4=61425 < 66712. This is the first A draw that does not reach 1 in W_4 on this generator, so P1's ">=7/8" is now in doubt. C1 s=0 stalls at 3605 = n(3N-5), so P3 holds on 1 draw.
- Only 1 of the planned 8 draws per arm finished. The n=35 runner died. The n=37 runner started but had produced no closures when the session ended. macdump round-3 falls were not run. CPU per closure is about 65 s at n=35, so the 45-minute cap is not binding.
- Blocker: the session ended before the jobs finished. To resume, run `src/run.py 35`, then `src/run.py 37`, then `src/rnd3.py n A s` (raw files append).
