mod core;


use std::process::exit;
use crate::core::core::ad_new_scanner;
use crate::core::std_io_rust::std_io_rust::{*};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand, ValueEnum};
use colored::{Colorize, CustomColor};
use boa_engine::{Context, Source};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "AD Scan")]
#[command(author = "JaneMandy. <1026111251@qq.com>")]
#[command(version = "1.0")]
#[command(about = "Template vulnerability scanner developed based on Rust", long_about = "AD Template Scan")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Args)]
#[derive(Debug)]
struct scanner_command_args {
    #[arg(long,short,help="Target HOSTS or target URL target IP address")]
    target :String


}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    vulnscan {
        //
        #[command(flatten)]
        target_args :scanner_command_args,
        #[arg(long,help="nuclei-templates path or yaml script Dir")]
        templates_path :Option<String>
    },
    portscan{

    }
}

fn main() {
  /*  let js_code = r#"
 let m = require('nuclei/net');
      let name=Host+':'+Port;
      let conn = m.OpenTLS('tcp', name);
      conn.Send('GET / HTTP/1.1\r\nHost:'+name+'\r\nConnection: close\r\n\r\n');
      resp = conn.RecvString();
"#;

// Instantiate the execution context
    let mut context = Context::default();

// Parse the source code
    match context.eval(Source::from_bytes(js_code)) {
        Ok(res) => {
            println!(
                "{}",
                res.to_string(&mut context).unwrap().to_std_string_escaped()
            );
        }
        Err(e) => {
            // Pretty print the error
            eprintln!("Uncaught {e}");
        }
    };*/
    let my_color = CustomColor::new(0, 120, 120);
    println!("{}", r#" █████╗ ██████╗       ███████╗ ██████╗ █████╗ ███╗   ██╗
██╔══██╗██╔══██╗      ██╔════╝██╔════╝██╔══██╗████╗  ██║
███████║██║  ██║█████╗███████╗██║     ███████║██╔██╗ ██║
██╔══██║██║  ██║╚════╝╚════██║██║     ██╔══██║██║╚██╗██║
██║  ██║██████╔╝      ███████║╚██████╗██║  ██║██║ ╚████║
╚═╝  ╚═╝╚═════╝       ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝
                                                        "#.bold().custom_color(my_color));
    println!(" -----  {} -- {} ---","SGCC".bold().custom_color(my_color),"www.sgcc.com.cn".bold().red());
    print_message(MessageType::Good, "AD Scanner version:1.0.0 ", None);

    let args = Cli::parse();
    match args.command{
        Commands::vulnscan {target_args,templates_path} => {
            let scanner_ret;
            let mut templates_path_string:String;
            match templates_path {
                None => {
                    if let Some(user_dir) = dirs::home_dir() {
                        // 构建路径

                        let templates_dirs = user_dir.join("nuclei-templates");

                        // 检查目录是否存在，如果不存在则使用默认路径
                        if !templates_dirs.exists() {
                            templates_path_string = "~/nuclei-templates".to_string();

                        } else {

                            // 使用 unwrap_or_default 处理文件名为空的情况
                            templates_path_string = String::from(templates_dirs.to_string_lossy());

                        }
                    } else {
                        // 如果无法获取用户主目录，则使用默认路径
                        templates_path_string = "~/nuclei-templates".to_string();
                    }
                }
                Some(path) => {
                    // 处理传递的模板路径
                    templates_path_string =path;
                }
            }

            // 调用 ad_new_scanner 并处理结果
            print_message(MessageType::Info, "Template path:", Option::from(format_args!("{}", templates_path_string)));
            scanner_ret = ad_new_scanner(&*templates_path_string);
            match scanner_ret {
                Ok(scanner) => {
                    if scanner.len()==0{
                        print_message(MessageType::Warning,"The number of loaded modules is Zero",None);
                    } else {
                        print_message(MessageType::Good, "The module loading is completed and the number of modules is:", Option::from(format_args!("{}", scanner.len())));
                    }

                     print_message(MessageType::Good,"Scanner！！！！！！",None);
                },
                Err(_) => {
                    print_message(MessageType::Error,"Template loading exception",None);
                    exit(-1);
                }
            }
        }
        Commands::portscan {}=>{
            print_message(MessageType::Good,"use port",None);
        }

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