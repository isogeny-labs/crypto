# V2 prospective corrections

V1 remains immutable and unexecuted. Independent review found that malformed/unreadable/missing-status compiler receipts could skip required post custody, and the systemd wait-client lacked a bounded supervisor wait. V2 changes only operational supervision and unique workspace/unit identities. All native mathematical controls and custody input checks remain unchanged. [pcc:unverified]

Receipt reading now catches errors, records a failed compiler row and proceeds to post custody when the service is confirmed terminal and budget remains. A missing or changing InvocationID, unknown cgroup, or active service blocks the next phase and records incomplete custody. [pcc:unverified]

A bounded wait reserves12 seconds for supervisor cleanup. Each phase retains its900/600/900-second ceiling; the service RuntimeMax is15 seconds below that ceiling, stop grace is explicitly5 seconds, and GNU timeout sends TERM20 seconds below the ceiling with5 seconds until KILL. Queries and stop requests have2-second bounds within the phase. The wait-client process group is separately reaped; that action never proves service termination. Stop requests require the captured unchanged InvocationID. [pcc:unverified]

Native replay source differs from V1 only in fresh persistent workspace identity. Its own phase work boundaries, complete file/runtime checks, four compiler calls,30 axiom names, parser and negative controls remain intact. V2 outer and native directories, and unit names, are fresh; there is still one attempt and no automatic retry. Prior300-second charge and46,800-second campaign with2026-11-06T04:00Z deadline remain unchanged. [pcc:unverified]

This is source preparation only. Publication and independent prospective review are required before execution; no V1 or V2 continuation was run. [pcc:unverified]
