# Materializer T1953: completed preparation

The native worker reports passed after 573.0646983869956 seconds, with compiler_invoked=false. Conservative accounting charges 900 active worker-seconds. It produced materialized-inputs.json SHA256 c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2. This is source/vendor/cache preparation, not compilation or a scientific cohort.

## Recorded contents and controls

The worker's files counter is 20,762. The generated manifest contains 18,714 immutable entries and 2,133 initial-cache entries; these are separate recorded quantities, not interchangeable unique-file counts. Native byte counters are source 187,387,559; vendor 260,412,537; cache 2,384,395,266. No compiler was invoked. Source revision is29861c9a55f9ae5a33e1e09b3866aa397fbde605. Exact launch argv and publication cd63196e3b0a190e3463a6e6369a55bb370043e0 are preserved in release.json.

Live initial readback and native receipt agree on CPU quota200000/100000, memory4,294,967,296 bytes, zero swap and512 tasks. Native inherited AS/DATA limits qualified. Internal work limit880 seconds, systemd885, outer timeout895+5, total accounting900. There was one attempt and no automatic retry. Systemd CPUUsageNSec was51,790,499,000; this is separate from worker wall time and conservative debit. No peak RSS measurement is asserted.

## Receipt and custody readback

The final cleanup journal has4,482 records and730,566 bytes. Its complete SHA matches the pointer, all rows equal the final inline array, and no cleanup row reports errors. Final receipt and input-manifest hashes match terminal.json. No unresolved includes or worker errors are recorded. This archival readback did not rehash all workspace/cache data or rerun preparation. Subsequent root validation checks its admitted immutable inputs.

Initial and terminal invocation44fced24fb3947ceb0813e3fde7c4ba7 and boot31b37cfb-1562-42ff-8b0f-fadc5cec5a12 match. At terminal capture, RemainAfterExit left ActiveState=active, SubState=exited, MainPID=0, Result=success and ExecMainStatus=0; the captured cgroup was absent. The later post-stop record shows not-found/inactive. We retain this exact sequence rather than treating active/exited as a running worker or relying on later absence alone.

## Archive and next boundary

The content-addressed native archive is1,085,289 bytes, SHA256 fc5e751b3ffb1e162327924435c047331394c52be2ca356c57732747c1a46d88. It is stored under s3://setec-astronomy-artifacts/store/sha256/fc/fc5e751b3ffb1e162327924435c047331394c52be2ca356c57732747c1a46d88. A separate full GET matched this hash. .artifacts.json and .gitignore retain the repository reference. raw-manifest.json lists every native archived file. Use ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull with this packet path.

The separately bound root-library validation may now use the exact produced manifest after its own source/release checks. This preparation does not report test success, runtime qualification or cohort cost ratios. Historical T1716 vendor timeout and all prior charges remain unchanged.
