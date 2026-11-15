# Private supervisor / qualifier interface

The private supervisor launches this exact source-bound callback inside the fresh qualification service, passing INVOCATION_ID equal to service_invocation_id:

```
python3 -B supervise.py --image-id sha256:... --binary-sha256 ... \
  --binding /absolute/admitted-binding.json --receipts /absolute/fresh-receipts \
  --private-endpoint /absolute/private-endpoint.json \
  --private-endpoint-sha256 <exact receipt SHA-256>
```

Endpoint schema software.private-runtime-endpoint/v1, mode qualification. Required keys: owner, socket_path, socket_uid, socket_gid, socket_mode, socket_device, socket_inode, daemon_id, daemon_pid, daemon_start_ticks, supervisor_pid, service_unit, service_invocation_id, service_cgroup, cgroup_parent, source_binding_sha256, image_id, input_manifest_sha256, assembly_receipt_sha256, expires_at. Socket is a nonsymlink root:root 0600 UNIX socket in root-owned 0700 directory. Endpoint values join private_owner, final_source_manifest_sha256, image_id, fixture_manifest_sha256 and private_assembly_receipt_sha256 in the separately admitted qualifier binding. Expiry is UTC Z and cannot exceed absolute_expiry.

The supervisor must prepare an empty delegated cgroup_parent strictly beneath service_cgroup, with cpu/memory/pids enabled in cgroup.subtree_control, and move its processes/daemon/callback to process leaves beneath that service. Its daemon explicitly uses cgroupfs. Each Docker create also specifies this exact cgroup parent. Docker info must match daemon ID, cgroupfs and cgroup v2. Every command rechecks socket inode/device/owner/mode, daemon PID start identity, daemon/supervisor/qualifier containment, invocation ID and expiry. Both Docker CLI and curl/archive metadata use the same endpoint; no shared socket, ambient selectors or sudo fallback.

Before tested execution, the newly derived no-fork gate publishes its existing PID1 receipt and waits at most 15 seconds for /work/gate-release. The qualifier checks the container host PID/start ticks, strict cgroup path ancestry, cgroup inode/device join to the gate receipt, and effective host pids.max/current/procs. It repeats identity/ancestry, then copies exactly that container's nonce plus newline as a root-owned 0444 regular file into existing /work tmpfs. The gate accepts only complete exact token and metadata; incomplete archive extraction or mismatch never releases and waits only within the original deadline. A preexisting token rejects before receipt. SIGINT/SIGTERM exit126. After release it rechecks effective PID controller values before exec. No container exec helper or second PID is used.

The endpoint receipt is dynamic local startup evidence, not runtime admission or image admission. Both admitted and qualification_admitted remain false in this draft. gate binary/control/proposal/review pins remain null. The new gate control receipt must bind gate_source_sha256 and held_release_controls_passed=true; old gate evidence cannot satisfy this join. The retained head_contract.py hash remains mandatory.
