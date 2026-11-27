#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= ECC2K-130 half-trace setup: baseline profile
TZH-20261123-baseline-profile. Two fresh identical baseline A/A invocations, fixed n131/d12 uniform query. Source065176db56; original binary7a1ffd3115e52049. This is absolute hotspot discovery, not an improvement comparison.

#table(columns: 2, [Invocation], [Whole cold simulated instructions], [Baseline], [44,038,405], [Identical A/A], [44,038,405])

Both outputs match the fresh native result and retain the known fixed-query disposition. Unsigned producer statement PCC-20261123-467720. [pcc:unverified]

#image("profile.svg", width: 100%)

The full inclusive annotation attributes9,844,704 instructions (22.35%) to basis-image construction. Inclusive costs include descendants; they are not summed with the self-cost bars above. General half-trace queries already use a cached byte map; this remaining cost constructs the map by squaring chains. [pcc:unverified]

Intel Xeon Platinum8573C; Valgrind3.24.0; CPU0,2GiB address-space cap,600second combined guard, zero retries or paid resources. Profile elapsed times are not native-runtime measurements.

Evidence: profile.protocol.json, profile-export-final.json, checks/profile-receipt.json and both complete framework profile directories. Complete raw output is retained in PROFILE-RAW.tar.xz. The exact baseline numerical/harness source matched its retained source inventory before profiling. No candidate data selected this profile.
