# Native log fixture source review

T-1973: bounded native log fixture source supported; no blocking defects identified. Exact harness9bf9b24a and launcherda57c822 pins match. No fixture or compiler executed during review.

The fixture reaches actual caller-created log files, systemd append redirection, UID/GID1000 worker reads and helper fsync. It exercises the extracted owner-capture function with a deliberate post-capture checkpoint failure and retains the invocation before cleanup. Only the same captured owner may be stopped. Empty/absent captured cgroup and terminal process state are required before pass.

One CPU,128MiB,zero swap,32tasks; native35-second maximum plus5-second stop,50-second controllerbudget and55+5 outertimeout fit the60-second allocation. Unresolved initial capture or interrupted controller remains failure requiring root reconciliation, not a pass or name-only stop.

This checks the concrete ownership regression and capture ordering. It does not qualify compiler workloads or all native faults. Root may follow the already authorized publication and serial release sequence after the current payload is terminal. Source hashes, exact findings and boundaries are in review.json.
