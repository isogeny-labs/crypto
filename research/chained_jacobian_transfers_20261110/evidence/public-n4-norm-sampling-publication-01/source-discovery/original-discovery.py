"""Read-only discovery because connector run lists omit pull_request_target events."""
from pathlib import Path
from datetime import datetime,timezone
import urllib.request,os,json
d=Path('/workspace/scratch/ct71-raw-publication-discovery-01');d.mkdir(exist_ok=True)
url='https://api.github.com/repos/isogeny-labs/crypto/actions/workflows/artifact-autofix.yml/runs?head_sha=97b05738059d0c46eff5178c6474ec5b1a86809e&per_page=30'
request=urllib.request.Request(url,headers={'Authorization':'Bearer '+os.environ['GH_TOKEN'],'Accept':'application/vnd.github+json'})
with urllib.request.urlopen(request,timeout=45)as response:raw=response.read()
data=json.loads(raw);stamp=datetime.now(timezone.utc).strftime('%H%M%S');(d/('artifact-autofix-runs-'+stamp+'.json')).write_bytes(raw)
(d/('request-'+stamp+'.json')).write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),url=url,method='GET',credential='GH_TOKENinmemoryonly;notretained',source='97b05738059d0c46eff5178c6474ec5b1a86809e'),indent=2)+'\n')
(d/'original-discovery.py').write_bytes(Path(__file__).read_bytes())
print(json.dumps([dict(id=x['id'],event=x['event'],status=x['status'],conclusion=x['conclusion'],head_sha=x['head_sha'],url=x['html_url'])for x in data['workflow_runs']]))
