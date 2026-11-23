# Native N83 continuation at800000conflicts

Serial K2048/K4096, each up to800000conflicts/4800seconds. Same retained
public query and indexed worker; [frozen protocol](../../verification/native-natural-800k-20261020/PROTOCOL.md).
Original query a=0 icv1-f2m83-tm6151469093347-debefd74, field x^83+x^45+x²+x+1,
subgroup2417851639230796216685689; seed2026100801, m5, identitymask0.

Observed n=1 per arm; descriptive native solver and supervised outer clocks.
The resumed EFS host and restored immutable Docker image are recorded;
no matched-host comparison, base ranking, fastest claim, UNSAT conclusion,
unknown-scalar recovery or end-to-end IC/rho result follows.
Initial v12 attempts at4GiB both ended with recorded OOMKilledtrue/exit137,
last nonterminal789000/702000conflicts. Neither has terminal models/relations/rank.
MEMORY-V13.md separately freezes one8GiB/no-extra-swap repeat per base, retaining
all other caps and gates; maximum cumulative24000worker seconds across5attempts.
These resource envelopes stay distinct and original failures are preserved.
Strict reservation fullcore14/30/runCPU14;1CPU/4GiB/no extra swap,
networknone/read-only/capdropALL/no-new-privileges/64PIDs. No heavy task work
within either measured arm. Actual quiet/eviction/resource/cleanup and
card/director records govern admission; fixed kernel threads and absent worker
context-switch/fault counters preclude automatic L2 classification.

Guard-v14 22unit+7execution-card tests pass. Fresh serial release root-library
validation:4407pass,0fail,118ignored (644.23seconds); exporter19pass and
independent model-checker2pass. All run after timing in a new target; the retained
measured worker is unchanged. Earlier logs and their socket-denial/rerun
qualifications remain historical evidence. Canonical cold/rho/identity/timeline/leaderboard
graphs checked and unchanged because no new promotion satisfies their gates.

[PDF](report.pdf), [graph SVG](cap-continuation.svg), [150dpi PNG](cap-continuation.png),
[recorded native summary](figure-data.json), [complete raw S3 publication](../../verification/native-natural-800k-20261020/PUBLICATION.md).

| run | audit | terminal conflicts | models | relations | rank | solver ms | outer ms |
|:--|:--|--:|--:|--:|--:|--:|--:|
| k2048-800k-4g-oom | OOM_CENSORED_NOT_ADMITTED | null | null | null | null | null | 2856001.340487 |
| k4096-800k-4g-oom | OOM_CENSORED_NOT_ADMITTED | null | null | null | null | null | 2575759.292233 |
| k2048-800k-v13-declaration-refusal | FAILED_NO_AUDIT | null | null | null | null | null | null |
| k2048-800k-8g | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | 800000 | 0 | 0 | 0 | 2866110.318261 | 2871741.354358 |
| k4096-800k-8g | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | 800000 | 0 | 0 | 0 | 2902661.855852 | 2913487.794923 |
