"""Exact original publication/FF and terminal held-byte custody; I/O only."""
from pathlib import Path
import datetime, hashlib, json, os, re, subprocess
from ct105_hold27_guard import intact, sha

r = Path('/workspace/crypto')
c = Path('/workspace/cryptanalysis')
control = Path('/workspace/scratch/ct104-whole-closed-science-handoff-27-attempt02')
out = Path('/workspace/scratch/ct106-handoff27-artifact-ff-01')
out.mkdir()
(out/'original-command.py').write_bytes(Path(__file__).read_bytes())
source = '00800a2b8f20a2812eccbc5200e85a99572b453d'
bot = '0b2bee203a133568f1c7e9a35bfe3be1a663a879'
prefix = 'research/chained_jacobian_transfers_20261110/evidence/public-n4-original-relation-rank-character-handoff-27'
objects = json.loads((r/prefix/'objects.json').read_text())
index = json.loads((r/prefix/'archive-parts.json').read_text())
baseline = json.loads((control/'owned-before.json').read_text())
fetch = json.loads(Path('/workspace/scratch/ct105-handoff27-RAM-bot-fetch-01/closed-new-remote-cache.json').read_text())
assert fetch['actual_exit'] == 0 and fetch['source'] == source and fetch['actual_bot'] == bot
env = os.environ.copy()
env['GIT_NO_LAZY_FETCH'] = '1'

def git(*args):
    return subprocess.check_output(['git', *args], cwd=r, env=env)

def owned(repo, scopes):
    names = sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scopes],cwd=repo).split(b'\0')) | set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=repo).split(b'\0'))) - {b''})
    return {n.decode(): dict(sha256=sha(repo/n.decode()), literal_link=os.readlink(repo/n.decode()) if (repo/n.decode()).is_symlink() else None) for n in names}

def guard(label):
    state = intact(control)
    for name, repo in [('root', r), ('consumer', c)]:
        assert owned(repo, baseline['scopes'][name]) == baseline['owned'][name], (label, name)
    (out/(label+'-complete-guard.json')).write_text(json.dumps(state,indent=2)+'\n')
    print(label, state['content_keys'], 'contents', state['literal_links'], 'links; owned exact', flush=True)
    return state

def run(name, cmd):
    q = subprocess.run(cmd,cwd=r,env=env,capture_output=True,timeout=120)
    (out/(name+'.txt')).write_bytes(q.stdout+q.stderr)
    (out/(name+'.exit')).write_text(str(q.returncode)+'\n')
    print(name,q.returncode,flush=True)
    assert q.returncode == 0, name

assert git('rev-parse','HEAD').decode().strip() == source
assert git('rev-parse',bot+'^').decode().strip() == source
assert not git('status','--porcelain=v1','--untracked-files=all')
guard('before')
log = Path('/workspace/scratch/ct105-handoff27-actual-original-publication-01/whole-job.log').read_bytes()
matches = [x for x in re.finditer(rb'\[detached HEAD ([0-9a-f]+)\]',log) if bot.startswith(x[1].decode())]
assert len(matches) == 1
replays = [x for x in re.finditer(rb'artifacts migrate: (\d+) remote objects replayed; staged pointers are ready',log)]
assert len(replays) == 1 and int(replays[0][1]) == 11 and replays[0].start() < matches[0].start()
migrations = {x['path'] for x in objects if x['original_policy_requires_store']}
assert len(objects) == 15 and len(migrations) == 11
expected = migrations | {prefix+'/'+n for n in ['.artifacts.json','.gitignore']}
changed = set(git('diff','--no-renames','--name-only',source,bot).decode().splitlines())
assert changed == expected and len(changed) == 13
manifest_bytes = git('show',bot+':'+prefix+'/.artifacts.json')
manifest = json.loads(manifest_bytes)
assert manifest['schema'] == 'artifacts.manifest/v1' and len(manifest['files']) == 11
assert {prefix+'/'+n for n in manifest['files']} == migrations
for row in objects:
    data = git('show',source+':'+row['path'])
    assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256']
    assert data == Path(row['local_original']).read_bytes()
    assert (r/row['path']).is_file() and not (r/row['path']).is_symlink()
    assert data == (r/row['path']).read_bytes()
    if row['original_policy_requires_store']:
        assert manifest['files'][Path(row['path']).name] == dict(bytes=row['bytes'],sha256=row['sha256'])
    else:
        assert git('show',bot+':'+row['path']) == data
whole = hashlib.sha256()
count = 0
for row in index['parts']:
    assert row['offset'] == count
    data = Path(row['local_original']).read_bytes()
    assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256']
    whole.update(data)
    count += len(data)
assert count == index['bytes'] and whole.hexdigest() == index['sha256']
assert sha(index['local_original']) == whole.hexdigest()
pdf = 'research/chained_jacobian_transfers_20261110/evidence/public-n4-norm-halving-handoff-16/report.pdf'
assert git('rev-parse',source+':'+pdf) == git('rev-parse',bot+':'+pdf)
guard('pre-ff')
run('exact-ff',['git','merge','--ff-only',bot])
assert git('rev-parse','HEAD').decode().strip() == bot
guard('after-ff')
run('artifact',['python3','tools/artifacts/artifacts.py','check',source,bot])
run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
terminal = guard('terminal')
(out/'canonical-manifest.json').write_bytes(manifest_bytes)
receipt = dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=source,bot=bot,
    original_run=43745805865,original_job=132908716494,original_whole_job_bytes=len(log),original_whole_job_sha256=hashlib.sha256(log).hexdigest(),
    exact_direct_parent=source,original_remote_replays_before_actual_bot=11,exact_changed_paths=sorted(changed),
    whole_archive_ordered_reassembly=dict(bytes=count,sha256=whole.hexdigest(),all_parts_and_whole_original_retained=True),
    all15_original_source_blobs_and_root_regular_payload_inputs_checked=True,exact11_canonical_entries=True,
    four_plainGit_keeps_unchanged=True,ordinary_original_PDF_tree_blob_unchanged=True,
    complete_terminal_guard=terminal,whole_start_of_hold_owned_baseline=baseline,all_owned_bytes_exact=True,
    normalFF=0,original_artifact_check=0,original_workflow_wiring_check=0,
    no_force_reset_stash_rebase_or_policy_change=True,
    hold27='RELEASED after actual original replays before actual bot, exact FF, whole ordered archive, all held content/link/ledger and owned-byte terminal guards',
    scope='unsigned byte custody; scientific, conditional proof, CI-wide, performance and merge standing unchanged')
(out/'dirty-preservation.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('HOLD27 RELEASED; 65886 content keys, 179 literal links, 11 original remote replays, exact13 paths, archive and all terminal checks0',flush=True)
