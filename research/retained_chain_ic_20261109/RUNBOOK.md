# Execution handoff

Stop before any paid build/trial until both admission blockers are resolved:
`scripts/checkpoint.sh gate` must succeed for this branch's complete resume
checkpoint, and shared daily/monthly balance, reservations and capacity must be
read live. The study's aggregate ceiling is USD 5, including failed builds and
jobs. Reuse the existing configured identity; no credential copying or workflow
permission changes are part of this study.

1. Commit and push source, protocol and specs. Run the maintained
   `check-experiment-git` check and retain its receipt. Record the exact 40-char
   source revision and protocol/spec SHA-256 hashes.
2. Use the maintained `isolab-experiment.yml` commit-building workflow with
   that explicit `source_revision`. A publish-only build is useful for the
   probe command. Record workflow revision/run ID, OCI digest and native
   `ecbench` binary hash independently. Building from an older default image
   would omit this command and is not an acceptable substitute.
3. Submit each diagnostic phase using the maintained command-job bridge and a
   digest-pinned image, `/usr/local/bin/ecbench retained-chain-study --phase
   probe|pipeline --out /work/NEW_DIR`. Freeze the exact request JSON and hash
   before launch. The command runner does not upload these files by itself:
   the submitted wrapper must preserve the exit status and upload the full
   output directory, including partial files, to the job's private S3 prefix
   before exiting. Freeze that wrapper with the request, too.
4. Use four CPUs, 4GiB, one concurrent job, original 1800-second deadline and
   zero retries. These command phases supply diagnostic counters, not L2
   timing admission. Check the original request/job after ambiguous submission;
   do not issue a newly named duplicate.
5. Submit `pipeline-m3.json` and `pipeline-m4.json` sequentially through the
   maintained native ecbench path with L2, one warmup, five measured rounds,
   matched reference and A/A, retaining the same source and public workload.
   Stop further submissions if the remaining study or shared budget is unknown
   or exhausted. Do not shrink a frozen panel to fit an unmeasured balance.
6. Retain launcher/request IDs, Job/Pod UIDs, manifests, every failed attempt,
   timeout and resource record. Download and hash-check private S3 outputs.
   Replay actual relation receipts with the existing useful-rank checker and
   check recovered scalars by multiplication. Timing requires the harness's
   independent isolation/replay/A/A gates; producer checks alone do not admit it.
7. Append receipts and a scoped report to this directory and the session log.
   Compare growth and complete pipeline costs in their recorded units; keep
   missing field/solver prices explicit. Submit measurements to PCC before
   promoting any claim. An unavailable run is not a mathematical negative.

See [the current admission record](admission.json). No image digest, cloud
request, S3 results prefix or execution receipt exists yet for this study.
