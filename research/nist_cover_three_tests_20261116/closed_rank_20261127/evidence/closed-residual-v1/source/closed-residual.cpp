#include "shifted-six-arithmetic.hpp"
#include "closed-residual-engine.hpp"

namespace shifted {
int run(int argc,char** argv) {
    require(argc==7,"usage: closed-residual QUARTIC BACKMAP BASELINE TANGENTS KAPPA_SIX OLD_NORM");
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),tangents=read_json(argv[4]),canonical=read_json(argv[5]),norm=read_json(argv[6]);
    require(canonical.at("controls_pass").scalar=="true"&&canonical.at("closed_rank").integer()==6&&canonical.at("distinct_rows_including_zero").integer()==7,"completed kappa six-symbol bound reused");
    require(norm.at("controls_pass").scalar=="true"&&norm.at("augmented").scalar=="false"&&norm.at("total_columns").integer()==225,"saved original pair stream reference");
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
    const AP kappa=target;audit::SSL ssl;
    require(named_sum(norm_test::dense(reference,225),base,kappa,ssl),"canonical reference group word checked before search");
    const auto& rq=baseline.at("pool").at("points").arr.at(0);
    const F rx=coefficient(rq.at("x")),ry=coefficient(rq.at("y"));
    require(dy*(rx-a[0])-dx*(ry-a[1])!=F(0),"reference line disjoint from exceptional R");
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
    std::set<Sparse> imported;
    for(const auto& saved:canonical.at("rows").arr) {
        const auto& values=saved.at("row").arr;require(values.size()==225,"imported canonical row width");SCRow row;
        for(const auto& value:values)row.emplace_back(value.scalar);
        require(norm_test::row_sum(row,base).zero&&named_sum(row,base,{},ssl)&&original_basis.representation(row).first,
                "imported complete kappa rows have group validity and exact baseline representation");
        require(imported.insert(norm_test::sparse(row)).second,"distinct imported kappa rows");
    }
    require(imported.size()==7&&imported.count(Sparse{}),"m4 necessary bound reused with all seven rows");
    const auto nonzero_control=closed_residual::control(false),identity_control=closed_residual::control(true);
    const auto pairs=norm_test::enumerate(base,kappa);closed_residual::complete_pairs(pairs.pairs,base.size());
    require(pairs.digest==norm.at("pair_stream_sha256").scalar,"reconstructed pair stream matches frozen original digest");
    const auto queries=closed_residual::multiple_queries(base);
    const auto found=closed_residual::lookup(pairs.pairs,queries.values,reference),checked=closed_residual::sorted_merge(pairs.pairs,queries.values,reference);
    closed_residual::agree(found,checked);closed_residual::verify(found,reference,base,kappa);
    size_t independent=0;
    for(const auto& [s,w]:found.rows) {
        (void)w;const auto row=norm_test::dense(s,225);auto extended=row;extended.resize(429);
        const bool dependent=original_basis.representation(row).first;
        require(full_basis.representation(extended).first==dependent,"old and auxiliary-eliminated residual dependence agrees");
        if(!dependent){original_basis.append(row,"closed-residual:"+std::to_string(independent));full_basis.append(extended,"closed-residual:"+std::to_string(independent));++independent;}
    }
    const size_t closed=sc_rank(original_basis.source),frank=sc_rank(full_basis.source);SCMat final_aux;
    for(const auto& row:full_basis.source)final_aux.emplace_back(row.begin()+225,row.end());
    const auto arank=sc_rank(final_aux);require(arank==102&&frank-arank==closed&&closed==6+independent,"separate closed residual elimination");
    std::set<std::string> distinct_queries;for(const auto& query:queries.values)distinct_queries.insert(query.key);
    std::cout<<"{\"scope\":\"degree-five-six-same-image-closed-residual-necessary-bound\",\"old_columns\":225,\"signed_alphabet\":451,\"degrees\":[5,6],\"reference_curve_indices\":[";
    for(size_t i=0;i<4;++i)std::cout<<(i?",":"")<<pole_ids[i];
    std::cout<<"],\"exceptional_point_index\":0,\"exceptional_map_verified\":true,\"reference\":";
    sc_emit_row(std::cout,norm_test::dense(reference,225));std::cout<<",\"kappa\":";emit_point(kappa);
    std::cout<<",\"pair_records_reconstructed_once\":"<<pairs.pairs.size()<<",\"pair_family_domain_slots\":"<<2*pairs.pairs.size()
             <<",\"pair_stream_sha256\":\""<<pairs.digest<<"\",\"original_pair_digest_matches\":true,\"query_records\":"<<queries.values.size()
             <<",\"distinct_query_points\":"<<distinct_queries.size()<<",\"query_stream_sha256\":\""<<queries.digest
             <<"\",\"independent_pair_and_query_streams_match\":true,\"independent_join_sets_match\":true,\"families\":[";
    for(size_t i=0;i<2;++i) {
        if(i)std::cout<<',';
        std::cout<<"{\"m\":"<<i+5<<",\"R_pole_weight\":"<<i+3<<",\"pair_domain\":"<<pairs.pairs.size()<<",\"query_domain\":451,\"raw_matches\":"<<found.matches[i]<<",\"identity_query_matches\":"<<found.identity_matches[i]<<'}';
    }
    std::cout<<"],\"raw_matches\":"<<found.slots.size()<<",\"distinct_rows_including_zero\":"<<found.rows.size()<<",\"independent_added_rows\":"<<independent
             <<",\"matrix_rows\":"<<full_basis.source.size()<<",\"matrix_columns\":429,\"full_rank\":"<<frank<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<closed
             <<",\"candidate_gain\":"<<closed-6<<",\"m4_bound_reused\":true,\"imported_kappa_rows\":7,\"old_six_symbol_enumerations_rerun\":0,\"controls_pass\":true,\"fixture_rank\":1,\"fixture_rows_in_actual_matrix\":0,\"nonzero_fixture_matches\":["
             <<nonzero_control.matches[0]<<','<<nonzero_control.matches[1]<<"],\"identity_fixture_identity_matches\":["<<identity_control.identity_matches[0]<<','<<identity_control.identity_matches[1]
             <<"],\"positive_gain_established\":false,\"rows\":[";
    bool comma=false;for(const auto& [row,w]:found.rows) {
        if(comma)std::cout<<',';
        comma=true;std::cout<<"{\"witness\":";closed_residual::emit_witness(w);std::cout<<",\"row\":";sc_emit_row(std::cout,norm_test::dense(row,225));std::cout<<'}';
    }
    std::cout<<"],\"raw_match_slots\":[";comma=false;
    for(const auto& slot:found.slots){if(comma)std::cout<<',';comma=true;std::cout<<'['<<slot[0]<<','<<slot[1]<<','<<slot[2]<<','<<slot[3]<<']';}
    std::cout<<"],\"standing\":\"producer-unverified;necessary image condition only;positive needs actual closed-fibre/RR/KM and separate replay\"}\n";
    return 0;
}
}
int main(int argc,char** argv){try{return shifted::run(argc,argv);}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
