# Retrospective replay helper, authored after the recorded source verification.
# Original executed commands/receipts remain in download-receipts.json and
# source-verification.json. This helper has not been executed.
from pathlib import Path
import json,subprocess,hashlib,sys
view=Path(__file__).parent;dest=Path(sys.argv[1]);dest.mkdir(exist_ok=False)
for n,rec in json.loads((view/'download-receipts.json').read_text()).items():
 subprocess.run(['curl','--fail','--location','--proto','=https','--tlsv1.2','--max-time','45','--output',str(dest/n),rec['url']],check=True)
 assert hashlib.sha256((dest/n).read_bytes()).hexdigest()==rec['sha256']
gpg=dest/'gnupg';gpg.mkdir(mode=0o700)
base=['gpg','--homedir',str(gpg),'--batch','--no-autostart']
# Import may report missing optional agent; fingerprints and signature are the gate.
r=subprocess.run(base+['--import',str(dest/'musl.pub')],capture_output=True,text=True)
(dest/'import.stdout').write_text(r.stdout);(dest/'import.stderr').write_text(r.stderr)
f=subprocess.check_output(base+['--with-colons','--fingerprint'],text=True)
assert [x.split(':')[9]for x in f.splitlines()if x.startswith('fpr:')][0]=='836489290BB6B70F99FFDA0556BCDB593020450F'
r=subprocess.run(base+['--status-fd','1','--verify',str(dest/'musl-1.2.6.tar.gz.asc'),str(dest/'musl-1.2.6.tar.gz')],capture_output=True,text=True)
(dest/'verify.stdout').write_text(r.stdout);(dest/'verify.stderr').write_text(r.stderr)
assert r.returncode==0 and '[GNUPG:] VALIDSIG 'in r.stdout
