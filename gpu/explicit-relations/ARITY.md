# Bounded factor-base arity through twelve slots

The standalone service, native reduction, CPU oracle and certificate parser now
accept2..12 ordered factor-base slots. The ordinary default remains m2; existing
frozen m8 panels keep their original source. C++ plan/device offset arrays have
13entries, and the Rust bounds share MAX_SLOTS=12. Tuple products, memory, work,
column and trial caps remain in force. More slots do not bypass admission.

This makes m10/m12 configurations available for a separate controlled scaling
panel. The reduction collects N+1 relations for N total slot-specific base
points, so smaller bases can reduce the number of relation queries. More slots
also increase tuple-construction work and can alter table sizes and coverage;
performance and memory need actual end-to-end measurements.

Paper checked: user-supplied Akshima et al., Section3.2 Lemmas9–10 and
Section3.4/Figure2. Lemma10's sufficient coverage condition is
L=(r-1)tau/s >= max(r^(1/m),10m). The automatic threshold already respects this
condition and is unchanged. An explicit threshold such as2ceil(r^(1/m))+2 can
fall below10m for larger m at these field sizes; it is an empirical configuration,
not an application of that sufficient coverage bound. Exact successful
certificates still prove their recorded relations and recovered scalar. Future
panels must preserve all incomplete cells and state the selected threshold.

Checks added: CPU/GPU service differentials through12slots, empty reprepare,
13-slot rejection, device query boundaries9/10/12, and a public known-answer
Rust certificate fixture with separate repeated slots, exact dependency,
truncated relation rejection and excessive-arity rejection. Larger service
fixtures use two points per slot to keep the correctness suite bounded.
Host service syntax/self-test is run locally; Rust/device validation follows on
the next separately pinned GPU build. No measurement or speed claim is recorded
for this extension yet.
