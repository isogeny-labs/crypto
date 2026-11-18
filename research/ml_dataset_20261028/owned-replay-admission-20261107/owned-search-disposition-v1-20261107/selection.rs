pub(in crate::ml_dataset::solvers) fn selection(c:&CaseConfig)->Option<(u32,boolean_search::ProposedPolicy)>{
    let ch=c.channels.as_ref()?;
    if ch.validate().is_err()||ch.schema!="crypto.ml-dataset.channel-requests/v5" {return None}
    let OwnedBooleanSearchRequest::MatrixContinuation{pattern,policy}=ch.owned_boolean_search.as_ref()?;
    if !profile(&c.family,c.n,c.m)||c.solver!="f4"||c.factor_base_size==0||c.work_limit==0||c.degree_bound==0{return None}
    let total=c.m.checked_sub(2).and_then(|n|u32::try_from(n).ok()).and_then(|n|1u32.checked_shl(n))?;
    if *pattern>=total{return None}
    let chosen=match policy{OwnedBooleanSearchPolicy::ConservativeAffineEnvelopeV1=>boolean_search::ProposedPolicy::ConservativeAffineEnvelopeV1};
    Some((*pattern,chosen))
}
pub(in crate::ml_dataset::solvers) fn supported(c:&CaseConfig)->bool{selection(c).is_some()}
