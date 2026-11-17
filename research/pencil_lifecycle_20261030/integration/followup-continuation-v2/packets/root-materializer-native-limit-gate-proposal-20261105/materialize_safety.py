"""Small pure interfaces for candidate identity and bounded owned-process cleanup."""
import hashlib,os,signal,subprocess

def copy_candidate(candidate,destination,expected):
    raw=candidate.read_bytes()
    assert hashlib.sha256(raw).hexdigest()==expected,'candidate input identity'
    destination.write_bytes(raw)
    assert hashlib.sha256(destination.read_bytes()).hexdigest()==expected,'candidate destination identity'
    return expected

def cleanup_owned_group(process,killpg=os.killpg,wait_seconds=2):
    # This process was launched start_new_session=True by this materializer.
    result={'pid':process.pid,'pgid':process.pid,'wait_bound_seconds':wait_seconds,'errors':[],'outer_cgroup_confirmation_required':True}
    try:killpg(process.pid,signal.SIGKILL)
    except ProcessLookupError:pass
    except BaseException as e:result['errors'].append({'kill':repr(e)})
    try:result['reaped_exit_code']=process.wait(timeout=wait_seconds);result['reaped']=True
    except BaseException as e:result['errors'].append({'wait':repr(e)});result['reaped']=False
    try:
        killpg(process.pid,0)
        result['group_absent']=False
        result['errors'].append({'group':'still visible after bounded cleanup'})
    except ProcessLookupError:result['group_absent']=True
    except BaseException as e:result['group_absent']=False;result['errors'].append({'probe':repr(e)})
    return result
