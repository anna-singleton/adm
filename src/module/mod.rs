use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{metadata,read_dir};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    src: String,
    dest: String,
    hook: Option<String>,
}

impl Module {
    pub fn begin_deploy(&self, vars: &HashMap<String, String>) -> Result<String, String> {
        let mut relative_path = PathBuf::new();
        let ret = self.deploy_step(&mut relative_path, vars);
        return ret;
    }

    fn deploy_step(&self, relative_path: &mut PathBuf, vars: &HashMap<String, String>) -> Result<String, String> {
        let mut path = PathBuf::new();
        path.push(&self.src);
        path.push(&relative_path);
        let Ok(src_meta) = metadata(&path) else {
            return Err(format!("Could not stat src for a template at {}. Exiting.",
                               self.src).to_string());
        };

        if src_meta.is_dir() {
            return self.deploy_directory(relative_path, vars);
        }
        else if src_meta.is_file() {
            return self.deploy_file(relative_path, vars)
        }
        else if src_meta.is_symlink() {
            return Err(format!("Tried to template a src which is a symlink at {}. This\
                               operation is currently unsupported. Exiting.",
                               self.src).to_string());
        }
        else {
            unreachable!("A file exists which is not a directory, file, or symlink.\
                         Please create an issue on adm's github page.")
        }
    }

    fn deploy_directory(&self, relative_path: &mut PathBuf, vars: &HashMap<String, String>) -> Result<String, String> {
        let Ok(contents) = read_dir(&self.src) else {
            return Err(format!("Could not read directory at {}. Exiting.",
                               self.src).to_string());
        };

        for i in contents {
            let Ok(item) = i else {
                return Err(format!("Could not read file in directory {}, Error \
                                   given: {}",
                                   self.src, i.unwrap_err()).to_string());
            };
        }

        // push onto relative path and re call deploy_step

        todo!()
    }

    fn deploy_file(&self, relative_path: &mut PathBuf, vars: &HashMap<String, String>) -> Result<String, String> {
        // use vars and some yucky regex to do string templating against 
        // the file at src/relative_path and write it out to dest/relative_path
        todo!()
    }
}
