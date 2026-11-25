map({prime,depth,base_size,catalog_charts,unsigned_tuples,signed_bindings,
  identity_exclusions,target_x_exclusions,rr5_bindings_checked,
  old_rank_two:([.charts[].old_effective_ranks[2]]|add),
  old_rank_three:([.charts[].old_effective_ranks[3]]|add),
  new_rank_seven:([.charts[].factorwise_ranks[7]]|add),
  minimum_terms:([.charts[].term_range[0]]|min),
  maximum_terms:([.charts[].term_range[1]]|max),
  maximum_equation_degree:([.charts[].maximum_equation_degree]|max),solver_calls})
