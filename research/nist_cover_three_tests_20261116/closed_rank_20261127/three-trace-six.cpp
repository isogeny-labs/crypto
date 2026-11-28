#include "shifted-six-arithmetic.hpp"
#include "three-trace-controls.hpp"

namespace shifted {
int run(int argc,char** argv) {
    require(argc==9,"usage: three-trace-six QUARTIC BACKMAP BASELINE TANGENTS PRIOR_SIX SHIFTED_SIX CANONICAL_SIX EXCEPTIONAL_SIX");
    const auto exceptional_result=read_json(argv[8]);
    require(exceptional_result.at("closed_rank").integer()==6&&exceptional_result.at("distinct_rows_including_zero").integer()==0&&exceptional_result.at("controls_pass").scalar=="true","completed target2kappa six-image bound");
    const auto canonical_result=read_json(argv[7]);
    require(canonical_result.at("closed_rank").integer()==6&&canonical_result.at("distinct_rows_including_zero").integer()==7&&canonical_result.at("controls_pass").scalar=="true","completed distinct H+2R bound");
    const auto shifted_result=read_json(argv[6]);
    require(shifted_result.at("closed_rank").integer()==6&&shifted_result.at("distinct_rows_including_zero").integer()==1&&shifted_result.at("controls_pass").scalar=="true","completed distinct 2E_ref shifted bound");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),tangents=read_json(argv[4]),prior=read_json(argv[5]);
    require(prior.at("domain_columns").integer()==225&&prior.at("closed_rank").integer()==6&&
            prior.at("distinct_nonzero_spanning_rows").integer()==0&&prior.at("controls_pass").scalar=="true","bound completed zero-target six result");
    ss_private(baseline,tangents);const auto curve=read_bp(model.at("terms"),true);std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"reference actual map global identity");
    std::vector<AP> base,all_base;std::set<std::string> keys;
    for(const auto& q:tangents.at("projected_base").arr) {
        auto p=audit::read_point(q);require(!p.zero&&audit::on_curve(p)&&p.y<=audit::p-p.y&&keys.insert(p.x.get_str()).second,"distinct canonical tangent columns");all_base.push_back(p);
    }
    require(all_base.size()==429&&baseline.at("projected_base").arr.size()==225,"frozen column dimensions");
    for(size_t i=0;i<225;++i){const auto p=audit::read_point(baseline.at("projected_base").arr[i]);require(p==all_base[i],"exact original prefix");base.push_back(p);}
    const auto& support=baseline.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr;
    require(support.size()==4,"canonical reference section degree four");
    std::array<int,6> pole_ids{};Sparse reference;Word formal{};AP target;::Point target_bn;
    std::array<std::array<F,2>,4> coordinates{};
    for(size_t i=0;i<6;++i) {
        const int id=i<4?support[i].integer():0;pole_ids[i]=id;
        const auto& q=baseline.at("pool").at("points").arr.at(size_t(id));
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        require(fs_eval(curve,x,y)==F(0),"actual pole support lies on curve");
        const int saved_precision=precision;const auto mapped=fs_image(curve,map,x,y);precision=saved_precision;
        const auto value=mapped.value;const auto& saved=baseline.at("images").arr.at(size_t(id));
        require(value==point(saved.at("point")),"fresh actual pole support image");
        if(i<4)coordinates[i]={x,y};
        else require(value.zero&&mapped.local,"R has actual exceptional image O");
        const auto actual=gmp_point(value);target=audit::plus(target,actual);target_bn=add(target_bn,value);
        const int column=saved.at("projected_index").integer(),sign=saved.at("sign").integer();
        if(column<0){require(column==-1&&sign==0&&value.zero,"identity pole label");formal[i]=0;}
        else {
            require(column<225&&(sign==1||sign==-1),"bounded pole image label");
            formal[i]=2*column+(sign==1?1:2);
            require(symbol(formal[i],base)==actual,"derived pole signed label matches actual image");
            reference[column]+=sign;
        }
    }
    for(auto it=reference.begin();it!=reference.end();)if(it->second==0)it=reference.erase(it);else ++it;
    require(gmp_point(target_bn)==target&&!target.zero,"actual nonzero canonical pole trace");
    const auto& a=coordinates[0];const auto& b=coordinates[1];const F dx=b[0]-a[0],dy=b[1]-a[1];
    require(dx!=F(0)||dy!=F(0),"distinct canonical support anchors");
    const auto restriction=ss_restrict(curve,a[0],a[1],dx,dy);
    require(restriction.degree()==4,"complete finite canonical section");Poly factors{F(1)};std::set<std::string> distinct;
    for(const auto& xy:coordinates) {
        require(distinct.insert(sp_key(xy[0],xy[1])).second&&(xy[0]-a[0])*dy==(xy[1]-a[1])*dx,"distinct support on actual canonical line");
        const F t=dx!=F(0)?(xy[0]-a[0])/dx:(xy[1]-a[1])/dy;
        factors=factors*Poly{-t,F(1)};
    }
    require(factors.scale(restriction.at(4))==restriction,"exact canonical line divisor factor product");
    const auto& rq=baseline.at("pool").at("points").arr.at(0);
    const F rx=coefficient(rq.at("x")),ry=coefficient(rq.at("y"));
    require(dy*(rx-a[0])-dx*(ry-a[1])!=F(0),"reference H disjoint from R");
    const F fx=fs_eval(curve.partial(0),rx,ry),fy=fs_eval(curve.partial(1),rx,ry);
    require(fx!=F(0)||fy!=F(0),"exceptional R is smooth");
    const AP kappa=target;target=norm_test::multiply(kappa,3);
    const auto actual_traces=actual_trace_controls(curve,map,baseline,kappa,base[0]);
    const SCBig half=(sc_n+1)/2;const AP rho=norm_test::multiply(target,half);
    for(auto& [column,c]:reference){(void)column;c*=3;}
    audit::SSL ssl;
    require(!target.zero&&!(target==kappa)&&!(target==audit::plus(kappa,kappa))&&!(target==audit::read_point(shifted_result.at("target"))),"new target distinct from completed zero/kappa/2kappa/2rho targets");
    require(audit::plus(rho,rho)==target,"native half of triple canonical target");
    auto named_target=ssl.make(target),named_half=ssl.make({});auto half_bn=audit::bn(half);
    require(EC_POINT_mul(ssl.group.get(),named_half.get(),nullptr,named_target.get(),half_bn.get(),ssl.context.get())==1&&ssl.encode(named_half.get())==audit::key(rho),"separate named half triple-target check");
    require(named_sum(norm_test::dense(reference,225),base,target,ssl),"triple canonical reference target named check");
    auto bad_reference=norm_test::dense(reference,225);++bad_reference[0];
    require(!(norm_test::row_sum(bad_reference,base)==target)&&!named_sum(bad_reference,base,target,ssl),"changed actual reference coefficient rejected before enumeration");
    SCMat old(6,SCRow(225));
    require(baseline.at("rows").arr.size()==6,"exact original six rows");
    for(size_t i=0;i<6;++i) {
        for(const auto& pair:baseline.at("rows").arr[i].arr)old[i].at(size_t(pair.arr[0].integer()))+=SCBig(pair.arr[1].scalar);
        require(norm_test::row_sum(old[i],base).zero&&named_sum(old[i],base,{},ssl),"original baseline row both engines");
    }
    require(sc_rank(old)==6,"frozen original six-dimensional span");auto full=sc_read_matrix(tangents.at("matrix"));
    require(full.size()==108&&sc_rank(full)==108,"full tangent rank108");SCMat auxiliary;
    for(const auto& row:full){require(row.size()==429&&norm_test::row_sum(row,all_base).zero&&named_sum(row,all_base,{},ssl),"full tangent rows both engines");auxiliary.emplace_back(row.begin()+225,row.end());}
    require(sc_rank(auxiliary)==102,"original auxiliary rank102");MIBasis original_basis(old),full_basis(full);
    controls();trace_closure_fixture();auto table=enumerate(base,rho);Counts grouped_counts,merged_counts;
    const auto found=grouped(table,reference,grouped_counts),checked=merged(table,reference,merged_counts);
    require(found==checked&&grouped_counts.opposite_pairs==merged_counts.opposite_pairs&&grouped_counts.identity_pairs==merged_counts.identity_pairs,
            "complete shifted joins and exact witness sets agree");
    verify(found,reference,base,target);
    size_t independent=0;
    for(const auto& [s,w]:found) {
        (void)w;auto row=norm_test::dense(s,225);const auto dependent=original_basis.representation(row).first;auto extended=row;extended.resize(429);
        require(full_basis.representation(extended).first==dependent,"old-column and auxiliary-eliminated candidate dependence agrees");
        if(!dependent){original_basis.append(row,"shifted-candidate:"+std::to_string(independent));full_basis.append(extended,"shifted-candidate:"+std::to_string(independent));++independent;}
    }
    const size_t closed=sc_rank(original_basis.source),frank=sc_rank(full_basis.source);SCMat final_aux;
    for(const auto& row:full_basis.source)final_aux.emplace_back(row.begin()+225,row.end());
    const auto arank=sc_rank(final_aux);require(arank==102&&frank-arank==closed&&closed==6+independent,"separate actual closure check");
    std::cout<<"{\"scope\":\"three-whole-residual-traces-six-image-bound\",\"old_columns\":225,\"signed_alphabet\":451,\"trace_count\":3,\"old_anchor_slots\":6,\"reference_curve_indices\":[";
    for(size_t i=0;i<4;++i)std::cout<<(i?",":"")<<pole_ids[i];
    std::cout<<"],\"canonical_reference_multiplier\":3,\"actual_split_trace_controls\":3,\"three_nonzero_trace_fixture_pass\":true,\"reference\":";
    sc_emit_row(std::cout,norm_test::dense(reference,225));std::cout<<",\"rho\":";emit_point(rho);std::cout<<",\"target\":";emit_point(target);
    std::cout<<",\"target_is_zero\":false,\"target_is_kappa\":"<<(target==kappa?"true":"false")<<",\"target_is_twice_kappa\":"<<(target==audit::plus(kappa,kappa)?"true":"false")
             <<",\"target_is_thrice_kappa\":true,\"unordered_triples\":"<<table.rows.size()<<",\"packed_record_bytes\":32,\"table_payload_bytes\":"<<table.rows.size()*sizeof(Packed)
             <<",\"join_index_payload_bytes\":"<<merged_counts.index_payload<<",\"triple_stream_sha256\":\""<<table.digest
             <<"\",\"independent_triple_streams_match\":true,\"independent_join_sets_match\":true,\"sum_groups\":"<<grouped_counts.groups
             <<",\"largest_sum_group\":"<<grouped_counts.largest<<",\"opposite_orientation_pairs\":"<<grouped_counts.opposite_pairs
             <<",\"identity_pairs\":"<<grouped_counts.identity_pairs<<",\"distinct_rows_including_zero\":"<<found.size()
             <<",\"independent_added_rows\":"<<independent<<",\"matrix_rows\":"<<full_basis.source.size()<<",\"matrix_columns\":429,\"full_rank\":"<<frank
             <<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<closed<<",\"candidate_gain\":"<<closed-6
             <<",\"controls_pass\":true,\"fixture_rank\":1,\"fixture_rows_in_actual_matrix\":0,\"formal_reference_required\":false,\"positive_gain_established\":false,\"rows\":[";
    bool comma=false;for(const auto& [row,w]:found) {
        if(comma)std::cout<<',';
        comma=true;std::cout<<"{\"labels\":[";
        for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<w.labels[i];
        std::cout<<"],\"overall_sign\":"<<w.sign<<",\"row\":";sc_emit_row(std::cout,norm_test::dense(row,225));std::cout<<'}';
    }
    std::cout<<"],\"actual_trace_witnesses\":";emit_trace_witnesses(actual_traces);
    std::cout<<",\"standing\":\"producer-unverified;necessary image condition only;independent candidates need actual RR/KM and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv){try{return shifted::run(argc,argv);}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
