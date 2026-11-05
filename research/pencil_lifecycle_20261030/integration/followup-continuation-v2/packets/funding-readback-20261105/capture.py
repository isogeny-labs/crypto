import concurrent.futures,datetime,hashlib,json,pathlib,subprocess,time
out=pathlib.Path(__file__).parent
commands={
"caller":["aws","sts","get-caller-identity","--output","json"],
"budgets":["aws","budgets","describe-budgets","--account-id","104186811093","--region","us-east-2","--output","json"],
"nodepools":["kubectl","--request-timeout=30s","get","nodepools.karpenter.sh","-o","json"],
"quota":["kubectl","--request-timeout=30s","get","resourcequota","-n","isolab","-o","json"],
"arc":["kubectl","--request-timeout=30s","get","autoscalingrunnersets.actions.github.com","-A","-o","json"],
"jobs":["kubectl","--request-timeout=30s","get","jobs","-n","isolab","-o","json"],
"nodes":["kubectl","--request-timeout=30s","get","nodes","-o","json"]}
def capture(item):
 name,argv=item;t=time.monotonic();start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 try:
  r=subprocess.run(argv,capture_output=True,timeout=50);rc=r.returncode;stdout=r.stdout;stderr=r.stderr
 except subprocess.TimeoutExpired as e:rc=None;stdout=e.stdout or b"";stderr=e.stderr or b""
 # Job and Node specifications can contain command environments: retain only admission/capacity metadata.
 if rc==0 and name in ("jobs","nodes"):
  d=json.loads(stdout);items=[]
  for x in d.get("items",[]):
   y={"metadata":{k:x.get("metadata",{}).get(k) for k in ("name","namespace","uid","creationTimestamp")},"status":x.get("status")}
   if name=="jobs":
    s=x.get("spec",{});ps=s.get("template",{}).get("spec",{});y["spec"]={k:s.get(k) for k in ("suspend","parallelism","completions","backoffLimit","activeDeadlineSeconds")};y["pod_resources"]=[{"name":c.get("name"),"image":c.get("image"),"resources":c.get("resources")} for c in ps.get("containers",[])];y["nodeSelector"]=ps.get("nodeSelector")
   items.append(y)
  stdout=(json.dumps({"items":items},indent=2)+"\n").encode()
 for suffix,b in (("stdout",stdout),("stderr",stderr)):(out/(name+"."+suffix)).write_bytes(b)
 receipt={"name":name,"argv":argv,"started_utc":start,"finished_utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),"elapsed_seconds":time.monotonic()-t,"exit_code":rc,"stdout_sha256":hashlib.sha256(stdout).hexdigest(),"stderr_sha256":hashlib.sha256(stderr).hexdigest()}
 (out/(name+".receipt.json")).write_text(json.dumps(receipt,indent=2)+"\n");return receipt
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
 for r in ex.map(capture,commands.items()):print(json.dumps(r))
