#include "replay_openssl.hpp"
#include "portable_arithmetic.hpp"

void bind_known(OpenSSLGroup& ec,const EP& point,const EP& norm,long scalar){
    mpz_class k=scalar;if(k<0)k+=order_r;auto n=number(k);auto expected=ec.empty();
    require(EC_POINT_mul(ec.group.get(),expected.get(),n.get(),nullptr,nullptr,ec.ctx.get())==1,"known scalar multiplication");
    require(EC_POINT_cmp(ec.group.get(),expected.get(),point.get(),ec.ctx.get())==0,"public known-scalar binding");ec.composition(point,norm);
}
void report_file(const fs::path& path,const std::string& data){require(!fs::exists(path),"new report path required");std::ofstream out(path);out<<data;require(bool(out),"write replay report");}
int main(int argc,char** argv){try{
    require(argc==4,"usage: replay_certificate REPO_ROOT RUN_DIRECTORY SOURCE_REVISION");fs::path root=fs::canonical(argv[1]),run=fs::canonical(argv[2]);std::string revision=argv[3];
    require(revision.size()==40&&std::all_of(revision.begin(),revision.end(),[](char c){return (c>='0'&&c<='9')||(c>='a'&&c<='f');}),"immutable source SHA");
    auto relation=read_json((run/"native/relations/result.json").string()),transfer=read_json((run/"native/transfer/result.json").string());
    auto relation_check=read_json((run/"native/relations/checker-result.json").string()),transfer_check=read_json((run/"native/transfer/checker-result.json").string());
    auto comparison=read_json((run/"comparison.json").string()),clock=read_json((run/"pipeline-clock.json").string());
    require(clock.at("exit_code").integer()==0,"successful complete pipeline child");require(comparison.at("result_files_compared").integer()==4&&comparison.at("corruptions_rejected").integer()==6,"typed comparison and controls");
    require(relation_check.at("corruption_controls").integer()==4&&transfer_check.at("changed_norm_control_rejected").scalar=="true","separate arithmetic controls");
    OpenSSLGroup ec;auto& fixtures=transfer.at("fixtures").arr;require(fixtures.size()==6,"fixed six transfer cases");std::array<long,6> scalars{0,1,-1,2,37,-2};
    for(size_t i=0;i<fixtures.size();++i)bind_known(ec,ec.point(read_point(fixtures[i].at("point"))),ec.point(read_point(fixtures[i].at("norm"))),scalars[i]);
    auto& bases=relation.at("projected_base").arr;std::vector<EP> points;for(auto& p:bases)points.push_back(ec.point(read_point(p)));
    std::vector<std::vector<mpz_class>> rows;
    auto check_row=[&](const std::vector<mpz_class>& row){auto sum=ec.empty();for(size_t i=0;i<row.size();++i)ec.add(sum,points[i],row[i]);require(ec.zero(sum),"certificate projected row sum");};
    for(auto& sparse:relation.at("rows").arr){std::vector<mpz_class> row(points.size());std::set<int> seen;for(auto& term:sparse.arr){int i=term.arr.at(0).integer();require(i>=0&&size_t(i)<row.size()&&seen.insert(i).second,"certificate sparse row index");row[size_t(i)]=mpz_class(term.arr.at(1).scalar);}require(std::any_of(row.begin(),row.end(),[](const mpz_class& n){return n!=0;}),"nonzero relation row");check_row(row);rows.push_back(row);}
    int rank=separate_rank(rows),count=int(rows.size()),columns=int(points.size()),pool=int(relation.at("pool").at("points").arr.size()),fibers=int(relation.at("pool").at("split_fibers").arr.size());
    require(rank==relation.at("rank_mod_r").integer()&&rank==relation_check.at("rank_mod_r").integer(),"independent rank binding");
    require(columns-rank==relation.at("rank_deficiency").integer()&&columns==relation.at("projected_base_size").integer(),"rank deficiency and base binding");
    require(count==relation.at("target_KM_zero_checks").integer()&&count==relation_check.at("elliptic_relation_checks").integer(),"target/source relation count binding");
    require(pool==relation_check.at("point_image_checks").integer()&&fibers==relation_check.at("fiber_factorizations").integer(),"pool/fiber independent binding");
    auto public_point=ec.point(read_point(fixtures[1].at("point"))),public_norm=ec.point(read_point(fixtures[1].at("norm")));
    must_reject([&]{auto bad=ec.point(read_point(fixtures[1].at("point")));ec.add(bad,public_point,1);bind_known(ec,bad,public_norm,1);});
    must_reject([&]{auto bad=ec.point(read_point(fixtures[1].at("norm")));ec.add(bad,public_point,1);bind_known(ec,public_point,bad,1);});
    must_reject([&]{auto bad=rows.at(0);auto item=std::find_if(bad.begin(),bad.end(),[](const mpz_class& n){return n!=0;});++*item;check_row(bad);});
    struct Headline {std::string name;int claimed,recomputed;};
    std::vector<Headline> headlines{{"rational_points",relation.at("factor_base_points").integer(),pool},{"split_fibers",relation.at("split_fibers").integer(),fibers},{"transported_rows",relation.at("relations").integer(),count},{"rank_mod_r",relation.at("rank_mod_r").integer(),rank},{"projected_columns",relation.at("projected_base_size").integer(),columns},{"rank_deficiency",relation.at("rank_deficiency").integer(),columns-rank},{"known_scalar_bindings",6,int(fixtures.size())},{"complete_transfer_cases",transfer.at("fresh_complete_transfer_checks").integer(),int(fixtures.size())}};
    for(auto& h:headlines)require(h.claimed==h.recomputed,"headline recomputation "+h.name);
    std::vector<std::string> raw{"native/relations/result.json","native/relations/checker-result.json","native/transfer/result.json","native/transfer/checker-result.json","comparison.json","pipeline-clock.json"};
    std::ostringstream data;data<<'[';for(size_t i=0;i<raw.size();++i){if(i)data<<',';auto file=run/raw[i];auto path=fs::relative(file,root);for(auto& component:path)require(component!="..","raw evidence inside checkout");data<<"{\"path\":\""<<path.generic_string()<<"\",\"sha256\":\""<<digest(file)<<"\"}";}data<<']';
    std::ostringstream cert;cert<<"{\"schema\":\"p192.scoped-replay-certificate/v1\",\"status\":\"pass\",\"standing\":\"unsigned; not independent signed verification\",\"source_revision\":\""<<revision<<"\",\"scalars_verified\":true,\"scalar_scope\":\"public known-answer fixture scalars 0,1,-1,2,37,-2 only\",\"scalar_recovery_executed\":false,\"runs\":[{\"id\":\"fresh-relation-collection\",\"relations\":"<<count<<"},{\"id\":\"fresh-complete-transfer\",\"cases\":6},{\"id\":\"independent-openssl-bindings\",\"known_scalars\":6}],\"raw_data\":"<<data.str()<<",\"adapter_corruptions_rejected\":3,\"execution_proof\":false,\"admitted_performance\":false}\n";
    std::ostringstream validator;validator<<"{\"schema\":\"p192.scoped-validator-report/v1\",\"status\":\"pass\",\"standing\":\"unsigned; not independent signed verification\",\"source_revision\":\""<<revision<<"\",\"raw_data\":"<<data.str()<<",\"headline\":[";
    for(size_t i=0;i<headlines.size();++i){if(i)validator<<',';auto& h=headlines[i];validator<<"{\"name\":\""<<h.name<<"\",\"claimed\":"<<h.claimed<<",\"recomputed\":"<<h.recomputed<<",\"rel_tol\":0}";}
    validator<<"],\"cost_scope\":\"pipeline-clock includes archive/input preparation,original compilers and four workers plus typed comparison; adapter bootstrap and report stage recorded separately; original map synthesis and machine provisioning unpriced\",\"admitted_performance\":false}\n";
    report_file(run/"replay-certificate.json",cert.str());report_file(run/"validator-report.json",validator.str());
    std::cout<<"{\"known_scalar_bindings\":6,\"relation_rows\":"<<count<<",\"rank_mod_r\":"<<rank<<",\"adapter_corruptions_rejected\":3,\"scalars_recovered\":0,\"signed\":false}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
