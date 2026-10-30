#define main diagnostic_main_unused
#include "/workspace/crypto/research/ideas_execution_20261018/targets_prime/diagnostic.cpp"
#undef main

static std::string slurp(const std::filesystem::path& path) {
    std::ifstream in(path);
    if(!in) throw std::runtime_error("review input open");
    std::ostringstream out;out<<in.rdbuf();return out.str();
}
static void create(const std::filesystem::path& path,const std::string& text) {
    if(std::filesystem::exists(path))throw std::runtime_error("review output exists");
    std::ofstream out(path);out<<text;
    if(!out)throw std::runtime_error("review output failed");
}
int main(int argc,char**argv) {
    try {
        if(argc!=2)throw std::runtime_error("usage: certificate-controls REVIEW_DIRECTORY");
        std::filesystem::path root=argv[1];
        auto input=std::filesystem::path("/workspace/crypto/research/ideas_execution_20261018/targets_prime/runs/run-20261018-strict4-v1");
        std::string results=slurp(input/"results.jsonl"),certs=slurp(input/"certificates.jsonl");
        auto renamed=root/"wrong-target-id";std::filesystem::create_directory(renamed);
        std::string changed=certs;
        auto pos=changed.find("\"target_id\":32");
        if(pos==std::string::npos)throw std::runtime_error("control anchor");
        changed.replace(pos,std::string("\"target_id\":32").size(),"\"target_id\":0");
        create(renamed/"results.jsonl",results);create(renamed/"certificates.jsonl",changed);
        auto empty=root/"missing-certificates";std::filesystem::create_directory(empty);
        create(empty/"results.jsonl",results);create(empty/"certificates.jsonl","");
        // Same public curve as curve_id=29 in the strict frozen grid.
        auto fixture29=fixture(509,false,2);auto& curve=fixture29.c;
        U B=4;std::vector<P> base;
        for(U x=0;x<B;++x){auto it=curve.roots.find(curve.rhs(x));if(it!=curve.roots.end())for(U y:it->second)base.push_back({x,y,false});}
        if(base.empty())throw std::runtime_error("control base unavailable");
        P point=base.front(),target=curve.mul(4,point);
        if(target.inf)throw std::runtime_error("control target infinity");
        auto repeated=root/"false-strict-claim";std::filesystem::create_directory(repeated);
        std::string record="{\"item\":14,\"curve_id\":29,\"p\":509,\"a\":"+std::to_string(curve.a)+",\"b\":"+std::to_string(curve.b)+",\"B\":4,\"target_id\":0,\"target\":"+pj(target)+",\"found\":true}\n";
        std::string certificate="{\"item\":14,\"curve_id\":29,\"target_id\":0,\"target\":"+pj(target)+",\"points\":["+pj(point)+","+pj(point)+","+pj(point)+","+pj(point)+"],\"distinct_abscissae\":true}\n";
        create(repeated/"results.jsonl",record);create(repeated/"certificates.jsonl",certificate);
        std::cout<<"Created three adversarial verifier controls in fresh review paths; original records unchanged.\n";
    }catch(const std::exception& error){std::cerr<<error.what()<<"\n";return 1;}
}
