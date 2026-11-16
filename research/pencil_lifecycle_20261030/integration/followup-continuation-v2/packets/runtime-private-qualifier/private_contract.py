"""Private runtime endpoint and host cgroup checks. No subprocess calls."""
import datetime
import os
from pathlib import Path, PurePosixPath
import re
import stat

class ContractError(ValueError):
    pass

def require(ok, message):
    if not ok:
        raise ContractError(message)

def canonical_path(value):
    require(isinstance(value, str) and value.startswith('/') and value != '/', 'absolute non-root path')
    p = PurePosixPath(value)
    require(str(p) == value and '..' not in p.parts and '\x00' not in value, 'canonical path')
    return p

def below(child, parent):
    c, p = canonical_path(child), canonical_path(parent)
    return p in c.parents

def timestamp(value):
    require(isinstance(value, str) and value.endswith('Z'), 'UTC expiry')
    return datetime.datetime.fromisoformat(value[:-1] + '+00:00').timestamp()

def validate_endpoint(endpoint, binding, now):
    require(endpoint['schema'] == 'software.private-runtime-endpoint/v1' and endpoint['mode'] == 'qualification', 'qualification endpoint receipt')
    require(endpoint['owner'] == binding['private_owner'], 'private owner')
    require(re.fullmatch(r'[0-9a-f-]{36}', endpoint['owner']) is not None, 'owner UUID')
    socket = str(canonical_path(endpoint['socket_path']))
    require(socket not in ('/var/run/docker.sock', '/run/docker.sock'), 'shared socket forbidden')
    require(endpoint['socket_uid'] == endpoint['socket_gid'] == 0 and endpoint['socket_mode'] == 0o600, 'private socket ownership/mode')
    require(type(endpoint['socket_inode']) is int and endpoint['socket_inode'] > 0, 'socket inode')
    require(type(endpoint['socket_device']) is int and endpoint['socket_device'] >= 0, 'socket device')
    require(isinstance(endpoint['daemon_id'], str) and endpoint['daemon_id'], 'daemon ID')
    require(re.fullmatch(r'[0-9a-f]{32}', endpoint['service_invocation_id']) is not None, 'service invocation ID')
    require(endpoint['service_unit'].endswith('.service'), 'service unit')
    require(below(endpoint['cgroup_parent'], endpoint['service_cgroup']), 'container parent outside service')
    require(endpoint['service_cgroup'].split('/')[-1] == endpoint['service_unit'], 'unit cgroup join')
    for key in ['daemon_pid', 'daemon_start_ticks', 'supervisor_pid']:
        require(type(endpoint[key]) is int and endpoint[key] > 0, 'process identity ' + key)
    require(now < timestamp(endpoint['expires_at']) <= timestamp(binding['absolute_expiry']), 'live qualification expiry')
    for ekey, bkey in [('source_binding_sha256', 'final_source_manifest_sha256'), ('image_id', 'image_id'), ('input_manifest_sha256', 'fixture_manifest_sha256'), ('assembly_receipt_sha256', 'private_assembly_receipt_sha256')]:
        require(endpoint[ekey] == binding[bkey], 'endpoint input join ' + ekey)
    return endpoint

def no_symlinks(path):
    path = Path(path)
    for component in [path, *path.parents]:
        require(not component.is_symlink(), 'symlink path component')

def socket_identity(endpoint):
    p = Path(endpoint['socket_path'])
    no_symlinks(p)
    s = p.lstat()
    require(stat.S_ISSOCK(s.st_mode), 'not UNIX socket')
    for actual, key in [(s.st_uid, 'socket_uid'), (s.st_gid, 'socket_gid'), (stat.S_IMODE(s.st_mode), 'socket_mode'), (s.st_dev, 'socket_device'), (s.st_ino, 'socket_inode')]:
        require(actual == endpoint[key], 'socket changed ' + key)
    # The owned directory prevents endpoint replacement by other local users.
    d = p.parent.stat()
    require(d.st_uid == 0 and stat.S_IMODE(d.st_mode) == 0o700, 'private socket directory')

def unified_cgroup(raw):
    lines = raw.splitlines()
    require(len(lines) == 1 and lines[0].startswith('0::'), 'unified host cgroup')
    value = lines[0][3:]
    canonical_path(value)
    return value

def process_identity(pid, proc_root=Path('/proc')):
    require(type(pid) is int and pid > 0, 'host PID')
    p = proc_root / str(pid)
    first = (p / 'stat').read_text()
    # comm may contain spaces and parentheses; fields after the last ')' start at field 3.
    fields = first.rsplit(')', 1)[1].split()
    require(len(fields) >= 20 and fields[0] not in ('Z', 'X'), 'live process stat')
    start = int(fields[19])
    cg = unified_cgroup((p / 'cgroup').read_text())
    second = (p / 'stat').read_text()
    second_fields = second.rsplit(')', 1)[1].split()
    require(int(second_fields[19]) == start and second_fields[0] not in ('Z', 'X'), 'process identity changed during observation')
    return {'pid': pid, 'start_ticks': start, 'cgroup': cg}

def check_host_ancestry(endpoint, container, gate_receipt, *, proc_root=Path('/proc'), cgroup_root=Path('/sys/fs/cgroup')):
    require(container['State']['Running'] is True, 'gate container not running')
    require(container['HostConfig']['CgroupParent'] == endpoint['cgroup_parent'], 'container configured parent')
    observed = process_identity(container['State']['Pid'], proc_root)
    require(below(observed['cgroup'], endpoint['cgroup_parent']) and below(observed['cgroup'], endpoint['service_cgroup']), 'container escaped service parent')
    path = cgroup_root / observed['cgroup'].lstrip('/')
    no_symlinks(path)
    s = path.stat()
    require(s.st_dev == gate_receipt['cgroup_device'] and s.st_ino == gate_receipt['cgroup_inode'], 'host/private cgroup inode join')
    require((path / 'cgroup.procs').read_text() == str(observed['pid']) + '\n', 'single host gate PID')
    require((path / 'pids.max').read_text() == '1\n' and (path / 'pids.current').read_text() == '1\n', 'host PID controller')
    observed.update(container_id=container['Id'], nonce=gate_receipt['nonce'], cgroup_device=s.st_dev, cgroup_inode=s.st_ino)
    return observed

def live_endpoint(endpoint, *, proc_root=Path('/proc'), cgroup_root=Path('/sys/fs/cgroup'), now=None):
    now = datetime.datetime.now(datetime.timezone.utc).timestamp() if now is None else now
    require(now < timestamp(endpoint['expires_at']), 'private endpoint expired')
    require(os.environ.get('INVOCATION_ID') == endpoint['service_invocation_id'], 'service invocation environment join')
    socket_identity(endpoint)
    daemon = process_identity(endpoint['daemon_pid'], proc_root)
    require(daemon['start_ticks'] == endpoint['daemon_start_ticks'] and below(daemon['cgroup'], endpoint['service_cgroup']), 'daemon lifetime/containment')
    supervisor = process_identity(endpoint['supervisor_pid'], proc_root)
    require(below(supervisor['cgroup'], endpoint['service_cgroup']), 'supervisor containment')
    own = process_identity(os.getpid(), proc_root)
    require(below(own['cgroup'], endpoint['service_cgroup']), 'qualifier containment')
    parent = cgroup_root / endpoint['cgroup_parent'].lstrip('/')
    no_symlinks(parent)
    require((parent / 'cgroup.procs').read_text() == '', 'container parent must be empty')
    require({'cpu', 'memory', 'pids'} <= set((parent / 'cgroup.subtree_control').read_text().split()), 'delegated controllers')
    return {'daemon': daemon, 'supervisor': supervisor, 'qualifier': own}

def docker_prefix(docker, config, endpoint):
    return [str(docker), '--config', str(config), '--host', 'unix://' + endpoint['socket_path']]

def archive_head_argv(curl, endpoint, url):
    return [str(curl), '-q', '--head', '--silent', '--show-error', '--max-time', '15', '--unix-socket', endpoint['socket_path'], '--noproxy', '*', url]


def success_boundary(pending_signal, monotonic_now, deadline, utc_now, expiry):
    return pending_signal is None and monotonic_now < deadline and utc_now < timestamp(expiry)
