
#[cfg(test)]
mod manifest_test{
    use crate::Manifest;

    #[test]
    fn lmfao(){
        let tested_man = init_full_testing_manifest();
    
        print!("jkfdjkdfjkdf\n{:#?}\nkjdfjkdf",tested_man);
        assert_eq!(10,20);
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
                "path2\\kjfdjkdf"
            ],
            "architecture" : {
                "64bit": {
                    "env_set": {"64bitenv_var2": "val2"},
                    "env_add_path" :["64bitpath1"],
                    "bin":[
                        "64binarch1",           
                        ["64binarch2","alias2"],
                        ["64binarch3","alias3","arg3"]
                    ]
                },
                "32bit":{
                    "env_set": {"32bitenv_var2": "val2"},
                    "env_add_path" :["32bitpath1"],
                    "bin":[
                        "32binarch1",           
                        ["32binarch2","alias2"],
                        ["32binarch3","alias3","arg3"]
                    ]
                },
                "arm64":{
                    "env_set": {"arm64env_var2": "val2"},
                    "env_add_path" :["arm64path1"],
                    "bin":[
                        "arm64binarch1",           
                        ["arm64binarch2","alias2"],
                        ["arm64binarch3","alias3","arg3"]
                    ]
                }
            }
        }
        "###;

        Manifest::from_str(body).unwrap()
    }
}