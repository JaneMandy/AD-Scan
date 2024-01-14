
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;
    use serde_yaml::Value;
    use walkdir::WalkDir;

    use crate::core::r#type::{TemplateContext, TemplateHeader};
    use crate::core::std_io_rust::std_io_rust::{*};

    fn find_yaml_files(dir_path: &str) -> Vec<PathBuf> {
        let mut result = Vec::new();

        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        result.push(path.to_path_buf());
                    }
                }
            }
        }

        result
    }

    pub fn show_reference(template_context:TemplateContext){
        match template_context.yaml_info.info.reference {
            Some(ref reference) => {
                match reference {
                    Value::Null => {}
                    Value::Bool(_) => {
                        //println!("Bool")
                    }
                    Value::Number(_) => {
                        //println!("Number")
                    }
                    Value::String(reference_string) => {
                        //println!("reference:{}",reference_string)
                    }
                    Value::Sequence(reference_sequence) => {
                        for reference_sequence_value in reference_sequence {
                            match reference_sequence_value {
                                Value::String(s) => {
                                    // println!("String element: {}", s);
                                    // 处理字符串类型的逻辑
                                }
                                _ => {}
                            }
                        }
                    }
                    Value::Mapping(_) => {}
                    Value::Tagged(_) => {}

                }
            },
            None => { }
        }
    }
    pub fn ad_new_scanner(dir_path: &str) -> Result<Vec<TemplateContext>,bool>{
        let mut template_info: Vec<TemplateContext> = vec![];
        let modules_path = find_yaml_files(dir_path);
        for path in modules_path {

            match  path.file_name() {
                None => {continue}
                Some(file_name) => {
                    if file_name.eq("recommended.yml") || file_name.eq("wappalyzer-mapping.yml"){
                        //print_message(MessageType::Error,"black yaml script",None);
                        continue
                    } else {

                    }
                }
            }
            match read_yaml_content_from_file(&path) {
                Ok(yaml_content) => {
                    match create_template_context_from_yaml(yaml_content.clone()){
                        Ok(nuclei_script_header)=>{

                            let template_context = TemplateContext::new(nuclei_script_header, yaml_content);
                            template_info.push(template_context);
                        }
                        Err(error)=>{

                            print_message(MessageType::Error, "Error Deserialize: ", Option::from(format_args!("Yaml File:{:?} {:?} ",path ,error)));
                            continue;
                        }
                    }
                }
                Err(error) => {

                    print_message(MessageType::Error, "Error reading file: ", Option::from(format_args!("Yaml File:{:?} {:?} ",path ,error)));
                    continue;
                }
            }
        }
        Ok(template_info)
    }

    #[derive(Debug)]
    enum CustomError {
        IoError(std::io::Error),
        YamlError(serde_yaml::Error),
        // 添加其他可能的错误类型
    }

    impl From<std::io::Error> for CustomError {
        fn from(error: std::io::Error) -> Self {
            CustomError::IoError(error)
        }
    }

    impl From<serde_yaml::Error> for CustomError {
        fn from(error: serde_yaml::Error) -> Self {
            CustomError::YamlError(error)
        }
    }

    fn read_yaml_content_from_file(yaml_path: &PathBuf) -> Result<String, CustomError> {
        let script_content = fs::read_to_string(yaml_path)?;

        Ok(script_content)
    }
    fn create_template_context_from_yaml(path_buf:String) -> Result<TemplateHeader,CustomError> {
        let script_value: serde_yaml::Value = serde_yaml::from_str(&path_buf)?;
        let nuclei_header: TemplateHeader = serde_yaml::from_value(script_value)?;
        Ok(nuclei_header)
    }

