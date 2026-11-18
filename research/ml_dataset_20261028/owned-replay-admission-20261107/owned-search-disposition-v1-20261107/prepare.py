from pathlib import Path
import difflib,hashlib,json
P=Path(__file__).parent;R=Path('/home/ec2-user/crypto-ml-ordinary-ownership-20261101/src/ml_dataset')
def extract(s,name):
 start=s.index('fn '+name+'(');start=s.rfind('\n',0,start)+1;brace=s.index('{',start);depth=1;end=brace+1
 while depth:
  depth+=(s[end]=='{')-(s[end]=='}');end+=1
 return s[start:end]+'\n'
oldtypes=(R/'types.rs').read_text();newtypes=oldtypes.replace('    RequiresActualSatSearchReplay, OutsideSatSearchProfile,','    RequiresActualSatSearchReplay, OutsideSatSearchProfile,\n    RequiresActualOwnedSearchReplay, OutsideOwnedSearchProfile,',1).replace('pub struct RequestedChannelDispositions {','pub struct RequestedChannelDispositions {\n    #[serde(default,skip_serializing_if="Option::is_none")]\n    pub owned_boolean_search:Option<ChannelDisposition<OwnedBooleanSearchRequest>>,',1)
oldsolver=(R/'solvers.rs').read_text();oldfn=extract(oldsolver,'requested_channel_disposition');newfn=oldfn.replace('    let sat_search=', '''    let owned_boolean_search=c.owned_boolean_search.as_ref().map(|r| {
        let eligible=koblitz_m3::sat_original_owner::owned_search_worker::supported(config);
        super::types::ChannelDisposition{request:r.clone(),status:if eligible{super::types::ChannelDispositionStatus::Requested}else{super::types::ChannelDispositionStatus::Unsupported},
            reason:if eligible{super::types::ChannelDispositionReason::RequiresActualOwnedSearchReplay}else{super::types::ChannelDispositionReason::OutsideOwnedSearchProfile},execution_verified:false,training_export_admitted:false}
    });
    let sat_search=''',1).replace('schema:"crypto.ml-dataset.requested-channels/v1".into(),sat_search,','schema:"crypto.ml-dataset.requested-channels/v1".into(),owned_boolean_search,sat_search,',1)
newsolver=oldsolver.replace(oldfn,newfn,1)
for name,old,new in [('types.rs',oldtypes,newtypes),('solvers.rs',oldsolver,newsolver)]:
 (P/name).write_text(new);(P/(name+'.base')).write_text(old);(P/(name+'.patch')).write_text(''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/src/ml_dataset/'+name,tofile='b/src/ml_dataset/'+name)))
(P/'disposition.rs').write_text(newfn)
# Legacy control is exact original function, with a None field solely to satisfy
# the new struct definition; skip-none prevents this fixture adaptation encoding.
(P/'legacy_disposition.rs').write_text(oldfn.replace('fn requested_channel_disposition(', 'fn legacy_disposition(',1).replace('schema:"crypto.ml-dataset.requested-channels/v1".into(),sat_search,','schema:"crypto.ml-dataset.requested-channels/v1".into(),owned_boolean_search:None,sat_search,',1))
s=(R/'owned_boolean_search_worker.rs').read_text();(P/'selection.rs').write_text(''.join(extract(s,n) for n in ['selection','supported']));(P/'profile.rs').write_text(extract((R/'sat_original_owner.rs').read_text(),'profile'))
(P/'SOURCE_BINDINGS.json').write_text(json.dumps([{'path':str(R/n),'sha256':hashlib.sha256((R/n).read_bytes()).hexdigest(),'bytes':(R/n).stat().st_size} for n in ['types.rs','solvers.rs','owned_boolean_search_worker.rs','sat_original_owner.rs']],indent=2)+'\n')
