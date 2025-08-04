pub use templates::get_template_dictionary;

pub mod templates {
    use std::collections::HashMap;
    use std::ffi::OsStr;
    use std::fs::{read_dir, read_to_string};
    use std::io::Error;
    use std::path::Path;

    const TEMPLATE_PATH: &str = "templates";

    pub fn load_template(name: &OsStr) -> Result<String, Error> {
        let path = Path::new(TEMPLATE_PATH).join(&name);
        let template_string = read_to_string(path)?;
        Ok(template_string)
    }

    pub fn get_template_dictionary() -> Result<HashMap<String, String>, Error> {
        let template_folder_path = Path::new(TEMPLATE_PATH);
        let template_map: HashMap<String, String> = read_dir(template_folder_path)
            .expect("Cannot read templates folder")
            .filter(|folder| folder.as_ref().is_ok_and(|folder| folder.path().is_file()))
            .map(|folder| {
                let file_name = folder.expect("Cannot open folder name").file_name();
                let path: &Path = file_name.as_ref();
                let file_key = path
                    .file_stem()
                    .expect("The provided file name has no stem");
                (
                    file_key
                        .to_os_string()
                        .into_string()
                        .expect("Cannot convert file key OsString to string"),
                    file_name
                        .to_os_string()
                        .into_string()
                        .expect("Cannot convert file name from &OsStr to String"),
                )
            })
            .collect();
        Ok(template_map)
    }
}
