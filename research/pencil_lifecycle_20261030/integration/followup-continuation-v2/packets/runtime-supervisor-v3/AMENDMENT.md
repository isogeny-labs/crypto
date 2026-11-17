# Supervisor v3 launch ownership correction

v2 remains immutable. Independent source review found that a raising signal handler could interrupt Popen before proc assignment and ownership marking, allowing container cleanup while a launched Docker client was untracked.

v3 defers SIGINT/SIGTERM to a pending-signal flag. It marks client ownership uncertain before invoking Popen, then checks the pending flag only after proc assignment. The command loop and next non-cleanup command boundary also check the flag. Cleanup commands can proceed after a known client has been reaped. If Popen raises before returning a process object, ownership stays uncertain and container cleanup is prohibited rather than inferred safe. The code never installs a child-inherited signal mask; it preserves the explicit child environment and session launch.

Only Python AST parsing was performed. No command client, container or supervisor was executed. The original pre-start effective PID gate remains unresolved and qualification_complete remains false. Final bindings remain null and admission false. This delta requires independent source review.
