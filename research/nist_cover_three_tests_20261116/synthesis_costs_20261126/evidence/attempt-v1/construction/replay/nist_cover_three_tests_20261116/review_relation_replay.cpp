// Portable artifact binding and typed JSON comparison. No curve arithmetic.
#include <openssl/evp.h>
#include <algorithm>
#include <cctype>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

void check(bool condition,const std::string& message){if(!condition)throw std::runtime_error(message);}
struct Value {
    enum Kind {Null,Boolean,Number,String,Array,Object} kind=Null;
    std::string scalar;
    std::vector<Value> array;
    std::map<std::string,Value> object;
};
struct Reader {
    std::string text;size_t pos=0;
    void space(){while(pos<text.size()&&std::isspace(static_cast<unsigned char>(text[pos])))++pos;}
    char take(){check(pos<text.size(),"truncated JSON");return text[pos++];}
    bool digit()const{return pos<text.size()&&text[pos]>='0'&&text[pos]<='9';}
    unsigned hex4(){unsigned n=0;for(int i=0;i<4;++i){char c=take();n*=16;if(c>='0'&&c<='9')n+=unsigned(c-'0');else if(c>='a'&&c<='f')n+=unsigned(c-'a'+10);else if(c>='A'&&c<='F')n+=unsigned(c-'A'+10);else throw std::runtime_error("invalid JSON unicode escape");}return n;}
    std::string string(){
        check(take()=='"',"expected JSON string");std::string out;
        for(;;){unsigned char c=static_cast<unsigned char>(take());if(c=='"')return out;check(c>=32,"JSON control character");if(c!='\\'){out+=char(c);continue;}
            char e=take();if(e=='"'||e=='\\'||e=='/')out+=e;
            else if(e=='b')out+='\b';else if(e=='f')out+='\f';else if(e=='n')out+='\n';else if(e=='r')out+='\r';else if(e=='t')out+='\t';
            else if(e=='u'){unsigned u=hex4();if(u>=0xd800&&u<=0xdbff){check(take()=='\\'&&take()=='u',"missing low surrogate");unsigned v=hex4();check(v>=0xdc00&&v<=0xdfff,"invalid low surrogate");u=0x10000+(u-0xd800)*1024+v-0xdc00;}else check(u<0xdc00||u>0xdfff,"unpaired low surrogate");
                if(u<128)out+=char(u);else if(u<2048){out+=char(0xc0|(u>>6));out+=char(0x80|(u&63));}else if(u<65536){out+=char(0xe0|(u>>12));out+=char(0x80|((u>>6)&63));out+=char(0x80|(u&63));}else{out+=char(0xf0|(u>>18));out+=char(0x80|((u>>12)&63));out+=char(0x80|((u>>6)&63));out+=char(0x80|(u&63));}}
            else throw std::runtime_error("invalid JSON string escape");}
    }
    Value value(int depth=0){
        check(depth<128,"JSON nesting cap");space();check(pos<text.size(),"empty JSON");Value n;char c=text[pos];
        if(c=='{'){++pos;n.kind=Value::Object;space();if(pos<text.size()&&text[pos]=='}'){++pos;return n;}for(;;){space();auto key=string();space();check(take()==':',"JSON object colon");check(n.object.emplace(key,value(depth+1)).second,"duplicate JSON key");space();char end=take();if(end=='}')return n;check(end==',',"JSON object separator");}}
        if(c=='['){++pos;n.kind=Value::Array;space();if(pos<text.size()&&text[pos]==']'){++pos;return n;}for(;;){n.array.push_back(value(depth+1));space();char end=take();if(end==']')return n;check(end==',',"JSON array separator");}}
        if(c=='"'){n.kind=Value::String;n.scalar=string();return n;}
        for(auto literal:std::vector<std::string>{"null","true","false"}){if(text.compare(pos,literal.size(),literal)==0){pos+=literal.size();n.kind=literal=="null"?Value::Null:Value::Boolean;n.scalar=literal;return n;}}
        n.kind=Value::Number;size_t start=pos;if(c=='-')++pos;check(digit(),"invalid JSON number");if(text[pos]=='0')++pos;else while(digit())++pos;
        if(pos<text.size()&&text[pos]=='.'){++pos;check(digit(),"JSON fraction");while(digit())++pos;}
        if(pos<text.size()&&(text[pos]=='e'||text[pos]=='E')){++pos;if(pos<text.size()&&(text[pos]=='+'||text[pos]=='-'))++pos;check(digit(),"JSON exponent");while(digit())++pos;}
        n.scalar=text.substr(start,pos-start);return n;
    }
    Value parse(){check(text.size()<=1048576,"JSON byte cap");auto n=value();space();check(pos==text.size(),"JSON trailing data");return n;}
};
Value read_value(const std::filesystem::path& path){check(std::filesystem::file_size(path)<=1048576,"JSON file byte cap");std::ifstream in(path);check(bool(in),"cannot open JSON "+path.string());std::stringstream data;data<<in.rdbuf();return Reader{data.str()}.parse();}
struct Comparison {size_t ignored_timings=0;bool openssl_equal=true;std::vector<std::string> versions;};
void equal(const Value& a,const Value& b,Comparison& receipt,const std::string& path="$"){
    check(a.kind==b.kind,"JSON type mismatch at "+path);
    if(a.kind==Value::Array){check(a.array.size()==b.array.size(),"array length mismatch at "+path);for(size_t i=0;i<a.array.size();++i)equal(a.array[i],b.array[i],receipt,path+"["+std::to_string(i)+"]");return;}
    if(a.kind==Value::Object){check(a.object.size()==b.object.size(),"object key count at "+path);
        for(auto& [key,av]:a.object){auto found=b.object.find(key);check(found!=b.object.end(),"missing key at "+path+"."+key);auto& bv=found->second;
            bool stage=path.rfind("$.stages[",0)==0&&path.back()==']';
            if((path=="$"||stage)&&(key=="wall_us"||key=="cpu_us")){check(av.kind==Value::Number&&bv.kind==Value::Number&&!av.scalar.empty()&&!bv.scalar.empty()&&av.scalar[0]!='-'&&bv.scalar[0]!='-',"invalid excluded timing");++receipt.ignored_timings;continue;}
            if(path=="$"&&key=="openssl"){check(av.kind==Value::String&&bv.kind==Value::String,"library version type");receipt.openssl_equal=receipt.openssl_equal&&(av.scalar==bv.scalar);receipt.versions.push_back(av.scalar);receipt.versions.push_back(bv.scalar);continue;}
            equal(av,bv,receipt,path+"."+key);}return;}
    check(a.scalar==b.scalar,"JSON scalar mismatch at "+path);
}
namespace fs=std::filesystem;
fs::path subject(const fs::path& root,const std::string& name){
    fs::path p(name);check(!p.empty()&&!p.is_absolute(),"manifest requires relative path");for(auto& component:p)check(component!="..","manifest path traversal");
    auto file=fs::canonical(root/p),base=fs::canonical(root);auto relative=fs::relative(file,base);for(auto& component:relative)check(component!="..","manifest symlink escape");return file;
}
std::string digest(const fs::path& file){
    std::ifstream in(file,std::ios::binary);check(bool(in),"manifest file missing");std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> ctx(EVP_MD_CTX_new(),EVP_MD_CTX_free);check(bool(ctx)&&EVP_DigestInit_ex(ctx.get(),EVP_sha256(),nullptr)==1,"SHA256 initialization");
    char data[65536];while(in){in.read(data,sizeof(data));auto count=in.gcount();if(count)check(EVP_DigestUpdate(ctx.get(),data,size_t(count))==1,"SHA256 update");}check(in.eof(),"SHA256 read error");unsigned char output[EVP_MAX_MD_SIZE];unsigned size=0;check(EVP_DigestFinal_ex(ctx.get(),output,&size)==1&&size==32,"SHA256 final");
    std::ostringstream hex;for(unsigned i=0;i<size;++i)hex<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(output[i]);return hex.str();
}
size_t manifest(const fs::path& root,std::istream& input){size_t count=0;std::string line;while(std::getline(input,line)){check(line.size()>66&&line[64]==' '&&(line[65]==' '||line[65]=='*'),"manifest line syntax");auto expected=line.substr(0,64);for(char c:expected)check((c>='0'&&c<='9')||(c>='a'&&c<='f'),"manifest digest syntax");check(digest(subject(root,line.substr(66)))==expected,"manifest SHA256 mismatch");++count;}check(count>0,"empty portable manifest");return count;}
template<class F> void reject(F fn){bool caught=false;try{fn();}catch(const std::exception&){caught=true;}check(caught,"corruption was accepted");}
void emit_string(const std::string& s){std::cout<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')std::cout<<'\\'<<char(c);else if(c<32)std::cout<<"\\u"<<std::hex<<std::setw(4)<<std::setfill('0')<<unsigned(c)<<std::dec;else std::cout<<char(c);}std::cout<<'"';}
int main(int argc,char** argv){try{
    check(argc==5,"usage: review_relation_replay REPO_ROOT EXPECTED_DIR OBSERVED_DIR MANIFEST_RELATIVE");fs::path root=argv[1],expected=argv[2],observed=argv[3];auto list=subject(root,argv[4]);std::ifstream input(list);size_t bound=manifest(root,input);Comparison receipt;std::vector<Value> old;
    std::vector<std::string> paths{"relations/result.json","relations/checker-result.json","transfer/result.json","transfer/checker-result.json"};
    for(auto& path:paths){auto a=read_value(expected/path),b=read_value(observed/path);equal(a,b,receipt);old.push_back(a);}
    auto altered=old[0];auto& coordinate=altered.object.at("images").array.at(1).object.at("point").object.at("y");check(coordinate.kind==Value::String,"finite image control fixture");coordinate.scalar=coordinate.scalar=="1"?"2":"1";reject([&]{Comparison r;equal(old[0],altered,r);});
    altered=old[0];altered.object.at("rank_mod_r").scalar="999";reject([&]{Comparison r;equal(old[0],altered,r);});
    altered=old[0];altered.object.at("stages").array.at(0).object.erase("name");reject([&]{Comparison r;equal(old[0],altered,r);});
    reject([&]{Comparison r;equal(Reader{"{\"v\":[]}"}.parse(),Reader{"{\"v\":{}}"}.parse(),r);});
    std::ifstream first(list);std::string line;check(bool(std::getline(first,line)),"manifest first subject");auto bad=line;bad[0]=bad[0]=='0'?'1':'0';reject([&]{std::istringstream stream(bad);manifest(root,stream);});
    reject([&]{std::istringstream stream(line.substr(0,66)+"../outside");manifest(root,stream);});
    altered=old[0];altered.object.at("stages").array.at(0).object.at("wall_us").scalar="999999999";Comparison timing;equal(old[0],altered,timing);check(timing.ignored_timings>0,"timing-only change counted");
    std::cout<<"{\"schema\":\"nist.portable-replay-comparison/v1\",\"standing\":\"producer-unverified comparison metadata\",\"manifest_subjects\":"<<bound<<",\"result_files_compared\":4,\"timing_fields_excluded\":"<<receipt.ignored_timings<<",\"openssl_version_equal\":"<<(receipt.openssl_equal?"true":"false")<<",\"openssl_versions\":[";
    for(size_t i=0;i<receipt.versions.size();++i){if(i)std::cout<<',';emit_string(receipt.versions[i]);}
    std::cout<<"],\"corruptions_rejected\":6,\"timing_only_control_accepted\":true,\"signed_independent_verdict\":false}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
