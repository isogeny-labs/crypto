from pathlib import Path
import json,hashlib,html,textwrap,subprocess
p=Path(__file__).parent
report='''Runtime qualification readiness — read-only snapshot

Observed 2026-10-31 01:32 UTC. Five retained artifacts match their recorded SHA256 and byte size: controller ZIP, runtime archive, controller, progress helper and fresh owner. This readback binds local bytes; it does not establish a currently deployed image or remote idleness.

Local tools: Docker 25.0.14 is installed. Access to the explicit /var/run/docker.sock is denied: mode0660 root:docker, current user ec2-user lacks docker group. docker info returned exit0 with empty template output AND a permission error; server availability, local images and live containers remain unobserved. No privilege change, daemon action or container action was attempted. Rust1.99.1 has the x86_64-unknown-linux-musl target installed. The private qualified musl-gcc wrapper exists; it is not on ordinary PATH. Root retains build ownership.

Pinned ingredients: Dockerfile.ecbench pins AWS CLI base sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39, copies ecbench, fresh owner, build-info.json and smoke-prime.json, and sets USER1000:1000. Runtime overlay adds /opt/isolab/recovery and sets USER0:0. Fresh owner belongs to the base image. Owner SHA256 f6f6016e549ab19d9e92a7a89a5a89d095de068775dd003ab5666217726b48b7 was rechecked. This upstream base pin is not a final ecbench image digest.

Minimum dependency sequence
1. Root reconciles its build owner/cache, freezes exact source and runs its retained supervised musl build. Bind actual binary SHA256, toolchain, linker-selected CRT/libc and build-info. Existing compiler qualification is not final ecbench qualification.
2. Freeze final-image qualification commands, caps, fixed software fixtures and expected outputs prospectively. Establish an authorized local container inspection/build context; the current socket permission observation is not permission to expand access. Preserve existing pinned owner and controller inputs.
3. Assemble only the reviewed four-file base context and explicit runtime overlay. Inspect actual final-image file hashes, static x86-64 ELF/interpreter/NEEDED, owner identity, metadata, users and custody permissions. Verify progress0755 in the final image: archive0755 previously became image0700 after extraction/COPY; archive headers alone do not qualify the image. Run the approved bounded uncredentialed interface/lifecycle checks only after their protocol is frozen.
4. Record immutable final base/runtime digests and source/binary/owner/runtime joins. New ZIP7791cec7... and archive2c10901a... differ from the earlier explicitly approved ZIP0bd84d40... and archive72b227c0... . Reviewed publication authorization for those replacement hashes remains a separate requirement; no publication is performed here.
5. Before any scientific execution, reconcile remote owners/jobs, complete actual execution/session bindings, cumulative accounting and prospective independent review. Reuse the retained nine-input inventory rather than regenerate it. A static inventory result does not admit a prepared session.

Authorization: retained limit is25200 active worker seconds and absolute deadline2026-10-31T06:00:00Z, recorded as planning_limits_only with scientific_execution_admitted=false. Remaining worker allowance is unknown here and requires authenticated cumulative receipts. No deadline extension, new retry or changed-instrument gate is inferred. Earlier failures and partial rows remain retained.

Missing observations/bindings: current final ecbench source/binary/build-info, final base/runtime image digests, actual final-image qualification, immutable replacement object versions, remote owner/idleness and remaining budget. They are null in the retained checklist; root may have later work in progress, which this snapshot neither interrupts nor declares complete. Environment-status tool is unavailable; /etc/codex/network-policy.json is absent. No network action was attempted.

Scope: read-only local inspection and artifact hashing only. No builds, downloads, jobs, controller operations, cloud queries, publication or scientific trials. Source bindings and raw command receipts accompany this PDF; no full-cohort cost ratio is reported.
'''
(p/'REPORT.md').write_text(report)
status={'schema':'research.runtime-readiness/v1','observed_at':'2026-10-31T01:32:53Z','scope':'read-only local tools and retained artifact custody','retained_artifacts_matching':5,'docker_client':'25.0.14','docker_server_observed':False,'docker_info_exit0_is_success':False,'musl_target_installed_toolchain':'1.99.1','final_image_qualified':None,'remote_state_reconciled':False,'remaining_worker_seconds':None,'authorization_worker_seconds':25200,'authorization_deadline':'2026-10-31T06:00:00Z','authorization_scope':'planning_limits_only','scientific_execution_admitted':False,'full_cohort_cost_ratio':None,'missing_bindings':['current final ecbench source/binary/build-info','final base and runtime image digest','actual final image qualification','replacement object versions and scoped publication approval','remote owners and remaining cumulative allowance'],'receipts':'local-observations.json','retained_readback':'retained-artifacts-readback.json','source_bindings':'source-bindings.json'}
(p/'status.json').write_text(json.dumps(status,indent=2)+'\n')
lines=[]
for para in report.splitlines(): lines.extend(textwrap.wrap(para,113) or [''])
height=90+len(lines)*18
svg=f'<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="{height}" viewBox="0 0 1000 {height}"><rect width="100%" height="100%" fill="white"/><g fill="#152536" font-family="DejaVu Sans" font-size="14">'
for n,line in enumerate(lines):svg+=f'<text x="35" y="{40+n*18}">{html.escape(line)}</text>'
svg+='</g></svg>'
(p/'readiness.svg').write_text(svg)
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'REPORT.pdf'),str(p/'readiness.svg')],check=True)
manifest={x.name:hashlib.sha256(x.read_bytes()).hexdigest() for x in sorted(p.iterdir()) if x.is_file() and x.name!='manifest.json'}
(p/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(hashlib.sha256((p/'manifest.json').read_bytes()).hexdigest())
