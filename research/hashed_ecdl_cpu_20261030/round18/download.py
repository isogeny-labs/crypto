import boto3,pathlib,tarfile,hashlib,json
root=pathlib.Path('/home/ec2-user/hashed-ecdl-cpu-round18')
s3=boto3.client('s3',region_name='us-east-2');key='validation/hashed-ecdl-cpu/20261030/round18/results.tar.gz'
s3.download_file('setec-astronomy-artifacts',key,str(root/'results.tar.gz'))
with tarfile.open(root/'results.tar.gz') as t: t.extractall(root/'download',filter='data')
receipt={'uri':'s3://setec-astronomy-artifacts/'+key,'sha256':hashlib.sha256((root/'results.tar.gz').read_bytes()).hexdigest()}
(root/'results-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(receipt)
