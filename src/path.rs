#[derive(Debug)]
pub struct Path {
    pub path: String,
    pub alias: Option<String>,
    pub args: Option<String>,
}

fn unwrap_or_gracefull<T>(option: Option<T>, found_in_value: &serde_json::Value)->T{
    if let Some(val) = option{
        val
    }else {
        println!("expected bin path to be string not something else: \n{found_in_value}\n is correctly formated!");
        std::process::exit(1)
    }
}

impl Path {
    pub fn new(
        path: String,
        alias: Option<String>,
        args: Option<String>,
    ) -> Path {
        Path { path, alias, args }
    }

    pub fn from_value(value: &serde_json::Value) -> Path {
        if value.is_string() {
            return Path::new(
                value.as_str().expect("\nA cosmic ray from the sun has modified a single bit from your pc memory and changed the output of this function, please run the program again\n").to_string(), 
                None, 
                None );
        } else {
            let arr = unwrap_or_gracefull(value.as_array(), value);

            let path: String = unwrap_or_gracefull( arr[0].as_str(), value).to_string();

            let mut alias: Option<String> = None;
            if arr.len() >= 2 {
                alias = Some(unwrap_or_gracefull(arr[1].as_str(), value).to_string());
            }

            let mut args: String = "".to_string();
            for i in 2..(arr.len()) {
                args += arr[i]
                    .as_str()
                    .expect(&format!("improper args{i}, in bin {value}"));
                args += " ";
            }
            
            return Path::new(path, alias, if args.is_empty() {None} else {Some(args)});
        }
    }

    /// Get the Path objects from a bin attr
    pub fn bin_to_paths(value: &serde_json::Value) -> Vec<Path> {
        if value.is_string() {
            return vec![Path::from_value(value)];
        }

        value
            .as_array()
            .expect(&format!("bin path {value} is not formated corectly"))
            .iter()
            .map(|val| Path::from_value(val))
            .collect()
    }
}
