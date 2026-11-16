#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 K = 16,627 prebuilt signed-Frobenius base

*Date:* 2026-10-17 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The frozen construction protocol is `PROTOCOL.md`; the separate memory-peak
recovery was preregistered in `PEAK_RECOVERY_PROTOCOL.md`. Full commands,
source and object digests, raw receipts and limitations are in
`RUN_REPORT.md`.

#image("construction-diagram.svg", width: 100%)

== Native construction and checks

#table(
  columns: (auto, auto, auto),
  inset: 6pt,
  [Phase], [Measured record], [Outcome],
  [First construction], [34.164 s L0 internal], [16,627 columns; 2,760,082 points],
  [Byte-identity repeat], [46.440 s L0 internal; 507,568,128 B peak], [exact object match],
  [Generic replay], [31.134 s container wall; 834,940,928 B peak], [PASS on every point],
  [S3 upload/download], [83,701,045 compressed bytes], [digest match; PASS],
)

The public `public_x_hash` policy used seed `2026100801`, with 83
Frobenius phases and both signs. Both builds used the same native arm64
Linux binary, one CPU, 4 GiB memory, zero swap, no network and hard wall
caps. The compressed BLAKE3 is
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
The replay used generic multi-limb arithmetic rather than the producer's
`Gf2_128`, on the same host and repository. The native uploader downloaded
the versioned S3 object and rehashed it. The first construction's memory
peak was not recorded; the displayed peak belongs to its byte-identical
repeat. Shared-host L0 wall times do not establish a speed comparison.

== Claim boundary and next gate

K = 16,627 is the first declared size where the uniform-target m = 4
multiset counting ceiling reaches one and becomes uninformative. This run
measured no natural relation, rank increment, target extraction or complete
cold IC cost. The next gate is a native m = 4 capacity check on this exact
object with a degree-83 planted relation positive control, followed by a
matched base and same-point rho comparison. The IC scoreboard is unchanged.
