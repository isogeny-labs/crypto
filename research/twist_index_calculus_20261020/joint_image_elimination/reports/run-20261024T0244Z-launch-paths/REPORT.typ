#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Two verifier launches failed before execution")

#text("The first external-audit managed service retained its command and waited for the busy reservation, then failed to resolve cargo from systemd's default PATH. external-audit-v1/metadata/compile-tests.stderr records FileNotFoundError; exit is 1. No compiler, native test or external-basis audit ran in that attempt.")


#text("The first factored quartic proof launch, quartic-factored-lean-v5, failed while resolving lean, before constructing its proof-gate command or starting Lean. Its source snapshot and start timestamp are retained; the managed service journal records exit 1. This has no theorem verdict.")


#image("../../figures/joint-mechanism.png", width: 100%)

#text("The replacement managed owners are twist-joint-external-audit-v2.service and twist-joint-proof-quartic-factored-v6.service. Both explicitly supply the existing Cargo/Elan executable directories in PATH. Source statements, one-worker settings, 8 GiB scientific limits, 1,200 s external-audit watchdogs and 900 s proof watchdogs remain unchanged. Fresh output directories preserve the failed attempts. This is an infrastructure repair, not a repeated scientific sample or an improvement claim.")
