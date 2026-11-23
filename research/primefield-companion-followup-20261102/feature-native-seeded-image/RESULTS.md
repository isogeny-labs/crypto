# Seeded image qualification

PCC-20261108-1bc6fc is unsigned producer evidence; independent signed review pending. [pcc:unverified]

Validation01 build, focused checks and inherited regression passed:

```
support_equalities=15264 true_images_retained=7560 wrong_sign_exclusions=7560 zero_exclusions=144 public_target_capacity_cases=2529 route_changes=180 call_changes=189 subsequent_updates=1995 single_call_cap_cases=280 invalid_signs=rejected phase_choices=2_of_4
```

The 15,264 state comparisons cover both sign choices, capacities 0/16/1024,
and radii 2/1/3/2, including repeated radii. Fields/m pairs are (17,4),
(41,5), (97,3), (193,3), (281,5), and (281,35). All scalars are used in the
first five; the last uses 0,1,2,3,7,17,280. Exact polynomial equality is checked
against gcd(old_image,Z^L-sign), plus branch canonical signatures. True images
are retained in 7,560 cases; wrong signs exclude them in 7,560; zero is excluded
in 144. Invalid sign inputs 0 and 2 are rejected. [pcc:unverified]

All 2,529 public cases recover correctly on E/F257: y²=x³+x+7,
P=(1,3), order281, m5: all281 targets at three lift caps (8/16/64) and three
power capacities (0/16/1024). These are capacity cases on one curve, not distinct
curve samples. The adapter's final elliptic-point verification remains active.
180 route differences and189 source-call-count differences are semantic trace
coverage, not a speed ranking.1995 subsequent updates execute. An additional
280 nonidentity targets with max_calls=1 stop after exactly one query. [pcc:unverified]

117 copied source files and131 compiler dependencies are hash-bound. Inherited
stdout is byte-identical to the planned-width reference. Initial metadata
capture failed because it resolved compiler dependency paths against caller cwd;
record.py now resolves against its own directory. The original metadata script
and error explanation are preserved. No compiler/native failure occurred.

No timing panel or cloud job ran. The original4563-case cohort is unchanged.
Source extraction and initial mode0 representation remain material costs.
The branch choice removes two of four phase-polynomial attempts; total-work
selection requires a frozen matched comparison. No new Lean proof is claimed;
the written argument is conditional on the existing phase/source invariants.
Historical full Cargo missing-fixture limitation remains.
