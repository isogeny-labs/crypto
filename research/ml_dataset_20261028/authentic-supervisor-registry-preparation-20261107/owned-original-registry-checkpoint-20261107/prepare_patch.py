from pathlib import Path
import difflib,json,hashlib,subprocess
r=Path('/home/ec2-user/crypto-ml-checkpoint-integration-20261107');b=Path(__file__).parent
changes={}
for name in ['mod.rs','solvers.rs']:
 raw=(r/'src/ml_dataset'/name).read_text();(b/(name+'.base')).write_text(raw);changes[name]=[raw,raw]
s=changes['mod.rs'][1];s=s.replace('mod execution;','mod execution;\nmod original_registry;',1);s=s.replace('        "worker-artifacts.json",','        "worker-artifacts.json",\n        original_registry::PATH,',1)
s=s.replace('''    let registry: Vec<Artifact> = if dir.join("worker-artifacts.json").exists() {''','''    let terminal_exists = dir.join("worker-artifacts.json").exists();
    let registry: Vec<Artifact> = if terminal_exists {''',1)
old='''    storage::verify_manifest(dir, &registry)?;
    let registered: BTreeMap<_, _> = registry.iter().map(|a| (a.path.as_str(), a)).collect();'''
new='''    // Original roles can survive interruption only through the immutable
    // pre-token receipt checkpoint. A conflicting final registry is an error.
    let registry = original_registry::merge(dir, registry, terminal_exists)?;
    let registered: BTreeMap<_, _> = registry.iter().map(|a| (a.path.as_str(), a)).collect();'''
assert old in s;s=s.replace(old,new,1);changes['mod.rs'][1]=s
s=changes['solvers.rs'][1];start=s.index("pub(super) fn verify_and_store_worker_originals<'a>");end=s.index('/// Retained, unacknowledged bytes',start);part=s[start:end]
part=part.replace('registry.push(public);','registry.push(public.clone());',1)
part=part.replace('''    registry.push(super::storage::write_json(
        root,
        "witness.json",
        "restricted-known-answer",
        &case.private_witness,
    )?);''','''    let witness = super::storage::write_json(
        root,
        "witness.json",
        "restricted-known-answer",
        &case.private_witness,
    )?;
    registry.push(witness.clone());''',1)
part=part.replace('''    registry.push(factor_base);
    Ok(VerifiedWorkerOriginal {''','''    registry.push(factor_base.clone());
    // Publish original storage receipts before handing out the producer token.
    // This separate immutable file does not preempt the terminal registry.
    registry.push(super::original_registry::publish(root, [&public, &witness, &factor_base])?);
    Ok(VerifiedWorkerOriginal {''',1)
s=s[:start]+part+s[end:]
marker='''    #[test]
    fn worker_token_rejects_self_consistent_policy_substitution_before_original_writes()'''
new='''    #[test]
    fn original_registry_storage_failure_prevents_worker_token() {
        let f = Fixture::with_shape(11, false, 4);
        std::fs::create_dir(f.root.join(dataset::original_registry::PATH)).unwrap();
        let mut registry = Vec::new();
        let token = verify_and_store_worker_originals(&f.config, &f.case, &f.root, Some(&f.binding), &mut registry);
        assert!(token.is_err(), "failed immutable checkpoint must not yield producer authority");
        assert!(f.root.join("public.json").exists());
        assert!(f.root.join("factor-base.json").exists());
        assert!(!registry.iter().any(|a|a.path==dataset::original_registry::PATH));
        assert!(!f.root.join("live/committed.json").exists());
    }

'''+marker
assert marker in s;s=s.replace(marker,new,1);changes['solvers.rs'][1]=s
patch=''
for name,(old,new) in changes.items():
 (b/name).write_text(new);patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/src/ml_dataset/'+name,tofile='b/src/ml_dataset/'+name))
new=(b/'original_registry.rs').read_text();patch+=''.join(difflib.unified_diff([],new.splitlines(True),fromfile='/dev/null',tofile='b/src/ml_dataset/original_registry.rs'))
(b/'implementation.patch').write_text(patch)
(b/'SOURCE_BINDINGS.json').write_text(json.dumps({'source_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip(),'files':{name:{'base_sha256':hashlib.sha256(old.encode()).hexdigest(),'proposed_sha256':hashlib.sha256(new.encode()).hexdigest()} for name,(old,new) in changes.items()},'new_module_sha256':hashlib.sha256((b/'original_registry.rs').read_bytes()).hexdigest(),'patch_only':True,'production_modified':False,'tests_executed':False},indent=2)+'\n')
print('Patch generated only:',len(patch.encode()),'bytes')
