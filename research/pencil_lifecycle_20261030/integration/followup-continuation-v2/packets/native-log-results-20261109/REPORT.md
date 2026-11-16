# Native redirected-log ownership qualification

The single native fixture recorded passed in 3.339560756983701 seconds; caller elapsed time was 3.408079123997595 seconds and exit was 0. Conservative accounting charges 60 active worker-seconds. No compiler or scientific target ran. Source publication is 800e454e809a079edd85a2c906b243b9a4d6ba3b; the exact fixture source SHA matches its reviewed 9bf9b24a binding.

## Observed file and ownership behavior

The caller created stdout and stderr files before systemd append redirection. Both actual files contain their exact 27-byte sentinels and have UID/GID 1000 and mode 0600. Worker and supervisor prefix records match each other and independent SHA256 hashes of the retained files. Prefix records explicitly retain sealed=false; a successful fixture does not change those fields into sealed manifests.

The fixture injected one checkpoint OSError after the actual extracted show function captured native ownership. Its receipt records ownership_survived_checkpoint_failure=true. Initial and worker invocation identities both equal 9a260668ffe94e34ae85562a76223904. The fixture then recorded owned_cleanup=true and terminal MainPID=0, inactive/not-found state. The captured cgroup is absent during this archival readback. No additional independent manager-journal replay was performed.

## Resource bounds and scope

Native controls recorded CPU quota100000/100000, memory134217728 bytes, zero swap and32 tasks. The unit used UID/GID1000, private network, read-only system/home and only the fixture output writable. Native runtime was bounded to35 seconds plus5-second stop grace; controller budget50 seconds and external55+5 seconds preserve the60-second allocation. CPU time and peak RSS were not recorded. No automatic retry occurred.

The archival check read actual log metadata/bytes, raw worker/result/caller records and the pinned fixture source. It found no mismatch and did not rerun the service. This result covers real redirected-log ownership/read/flush and retained ownership following one injected checkpoint failure. It does not establish compiler success, all supervisor faults or physical crash durability. The earlier failed full validation remains a separate immutable attempt and debit.

## Reviewable artifacts

The packet supplies CSV/JSON, SVG and this PDF. The diagram shows the executed ownership sequence; two equal-length sentinels are not a performance comparison. Nine raw native/caller files are archived with individual hashes. The content-addressed archive is2163 bytes, SHA2564cee4cc770d31093cbcbab072aac798e653e98db5d9b228ad7db8ed891baa669. Artifact upload, remote verification and a separate full GET/SHA check preserve the exact bytes. Use .artifacts.json and raw-manifest.json to retrieve and verify it.
