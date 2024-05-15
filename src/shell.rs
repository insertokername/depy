/// **assumes that all nevesary apps have been allready installed** enters a dev shell with all environment variables set
pub fn enter_shell(add_path: crate::path::Path) -> String {
    let new_path = add_path.path;
    duct::cmd!("cmd", "/C", "echo caca").env(, std::env::var).read().unwrap()
    
}
