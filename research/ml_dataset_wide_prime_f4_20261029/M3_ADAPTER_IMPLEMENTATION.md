# Supervised wide m3 adapter implementation

Implementation increment following frozen `M3_ADAPTER_CONTEXT.md` (545625c7d8a).
Native prerequisite is b2867d1. This document reports source implementation and
AUTHORED tests only; this adapter author has not built or run any tests, fixtures,
CLI gates, scientific draws or cloud jobs. Parent owns execution and review.
The complete m2..15, solver/family/policy/corpus objective remains unfinished.

## Source and protocol boundary

`solvers.rs` dispatches supervised prime m3 before the legacy high-arity branch.
Small-prime and no-context behavior are unchanged. The original chain input is
reconstructed by the existing builder without changing schema, strategy, original
narrow-visitor metadata or unrestricted u0 domain. Four-variable native conversion
packs all four coordinates; no tower rules or truncated higher-m coordinates.

`prime_f4_wide_m3.rs` owns typed m3 result/extraction records and four-exponent
basis rows. The sibling m2 module exposes only dimension-neutral admission,
serialization, native diagnostics, descriptor and claim helpers. Its typed m2
input/result/extraction protocol is unchanged. The existing m2 capability remains;
a separate supervised_wide_prime_m3_profile describes this increment.

Paths and schemas:

* prime-f4-input.json: unchanged ml-dataset.prime-f4-chain-input/v1.
* prime-f4-wide-m3-result.json: crypto.ml-dataset.wide-prime-f4-m3-result/v1,
  role restricted-wide-prime-f4-m3-result/v1.
* prime-f4-wide-m3-events.jsonl: existing typed native boundary shape, role
  restricted-wide-prime-f4-m3-boundary-events/v1.
* prime-f4-wide-m3-group-extraction.json:
  crypto.ml-dataset.wide-prime-f4-m3-extraction/v1, role
  restricted-wide-prime-f4-m3-group-replay/v1.
* pdp-witness.json: existing independently-verified-pdp certificate with three
  ordered summands, target, sum and replay_group_add_calls=3.

Exporter unchanged; consumer compatibility and CLI gates require independent
validation. Native basis derivation, complete execution traces, total instructions,
scalar recovery, ECDLP completion and fall degrees remain unverified/unknown.

## Exact bounded extraction and retention

The total group-add ceiling is config.work_limit, INCLUDING both per-triple adds
and all first-certificate replay adds. The additional triple ceiling
min(config.work_limit,1024) is not a separate additive work allowance. Native work
and conservative storage reservations keep their independently frozen limits.
Original-equation evaluation is fixed by the admitted five polynomials, <=125
terms, four coordinates and <=32 degree, and is counted explicitly per evaluation.
Each affine row evaluates five polynomials; the first affine certificate repeats
five evaluations. Repeated tuples are allowed and order is lexicographic k-fastest.

Before a triple, require two remaining group calls and at least4096 bytes in the
extraction arena after encoding the current prefix. This is a worst-case row plus
counter-growth reservation, not a post-computation estimate. For admitted <=61-bit
coordinates, a row's four decimal coordinates, five decimal residues, at most one
full two-coordinate group point, three bounded indices, tags and booleans fit well
within4032 bytes; producer and verifier additionally check that limit. The64-byte
slack covers punctuation and bounded counter growth. A partial row has fewer
fields. These are reusable live-arena permissions, not cumulative allocated bytes
or RSS. Actual retained bytes are separately sealed in artifact descriptors.

The extraction prefix arena is adapter_bytes/8; its final serialization allowance
adds64KiB terminal metadata. Witness and canonical input each have independent
64KiB caps. Native event bytes have min(adapter_bytes/8,256KiB), final result uses
only remaining aggregate adapter space. adapter_bytes is
min(original trace_bytes/32,4MiB), with >256KiB required for profile admission.
All serialized descriptors, including whitespace, count toward the same aggregate
allowance before replay parses any mathematical payload. Final basis retention
can be omitted under that budget and is explicitly marked incomplete.

A triple records either Affine{coordinates,residues} or
ExceptionalInfinityPrefix. Infinity carries no fabricated auxiliary coordinate or
residue list. total/matches_target are both absent only after a first-add deadline
stop. Group arithmetic and BigUint original-polynomial evaluation are separate
from native elimination. All-zero affine residues can contradict native unit;
an exceptional group match cannot. A full negative needs every signed triple,
including infinity prefixes, both additions, and no matching candidate at all.

The first matching triple reserves three further group calls before certificate
replay. Deadline checks occur before each add and the original deadline is never
reset. Partial replay adds remain counted; they cannot certify a positive or a
negative. Traversal continues after the first successfully replayed certificate.
The certificate cursor remains stable, and later caps do not erase it. Later
exceptional matches therefore survive even when an affine match occurred first.
Bounded live events retain completed triples and stop/partial-stage information.
A live refusal stops further work/submissions and retains its first bounded error.
Staged canonical artifacts remain available for worker finalization even when live
persistence failed; this is not a durability claim until storage completes.

Producer status is verified only for a positive certificate without persistence
failure; otherwise partial. solver_completed is positive or full negative, gated
by persistence. Result extraction_complete and extraction.domain_complete encode
full signed-domain traversal. The proposed redundant aliases group_domain_complete,
group_prefix_complete, group_infinity_prefixes_covered and first_certificate_retained
are NOT emitted in this version, and cannot be promoted via arbitrary outer labels.
Counts of started/completed/affine/exceptional/matching triples are exactly derivable
from strict rows; actual group/equation/replay counters are also retained and checked.

## Central verification and censoring

The m3 verifier runs before the old m2 verifier and generic chain reconstruction.
Config/context, reserved paths/roles and promoted claims activate it independently
of strategy or shared-input presence. Public/base/context must be uniquely bound;
public family/n/m/twist must equal frozen config. Canonical input is compared in
full to reconstructed input. Per-file and aggregate byte checks precede parsing.
The verifier replays ordered row indices, prefix branches, original equations,
group totals, matches, exact counters, first-certificate selection and all-full
criterion. It recomputes producer preadmission for every retained prefix.

An explicit WidePrimeReplay::CensoredM3Prefix disposition is available only after
outer censor/no-promoted-claim guards, raw native-role checks and byte admission.
Central manifest hashes are checked before this call. This disposition permits
later chain reconstruction to skip truncated shared-input bytes. Registered
mathematical/producer evidence cannot masquerade as this raw exception. Registered
censored runs instead require bound result.json, replay their completed producer
evidence, and apply forbidden-claim and classification checks to both outer and
retained outcomes. Ordinary pre-solver errors have a separate narrow retained-error
exception with no solver files, domain or witness evidence.

## Authored controls; execution pending

Fixed seed42/curve7/generator1/key1, n47/53/61, both twists, base P,-P:
3P positive,4P complete negative with actual point-order>7 precondition, and P
showing an earlier affine match followed by a matching infinity prefix. Tests
require actual native invocation, eight retained triples, original-equation and
group replay; no replacement seed or target search.

Group budget exact/one-less controls, retained positive under later cap,
deterministic injected clock boundaries before/between adds and certificate replay,
initial persistence refusal, typed branch/cursor/counter/certificate mutations.
Central stored-cell tests rehash public dimensions, auxiliary domains, result
claims and extraction counters, strip strategy with combined omission or oversized
shared input/events, omit dependencies, mutate retained outcomes, preserve a real
truncated raw-censored prefix, and reject promoted outer fields. Parent performs
builds/tests, independent source review and CLI/publication gates before claiming
this implementation is validated or producing a scientific corpus.

## Independent source review correction A–F

The parent executed immutable e3fbd87b36f focused validation:282 passed,0 failed,
1 ignored (19 native,5 Fp64,232 dataset,26 checked); this is focused evidence,
not a full-suite claim. All seven initially authored tests executed there.
Independent review subsequently identified six replay-envelope gaps. This separate
correction requires the full three-add permit for partial/deadline certificate
replay, binds effective outcome counters exactly, reconstructs the actual
pre-next-row byte shortage, binds exact solver-domain descriptor/value after work,
rejects admission-refusal labels under an admitted context, and binds acknowledged
terminal counters exactly. Nonterminal boundary/result differences remain allowed.
Additional tests are authored, not yet executed at this correction's publication.
The first-match dedicated live-certificate persistence issue remains a separate
follow-up and is not silently treated as solved by these semantic replay changes.

## Dedicated first-certificate persistence follow-up

A separate source change now uses one bounded compact witness_payload constructor
for immediate live publication and later canonical staging. After successful
three-add and affine-equation replay, it publishes the dedicated certificate
before any subsequent triple. A live refusal preserves the first cursor and
staged certificate, records the sticky failure and stops further work. Existing
flush behavior prevents retries after that failure; successful duplicate live
publication is byte-identical and idempotent in CaptureEmitter.

Authored control wraps the real bound CaptureEmitter, requires the file already
exist when a later triple callback fails, and checks identical canonical bytes
and duplicate publication. A real zero-payload admission refusal stops at the
first verified candidate with no later event submissions. The central control
calls the actual collector on an early nested live/payloads/pdp-witness.json,
checks opaque native classification, preserves the censored truncated prefix,
and rejects a promoted positive outer label. These additional controls have not
been run by this author. See M3_FIRST_CERTIFICATE_PERSISTENCE.md for the design.
