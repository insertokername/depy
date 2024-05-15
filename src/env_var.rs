#[derive(Debug)]
pub struct EnvVar {
    name: String,
    value: String,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseJsonError {
    #[error("Error: Improperly formated environment variable!")]
    ImproperEnvVarFormat,
}

impl EnvVar {
    pub fn new(
        name: String,
        value: String,
    ) -> EnvVar {
        EnvVar { name, value }
    }

    /// Transforms a serde val into a vec of environment variables
    pub fn from_multiple_values(value: &serde_json::Value) -> Result<Vec<EnvVar>, ParseJsonError> {
        let mut env_var_iter = if let Some(out_as_obj) = value.as_object() {
            out_as_obj
        } else {
            println!("Improperly formated environment variable {value}!");
            return Err(ParseJsonError::ImproperEnvVarFormat);
        }
        .iter();

        if !env_var_iter.all(|val| val.1.is_string()) {
            println!("Improperly formated environment variable {value}!");
            return Err(ParseJsonError::ImproperEnvVarFormat);
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
