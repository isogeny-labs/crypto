import concurrent.futures,datetime,hashlib,json,pathlib,subprocess,time
p=pathlib.Path(__file__).parent
k=["kubectl","--kubeconfig","/home/ec2-user/ops/eks-isolab/kubeconfig","--context","i-01f85a6c905181498@isolab.us-east-2.eksctl.io","--request-timeout=30s"]
commands={"nodepool":k+["get","nodepools.karpenter.sh","-o","json"],"quota":k+["get","resourcequota","jobs","-n","isolab","-o","json"],"github-refs":["git","ls-remote","origin","refs/heads/codex/pencil-evidence-recovery-20261105","refs/pull/3471/head","refs/pull/3471/merge"],"github-pr":["gh","api","repos/isogeny-labs/crypto/pulls/3471","--jq","{head:{ref:.head.ref,sha:.head.sha,repo:.head.repo.full_name},base:{ref:.base.ref,sha:.base.sha},mergeable,mergeable_state,updated_at,merge_commit_sha}"]}
def run(item):
 name,argv=item;start=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic()
 try:r=subprocess.run(argv,capture_output=True,timeout=45);rc=r.returncode;out=r.stdout;err=r.stderr
 except subprocess.TimeoutExpired as e:rc=None;out=e.stdout or b"";err=e.stderr or b""
 (p/(name+".stdout")).write_bytes(out);(p/(name+".stderr")).write_bytes(err)
 receipt={"argv":argv,"started_utc":start,"finished_utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),"elapsed_seconds":time.monotonic()-t,"exit_code":rc,"stdout_sha256":hashlib.sha256(out).hexdigest(),"stderr_sha256":hashlib.sha256(err).hexdigest()};(p/(name+".receipt.json")).write_text(json.dumps(receipt,indent=2)+"\n");return {"name":name,**receipt}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
 for r in ex.map(run,commands.items()):print(json.dumps(r))
