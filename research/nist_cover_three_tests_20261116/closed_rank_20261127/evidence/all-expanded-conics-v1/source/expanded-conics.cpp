#define main frozen_expanded_conic_norm_entrypoint
#include "norms.cpp"
#undef main
#include "conic_arithmetic.hpp"
#include <cstdint>

using ECSparse=norm_test::Sparse;
struct ECPencil {std::array<int,2> image;Anchors anchors;};
struct ECDigest {
    std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> state{EVP_MD_CTX_new(),EVP_MD_CTX_free};
    ECDigest(){require(bool(state)&&EVP_DigestInit_ex(state.get(),EVP_sha256(),nullptr)==1,"conic digest initialize");}
    void add(size_t pencil,const std::string& key,const std::vector<int>& ids) {
        std::string payload=std::to_string(pencil)+":"+key;
        for(int id:ids)payload+=std::to_string(id)+",";
        require(payload.size()<65536,"bounded candidate digest frame");
        const auto n=std::uint32_t(payload.size());
        std::array<unsigned char,4> frame{};
        for(size_t i=0;i<4;++i)frame[i]=static_cast<unsigned char>((n>>(8*i))&255U);
        require(EVP_DigestUpdate(state.get(),frame.data(),frame.size())==1&&
                EVP_DigestUpdate(state.get(),payload.data(),payload.size())==1,"conic digest update");
    }
    std::string finish() {
        std::array<unsigned char,32> bytes{};unsigned int n=0;
        require(EVP_DigestFinal_ex(state.get(),bytes.data(),&n)==1&&n==32,"conic digest final");
        std::ostringstream out;
        for(auto b:bytes)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);
        return out.str();
    }
};
audit::Point ec_gmp(const ::Point& p) {
    if(p.zero)return {};
    return {false,audit::Big(decimal(p.x.p.at(0))),audit::Big(decimal(p.y.p.at(0)))};
}
ECSparse ec_sparse(const SCRow& row) {
    ECSparse out;
    for(size_t i=0;i<row.size();++i) {
        SCBig value=sc_reduce(row[i]);if(value>sc_n/2)value-=sc_n;
        require(value>=-16&&value<=16,"small conic source coefficient");
        if(value!=0)out[int(i)]=int(value.get_si());
    }
    return out;
}
ECSparse ec_read_sparse(const Json& row,size_t width) {
    ECSparse out;
    for(const auto& pair:row.arr) {
        const int column=pair.arr.at(0).integer(),coefficient=pair.arr.at(1).integer();
        require(column>=0&&size_t(column)<width&&coefficient&&coefficient>=-16&&coefficient<=16&&
                out.emplace(column,coefficient).second,"canonical saved sparse row");
    }
    return out;
}
bool ec_ssl_zero(const ECSparse& row,const std::vector<audit::Point>& base,const audit::SSL& ssl) {
    auto sum=ssl.make({}),term=ssl.make({});
    for(auto [column,coefficient]:row) {
        auto p=ssl.make(base.at(size_t(column)));
        auto scalar=audit::bn(audit::Big(std::abs(coefficient)));
        require(EC_POINT_mul(ssl.group.get(),term.get(),nullptr,p.get(),scalar.get(),ssl.context.get())==1,
                "conic named coefficient multiplication");
        if(coefficient<0)require(EC_POINT_invert(ssl.group.get(),term.get(),ssl.context.get())==1,"conic named negation");
        require(EC_POINT_add(ssl.group.get(),sum.get(),sum.get(),term.get(),ssl.context.get())==1,"conic named row sum");
    }
    return EC_POINT_is_at_infinity(ssl.group.get(),sum.get())==1;
}
SCRow ec_embed(const SCRow& row) {
    require(row.size()==481,"expanded geometric row width");
    SCRow result(685);
    for(size_t i=0;i<row.size();++i)result[i<225?i:i+204]=row[i];
    return result;
}
size_t ec_aux_rank(const SCMat& matrix) {
    SCMat auxiliary;
    for(const auto& row:matrix)auxiliary.emplace_back(row.begin()+225,row.end());
    return sc_rank(auxiliary);
}
void ec_emit_sparse(const SCRow& row) {
    std::cout<<'[';bool comma=false;
    for(auto [column,coefficient]:ec_sparse(row)) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<'['<<column<<','<<coefficient<<']';
    }
    std::cout<<']';
}

int main(int argc,char** argv) {
try {
#ifdef EXPANDED_CONIC_ALL
    require(argc==8,"usage: all-expanded-conics QUARTIC BACKMAP BASELINE TANGENTS OLD_SCAN EXPANDED_ANCHORS PREVIOUS_CONICS");
    const auto previous=read_json(argv[7]);
    require(previous.at("controls_pass").scalar=="true"&&previous.at("supported_conics").integer()==0&&
            previous.at("closed_rank").integer()==6&&previous.at("pencil_count").integer()==16,
            "frozen successful empty first-pencil result");
#else
    require(argc==7,"usage: expanded-conics QUARTIC BACKMAP BASELINE TANGENTS OLD_SCAN EXPANDED_ANCHORS");
#endif
    const auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]),
               tangents=read_json(argv[4]),scan=read_json(argv[5]),expanded=read_json(argv[6]);
    ss_private(baseline,tangents);
    require(expanded.at("controls_pass").scalar=="true"&&expanded.at("closed_rank").integer()==6&&
            expanded.at("expanded_curve_pool").integer()==1434&&expanded.at("selected_sections").integer()==128,
            "successful frozen expansion dependency");
    const auto curve=read_bp(model.at("terms"),true);
    std::array<BP,4> map;size_t position=0;
    for(const auto* name:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[position++]=read_bp(back.at(name));
    require(global_identity(curve,map),"actual expanded-conic map identity");
    std::vector<F> leading(5);
    for(const auto& [e,a]:curve.t)if(e[0]+e[1]==4)leading[size_t(e[1])]=a;
    const Poly infinity(std::move(leading)),z{F(0),F(1)};
    require(gcd(infinity,wide_power(z,prime(),infinity)-z).degree()==0,"no omitted rational source infinity");
    std::vector<::Point> base;
    std::vector<audit::Point> gmp_base,original_base;
    std::set<std::string> column_x;
    for(const auto& q:tangents.at("projected_base").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"original429canonical image columns");
        original_base.push_back(ec_gmp(p));
        if(base.size()<225){base.push_back(p);gmp_base.push_back(ec_gmp(p));}
    }
    require(original_base.size()==429,"original full image width");
    for(size_t i=0;i<225;++i)require(base[i]==point(baseline.at("projected_base").arr.at(i)),"old225prefix unchanged");
    require(expanded.at("selected_images").arr.size()==256&&expanded.at("fibres").arr.size()==256,
            "all256new image/fibre records");
    for(const auto& q:expanded.at("selected_images").arr) {
        const auto p=point(q);valid(p);
        require(!p.zero&&BN_cmp(p.y.p.at(0).n.get(),(-p.y.p.at(0)).n.get())<=0&&
                column_x.insert(decimal(p.x.p.at(0))).second,"new canonical auxiliary image column");
        base.push_back(p);gmp_base.push_back(ec_gmp(p));
    }
    require(base.size()==481,"expanded geometric image width");
    std::vector<CPoint> points;
    std::map<std::string,int> coordinates;
    auto append=[&](const Json& q,int index,int sign) {
        const auto x=coefficient(q.at("x")),y=coefficient(q.at("y"));
        const auto actual=fs_image(curve,map,x,y).value;
        require(actual==point(q.at("image"))&&coordinates.emplace(sp_key(x,y),int(points.size())).second,
                "fresh distinct expanded conic-pool image");
        if(index<0)require(index==-1&&sign==0&&actual.zero,"identity pool label");
        else {
            require(size_t(index)<base.size()&&(sign==1||sign==-1),"signed pool label bounds");
            auto expected=base.at(size_t(index));if(sign<0)expected.y=-expected.y;
            require(actual==expected,"pool exact canonical image/sign");
        }
        const auto fx=fs_eval(curve.partial(0),x,y),fy=fs_eval(curve.partial(1),x,y);
        require(fx!=F(0)||fy!=F(0),"smooth expanded pool point");
        points.push_back({{x,y,actual,index,sign},{F(1),x,y,x*x,x*y,y*y},
                          {F(0),fy,-fx,F(2)*x*fy,y*fy-x*fx,-F(2)*y*fx}});
    };
    for(const auto& q:scan.at("points").arr) {
        require(q.at("index").integer()<225,"old scan uses only original image domain");
        append(q,q.at("index").integer(),q.at("sign").integer());
    }
    require(points.size()==652,"unchanged old pool prefix");
    for(size_t i=0;i<256;++i) {
        const auto& fibre=expanded.at("fibres").arr.at(i);
        require(fibre.at("target").integer()==int(i),"ordered frozen fibre inventory");
        for(const auto& q:fibre.at("points").arr) {
            const auto claimed=point(q.at("image"));
            require(!claimed.zero&&claimed.x==base[225+i].x,"exact new fibre image x");
            append(q,int(225+i),claimed.y==base[225+i].y?1:-1);
        }
    }
    require(points.size()==1434&&points.size()<=3000,"complete bounded actual expanded pool");
    for(size_t i=0;i<226;++i) {
        const auto& q=baseline.at("pool").at("points").arr.at(i);
        require(decimal(points[i].value.x)==q.at("x").scalar&&decimal(points[i].value.y)==q.at("y").scalar&&
                points[i].value.image==point(baseline.at("images").arr.at(i).at("point")),"original226bindings");
    }
    std::vector<Anchors> known;
    std::vector<BP> lines;
    for(const auto& fibre:baseline.at("pool").at("split_fibers").arr) {
        Anchors ids{};
        require(fibre.at("point_indices").arr.size()==4,"original split degree4");
        for(size_t i=0;i<4;++i)ids[i]=fibre.at("point_indices").arr[i].integer();
        const auto& a=points.at(size_t(ids[0])).value;
        const auto& b=points.at(size_t(ids[1])).value;
        BP line;line.term({0,0},b.y*a.x-b.x*a.y);line.term({1,0},a.y-b.y);line.term({0,1},b.x-a.x);
        for(int id:ids)require(fs_eval(line,points.at(size_t(id)).value.x,points.at(size_t(id)).value.y)==F(0),
                               "original split line incidence");
        known.push_back(ids);lines.push_back(std::move(line));
    }
    require(known.size()==7,"original seven complete sections");
    SCRow reference(481);::Point kappa;
    for(int id:known[0]) {
        const auto& p=points.at(size_t(id)).value;kappa=add(kappa,p.image);
        if(p.index>=0)reference[size_t(p.index)]+=p.sign;
    }
    audit::SSL ssl;
    auto check_row=[&](const SCRow& row) {
        require(norm_test::row_sum(row,gmp_base).zero&&ec_ssl_zero(ec_sparse(row),gmp_base,ssl),
                "independent GMP/named conic or seed row sum");
    };
    auto matrix=sc_read_matrix(tangents.at("matrix"));
    for(auto& row:matrix) {
        require(norm_test::row_sum(row,original_base).zero&&ec_ssl_zero(ec_sparse(row),original_base,ssl),
                "original108row sums in both implementations");
        row.resize(685);
    }
    std::set<ECSparse> seed_rows,allowed_rows;
    allowed_rows.insert(ECSparse{});
    for(const auto& row:baseline.at("rows").arr) {
        const auto sparse=ec_read_sparse(row,225);
        allowed_rows.insert(sparse);allowed_rows.insert(norm_test::opposite(sparse));
    }
    for(const auto& saved:expanded.at("seed_sections").arr) {
        const int i=saved.at("i").integer(),j=saved.at("j").integer();
        require(i>=0&&j>=0&&i<652&&j<652,"saved seed origins in old pool");
        const bool tangent=saved.at("tangent").scalar=="true";
        require(tangent?i==j:i<j,"saved seed secant/tangent order");
        const auto& a=points[size_t(i)].value;const auto& b=points[size_t(j)].value;
        const auto dx=coefficient(saved.at("direction").arr[0]),dy=coefficient(saved.at("direction").arr[1]);
        require(tangent?(dx==fs_eval(curve.partial(1),a.x,a.y)&&dy==-fs_eval(curve.partial(0),a.x,a.y)):
                        (dx==b.x-a.x&&dy==b.y-a.y),"saved seed direction binding");
        const auto restriction=ss_restrict(curve,a.x,a.y,dx,dy);
        require(restriction==read_poly(saved.at("restriction")),"actual saved seed restriction");
        const auto roots=ss_quadratic(restriction.exact(tangent?Poly{F(0),F(0),F(1)}:Poly{F(0),F(-1),F(1)}));
        require(roots.size()==2,"saved split residual factor product");
        std::array<std::string,2> computed{sp_key(a.x+dx*roots[0],a.y+dy*roots[0]),sp_key(a.x+dx*roots[1],a.y+dy*roots[1])},recorded;
        std::array<int,4> ids{i,j,0,0};
        for(size_t k=0;k<2;++k) {
            const auto& q=saved.at("residual_points").arr.at(k);
            recorded[k]=q.at("x").scalar+":"+q.at("y").scalar;
            ids[2+k]=coordinates.at(recorded[k]);
            require(points[size_t(ids[2+k])].value.image==point(q.at("image")),"saved residual exact image");
        }
        std::sort(computed.begin(),computed.end());std::sort(recorded.begin(),recorded.end());
        require(computed==recorded,"saved residual geometry and factors agree");
        SCRow row(481);
        for(int id:ids){const auto& p=points[size_t(id)].value;if(p.index>=0)row[size_t(p.index)]+=p.sign;}
        for(size_t k=0;k<row.size();++k)row[k]-=reference[k];
        const auto sparse=ec_sparse(row);
        require(sparse==ec_read_sparse(saved.at("row"),481)&&seed_rows.insert(sparse).second,"saved seed row exact binding");
        check_row(row);matrix.push_back(ec_embed(row));allowed_rows.insert(sparse);
    }
    require(seed_rows.size()==128&&matrix.size()==236,"all128actual seed rows");
    std::set<ECSparse> norm_rows;
    for(const auto& row:expanded.at("norm_rows").arr) {
        const auto sparse=ec_read_sparse(row.at("row"),481);
        require(allowed_rows.count(sparse)&&norm_rows.insert(sparse).second,"saved norm row belongs to seed/baseline span");
    }
    require(norm_rows.size()==135,"all saved necessary norm rows bound");
    for(const auto& row:seed_rows)require(norm_rows.count(row),"all seeds in saved necessary norm inventory");
    require(sc_rank(matrix)==236&&ec_aux_rank(matrix)==230,"starting full236aux230closed6");
    auto changed_seed=norm_test::dense(*seed_rows.begin(),481);++changed_seed[0];
    require(!norm_test::row_sum(changed_seed,gmp_base).zero&&!ec_ssl_zero(ec_sparse(changed_seed),gmp_base,ssl),
            "changed seed coefficient rejects in both implementations");
    std::map<std::array<int,2>,std::vector<int>> groups;
    for(size_t i=652;i<points.size();++i) {
        const auto& p=points[i].value;
        require(p.index>=225,"new pool points stay auxiliary");
        groups[{p.index,p.sign}].push_back(int(i));
    }
    std::vector<ECPencil> pencils;
    size_t four_point_groups=0,qualifying_groups=0,degenerate_groups=0,reused_pencils=0,
           degenerate_quadruples=0,total_general_pencils=0,previous_candidates=0;
#ifdef EXPANDED_CONIC_ALL
    previous_candidates=size_t(previous.at("candidate_occurrences").integer());
    std::set<Anchors> completed;
    for(const auto& saved:previous.at("pencils").arr) {
        Anchors a{};for(size_t i=0;i<4;++i)a[i]=saved.at("anchors").arr.at(i).integer();
        require(completed.insert(a).second,"distinct frozen completed pencil");
    }
#endif
    for(const auto& [image,ids]:groups) {
        require(ids.size()<=6,"degree-six signed image fibre bound");
        if(ids.size()<4)continue;
        ++four_point_groups;
        std::optional<Anchors> first;
#ifdef EXPANDED_CONIC_ALL
        for(size_t i=0;i<ids.size();++i)
            for(size_t j=i+1;j<ids.size();++j)
                for(size_t k=j+1;k<ids.size();++k)
                    for(size_t l=k+1;l<ids.size();++l) {
                        const Anchors a{ids[i],ids[j],ids[k],ids[l]};
                        if(!cgeneral(a,points)){++degenerate_quadruples;continue;}
                        if(!first)first=a;
                        ++total_general_pencils;
                        require(total_general_pencils<=128,"total same-image pencil admission cap;incomplete if exceeded");
                        if(completed.count(a))++reused_pencils;
                        else pencils.push_back({image,a});
                    }
#else
        for(size_t i=0;i<ids.size()&&!first;++i)
            for(size_t j=i+1;j<ids.size()&&!first;++j)
                for(size_t k=j+1;k<ids.size()&&!first;++k)
                    for(size_t l=k+1;l<ids.size()&&!first;++l) {
                        const Anchors a{ids[i],ids[j],ids[k],ids[l]};
                        if(cgeneral(a,points))first=a;
                    }
#endif
        if(!first){++degenerate_groups;continue;}
        ++qualifying_groups;
#ifdef EXPANDED_CONIC_ALL
        if(qualifying_groups<=16) {
            const auto& saved=previous.at("pencils").arr.at(qualifying_groups-1);
            require(image[0]==saved.at("image_index").integer()&&image[1]==saved.at("sign").integer(),
                    "frozen pencil group prefix binding");
            for(size_t i=0;i<4;++i)require((*first)[i]==saved.at("anchors").arr.at(i).integer(),
                                         "frozen first-general quadruple binding");
        }
#else
        if(pencils.size()<16)pencils.push_back({image,*first});
#endif
    }
#ifdef EXPANDED_CONIC_ALL
    require(reused_pencils==16&&pencils.size()+reused_pencils==total_general_pencils&&!pencils.empty(),
            "complete old/new same-image quadruple accounting");
    const size_t expected_new_pencils=total_general_pencils-reused_pencils;
#else
    require(!pencils.empty()&&pencils.size()==std::min(size_t(16),qualifying_groups),"complete declared new-pencil selection");
    total_general_pencils=pencils.size();
    const size_t expected_new_pencils=std::min(size_t(16),qualifying_groups);
#endif
    auto omitted_pencils=pencils;omitted_pencils.pop_back();
    fs_reject([&]{require(omitted_pencils.size()==expected_new_pencils,"omitted pencil accounting");});
    std::map<int,Jet> jets;precision=9;
    auto multiplicity=[&](const CV& q,int index) {
        const auto& p=points.at(size_t(index));
        require(cdot(q,p.monomial)==F(0),"expanded conic local incidence");
        if(cdot(q,p.derivative)!=F(0))return 1;
        auto at=jets.find(index);
        if(at==jets.end())at=jets.emplace(index,lift(homogenize(curve),
            {R{K(p.value.x)},R{K(p.value.y)},R{K(1)}},2)).first;
        const int order_value=order(evaluate(homogenize(cbp(q),2),at->second.xyz));
        require(order_value>=2&&order_value<=8,"exact expanded conic tangent order");
        return order_value;
    };
    auto section=[&](const CV& q,const std::vector<int>& ids)->std::optional<CSection> {
        int degree=0;std::map<int,int> support;SCRow row(481);::Point sum;
        for(int index:ids) {
            const int m=multiplicity(q,index);
            require(support.emplace(index,m).second,"distinct complete conic support");
            degree+=m;
            const auto& p=points.at(size_t(index)).value;
            for(int n=0;n<m;++n)sum=add(sum,p.image);
            if(p.index>=0)row[size_t(p.index)]+=m*p.sign;
        }
        require(degree<=8,"proper conic/quartic intersection degree");
        if(degree!=8)return {};
        auto negative=kappa;if(!negative.zero)negative.y=-negative.y;
        require(add(add(sum,negative),negative).zero,"actual degree-eight image row");
        for(size_t i=0;i<row.size();++i)row[i]-=2*reference[i];
        check_row(row);
        return CSection{q,std::move(support),std::move(row)};
    };
    ECDigest digest;
    size_t candidate_occurrences=0;
    auto collect=[&](const Anchors& anchors,bool actual,size_t pindex) {
        require(cgeneral(anchors,points),"selected conic anchors in general position");
        const auto basis=cbasis(anchors,points);
        std::map<std::string,std::pair<CV,std::vector<int>>> candidates;
        for(size_t i=0;i<points.size();++i) {
            if(std::find(anchors.begin(),anchors.end(),int(i))!=anchors.end())continue;
            const auto q=cthrough(basis,points[i].monomial);
            auto [at,inserted]=candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));
            (void)inserted;at->second.second.push_back(int(i));
        }
        for(int index:anchors) {
            const auto q=cthrough(basis,points[size_t(index)].derivative);
            candidates.emplace(ckey(q),std::make_pair(q,std::vector<int>{}));
        }
        if(actual)candidate_occurrences+=candidates.size();
        std::map<std::string,CSection> supported;
        for(const auto& [key,value]:candidates) {
            std::vector<int> ids(anchors.begin(),anchors.end());
            ids.insert(ids.end(),value.second.begin(),value.second.end());
            require(ids.size()<=8,"complete distinct conic intersection bound");
            if(actual)digest.add(pindex,key,ids);
            bool tangent=false;
            for(int index:ids) {
                require(cdot(value.first,points[size_t(index)].monomial)==F(0),"complete parameter bucket incidence");
                if(cdot(value.first,points[size_t(index)].derivative)==F(0))tangent=true;
            }
            if(ids.size()<8&&!tangent)continue;
            const auto accepted=section(value.first,ids);
            if(accepted)supported.emplace(key,*accepted);
        }
        return supported;
    };
    auto coefficients=[&](const BP& q) {
        CV out{};const std::array<Exponent,6> powers{{{0,0},{1,0},{0,1},{2,0},{1,1},{0,2}}};
        for(size_t i=0;i<6;++i) {
            const auto at=q.t.find(powers[i]);out[i]=at==q.t.end()?F(0):at->second;
        }
        return cnormalize(out);
    };
    const Anchors control_anchors{known[0][0],known[0][1],known[1][0],known[1][1]};
    const auto control_key=ckey(coefficients(lines[0]*lines[1]));
    const auto control_pencil=collect(control_anchors,false,0);
    require(control_pencil.count(control_key)==1,"nonempty same-collector known line-product fixture");
    require(sc_rank(SCMat{control_pencil.at(control_key).row})==1,"control pencil recovers nonzero relation");
    auto dropped_control=control_pencil;dropped_control.erase(control_key);
    fs_reject([&]{require(dropped_control.count(control_key)==1,"missing known control conic");});
    SCMat control_rows;
    for(size_t i=0;i<7;++i)for(size_t j=i+1;j<7;++j) {
        const auto q=coefficients(lines[i]*lines[j]);
        std::vector<int> incident;
        for(size_t k=0;k<points.size();++k)if(cdot(q,points[k].monomial)==F(0))incident.push_back(int(k));
        const auto accepted=section(q,incident);
        require(bool(accepted),"all21direct known line-product controls");
        control_rows.push_back(accepted->row);
    }
    require(control_rows.size()==21&&sc_rank(control_rows)==6,"direct control rank6");
    const auto repeated=coefficients(lines[0]*lines[0]);
    std::vector<int> reference_ids(known[0].begin(),known[0].end());
    const auto square=section(repeated,reference_ids);
    require(bool(square)&&square->support.size()==4,"repeated reference conic control");
    for(auto [index,m]:square->support){(void)index;require(m==2,"repeated reference multiplicity2");}
    require(ec_sparse(square->row).empty(),"repeated reference zero row");
    auto changed=repeated;changed[0]=changed[0]+F(1);
    fs_reject([&]{(void)section(changed,reference_ids);});
    reference_ids.pop_back();require(!section(repeated,reference_ids),"missing support remains incomplete");

    std::map<std::string,CSection> sections;
    for(size_t i=0;i<pencils.size();++i) {
        const auto found=collect(pencils[i].anchors,true,i);
        for(const auto& [key,value]:found)sections.emplace(key,value);
        require(sections.size()<=4096,"supported conic cap;incomplete if exceeded");
        std::cerr<<"expanded pencil "<<i<<" accepted "<<sections.size()<<'\n';
    }
    require(matrix.size()+sections.size()<=256&&matrix[0].size()<=1024,"original dense caps;incomplete if exceeded");
    for(const auto& [key,value]:sections){(void)key;matrix.push_back(ec_embed(value.row));}
    const size_t full=sc_rank(matrix),auxiliary=ec_aux_rank(matrix);
    require(full>=auxiliary+6,"original closed baseline retained");
    std::cout<<"{\"scope\":\""
#ifdef EXPANDED_CONIC_ALL
             <<"all-new-signed-fibre-four-subset-conic-pencils-on-expanded-actual-pool"
#else
             <<"bounded-new-signed-fibre-conic-pencils-on-expanded-actual-pool"
#endif
             <<"\","
             <<"\"pool_size\":1434,\"old_columns\":225,\"original_auxiliary_columns\":204,\"new_auxiliary_columns\":256,"
             <<"\"signed_groups_with_at_least_four_points\":"<<four_point_groups
             <<",\"qualifying_groups\":"<<qualifying_groups<<",\"degenerate_groups_excluded\":"<<degenerate_groups
             <<",\"pencil_count\":"<<pencils.size()<<",\"reused_pencils\":"<<reused_pencils
             <<",\"combined_pencil_count\":"<<total_general_pencils<<",\"degenerate_quadruples_excluded\":"<<degenerate_quadruples
             <<",\"candidate_occurrences\":"<<candidate_occurrences<<",\"previous_candidate_occurrences\":"<<previous_candidates
             <<",\"combined_candidate_occurrences\":"<<candidate_occurrences+previous_candidates
             <<",\"candidate_bucket_stream_sha256\":\""<<digest.finish()<<"\",\"supported_conics\":"<<sections.size()
             <<",\"starting_rows\":236,\"starting_full_rank\":236,\"starting_auxiliary_rank\":230,"
             <<"\"matrix_rows\":"<<matrix.size()<<",\"matrix_columns\":"<<matrix[0].size()
             <<",\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<auxiliary<<",\"closed_rank\":"<<full-auxiliary
             <<",\"candidate_gain\":"<<full-auxiliary-6<<",\"control_pencil_supported_conics\":"<<control_pencil.size()
             <<",\"direct_line_product_controls\":21,\"control_rank\":6,\"controls_pass\":true,"
             <<"\"positive_gain_established\":false,\"pencils\":[";
    for(size_t i=0;i<pencils.size();++i) {
        if(i)std::cout<<',';
        const auto& pencil=pencils[i];
        std::cout<<"{\"image_index\":"<<pencil.image[0]<<",\"sign\":"<<pencil.image[1]<<",\"anchors\":[";
        for(size_t j=0;j<4;++j)std::cout<<(j?",":"")<<pencil.anchors[j];
        std::cout<<"]}";
    }
    std::cout<<"],\"sections\":[";bool comma=false;
    for(const auto& [key,value]:sections) {
        (void)key;
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"coefficients\":[";
        for(size_t j=0;j<6;++j)std::cout<<(j?",":"")<<'"'<<decimal(value.coefficients[j])<<'"';
        std::cout<<"],\"support\":[";bool sep=false;
        for(auto [index,m]:value.support) {
            if(sep)std::cout<<',';
            sep=true;
            std::cout<<'['<<index<<','<<m<<']';
        }
        std::cout<<"],\"row\":";ec_emit_sparse(value.row);std::cout<<'}';
    }
    std::cout<<"],\"standing\":\"producer-unverified;only selected new pencils;positive needs actual KM and separate elliptic/elimination replay\"}\n";
    return 0;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}
}
