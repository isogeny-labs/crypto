# Additive supervisor v2 source review

Verdict: changes required before execution. The immutable v2 manifest is fe9b546029f700675665e667edb14c1405707c7573d18d6dc0a192974f7730d6. All16 listed file hashes match. No Docker/client/container/cloud or reviewed executable was launched.

The source changes address R1 fixture member binding; R3 exact runtime file/directory kinds, file lengths and hashes; R4 anonymous transient inspect stdout and failure cleanup; R5 added mount/namespace/device/capability/tmpfs checks; R6 executable uid/gid. Outer failure now correctly records unknown action state. These are source observations, not measured behavior or runtime qualification.

R2 remains incomplete at the launch-ownership boundary. supervise.py71 assigns proc only after subprocess.Popen returns, and72 sets unreaped_client=True afterward. The installed SIGINT/SIGTERM handler raises Stop asynchronously. An exception during Popen after process creation but before assignment can enter finally with proc=None and unreaped_client=False. The finalizer then has no process handle, and cleanup_one can proceed without the intended no-race guard. An interruption between71 and72 likewise leaves the flag false if termination subsequently fails. This is a source-level possible interleaving, not an observed failing run.

Required correction: defer termination signals into a flag across launch and ownership registration, checking it promptly after the handle is retained; or use a fully reviewed masking/ownership protocol that also handles child mask inheritance. Mark launch uncertainty before invocation and fail closed if a handle cannot be recovered. The corrected source must preserve bounded client reaping and distinguish client termination from container death. Signal checks must also stop before initiating another noncleanup command.

The prestart effective pids gate stays explicitly unresolved and qualification_complete remains false. This reporting is correct. Exact SecurityOpt names show requested/default selection, not independent proof of a kernel seccomp profile. The /opt archive remains a prospective scope extension, covers only /opt ancestry, and may fail closed on unrelated base-image members. Final image/client/build joins and prospective admission remain separate.

Review continues the v1 frozen source-only plan and six findings. Producer files are unchanged. An additive revision is required; this review does not approve unseen changes.
