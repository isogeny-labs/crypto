pub struct LiftRecord {
    pub partition:usize,pub record:u64,pub model:Vec<bool>,
    pub progress:checked_model::Progress,
    pub callback_result:Option<Result<(),continuation::Failure>>,
    pub returned:Option<Result<signed_lift::Outcome,Stop>>,
}
struct RetainedRecords<'a>(&'a [LiftRecord]);
impl exclusions::Records for RetainedRecords<'_>{
    fn len(&self)->usize{self.0.len()}
    fn get(&self,index:usize)->Option<exclusions::RecordView<'_>>{
        let r=self.0.get(index)?;
        Some(exclusions::RecordView{partition:r.partition,record:r.record,model:&r.model,
            callback_result:r.callback_result,returned_outcome:r.returned.and_then(Result::ok)})
    }
}
