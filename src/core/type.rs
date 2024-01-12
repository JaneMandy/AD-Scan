use std::collections::HashMap;
use serde::{Deserialize};
use serde_yaml::Value;




#[derive(Debug)]
pub(crate) struct TemplateContext {
    pub(crate) yaml_info: TemplateHeader,
    pub(crate) yaml_file_context: String,  // 修改这里的类型为 String
}

impl TemplateContext {
    pub(crate) fn new(yaml_info: TemplateHeader, yaml_file_context: String) -> TemplateContext {
        TemplateContext {
            yaml_info,
            yaml_file_context,
        }
    }
}
#[derive(Debug, Deserialize)]
pub(crate) struct TemplateHeader {//TemplateHeader
pub(crate) id:String,
    pub(crate) info: TemplateInfo,
}


#[derive(Debug, Deserialize)]
pub struct TemplateInfo {
    name: String,
    author: String,
    #[serde(default)]
    severity: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    pub(crate) reference: Option<Value>,
    #[serde(default)]
    remediation: Option<String>,
    #[serde(default)]
    classification: Option<TemplateClassification>,
    #[serde(default)]
    metadata: Option<HashMap<String, serde_yaml::Value>>,
    #[serde(default)]
    tags: Option<String>,
}
#[derive(Debug, Deserialize)]
struct TemplateClassification {
    #[serde(rename = "cvss-metrics")]
    cvss_metrics: Option<String>,
    #[serde(rename = "cvss-score")]
    cvss_score: Option<f64>,
    #[serde(rename = "cwe-id")]
    cwe_id: Option<Value>,
}