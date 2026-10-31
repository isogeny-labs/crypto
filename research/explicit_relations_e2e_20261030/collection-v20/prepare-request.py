# Administrative request preparation only; does not launch a pod.
import base64,json,os
from pathlib import Path
import boto3
root=Path(__file__).resolve().parent
prior=root.parent/'ecbench-e2e-20261030-v19'
assert (prior/'controller.exit').read_text().strip()=='0'
assert (prior/'deletion.txt').read_text().strip()=='DELETION_VERIFIED '+(prior/'pod-id').read_text().strip()
assert not (root/'pod-id').exists()
assert not (root/'request.json').exists()
assert (root/'private/auth.curl').is_file()
x=json.loads((root/'request-template.json').read_text())
x['env']={'BOOTSTRAP_BASE64':base64.b64encode((root/'bootstrap.sh').read_bytes()).decode(),
 'ARTIFACT_UPLOAD_URL':boto3.client('s3',region_name='us-east-2').generate_presigned_url('put_object',Params={'Bucket':'setec-astronomy-artifacts','Key':'validation/ecbench-explicit-x/e2e-20261030-v20/results.tgz'},ExpiresIn=21600)}
x['env']['SOURCE_ARCHIVE_URL']=boto3.client('s3',region_name='us-east-2').generate_presigned_url('get_object',Params={'Bucket':'setec-astronomy-artifacts','Key':'validation/ecbench-explicit-x/e2e-20261030-v20/source-checkout.tgz'},ExpiresIn=21600)
x['env']['SNAPSHOT_UPLOAD_URL']=boto3.client('s3',region_name='us-east-2').generate_presigned_url('put_object',Params={'Bucket':'setec-astronomy-artifacts','Key':'validation/ecbench-explicit-x/e2e-20261030-v20/partial-latest.tgz'},ExpiresIn=21600)
x['entrypoint']=['/bin/bash','-c']
x['cmd']=[(root/'pod-command.sh').read_text()]
assert x['gpu']['count']==1
os.umask(0o077)
(root/'request.json').write_text(json.dumps(x))
print('Prepared private v20 request; no pod launched')
