fn main(){
    println!("{:#?}", depy::package::Package::query_local_buckets("neovim").unwrap());
}