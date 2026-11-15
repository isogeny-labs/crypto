struct RetainedRecords<'a>(&'a [LiftRecord]);
impl exclusions::Records for RetainedRecords<'_>{
    fn len(&self)->usize{self.0.len()}
    fn get(&self,index:usize)->Option<exclusions::RecordView<'_>>{
        let r=self.0.get(index)?;
        Some(exclusions::RecordView{partition:r.partition,record:r.record,model:&r.model,
            callback_result:r.callback_result,returned_outcome:r.returned.and_then(Result::ok)})
    }
}
