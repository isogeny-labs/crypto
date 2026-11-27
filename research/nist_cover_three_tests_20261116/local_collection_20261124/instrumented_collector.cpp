#include "target_traces_arithmetic.hpp"
#include "rank_roots.hpp"
#include <chrono>

void sc_point(std::ostream& out, const EC& point) {
    out << "{\"infinity\":" << (point.zero ? "true" : "false");
    if (!point.zero) out << ",\"x\":\"" << scalar(point.x).x << "\",\"y\":\"" << scalar(point.y).x << '"';
    out << '}';
}
void sc_self_test(const std::string& fixtures, const std::string& destination) {
    const auto data = read_json(fixtures);
    std::ofstream out(destination);
    require(bool(out), "new synthetic output");
    const std::vector<std::array<SCBig,3>> polynomials{
        {6,-5,1}, {42,-35,7}, {25,-10,1}, {1,0,1},
        {sc_reduce(SCBig((sc_p-2)*(sc_p-3)),sc_p),5,1}};
    const std::vector<std::string> categories{"distinct","distinct","repeated","irreducible","distinct"};
    out << "{\"standing\":\"producer-unverified\",\"scope\":\"synthetic-scalar-polynomials-and-coefficient-matrices\",\"quadratics\":[";
    for (size_t i=0; i<polynomials.size(); ++i) {
        auto result=sc_roots(polynomials[i]);
        require(result.category==categories[i], "declared quadratic category");
        if(i) out << ',';
        out << "{\"coefficients\":";
        sc_emit_row(out, SCRow(polynomials[i].begin(),polynomials[i].end()));
        out << ",\"category\":\"" << result.category << "\",\"roots\":";
        sc_emit_row(out,result.roots);
        out << '}';
    }
    out << "],\"cases\":[";
    const std::vector<size_t> gains{0,1,1,0,1,1};
    const auto& cases=data.at("cases").arr;
    require(cases.size()==gains.size(),"six declared matrix fixtures");
    for(size_t i=0;i<cases.size();++i) {
        const auto matrix=sc_read_matrix(cases[i].at("matrix"));
        const auto proof=sc_close(matrix,3,1);
        require(proof.closed_rank-proof.baseline_rank==gains[i],"declared fixture gain");
        if(i) out << ',';
        out << "{\"name\":\"" << cases[i].at("name").scalar << "\",\"matrix\":";
        sc_emit_matrix(out,matrix);
        out << ",\"old_width\":3,\"proof\":";
        sc_emit_closure(out,proof);
        out << '}';
    }
    out << "],\"new_curve_points\":0,\"actual_collection_executed\":false}\n";
    require(bool(out),"complete synthetic fixture output");
}
struct SCPoint { Z x,y; };
struct SCSection { int origin; std::array<Z,3> line; CFPoly restriction; std::array<int,2> residual; };
void sc_collect(const std::string& root, const std::string& baseline_path,
                const std::string& tangent_path, const std::string& destination) {
    const auto started=std::chrono::steady_clock::now();
    using LocalClock=std::chrono::steady_clock;
    std::map<std::string,long long> phases;
    auto mark=[&](const std::string& name,LocalClock::time_point begin) {
        phases[name]+=std::chrono::duration_cast<std::chrono::microseconds>(LocalClock::now()-begin).count();
    };
    initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));
    Level2Input level;
    const auto model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json");
    const auto baseline=read_json(baseline_path), tangents=read_json(tangent_path);
    BRMap map(model,read_json(root+"/backmap.json"));
    QuarticKM km(model,baseline.at("pool"));
    require(prime==sc_p,"exact collector field");
    std::vector<SCPoint> points;
    std::map<std::string,int> point_index;
    auto intern=[&](const Z& x,const Z& y) {
        require(map.curve.eval(x,y)==Z(0),"every reconstructed point on quartic");
        const std::string key=x.x.get_str()+":"+y.x.get_str();
        auto hit=point_index.find(key);
        if(hit!=point_index.end()) return hit->second;
        int index=int(points.size());
        point_index.emplace(key,index);
        points.push_back({x,y});
        require(points.size()<=430,"point cap");
        return index;
    };
    for(const auto& point:baseline.at("pool").at("points").arr)
        intern(Z(SCBig(point.at("x").scalar)),Z(SCBig(point.at("y").scalar)));
    require(points.size()==226,"original pool and coordinate uniqueness");
    mark("input_map_and_KM_setup_us",started);
    std::vector<SCSection> sections;
    const auto& records=tangents.at("tangents").arr;
    require(records.size()==226,"all frozen selection outcomes");
    size_t target_checks=0;
    for(size_t i=0;i<records.size();++i) {
        const auto selection_started=LocalClock::now();
        const auto& record=records[i];
        require(record.at("point_index").integer()==int(i),"ordered frozen tangent records");
        std::array<Z,3> line;
        for(size_t j=0;j<3;++j) line[j]=Z(SCBig(record.at("line").arr.at(j).scalar));
        const auto point=points.at(i);
        const Z gx=map.fx.eval(point.x,point.y),gy=map.fy.eval(point.x,point.y);
        require(gx!=Z(0)||gy!=Z(0),"smooth tangent point");
        require(line[0]*point.x+line[1]*point.y+line[2]==Z(0)&&line[0]*gy==line[1]*gx,
                "actual gradient tangent");
        CFPoly parameter{Z(0),Z(1)}, x=CFPoly{point.x}+parameter.scale(line[1]),
            y=CFPoly{point.y}-parameter.scale(line[0]);
        CFPoly restriction;
        for(const auto& term:map.curve.terms) {
            CFPoly value{term.second};
            for(int j=0;j<term.first[0];++j) value=value*x;
            for(int j=0;j<term.first[1];++j) value=value*y;
            restriction=restriction+value;
        }
        require(restriction==tt_poly(record.at("restriction")),"fresh full tangent restriction");
        const auto residual=restriction.exact(CFPoly{Z(0),Z(0),Z(1)});
        require(residual==tt_poly(record.at("residual"))&&residual.degree()==2,"exact frozen residual quadratic");
        const auto roots=sc_roots({residual.at(0).x,residual.at(1).x,residual.at(2).x});
        const std::string category=roots.category=="distinct"?"split-distinct-rational":
            roots.category=="repeated"?"repeated-contact":"irreducible-quadratic";
        require(category==record.at("category").scalar,"recheck every selection label");
        mark("tangent_restrictions_and_roots_us",selection_started);
        if(roots.category!="distinct") continue;
        const auto target_started=LocalClock::now();
        require(km.zero(tt_section(km,map.curve,line,point.x,point.y,restriction)),"actual tangent target principal class");
        mark("target_principality_us",target_started);
        ++target_checks;
        const auto support_started=LocalClock::now();
        std::array<int,2> ids;
        for(size_t j=0;j<2;++j) {
            const Z t(roots.roots[j]);
            ids[j]=intern(point.x+line[1]*t,point.y-line[0]*t);
        }
        sections.push_back({int(i),line,restriction,ids});
        mark("support_reconstruction_us",support_started);
    }
    require(sections.size()==102&&target_checks==102,"complete frozen split screen");
    const auto base_started=LocalClock::now();
    std::vector<EC> base,images;
    std::map<std::string,int> base_index;
    for(const auto& point:baseline.at("projected_base").arr) {
        const Z x(SCBig(point.at("x").scalar)),y(SCBig(point.at("y").scalar));
        require(y.x<=prime-y.x,"original canonical base sign");
        require(base_index.emplace(x.x.get_str()+":"+y.x.get_str(),int(base.size())).second,"unique original base");
        base.emplace_back(F(Poly<Z>{x}),F(Poly<Z>{y}));
    }
    require(base.size()==225,"original base width");
    mark("old_base_setup_us",base_started);
    std::vector<std::array<int,2>> projection;
    size_t local=0;
    for(size_t i=0;i<points.size();++i) {
        const auto mapping_started=LocalClock::now();
        const auto image=map.image(points[i].x,points[i].y);
        if(image.local) ++local;
        auto point=image.point;
        images.push_back(point);
        mark("point_mapping_us",mapping_started);
        if(point.zero) {
            const auto merging_started=LocalClock::now();
            if(i<226) {
                const auto& old=baseline.at("images").arr.at(i);
                require(old.at("projected_index").integer()==-1&&old.at("sign").integer()==0,"unchanged original identity binding");
            }
            projection.push_back({-1,0});
            mark("signed_base_merging_us",merging_started);
            continue;
        }
        const auto validation_started=LocalClock::now();
        require(point.y*point.y==point.x*point.x*point.x-F(3)*point.x+
            F(Poly<Z>{Z(SCBig("2455155546008943817740293915197451784769108058161191238065"))}),"elliptic equation");
        require(times(point,sc_n).zero,"elliptic subgroup");
        mark("image_equation_and_subgroup_checks_us",validation_started);
        const auto merging_started=LocalClock::now();
        int sign=1;
        if(scalar(point.y).x>prime-scalar(point.y).x) { point=point.neg(); sign=-1; }
        const std::string key=scalar(point.x).x.get_str()+":"+scalar(point.y).x.get_str();
        auto hit=base_index.find(key);
        int index;
        if(hit==base_index.end()) { index=int(base.size()); base_index.emplace(key,index); base.push_back(point); }
        else index=hit->second;
        projection.push_back({index,sign});
        if(i<226) {
            const auto& old=baseline.at("images").arr.at(i);
            require(old.at("projected_index").integer()==index&&old.at("sign").integer()==sign,"unchanged original binding");
        }
        mark("signed_base_merging_us",merging_started);
    }
    const auto rows_started=LocalClock::now();
    SCMat matrix;
    for(const auto& sparse:baseline.at("rows").arr) {
        SCRow row(base.size());
        for(const auto& entry:sparse.arr) row.at(size_t(entry.arr.at(0).integer()))=SCBig(entry.arr.at(1).scalar);
        matrix.push_back(row);
    }
    require(matrix.size()==6,"six original rows");
    const auto& reference=baseline.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr;
    for(const auto& section:sections) {
        SCRow row(base.size());
        auto add=[&](int i,int coefficient) { const auto p=projection.at(size_t(i)); if(p[0]>=0) row.at(size_t(p[0]))+=coefficient*p[1]; };
        add(section.origin,2);
        for(int i:section.residual) add(i,1);
        for(const auto& i:reference) add(i.integer(),-1);
        matrix.push_back(row);
    }
    mark("row_assembly_us",rows_started);
    const auto sums_started=LocalClock::now();
    for(const auto& row:matrix) {
        EC sum;
        for(size_t i=0;i<row.size();++i) if(row[i]!=0) {
            const SCBig magnitude=row[i]<0?SCBig(-row[i]):row[i];
            sum=sum.add(times(row[i]<0?base[i].neg():base[i],magnitude));
        }
        require(sum.zero,"every transported row sum");
    }
    mark("elliptic_row_sums_us",sums_started);
    const auto closure_started=LocalClock::now();
    const auto closure=sc_close(matrix,225,6);
    mark("closure_elimination_us",closure_started);
    const auto incidence_started=LocalClock::now();
    std::vector<std::array<long,3>> private_columns;
    std::vector<size_t> owned(matrix.size());
    for(size_t j=225;j<base.size();++j) {
        size_t count=0,owner=0;
        for(size_t i=0;i<matrix.size();++i) if(matrix[i][j]!=0) {++count;owner=i;}
        require(count==1&&owner>=6,"every actual new column has one added-row owner");
        require(matrix[owner][j]==1||matrix[owner][j]==-1,"unit private-column coefficient");
        private_columns.push_back({long(owner),long(j),matrix[owner][j].get_si()});++owned[owner];
    }
    require(private_columns.size()==204,"actual private-column population");
    for(size_t i=6;i<matrix.size();++i) require(owned[i]==2,"two private new columns per actual tangent row");
    mark("private_column_incidence_us",incidence_started);
    const auto serialization_started=LocalClock::now();
    std::ofstream out(destination);
    require(bool(out),"new production result");
    out << "{\"standing\":\"producer-unverified\",\"scope\":\"fixed-split-tangent-collection\",\"original_points\":226,\"original_base_width\":225,\"selection_outcomes\":226,\"target_principal_checks\":" << target_checks
        << ",\"local_map_charts\":" << local << ",\"points\":[";
    for(size_t i=0;i<points.size();++i) {
        if(i) out << ',';
        out << "{\"x\":\"" << points[i].x.x << "\",\"y\":\"" << points[i].y.x << "\"}";
    }
    out << "],\"images\":[";
    for(size_t i=0;i<images.size();++i) {
        if(i) out << ',';
        out << "{\"point\":"; sc_point(out,images[i]);
        out << ",\"projected_index\":" << projection[i][0] << ",\"sign\":" << projection[i][1] << '}';
    }
    out << "],\"projected_base\":[";
    for(size_t i=0;i<base.size();++i) { if(i) out << ','; sc_point(out,base[i]); }
    out << "],\"sections\":[";
    for(size_t i=0;i<sections.size();++i) {
        if(i) out << ',';
        out << "{\"point_index\":" << sections[i].origin << ",\"residual_indices\":[" << sections[i].residual[0] << ',' << sections[i].residual[1] << "]}";
    }
    out << "],\"matrix\":"; sc_emit_matrix(out,matrix);
    out << ",\"closure\":"; sc_emit_closure(out,closure);
    out << ",\"private_columns\":[";
    for(size_t i=0;i<private_columns.size();++i) {
        if(i) out<<',';
        out<<'['<<private_columns[i][0]<<','<<private_columns[i][1]<<','<<private_columns[i][2]<<']';
    }
    out<<']';
    mark("serialization_prefix_us",serialization_started);
    const auto elapsed=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-started).count();
    long long assigned=0;for(const auto& phase:phases) assigned+=phase.second;
    require(assigned<=elapsed,"nonoverlapping phase clocks");phases["unassigned_overhead_us"]=elapsed-assigned;
    out<<",\"phase_us\":{";bool comma=false;
    for(const auto& phase:phases) {if(comma)out<<',';comma=true;out<<'"'<<phase.first<<"\":"<<phase.second;}
    out<<'}';
    out << ",\"wall_us_before_final_serialization\":" << elapsed << ",\"actual_collection_executed\":true,\"signed_independent_verdict\":false}\n";
    require(bool(out),"complete production result");
}
int main(int argc,char** argv) {
    try {
        if(argc==4&&std::string(argv[1])=="--self-test") {
            require(!std::ifstream(argv[3]).good(),"output already exists");
            sc_self_test(argv[2],argv[3]);
        }
        else if(argc==6&&std::string(argv[1])=="--collect") {
            require(!std::ifstream(argv[5]).good(),"output already exists");
            sc_collect(argv[2],argv[3],argv[4],argv[5]);
        }
        else throw std::runtime_error("usage: collector --self-test FIXTURES.json OUTPUT.json | --collect INPUT-directory BASELINE.json TANGENTS.json OUTPUT.json");
    } catch(const std::exception& error) { std::cerr << "REJECT " << error.what() << '\n'; return 1; }
}
