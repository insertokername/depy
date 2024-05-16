pub fn get_depy_dir_location() -> String {
    let user_profile =
        std::env::var("USERPROFILE").expect("%USERPROFILE% environment variable not found");
    format!("{}\\depy\\scoop", user_profile)
}

pub fn get_scoop_dir_location() -> String {
    let user_profile =
        std::env::var("USERPROFILE").expect("%USERPROFILE% environment variable not found");
    format!("{}\\scoop", user_profile)
}

fn clear_directory<P: AsRef<std::path::Path>>(dir: P) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            std::fs::remove_file(path)?;
        } else if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
    }
    Ok(())
}

pub fn init_depy_dir() -> Result<(), Box<dyn std::error::Error>> {
    let str_path = get_depy_dir_location();
    let str_bucketpath = [&str_path,"\\buckets"].concat();
    let bucketpath = std::path::Path::new(&str_bucketpath);
    if !bucketpath.exists() {
        std::fs::create_dir_all(&bucketpath).expect("Failed to create depy/scoop/buckets dir! Check read/write privileges!");
    }

    let str_shimpath = [&str_path,"\\shims"].concat();
    let shimpath = std::path::Path::new(&str_shimpath);
    if !shimpath.exists(){
        std::fs::create_dir_all(&shimpath).expect("Failed to create depy/scoop/shims dir! Check read/write privileges!");
    }
    
    clear_directory(&shimpath).expect("Failed to clear contents of shims folder! Check read/write privileges!");

    let str_apppath = [&str_path,"\\apps"].concat();
    let apppath = std::path::Path::new(&str_apppath);
    if !apppath.exists(){
        std::fs::create_dir_all(&apppath).expect("Failed to create depy/scoop/shims dir! Check read/write privileges!");
    }

    let str_scooplocation = get_scoop_dir_location() + "\\apps\\scoop";
    let str_depy_scooplocation = str_apppath+"\\scoop";
    let scooplocation = std::path::Path::new(&str_scooplocation);
    let depy_scooplocation = std::path::Path::new(&str_depy_scooplocation);
    if !depy_scooplocation.exists(){
        copy_dir::copy_dir(&scooplocation, &depy_scooplocation)?;
    }

    
    Ok(())
}
