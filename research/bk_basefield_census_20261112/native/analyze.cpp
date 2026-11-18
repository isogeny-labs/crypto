#include <sqlite3.h>
#include <fstream>
#include <iostream>
#include <iterator>
#include <stdexcept>
#include <string>

std::string read_file(const char*path){std::ifstream in(path,std::ios::binary);if(!in)throw std::runtime_error(std::string("cannot read ")+path);return {std::istreambuf_iterator<char>(in),std::istreambuf_iterator<char>()};}
int output(void*,int n,char**values,char**){if(n!=1||!values[0])return 1;std::cout<<values[0]<<'\n';return std::cout?0:1;}
void execute(sqlite3*db,const std::string&sql,bool emit=false){char*error=nullptr;if(sqlite3_exec(db,sql.c_str(),emit?output:nullptr,nullptr,&error)!=SQLITE_OK){std::string why=error?error:"sqlite error";sqlite3_free(error);throw std::runtime_error(why);}}
int main(int argc,char**argv){sqlite3*db=nullptr;sqlite3_stmt*insert=nullptr;try{
 bool negative=argc==5&&std::string(argv[1])=="--negative-control";int start=negative?2:1;
 if(argc!=(negative?5:4))throw std::runtime_error("usage: analyze [--negative-control] reference.jsonl aa.jsonl queries.sql");
 const std::string a=read_file(argv[start]),b=read_file(argv[start+1]);if(a!=b)throw std::runtime_error("A/A census bytes differ");
 if(sqlite3_open(":memory:",&db)!=SQLITE_OK)throw std::runtime_error("sqlite open failed");
 execute(db,"CREATE TABLE rows(id INTEGER PRIMARY KEY,payload TEXT NOT NULL CHECK(json_valid(payload))); BEGIN;");
 if(sqlite3_prepare_v2(db,"INSERT INTO rows(payload) VALUES(?)",-1,&insert,nullptr)!=SQLITE_OK)throw std::runtime_error("sqlite prepare failed");
 std::ifstream in(argv[start]);std::string line;int rows=0;
 while(std::getline(in,line)){
  if(line.empty())throw std::runtime_error("empty JSONL row");
  if(sqlite3_bind_text(insert,1,line.data(),int(line.size()),SQLITE_TRANSIENT)!=SQLITE_OK||sqlite3_step(insert)!=SQLITE_DONE)throw std::runtime_error("sqlite insert failed");
  if(sqlite3_reset(insert)!=SQLITE_OK||sqlite3_clear_bindings(insert)!=SQLITE_OK)throw std::runtime_error("sqlite reset failed");
  rows++;
 }
 sqlite3_finalize(insert);insert=nullptr;execute(db,"COMMIT;");
 if(negative)execute(db,"UPDATE rows SET payload=json_set(payload,'$.orbit_size',json_extract(payload,'$.orbit_size')+1) WHERE id=(SELECT MIN(id) FROM rows WHERE json_extract(payload,'$.kind')='model');");
 std::cout<<"{\"kind\":\"aa\",\"identical_bytes\":1,\"jsonl_rows\":"<<rows<<",\"sqlite\":\""<<sqlite3_libversion()<<"\"}\n";
 execute(db,read_file(argv[start+2]),true);
 if(sqlite3_close(db)!=SQLITE_OK)throw std::runtime_error("sqlite close failed");
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';if(insert)sqlite3_finalize(insert);if(db)sqlite3_close(db);return 1;}}
