# Amendments to PROTOCOL.md (frozen 270353a57; PROTOCOL.md is unchanged)

Dated 2026-10-18, written before any cost cell was run (instances.jsonl was generated and
checked first). Each item states the reason.

- **A1 (instance verification cost).** Item 3 of "Instance verification" (`N * pi0(P) = O`)
  is checked for every rational point for `k <= 14` and for a seeded sample of about 20000
  points for `k >= 15`; "pi0(P) defined and on E0" is checked for every point at every `k`.
  Reason: a 63-bit scalar multiplication per point at `q ~ 2^21` costs about 10 min per
  instance on the oversubscribed host.
- **A2 (N2, N3).** `N2 = #C(F_{q^2})` and `N3 = #C(F_{q^3})` exhaustion were not run
  (no F_{q^2}/F_{q^3} root-finding code was written within budget). The L-polynomial is
  inherited from the construction; live evidence is `N1 = q+1` for every instance and the
  pushforward checks. Stated as a limit, as the protocol already allows.
- **A3 (rho walk).** Walkers continue after a distinguished point (standard parallel rho);
  a walker that re-reports an already stored `(a,b)` is on a cycle and is restarted.
  The protocol did not specify restarts. No negation map is implemented, so the rho
  figures are without it (conservative against rho by at most about sqrt 2).
- **A4 (budget rule).** A cell whose predicted wall (power fit of same-arm cells at lower
  `k`) exceeds 10 min runs on instance index 0 only; above 1 h it is skipped (and for the
  linear-algebra phase alone: relation phase measured, `rel_only`, LA extrapolated).
  Predictions use the last three measured `k`.
- **A5 (relation dedup).** The same line section reached from different pairs of its
  points is counted once (hash of the sorted four-point key); `dups` is recorded.
- **A6 (holdout).** The Wiedemann solve uses all but the last `max(20, 5%)` eliminated rows;
  those rows are checked afterwards together with the used rows.
- **A7 (rho code).** `pollard_rho.rs` / ecbench were not reused: the target group is
  `E0(F_{q^3})` with counted `F_q` arithmetic and batched `K`-inversion, which neither
  path provides. Repository sparse LA (`groebner_f4`, `jv_quartic`, ...) works over other
  rings; Wiedemann mod `ell` was written in the module as the protocol specifies.
