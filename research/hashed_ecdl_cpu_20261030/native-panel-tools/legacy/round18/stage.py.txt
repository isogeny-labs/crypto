import pathlib,json,hashlib,tarfile,boto3,shlex
root=pathlib.Path('/home/ec2-user/hashed-ecdl-cpu-round18'); b=root/'bundle'
with tarfile.open(root/'input.tar.gz','w:gz') as t:
 for f in sorted(b.iterdir()): t.add(f,arcname=f.name)
s3=boto3.client('s3',region_name='us-east-2'); bucket='setec-astronomy-artifacts'; prefix='validation/hashed-ecdl-cpu/20261030/round18/'
s3.upload_file(str(root/'input.tar.gz'),bucket,prefix+'input.tar.gz')
get=s3.generate_presigned_url('get_object',Params={'Bucket':bucket,'Key':prefix+'input.tar.gz'},ExpiresIn=3600)
put=s3.generate_presigned_url('put_object',Params={'Bucket':bucket,'Key':prefix+'results.tar.gz'},ExpiresIn=3600)
digest=hashlib.sha256((root/'input.tar.gz').read_bytes()).hexdigest()
script='''set -euo pipefail
mkdir -p /work/cpu-round18 /work/results
cd /work/cpu-round18
curl -fsSL --retry 2 GET_URL -o input.tar.gz
printf '%s  input.tar.gz\\n' INPUT_SHA | sha256sum -c -
python3 -m tarfile -e input.tar.gz .
chmod +x baseline candidate previous inverse analyze
export EPHEMERAL_OUT=/work/results
set +e
bash run.sh
code=$?
set -e
printf '%s\\n' "$code" > /work/results/exit-code.txt
python3 -m tarfile -c /work/cpu-round18/results.tar.gz /work/results
curl -fsS --retry 2 -X PUT --upload-file /work/cpu-round18/results.tar.gz PUT_URL
exit "$code"
'''.replace('GET_URL',shlex.quote(get)).replace('PUT_URL',shlex.quote(put)).replace('INPUT_SHA',shlex.quote(digest))
p=json.loads((b/'protocol.json').read_text())
request={'job_name':'hashed-ecdl-cpu-round18-20261030','command':['bash','-lc',script],'cpu':2,'memory':'4Gi','seconds':300,'retries':0,'image':p['execution']['placement']['runtime_image']}
(root/'submit-private.json').write_text(json.dumps(request)); (root/'submit-private.json').chmod(0o600)
(root/'input-receipt.json').write_text(json.dumps({'uri':'s3://'+bucket+'/'+prefix+'input.tar.gz','sha256':digest},indent=2)+'\n')
print('Staged pinned round18 input; private submission saved')
