#!/usr/bin/env python3
# minimal SigV4 S3 GET (stdlib only): s3get.py <bucket> <key> <dest> [region]
import sys, os, hmac, hashlib, datetime, urllib.request, urllib.error, ssl
bucket, key, dest = sys.argv[1:4]
region = sys.argv[4] if len(sys.argv) > 4 else 'us-east-1'
ak, sk = os.environ['AWS_ACCESS_KEY_ID'], os.environ['AWS_SECRET_ACCESS_KEY']
tok = os.environ.get('AWS_SESSION_TOKEN')
host = f'{bucket}.s3.{region}.amazonaws.com' if region != 'us-east-1' else f'{bucket}.s3.amazonaws.com'
t = datetime.datetime.now(datetime.timezone.utc); amz = t.strftime('%Y%m%dT%H%M%SZ'); ds = t.strftime('%Y%m%d')
ph = hashlib.sha256(b'').hexdigest()
hdrs = {'host': host, 'x-amz-content-sha256': ph, 'x-amz-date': amz}
if tok: hdrs['x-amz-security-token'] = tok
sh = ';'.join(sorted(hdrs)); ch = ''.join(f'{k}:{hdrs[k]}\n' for k in sorted(hdrs))
uri = '/' + key
cr = f'GET\n{uri}\n\n{ch}\n{sh}\n{ph}'
scope = f'{ds}/{region}/s3/aws4_request'
sts = f'AWS4-HMAC-SHA256\n{amz}\n{scope}\n{hashlib.sha256(cr.encode()).hexdigest()}'
def H(k, m): return hmac.new(k, m.encode(), hashlib.sha256).digest()
k = H(H(H(H(('AWS4' + sk).encode(), ds), region), 's3'), 'aws4_request')
sig = hmac.new(k, sts.encode(), hashlib.sha256).hexdigest()
auth = f'AWS4-HMAC-SHA256 Credential={ak}/{scope}, SignedHeaders={sh}, Signature={sig}'
req = urllib.request.Request(f'https://{host}{uri}', headers={**{h: v for h, v in hdrs.items() if h != 'host'}, 'Authorization': auth})
try:
    with urllib.request.urlopen(req, timeout=60) as r, open(dest, 'wb') as f:
        n = 0
        while True:
            b = r.read(1 << 20)
            if not b: break
            f.write(b); n += len(b)
    print('ok', n, 'bytes')
except urllib.error.HTTPError as e:
    print('HTTP', e.code, e.read()[:400].decode(errors='replace'))
except Exception as e:
    print('error', type(e).__name__, str(e)[:300])
