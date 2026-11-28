fn check_snapshot_binding(s:&Snapshot,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 let root=s.decoded.as_ref().and_then(|d|d.value.as_ref()).ok_or("snapshot decode absent")?;
 check_identity_binding(root,s.index,c,map_ack,map,pattern)
}
/// Caller must reserve the same identity-check work/bytes as the complete path
/// and check that same absolute deadline after these comparisons before use.
/// This checks original identity only; it never grants row export admission.
fn check_prefix_binding(a:&wire::strict_prefix::Audit,index:u64,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 if a.result()!=Ok(wire::strict_prefix::State::IncompleteIdentityAvailable){return Err("wire prefix identity unavailable".into())}
 let root=a.binding_prefix().ok_or("wire prefix identity absent")?;
 check_identity_binding(root,index,c,map_ack,map,pattern)
}
/// Common complete-root/prefix binding checks under the caller's prior admission.
fn check_identity_binding(root:&wire::Value,index:u64,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 if root.get("snapshot_index").and_then(wire::Value::as_u64)!=Some(index){return Err("wire snapshot index".into())}
 let id=root.get("identity").ok_or("wire identity absent")?;
 for(k,h)in [("public_sha256",&c.public_sha256),("factor_sha256",&c.factor_sha256),("launch_sha256",&c.launch_sha256),("binding_sha256",&c.binding_sha256)]{
  let raw=id.get(k).and_then(wire::Value::as_bytes).ok_or("wire identity bytes")?;if raw.len()!=32||hex::encode(raw)!=*h{return Err("wire original identity mismatch".into())}
 }
 if id.get("pattern").and_then(wire::Value::as_u64)!=Some(pattern as u64)||id.get("map_bytes").and_then(wire::Value::as_bytes)!=Some(map)||id.get("map_published").and_then(wire::Value::as_bool)!=Some(true)||id.get("publication_attempted").and_then(wire::Value::as_bool)!=Some(true){return Err("wire original map mismatch".into())}
 let a=id.get("map_ack").ok_or("wire map receipt")?;
 for(k,v)in [("event_count",map_ack.event_count),("byte_end",map_ack.byte_end)]{if a.get(k).and_then(wire::Value::as_u64)!=Some(v){return Err("wire map receipt count".into())}}
 for(k,v)in [("binding_sha256",&map_ack.binding_sha256),("last_event_sha256",&map_ack.last_event_sha256)]{if a.get(k).and_then(wire::Value::as_str)!=Some(v.as_str()){return Err("wire map receipt hash".into())}}
 Ok(())
}
