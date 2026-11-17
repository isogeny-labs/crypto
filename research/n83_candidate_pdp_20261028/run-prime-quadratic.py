"""One bounded process per template; every m is attempted even after failure."""
from pathlib import Path
import hashlib,json,os,resource,signal,subprocess,sys,time

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
    return h.hexdigest()

def child(command,log,seconds,memory_bytes):
    def limit():resource.setrlimit(resource.RLIMIT_AS,(memory_bytes,memory_bytes))
    env=dict(os.environ,N83_QUADRATIC_FROZEN_PRODUCTION='1')
    with Path(log).open('wb') as output:
        try:proc=subprocess.Popen(command,stdout=output,stderr=subprocess.STDOUT,env=env,preexec_fn=limit,start_new_session=True)
        except OSError as error:
            output.write((type(error).__name__+': '+str(error)+'\n').encode());return {'exit_code':126,'timed_out':False,'started':False}
        timeout=False
        try:code=proc.wait(timeout=seconds)
        except subprocess.TimeoutExpired:
            timeout=True;os.killpg(proc.pid,signal.SIGTERM)
            try:code=proc.wait(timeout=5)
            except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);code=proc.wait()
    return {'exit_code':code,'timed_out':timeout,'started':True}

def templates(binary,root,limits,seconds,memory_bytes):
    root=Path(root);statuses=[]
    for m in range(3,10):
        out=root/'templates'/f'm{m}';out.mkdir(parents=True,exist_ok=False)
        native=out/'native'
        producer=child([str(binary),'template',str(m),str(limits),str(native)],out/'producer.log',seconds,memory_bytes)
        replay={'exit_code':125,'timed_out':False,'started':False}
        if producer['exit_code']==0 and (native/'result.json').exists():
            replay=child([str(binary),'template-replay',str(m),str(limits),str(native)],out/'replay.log',seconds,memory_bytes)
        result={'m':m,'producer_exit_code':producer['exit_code'],'replay_exit_code':replay['exit_code'],'producer':producer,'replay':replay,
                'result_sha256':sha(native/'result.json') if (native/'result.json').exists() else None,
                'journal_sha256':sha(native/'journal.jsonl') if (native/'journal.jsonl').exists() else None,
                'no_certificate_without_both_successful':True}
        (out/'attempt.json').write_text(json.dumps(result,indent=2)+'\n');statuses.append(result)
        print(json.dumps({'event':'template_finished','m':m,'producer_exit_code':producer['exit_code'],'replay_exit_code':replay['exit_code']}),flush=True)
    return statuses

def main():
    binary,root,protocol=map(Path,sys.argv[1:4]);p=json.loads(protocol.read_text())
    assert p['freeze']['status']=='frozen' and sha(binary)==p['arms'][0]['instrument']['binary_sha256']
    limits=root/'template-limits.json';limits.write_text(json.dumps(p['extensions']['template_limits'],indent=2)+'\n')
    seconds=p['extensions']['template_timeout_seconds'];memory=p['extensions']['address_space_bytes']
    statuses=templates(binary,root,limits,seconds,memory)
    (root/'template-attempts.json').write_text(json.dumps(statuses,indent=2)+'\n')
    result=child([str(binary),'assemble',str(root),str(protocol),str(root/'results')],root/'assemble.log',120,memory)
    replay={'exit_code':125,'timed_out':False,'started':False}
    if result['exit_code']==0:
        replay=child([str(binary),'assemble-replay',str(root),str(protocol),str(root/'results')],root/'assemble-replay.log',120,memory)
    summary={'templates_attempted':len(statuses),'templates_producer_success':sum(x['producer_exit_code']==0 for x in statuses),
             'templates_fresh_replay_success':sum(x['replay_exit_code']==0 for x in statuses),'assembly':result,'assembly_replay':replay,
             'capped_or_failed_templates_do_not_imply_degree_bounds':True}
    (root/'status.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary))
    return 0 if result['exit_code']==replay['exit_code']==0 else 1

if __name__=='__main__':raise SystemExit(main())
