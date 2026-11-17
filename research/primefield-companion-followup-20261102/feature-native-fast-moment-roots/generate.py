from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent
root=R/'feature-native-moment-root/lib.rs';s=root.read_text()
roots='pub struct RootSet{'+s.split('pub struct RootSet{',1)[1].split('\nimpl Projector',1)[0]+'\nimpl RootSet{'+s.split('impl RootSet{',1)[1].split('\nenum Representation',1)[0]+'\n'
source=R/'feature-native-inert-seed/lib.rs';s=source.read_text()
recovery='pub struct RecoverySource{'+s.split('pub struct RecoverySource{',1)[1].split('\n#[cfg(test)]mod inert_tests',1)[0]+'\n'
(D/'roots.rs').write_text(roots);(D/'recovery.rs').write_text(recovery)
sha=lambda b:hashlib.sha256(b).hexdigest()
(D/'generation.json').write_text(json.dumps({'sources':{str(p.relative_to(R)):sha(p.read_bytes()) for p in [root,source]},'outputs':{'roots.rs':sha(roots.encode()),'recovery.rs':sha(recovery.encode())},'transformation':'Extract RootSet and RecoverySource declarations/implementations unchanged; Consecutive resolves to new local fast-conversion adapter'},indent=2)+'\n')
