pub(super) fn requested_channel_disposition(config:&CaseConfig)->Result<Option<Value>,String>{
    let Some(c)=&config.channels else {return Ok(None)};
    c.validate()?;
    let owned_boolean_search=c.owned_boolean_search.as_ref().map(|r| {
        let eligible=koblitz_m3::sat_original_owner::owned_search_worker::supported(config);
        super::types::ChannelDisposition{request:r.clone(),status:if eligible{super::types::ChannelDispositionStatus::Requested}else{super::types::ChannelDispositionStatus::Unsupported},
            reason:if eligible{super::types::ChannelDispositionReason::RequiresActualOwnedSearchReplay}else{super::types::ChannelDispositionReason::OutsideOwnedSearchProfile},execution_verified:false,training_export_admitted:false}
    });
    let sat_search=c.sat_search.as_ref().map(|r| {
        let eligible=koblitz_m3::sat_original_owner::search_worker::supported(config);
        super::types::ChannelDisposition{request:r.clone(),status:if eligible{super::types::ChannelDispositionStatus::Requested}else{super::types::ChannelDispositionStatus::Unsupported},
            reason:if eligible{super::types::ChannelDispositionReason::RequiresActualSatSearchReplay}else{super::types::ChannelDispositionReason::OutsideSatSearchProfile},execution_verified:false,training_export_admitted:false}
    });
    let boolean=c.boolean_consequence.as_ref().map(|r| {
        let eligible=config.family=="binary" && config.m==3
            && ((23..=31).contains(&config.n)||(47..=61).contains(&config.n))
            && matches!(config.solver.as_str(),"f4"|"f5") && config.factor_base_size==4
            && (1..=6).contains(&config.degree_bound) && config.work_limit>0;
        super::types::ChannelDisposition {request:r.clone(),
            status:if eligible {super::types::ChannelDispositionStatus::Requested} else {super::types::ChannelDispositionStatus::Unsupported},
            reason:if eligible {super::types::ChannelDispositionReason::RequiresActualOriginalShapeAndRetainedRow} else {super::types::ChannelDispositionReason::OutsideOrdinaryN3Profile},
            execution_verified:false,training_export_admitted:false}
    });
    let sat=c.sat_initial.as_ref().map(|r| {
        let eligible=koblitz_m3::sat_original_owner::worker::eligibility(config)
            == koblitz_m3::sat_original_owner::worker::Eligibility::Requested;
        super::types::ChannelDisposition {request:r.clone(),
            status:if eligible {super::types::ChannelDispositionStatus::Requested} else {super::types::ChannelDispositionStatus::Unsupported},
            reason:if c.schema == "crypto.ml-dataset.channel-requests/v1" {super::types::ChannelDispositionReason::SatCentralWorkerReplayNotIntegrated} else if eligible {super::types::ChannelDispositionReason::RequiresActualSatInitialReplay} else {super::types::ChannelDispositionReason::OutsideSatInitialProfile},
            execution_verified:false,training_export_admitted:false}
    });
    let affine_boolean_row=c.affine_boolean_row.as_ref().map(|r| {
        let eligible=koblitz_m3::sat_original_owner::affine_worker::supported(config);
        super::types::ChannelDisposition {request:r.clone(),
            status:if eligible {super::types::ChannelDispositionStatus::Requested} else {super::types::ChannelDispositionStatus::Unsupported},
            reason:if eligible {super::types::ChannelDispositionReason::RequiresActualAffineUnitRow} else {super::types::ChannelDispositionReason::OutsideAffineUnitRowProfile},
            execution_verified:false,training_export_admitted:false}
    });
    Ok(Some(serde_json::to_value(super::types::RequestedChannelDispositions {
        schema:"crypto.ml-dataset.requested-channels/v1".into(),owned_boolean_search,sat_search,boolean_consequence:boolean,sat_initial:sat,affine_boolean_row,
    }).map_err(|e|e.to_string())?))
}
