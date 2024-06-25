use crate::parse_json;

#[derive(Debug, PartialEq)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl EnvVar {
    pub fn new(name: String, value: String) -> EnvVar {
        EnvVar { name, value }
    }

    /// Transforms a serde val into a vec of environment variables
    pub fn from_value(value: &serde_json::Value) -> Result<Vec<EnvVar>, parse_json::ParseJsonError> {
        let mut env_var_iter = if let Some(out_as_obj) = value.as_object() {
            out_as_obj
        } else {
            log::error!("Improperly formated environment variable {value}!");
            return Err(parse_json::ParseJsonError::EnvVarFormatError);
        }
        .iter();

        if !env_var_iter.all(|val| val.1.is_string()) {
            log::error!("Improperly formated environment variable {value}!");
            return Err(parse_json::ParseJsonError::EnvVarFormatError);
        }

        // this is safe because of earlier checks
        Ok(value
            .as_object()
            .unwrap()
            .iter()
            .map(|val| EnvVar::new(val.0.clone(), val.1.as_str().unwrap().to_string()))
            .collect())
    }
}
