# Independent completed-square certificate replay

Changed source from v2: the orientation negative control now substitutes the opposite valid characteristic root, exercising selected-kernel rejection rather than the unsupported-root guard. The mathematical verifier and five existing input certificates are unchanged. This is a changed software qualification, not an unchanged retry or a repeated scientific scan. All five positive replays and fifteen altered norm/label/orientation controls pass, without clock collection or minimization.

V1 failed compiling because Boost headers were absent. The replacement uses GMP 6.2.1; dependency installation receipts are retained under v2. V2 passes with preliminary controls; v3 is the final qualification. Per-invocation limits remain 120 seconds, one process/thread and 2 GiB address space. No cloud submission or new job was launched.
