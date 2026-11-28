# R134 handoff

- Four ordinary base places yield 20 unramified lambda places with degree/count groups: P11/11 `(4,4)`; P11/55 `(1,4),(3,4)`; P22/44 `(4,4)`; P22/88 `(2,2),(6,2)`.
- Result SHA-256 `ea8b73bb49965f801dec664c16e004861c686081ca3c41387fe5f2e12b089fce`; weighted contribution 3168. Independent arithmetic, byte replay, changed-input rejection and Lean degree checks passed.
- Run `python3 research/koblitz_modular_ic_20261018/rounds/134/lambda134.py research/koblitz_modular_ic_20261018/rounds/134` and `python3 research/koblitz_modular_ic_20261018/rounds/134/verify134.py`.
- Next: Hasse/Newton normalization above P11/y=1242 and P22/y=1612991. Then compute N51 at all normalized lifts. The full x-fiber still has 528 weighted degrees unresolved.
