pub use template::get_template_dictionary;

pub mod template {
    use core::panic;
    use std::collections::HashMap;
    use std::error::Error as DynError;
    use std::ffi::OsStr;
    use std::fs::{read_dir, read_to_string};
    use std::io::Error;
    use std::path::Path;

    const TEMPLATE_PATH: &str = "templates";

    pub fn load_template(name: &OsStr) -> Result<String, Error> {
        let path = Path::new(TEMPLATE_PATH).join(name);
        let template_string = read_to_string(path)?;
        Ok(template_string)
    }

    pub fn get_template_dictionary() -> Result<HashMap<String, String>, Box<dyn DynError>> {
        let template_folder_path = Path::new(TEMPLATE_PATH);
        let template_map: Result<HashMap<String, String>, Box<dyn DynError>> =
            read_dir(template_folder_path)?
                .filter(|folder| folder.as_ref().is_ok_and(|folder| folder.path().is_file()))
                .map(|folder| {
                    let file_name = folder?.file_name();
                    let path: &Path = file_name.as_ref();
                    let file_key = path
                        .file_stem()
                        .ok_or("Value not found")?
                        .to_os_string()
                        .into_string()
                        .map_err(|os_str| panic!("Invalid UTF-8 for file key: {:?}", os_str))?;
                    let template_file_path =
                        file_name.to_os_string().into_string().map_err(|os_str| {
                            panic!("Invalid UTF-8 for template_file_path: {:?}", os_str)
                        })?;
                    Ok((file_key, template_file_path))
                })
                .collect();
        template_map
    }
}
