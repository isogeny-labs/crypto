import pathlib,json,subprocess,re
p=pathlib.Path(__file__).parent;prop=pathlib.Path('/var/tmp/pencil-musl-sparse-proposal-20261031');s={r['path']:r for r in json.loads((prop/'selected.json').read_text())['files']};scan=json.loads((prop/'include-scan.json').read_text());names=set(scan['scanned_sources']);names.update(n for n in s if n.startswith('research/five_leads_e2e_20261019/native/') and n.endswith('.rs'))
proc=subprocess.Popen(['git','cat-file','--batch'],cwd='/home/ec2-user/crypto-pencil-signed-graph-20261029',stdin=subprocess.PIPE,stdout=subprocess.PIPE);hits=[];extra=[];fixtures=[]
for n in sorted(names):
 proc.stdin.write((s[n]['object']+'\n').encode());proc.stdin.flush();h=proc.stdout.readline().split();b=proc.stdout.read(int(h[-1]));assert proc.stdout.read(1)==b'\n';t=b.decode()
 for i,l in enumerate(t.splitlines(),1):
  if re.search(r'include(?:_str|_bytes)?\s*!\s*[({\[]',l):
   hit={'source':n,'line':i,'text':l};hits.append(hit)
   if not re.search(r'include(?:_str|_bytes)?!\s*\(',l) or n not in scan['scanned_sources']:extra.append(hit)
  if re.search(r'include_dir!|RustEmbed|query_file!|OUT_DIR|CARGO_MANIFEST_DIR',l):fixtures.append({'source':n,'line':i,'text':l})
proc.stdin.close();assert proc.wait()==0
(p/'expanded-lexical-scan.json').write_text(json.dumps({'sources':len(names),'hits':hits,'additional_hits':extra,'other_embedding_markers':fixtures},indent=2)+'\n');print('sources',len(names),'hits',len(hits),'additional',len(extra),'other',len(fixtures));print(json.dumps(extra)[:6000]);print(json.dumps(fixtures)[:3000])
