# Root v4 permit qualification: failed fixture construction

One released invocation exited with code 1 after 0.064546608 seconds. The complete caller receipt and stderr were retained. The process was reaped; no cache snapshot was launched and no retry occurred. The full 60-second reservation is charged, bringing campaign charges to 16320 seconds.

The traceback identifies qualify-permit.py line 64: class DT assigns datetime=DateTime and then evaluates timezone=datetime.timezone. Python resolves datetime to the preceding class-local DateTime binding, which has no timezone attribute. This is a fixture construction error after the worker cases. The published source places fourteen test calls before line 64; reaching this traceback implies those calls returned. This is a control-flow inference, not fourteen saved case records. Supervisor cases and final serialization were not reached.

The native result directory was never created. Twenty-four cases were planned, but no completed case rows were durably emitted; completed cases are recorded as unknown rather than zero or inferred successes. There is no complete permit qualification result from this attempt.

Exact source publication: cd96a80ff2033731335d2f58a75257b902037357. Published source blobs were compared with local qualify source, worker, launcher, plan and pins before invocation. Limits were 512 MiB address space, 1 MiB per file, internal 50-second alarm, outer 55-second TERM and 5-second kill allowance. The caller retained publication/release/ledger hashes, command, times, exit and logs. The archive contains all four caller files; no native result is fabricated.
