mod core;


use crate::core::core::ad_new_scanner;
use crate::core::std_io_rust::std_io_rust::{*};


fn main() {
    print_message(MessageType::Good, "AD Scanner ", None);

    let scanner_ret = ad_new_scanner("～/nuclei-templates/");
    match scanner_ret {
        Ok(scanner) => {},
        Err(_) => {println!("error")}
    }


    // Parse the YAML script into a serde_yaml::Value
//    let script_value: serde_yaml::Value = serde_yaml::from_str(&script_content).expect("Failed to parse YAML");

    // Deserialize the YAML script into your custom struct
  //  let nuclei_script: NucleiScript = serde_yaml::from_value(script_value).expect("Failed to deserialize into struct");

    // Access information from the NucleiScript struct
    //println!("id: {}", nuclei_script.id);

   // for request in nuclei_script.requests {
    //    println!("Request Method: {}", request.method);
    //    println!("Request Path: {}", request.path);
        // Access more fields as needed
    //}


}