from pathlib import Path
import json,hashlib,os
D=Path(__file__).resolve().parent;old=D.parent/'feature-native-resultant-protocol';p=json.loads((old/'protocol.draft.json').read_text())
p['version']=2;p['study_dir']='research/primefield-companion-followup-20261102/feature-native-resultant-failure-flow'
for item in p['extensions']['bindings'].values():item['path']=os.path.relpath((old/item['path']).resolve(),D)
p['analysis_plan']['script']['path']=os.path.relpath((old/p['analysis_plan']['script']['path']).resolve(),D)
for arm in p['arms']:
 arm['instrument']['commit']=None;arm['instrument']['binary_sha256']=None
 arm['instrument']['build'][0][-3]='research/primefield-companion-followup-20261102/feature-native-resultant-failure-flow/main.rs'
for name,path in {'panel_main':D/'main.rs','journal':D/'journal.rs','capture':D/'capture.py','success_adapter':D/'success.py','failure_adapter':D/'failure.py','predecessor_draft':old/'protocol.draft.json'}.items():
 p['extensions']['bindings'][name]={'path':os.path.relpath(path,D),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
p['extensions']['failure_export_policy']='Export only journal-started cells; list never-started cells separately. Failed exports are schema-valid but retain canonical completeness findings and exclude all rows. Complete success requires all240 completion markers.'
p['invalidation_rules'].append('Missing/truncated/contradictory journal or absent completion markers invalidates successful comparison; failure exports never authorize retry.')
p['links']['status']='DRAFT NOT ADMITTED: new journal source revision/production binary binding, admitted preflight, freeze/publication and actual funds/capacity evidence still missing.'
(D/'protocol.draft.json').write_text(json.dumps(p,indent=2)+'\n')
