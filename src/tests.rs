
#[cfg(test)]
mod manifest_test{
    use crate::manifest::Manifest;

    #[test]
    fn correct_manifest_build(){
        let tested_man = init_full_testing_manifest();
    
        let arch = match std::env::consts::ARCH {
        "x86" => "32bit",
        "x86_64" => "64bit",
        "aarch64" => "arm64",
            _ => panic!("this program won't work on your cpu architecture"),
        };


        let bin_paths = vec![
            crate::path::Path::new("bin1".to_string(), None, None),
            crate::path::Path::new("bin2".to_string(), Some("alias2".to_string()), None),
            crate::path::Path::new("bin3".to_string(), Some("alias3".to_string()), Some("arg3".to_string())),
            crate::path::Path::new(format!("{arch}binarch1"),None,None)
        ];

        let added_paths = vec![
            crate::path::Path::new("path1".to_string(),None,None),
            crate::path::Path::new("path2\\random".to_string(),None,None),
            crate::path::Path::new(format!("{arch}path1"),None,None),
        ];

        let env_vars = vec![
            crate::env_var::EnvVar::new("env_var1".to_string(), "val1".to_string()),
            crate::env_var::EnvVar::new("env_var2".to_string(), "val2".to_string()),
            crate::env_var::EnvVar::new(format!("{arch}env_var2"), "val2".to_string()),
        ];

        let version = "20.584".to_string();

        let name = "pkg_name".to_string();

        let correct = Manifest { version, name, bin_paths, added_paths, env_vars};
        print!("jkfdjkdfjkdf\n{:#?}\nkjdfjkdf",tested_man);
        print!("jkfdjkdfjkdf\n{:#?}\nkjdfjkdf",correct);
        assert_eq!(correct, tested_man);
    }

    fn init_full_testing_manifest()->Manifest{
        let body = 
        r###"
        {
            "version": "20.584",
            "bin":[
                "bin1",
                ["bin2","alias2"],
                ["bin3","alias3","arg3"]
            ],
            "env_set": {
                "env_var1": "val1",
                "env_var2": "val2"
            },
            "env_add_path" : [
                "path1",
                "path2\\random"
            ],
            "architecture" : {
                "64bit": {
                    "env_set": {"64bitenv_var2": "val2"},
                    "env_add_path" :["64bitpath1"],
                    "bin":[
                        "64bitbinarch1"
                    ]
                },
                "32bit":{
                    "env_set": {"32bitenv_var2": "val2"},
                    "env_add_path" :["32bitpath1"],
                    "bin":[
                        "32bitbinarch1"
                    ]
                },
                "arm64":{
                    "env_set": {"arm64env_var2": "val2"},
                    "env_add_path" :["arm64path1"],
                    "bin":[
                        "arm64binarch1"
                    ]
                }
            }
        }
        "###;

        Manifest::from_str(body, "pkg_name".to_string()).unwrap()
    }
}