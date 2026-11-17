"""Private runtime endpoint and host cgroup checks. No subprocess calls."""
import datetime
import hashlib
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


def write_release_token(path, nonce, *, expected_uid=0, expected_gid=0):
    """Create exact token despite service umask; production requires root custody."""
    require(re.fullmatch(r'[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}', nonce) is not None, 'release nonce')
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o444)
    with os.fdopen(fd, 'wb') as f:
        f.write((nonce + '\n').encode())
        f.flush()
        os.fchmod(f.fileno(), 0o444)
        os.fsync(f.fileno())
        s = os.fstat(f.fileno())
        require(stat.S_ISREG(s.st_mode) and stat.S_IMODE(s.st_mode) == 0o444 and s.st_nlink == 1 and s.st_size == 37 and s.st_uid == expected_uid and s.st_gid == expected_gid, 'release token custody')
    return {'sha256': hashlib.sha256((nonce + '\n').encode()).hexdigest(), 'device': s.st_dev, 'inode': s.st_ino, 'uid': s.st_uid, 'gid': s.st_gid, 'mode': stat.S_IMODE(s.st_mode), 'bytes': s.st_size}

# Linux x86-64 descriptor transport. No container commands or permission fallback.
import ctypes
import platform
import select
import time

class _StatFS(ctypes.Structure):
    _fields_ = [('type',ctypes.c_long),('bsize',ctypes.c_long),
                ('blocks',ctypes.c_ulong),('bfree',ctypes.c_ulong),('bavail',ctypes.c_ulong),
                ('files',ctypes.c_ulong),('ffree',ctypes.c_ulong),('fsid',ctypes.c_int*2),
                ('namelen',ctypes.c_long),('frsize',ctypes.c_long),('flags',ctypes.c_long),
                ('spare',ctypes.c_long*4)]

def _linux_libc():
    require(platform.system()=='Linux' and platform.machine()=='x86_64' and
            ctypes.sizeof(ctypes.c_long)==8 and ctypes.sizeof(_StatFS)==120,'release ABI')
    libc=ctypes.CDLL(None,use_errno=True)
    libc.fstatfs.argtypes=[ctypes.c_int,ctypes.POINTER(_StatFS)];libc.fstatfs.restype=ctypes.c_int
    libc.renameat2.argtypes=[ctypes.c_int,ctypes.c_char_p,ctypes.c_int,ctypes.c_char_p,ctypes.c_uint]
    libc.renameat2.restype=ctypes.c_int
    return libc

def _fs_type(fd):
    s=_StatFS();libc=_linux_libc()
    if libc.fstatfs(fd,ctypes.byref(s))!=0:raise OSError(ctypes.get_errno(),'fstatfs')
    return s.type

def _rename_exclusive(fd,stage):
    libc=_linux_libc()
    if libc.renameat2(fd,stage.encode('ascii'),fd,b'gate-release',1)!=0:
        raise OSError(ctypes.get_errno(),'exclusive release publication')

def _bounded_at(fd,name,limit):
    f=os.open(name,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK|os.O_CLOEXEC,dir_fd=fd)
    try:
        require(stat.S_ISREG(os.fstat(f).st_mode),'proc observation type')
        data=bytearray()
        while len(data)<=limit:
            chunk=os.read(f,limit+1-len(data))
            if not chunk:break
            data.extend(chunk)
        require(len(data)<=limit,'proc observation bound')
        return bytes(data).decode('ascii')
    finally:os.close(f)

def _held_identity(fd,pid):
    def readstat():
        raw=_bounded_at(fd,'stat',4096);end=raw.rfind(')');fields=raw[end+2:].split()
        require(end>0 and raw.split(' ',1)[0]==str(pid) and len(fields)>19,'held PID stat')
        require(fields[0] not in ('Z','X','x'),'held PID dead')
        return int(fields[19])
    ticks=readstat();raw=_bounded_at(fd,'cgroup',4096)
    require(raw.startswith('0::/') and raw.count('\n')==1 and raw.endswith('\n'),'held unified cgroup')
    require(readstat()==ticks,'held PID changed')
    return ticks,raw[3:-1]

def _pid_live(fd):
    poll=select.poll();poll.register(fd,select.POLLIN|select.POLLHUP|select.POLLERR)
    require(not poll.poll(0),'held PID exited')

def _work_custody(root,work):
    require(os.fstatvfs(root).f_flag & os.ST_RDONLY,'container root not readonly')
    st=os.fstat(work);v=os.fstatvfs(work)
    require(stat.S_ISDIR(st.st_mode) and st.st_uid==0 and st.st_gid==0 and
            stat.S_IMODE(st.st_mode)==0o1777,'work directory custody')
    require(_fs_type(work)==0x01021994,'work not tmpfs')
    required=os.ST_NOSUID|os.ST_NODEV|os.ST_NOEXEC
    require(not v.f_flag&os.ST_RDONLY and v.f_flag&required==required,'work mount flags')
    require(0<v.f_blocks*v.f_frsize<=16*1024**2,'work capacity')
    return st.st_dev,st.st_ino

def _token_custody(fd,nonce):
    st=os.fstat(fd)
    require(stat.S_ISREG(st.st_mode) and st.st_uid==0 and st.st_gid==0 and
            stat.S_IMODE(st.st_mode)==0o444 and st.st_nlink==1 and st.st_size==37,'release token custody')
    os.lseek(fd,0,os.SEEK_SET);raw=os.read(fd,38)
    require(raw==(nonce+'\n').encode('ascii') and os.read(fd,1)==b'','release token bytes')
    return {'device':st.st_dev,'inode':st.st_ino,'uid':st.st_uid,'gid':st.st_gid,
            'mode':stat.S_IMODE(st.st_mode),'nlink':st.st_nlink,'size':st.st_size,
            'sha256':hashlib.sha256(raw).hexdigest()}

def _publish_token(work,nonce,prepublish,record):
    """Only called after verified work custody. No retry, including ambiguous rename."""
    require(re.fullmatch(r'[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}',nonce),'release nonce')
    stage='.gate-release-'+nonce+'.partial';token=None;identity=None;attempted=False
    receipt={'schema':'software.descriptor-release/v1','publication_attempted':False,
             'publication_possible':False,'publication_confirmed':False,'nonce':nonce}
    try:
        token=os.open(stage,os.O_RDWR|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW|os.O_NONBLOCK|os.O_CLOEXEC,0o600,dir_fd=work)
        st=os.fstat(token);identity=(st.st_dev,st.st_ino)
        require(stat.S_ISREG(st.st_mode) and st.st_uid==0 and st.st_gid==0 and st.st_nlink==1 and st.st_size==0,'staging custody')
        raw=(nonce+'\n').encode('ascii');sent=0
        while sent<len(raw):
            n=os.write(token,raw[sent:]);require(0<n<=len(raw)-sent,'release short write');sent+=n
        os.fsync(token);os.fchmod(token,0o444);os.fsync(token)
        receipt['token']=_token_custody(token,nonce)
        prepublish()
        receipt['publication_attempted']=True;receipt['publication_possible']=True
        record(dict(receipt))  # Durable host evidence precedes the sole publish syscall.
        prepublish()  # Receipt I/O can deliver a signal or consume the remaining deadline.
        attempted=True
        _rename_exclusive(work,stage)
        receipt['publication_confirmed']=True
        record(dict(receipt))
        return receipt
    except BaseException as exc:
        receipt['error']=repr(exc);receipt['publication_uncertain']=attempted
        try:record(dict(receipt))
        except BaseException:pass
        raise
    finally:
        if token is not None:
            if not attempted:
                try:
                    st=os.stat(stage,dir_fd=work,follow_symlinks=False)
                    if (st.st_dev,st.st_ino)==identity:os.unlink(stage,dir_fd=work)
                except OSError:pass
            os.close(token)

def publish_release_fd(expected,nonce,deadline,revalidate,record):
    """Pin process/mount view, verify all custody before any tmpfs write."""
    require(expected['nonce']==nonce,'ancestry nonce join')
    require(isinstance(expected['pid'],int) and expected['pid']>1,'host PID shape')
    fds=[];flags=os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW|os.O_CLOEXEC
    def hold(fd):fds.append(fd);return fd
    def boundary():
        require(time.monotonic()<deadline,'release deadline')
        require(revalidate()==expected,'release ancestry changed')
    try:
        boundary()
        proc=hold(os.open('/proc',flags));require(_fs_type(proc)==0x9fa0,'proc filesystem')
        task=hold(os.open(str(expected['pid']),flags,dir_fd=proc))
        pidfd=hold(os.pidfd_open(expected['pid'],0))
        def taskcheck():
            _pid_live(pidfd)
            require(_held_identity(task,expected['pid'])==(expected['start_ticks'],expected['cgroup']),'held ancestry mismatch')
        taskcheck()
        # Deliberately follow exactly this kernel magic link; NOFOLLOW elsewhere.
        root=hold(os.open('root',os.O_RDONLY|os.O_DIRECTORY|os.O_CLOEXEC,dir_fd=task))
        work=hold(os.open('work',flags,dir_fd=root));custody=_work_custody(root,work)
        taskcheck();boundary()
        def before():
            boundary();taskcheck()
            again=os.open('work',flags,dir_fd=root)
            try:require(_work_custody(root,again)==custody,'work identity changed')
            finally:os.close(again)
            require(_work_custody(root,work)==custody,'held work changed')
            taskcheck();require(time.monotonic()<deadline,'final release deadline')
        return _publish_token(work,nonce,before,record)
    finally:
        for fd in reversed(fds):os.close(fd)
