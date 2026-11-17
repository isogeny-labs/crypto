def compact: {id,curve,kind,bases,labels,columns:.metrics.columns,signed_points:.metrics.signed_points,support:.metrics.support,new_cross_targets,single_support_union,rank:.metrics.same_target_difference_rank_nonzero_projection_columns,zero_projection_columns:.metrics.zero_prime_projection_columns,active_columns:.metrics.active_columns,active_components:.metrics.active_pair_graph_components};
{
  schema:"factor-base-chart-summary/v1",
  input_cases:(.input_cases|length), distinct_nonempty_bases,
  case_count:(.rows|length), union_count:([.rows[]|select(.kind=="union")]|length),
  capped_cases:(.skipped|length),
  unique_pair_direct_scalar_checks:([.curve_audits[].unique_pair_direct_scalar_checks]|add),
  curve_audits,
  selections:[.rows|group_by(.curve)[]|{
    curve:.[0].curve,
    singleton_cases:([.[]|select(.kind=="single")]|length),
    union_cases:([.[]|select(.kind=="union")]|length),
    best_cross:([.[]|select(.kind=="union")]|sort_by(-.new_cross_targets,.id)|.[0]|compact),
    smallest_cross:([.[]|select(.kind=="union" and .new_cross_targets>0)]|sort_by(.metrics.columns,-.new_cross_targets,.id)|.[0]|compact),
    highest_useful_rank:([.[]|select(.kind=="union")]|sort_by(-.metrics.same_target_difference_rank_nonzero_projection_columns,.metrics.columns,.id)|.[0]|compact)
  }],
  cases:[.rows[]|compact],
  plot_examples:[.rows[]|select(.id=="B029" or .id=="B031" or .id=="B035" or .id=="B027+B033" or .id=="B041" or .id=="B031+B041")|compact],
  matched_size_note:"The 14-column graft example was selected after inspecting results; the protocol's three selection rules are separately reported. No held-out test or equal-cost comparison.",
  certified_regularity:null, successful_f4_work:null, native_speedup:null, full_pipeline_cost:null
}
