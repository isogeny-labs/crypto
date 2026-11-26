# Admission execution details, before implementation

The published draft remains unchanged. This prospective addendum fixes the
native program's input, control census and exact classification before its
first source/build/run. No admission or solver result is asserted.
[pcc:unverified]

Input: twelve TSV rows, ordered case0..5 then seed101/202. Each row contains
id,case,seed,p,a,b,r,Gx,Gy,Qx,Qy from a-case-seed-0's original certificate.
Recovered scalars and timings are omitted. Pin each complete source archive
and the projected input by SHA256; keep originals unchanged. Metadata
projection uses Node only for field extraction/serialization. The C parser
requires exact row identities/counts and canonical decimal unsigned fields.
[pcc:unverified]

The checker returns a certified full-order result, a specific mathematical
rejection, or unknown because a resource/interval condition is insufficient.
Infrastructure/OpenSSL operation failures abort the attempt. Check the
p/r<2^48 resource limit before arithmetic. In that domain, use exact trial
division, canonical/on-curve tests, explicit point addition/doubling and
integer Newton square root. Configure no generator or advertised order in
OpenSSL; assert that its group has no supplied order or finite generator.
Do not call EC_POINT_mul. The Hasse/order result remains justified by the
established mathematical theorems, not a claim of full C/curve Lean refinement.
[pcc:unverified]

Controls, all before the twelve real rows:

- One aggregate square-root postcondition control for integers0..65535.
- Three composite controls561,2047,3215031751, with known divisors3,23,151.
- All144 draft small parameter cells. Direct x/y enumeration includes infinity;
  a literal small-prime table is the independent order-primality oracle.
  Singular cells reject as singular; orders below3 are out of range; composite
  claimed orders reject specifically; prime orders with insufficient upper
  interval remain unknown. Other prime-order cells require an exact certificate.
- Two literal controls: anomalous p7,a0,b5,r7,G=Q=(3,2) certifies order but
  must be flagged anomalous/ineligible; p7,a0,b0 rejects as singular.
- Eight mutants for each of six domains, based on its seed101 header:
  field9p,order9r,wrong prime order3,G=(0,0),Q=(0,0),Gx+p,
  infinity G and infinity Q. Check b!=0 and mutation-range preconditions.
  Require each exact rejection cause; identity controls extend the draft
  prospectively and retain nonidentity as a separately exercised condition.
- One oversized-field control p=2^49 returns unknown/resource with zero
  primality and point work; do not classify it as an arithmetic rejection.

Thus199 control rows precede12 real rows; one final native summary gives212
JSONL rows on full success. This is a prospective census, not an observation.
Small-curve and mutant rows remain distinct from real fixtures. On any
unexpected result, stop, retain output and report unattempted real rows.
[pcc:unverified]

Only certified orders receive trace, anomaly/supersingularity, j0/j1728 and
embedding-degree1..99 flags. The exceptional-class census is descriptive.
Core acceptance requires all12 full-order/point certificates and every
control expectation; it does not assume that every fixture has every
benchmark-eligibility flag. No new solver call, target generation or timing
comparison is part of this execution. [pcc:unverified]

One compile/native worker;600s phase,60s native, unchanged sampled
15032385536-byte whole-cgroup stop. Strict C11/O2 compiler warnings are errors.
Freeze source,input,compiler/library identity and qualified copied binary
before the sole census invocation. Exclusive raw-output creation prevents
overwriting or relabeling a prior attempt. Local only, no S3. [pcc:unverified]
