#!/usr/bin/env python3
"""Single-use advice-only control metadata; native helper hashes/advises files.

No scientific, performance or harness computation. Resource arithmetic is exact
admission metadata. Immediately after the sole native call, read only proc/cgroup
metadata and save it before any software/library/input/Git custody re-read.
"""
import datetime
import json
import os
from pathlib import Path
import subprocess

OWNER = Path('/workspace/crypto-native-smallroot')
PROGRAMME = Path('/workspace/crypto-autoresearcher')
BASE = PROGRAMME / 'coordination/goals/GOAL-PFDR-fd09e0/batches/BATCH-ee839d/continuations/DEC-20261029-03e304'
TASK = OWNER / 'experiments/joint-smallroot-native/spec/checks/TASK-20261029-6f4f28'
CONTROL = TASK / 'advice-control'
GATE = BASE / 'admission/TASK-20261029-6f4f28/advice-admission.json'
PROOF = Path('/workspace/smallroot-session/20261029-q4-current-lease.json')
GIT = ['git', '-c', 'gc.auto=0', '-c', 'maintenance.auto=false']
THRESHOLD = 12884901888

def timestamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def digest(path):
    return subprocess.check_output(['sha256sum', '--', str(path)], text=True).split()[0]

def checked_reference(entry, root=None):
    path = Path(entry['path'])
    if not path.is_absolute():
        assert root is not None
        path = root / path
    assert digest(path) == entry['sha256'], str(path)
    assert path.stat().st_size == entry['bytes'], str(path)
    return dict(path=str(path), bytes=entry['bytes'], sha256=entry['sha256'], match=True)

def put(name, value):
    with (CONTROL / name).open('x') as f:
        f.write(value if isinstance(value, str) else json.dumps(value, indent=2) + '\n')

def ownership():
    proof = json.loads(PROOF.read_text())
    now = datetime.datetime.now(datetime.timezone.utc)
    assert proof['task_id'] == 'ee90db91-e8c0-4f26-b029-9c5dedf4060a'
    assert proof['lease_id'] == 'de65fada-0fdb-4ec3-8d5f-4f8793fe2064'
    assert proof['task_ref'] == 'T-6' and proof['fencing_epoch'] == 1
    assert proof['live'] and proof['expected_principal'] and proof['released_at'] is None
    assert datetime.datetime.fromisoformat(proof['expires_at']) > now
    assert 0 <= (now-datetime.datetime.fromisoformat(proof['observed_at'])).total_seconds() < 75
    claim_path = BASE / 'claims/TASK-20261029-6f4f28.1.claim.json'
    claim = json.loads(claim_path.read_text())
    assert claim['owner'] == 'executor-native-q4case-6f4f28'
    assert claim['session'] == 'native-q4case-20261029' and claim['epoch'] == 1
    assert datetime.datetime.fromisoformat(claim['expires_at'].replace('Z', '+00:00')) > now
    assert not (BASE / 'claims/TASK-20261029-6f4f28.1.release.json').exists()
    return dict(recorded_at=timestamp(), conductor=proof, goal_claim=claim)

def resource_snapshot():
    # This function launches no subprocess and reads no software/candidate file.
    # All modules and callable code are resident before the native advice call.
    pid = os.getpid()
    proc = Path('/proc') / str(pid)
    snapshot = dict(started_at=timestamp(), observer_pid=pid,
                    process_cgroup=(proc/'cgroup').read_text(),
                    observer_proc_stat=(proc/'stat').read_text(),
                    observer_identity='\n'.join(x for x in (proc/'status').read_text().splitlines()
                        if x.startswith(('Uid:', 'Gid:', 'Cap', 'Seccomp', 'NoNewPrivs:'))) + '\n')
    mounts = [x for x in (proc/'mountinfo').read_text().splitlines() if ' - cgroup2 ' in x]
    snapshot['cgroup2_mountinfo'] = mounts
    assert snapshot['process_cgroup'] == '0::/\n' and len(mounts) == 1
    mount_root, mountpoint = mounts[0].split()[3:5]
    assert mountpoint == '/sys/fs/cgroup'
    root = Path('/sys/fs/cgroup')
    root_before = (root/'cgroup.procs').read_text()
    snapshot['root_membership_before'] = root_before
    if mount_root == '/..':
        init_before = (root/'init/cgroup.procs').read_text()
        snapshot['init_membership_before'] = init_before
        assert str(pid) in init_before.splitlines() and str(pid) not in root_before.splitlines()
        levels = [root/'init', root]
    else:
        assert mount_root == '/' and str(pid) in root_before.splitlines()
        levels = [root]
    samples = []
    for level in levels:
        sample = dict(path=str(level), started_at=timestamp(), raw={}, read_errors={})
        for key in ('memory.max', 'memory.current', 'memory.swap.max', 'memory.swap.current',
                    'memory.events', 'memory.peak', 'memory.stat', 'cpu.max', 'cpu.stat',
                    'pids.max', 'pids.current', 'cgroup.procs', 'cgroup.events'):
            try:
                sample['raw'][key] = (level/key).read_text()
            except OSError as error:
                sample['read_errors'][key] = str(error)
                assert key not in ('memory.max', 'memory.current', 'memory.swap.max',
                                   'memory.swap.current', 'cgroup.procs')
        sample['ended_at'] = timestamp()
        samples.append(sample)
    snapshot['enclosing_samples'] = samples
    snapshot['host_meminfo'] = Path('/proc/meminfo').read_text()
    available_kib = [line.split()[1] for line in snapshot['host_meminfo'].splitlines()
                     if line.startswith('MemAvailable:')]
    assert len(available_kib) == 1
    available = int(available_kib[0]) * 1024
    H = available
    for sample in samples:
        raw = sample['raw']
        if raw['memory.max'].strip() != 'max':
            H = min(H, max(0, int(raw['memory.max'])-int(raw['memory.current'])))
    snapshot['root_membership_after'] = (root/'cgroup.procs').read_text()
    if mount_root == '/..':
        snapshot['init_membership_after'] = (root/'init/cgroup.procs').read_text()
        assert str(pid) in snapshot['init_membership_after'].splitlines()
        assert str(pid) not in snapshot['root_membership_after'].splitlines()
    else:
        assert str(pid) in snapshot['root_membership_after'].splitlines()
    snapshot['process_cgroup_after'] = (proc/'cgroup').read_text()
    assert snapshot['process_cgroup_after'] == snapshot['process_cgroup']
    snapshot.update(host_MemAvailable_bytes=available, H_bytes=H,
                    required_H_bytes=THRESHOLD, H_sufficient=H >= THRESHOLD,
                    new_worker_exists=False,
                    visibility='Only membership-verified visible levels; hidden ancestors unobserved. No lexical .. traversal, remount or descendant scan.',
                    scope='Raw resource/admission metadata only; not Q4 admission or reclamation attribution.',
                    ended_at=timestamp())
    return snapshot

assert os.getuid() == 1000
initial_ownership = ownership()
gate = json.loads(GATE.read_text())
assert digest(GATE) == '244227fadb58682e3eba280281985c1e7042df4d8d9863313f39cf918b442350'
assert GATE.stat().st_size == 11288
assert gate['verdict'] == 'admitted_advice_only' and gate['task_id'] == 'TASK-20261029-6f4f28'
assert gate['Q4_admitted'] is False and gate['scientific_calls_authorized'] == 0
assert subprocess.check_output(GIT+['show', '32786901e02b3a72241842e4e4612580f6c03c66:'+str(GATE.relative_to(PROGRAMME))], cwd=PROGRAMME) == GATE.read_bytes()
chain = gate['published_source_chain']
checks = [checked_reference(chain[x], PROGRAMME) for x in ('prep_publication', 'claim_publication',
          'actual_source_publication', 'launcher', 'source_runtime_seal',
          'source_binary_library_manifest', 'expected_checks')]
for key in ('prep_publication', 'claim_publication'):
    entry = chain[key]
    assert subprocess.check_output(GIT+['show', entry['commit']+':'+entry['path']], cwd=PROGRAMME) == (PROGRAMME/entry['path']).read_bytes()
for key in ('launcher', 'source_runtime_seal'):
    entry = chain[key]
    assert subprocess.check_output(GIT+['show', 'e9467d351eda0b7e92652642176cc12594838ce0:'+str(Path(entry['path']).relative_to(OWNER))], cwd=OWNER) == Path(entry['path']).read_bytes()
source = json.loads(Path(chain['source_binary_library_manifest']['path']).read_text())
for entry in source['sources'] + source['binaries'] + source['host_libraries']:
    checks.append(checked_reference(entry))
inputs = gate['exact_advice_inputs']
for key in ('eligible_allowlist', 'eligible_parts_index', 'eligible_tsv', 'old_native_seal_result', 'native_helper'):
    checks.append(checked_reference(inputs[key]))
allowlist = json.loads(Path(inputs['eligible_allowlist']['path']).read_text())
assert allowlist['accepted_count'] == 80 and allowlist['accepted_bytes'] == 4557159684
assert allowlist['excluded_count'] == 0 and len(allowlist['accepted']) == 80
assert Path(inputs['eligible_parts_index']['path']).read_text() == inputs['eligible_tsv']['path']+'\n'
assert len(Path(inputs['eligible_tsv']['path']).read_text().splitlines()) == 80
selectors = {key: 'unset_or_empty' if not os.environ.get(key) else 'set_nonempty'
             for key in ('LD_LIBRARY_PATH', 'LD_PRELOAD', 'LD_AUDIT')}
assert all(value == 'unset_or_empty' for value in selectors.values())
argv = inputs['argv']
assert argv == [inputs['native_helper']['path'], '--advise',
                inputs['eligible_parts_index']['path'], str(TASK/'advice-1')]
assert not (TASK/'advice-1').exists()
CONTROL.mkdir()  # Exclusive single-use control record; no automatic retry.
put('admission.json', GATE.read_text())
put('authority-input-checks.json', dict(recorded_at=timestamp(),
    published_advice_admission_commit='32786901e02b3a72241842e4e4612580f6c03c66',
    checked_inputs=checks, initial_ownership=initial_ownership,
    loader_selectors=selectors, environment_policy='Inherited guarded environment unchanged; allowlisted selector status only, no environment dump.',
    inference=dict(managed_model='gpt-6.1-sol', effort='high', session='native-q4case-20261029',
                   actually_selected=True, APIprobe='unverified', fallback_used=False),
    journal='Explicitly deferred under Copilot conflict hold; no journal writes.',
    native_seal_invocations=0, manual80_candidate_rehash=False, Q4_admitted=False))
put('lease-before-decision.json', ownership())
pre = resource_snapshot()
put('pre-resources.json', pre)
if pre['H_sufficient']:
    # Do not advise unnecessarily; preserve immediately subsequent raw metadata.
    post = resource_snapshot()
    put('post-resources.json', post)
    passes, resultcode = 0, None
    put('advice-skipped', 'Fresh H already>=12884901888; zero advice passes.\n')
else:
    put('lease-before-invocation.json', ownership())
    put('invocation.json', dict(argv=argv, cwd=str(OWNER), recorded_at=timestamp(),
        native_advice_attempt=1, maximum_attempts=1, selectors=selectors,
        sources_and_libraries_checked_before_pre_snapshot=True,
        post_snapshot_order='Native child returns, then immediate in-process proc/cgroup resource snapshot, before any software/library/candidate/Git read.'))
    with (CONTROL/'stdout').open('x') as stdout, (CONTROL/'stderr').open('x') as stderr:
        result = subprocess.run(argv, cwd=OWNER, stdout=stdout, stderr=stderr, check=False)
    # FIRST post-return operation that samples inputs: proc/cgroup metadata only.
    post = resource_snapshot()
    put('post-resources.json', post)
    resultcode = result.returncode
    passes = 1
    put('exit', str(resultcode)+'\n')
    put('native-return-recorded-after-raw-post-snapshot', timestamp()+'\n')
native_result = None
if passes and (TASK/'advice-1/result').exists():
    native_result = (TASK/'advice-1/result').read_text()
if passes and resultcode != 0:
    state = 'advice_failed_no_Q4'
elif not post['H_sufficient']:
    state = 'infrastructure_not_started'
else:
    state = 'paused_for_separate_actual_Q4_admission'
summary = dict(recorded_at=timestamp(), task_id='TASK-20261029-6f4f28', phase='ADVICE_ONLY',
    advice_passes=passes, advice_exit=resultcode, native_result=native_result,
    pre_H_bytes=pre['H_bytes'], post_H_bytes=post['H_bytes'],
    required_H_bytes=THRESHOLD, post_H_sufficient=post['H_sufficient'],
    state=state, Q4_attempts=0, workers=0, allocators=0, containers=0,
    scientific_calls=0, native_seal_invocations=0, retries=0,
    capacity_is_not_reclaimed_bytes=True,
    post_snapshot_before_software_library_candidate_or_Git_rewarming=True,
    next_action='STOP. Root/Coordinator receives raw resources; no Q4 or second advice authorized.',
    journal='Deferred under explicit Copilot conflict hold; untouched.')
put('outcome.json', summary)
print(json.dumps(summary, indent=2))
