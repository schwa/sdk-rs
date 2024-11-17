use serde::{Deserialize, Serialize};
use std::process::Command;
type Sdks = Vec<Sdk>;
use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::path::PathBuf;
use tabled::{Table, Tabled};
use trigram::similarity;

#[derive(Tabled, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sdk {
    #[tabled(skip)]
    canonical_name: String,
    display_name: String,
    is_base_sdk: bool,
    platform: String,
    #[tabled(skip)]
    platform_path: PathBuf,
    platform_version: String,
    #[tabled(skip)]
    sdk_path: PathBuf,
    sdk_version: String,
    #[serde(rename = "buildID")]
    #[tabled(display_with = "display_option")]
    #[tabled(skip)]
    build_id: Option<String>,
    #[tabled(display_with = "display_option")]
    product_build_version: Option<String>,
    #[tabled(display_with = "display_option")]
    #[tabled(skip)]
    product_copyright: Option<String>,
    #[tabled(display_with = "display_option")]
    product_name: Option<String>,
    #[tabled(display_with = "display_option")]
    product_version: Option<String>,
    #[serde(rename = "iOSSupportVersion")]
    #[tabled(display_with = "display_option")]
    #[tabled(skip)]
    i_ossupport_version: Option<String>,
    #[tabled(display_with = "display_option")]
    #[tabled(skip)]
    product_user_visible_version: Option<String>,
}

fn display_option(option: &Option<String>) -> String {
    match option {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A tool to show the path of an Xcode SDK. Usage: `sdk mac`
struct Cli {
    /// The name of the SDK. The best match will be used if multiple SDKs match the name.
    name: Option<String>,

    /// Use the '/System/Library/Frameworks' path instead of the SDK path.
    #[arg(short, long)]
    frameworks: bool,

    /// Open the path in Finder.
    #[arg(short, long)]
    open: bool,

    /// Reveal the path in Finder.
    #[arg(short, long)]
    reveal: bool,

    /// Copy the path to the clipboard.
    #[arg(short, long)]
    copy: bool,
}

fn main() {
    let cli = Cli::parse();

    let command = "xcodebuild -showsdks -json";
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    let mut sdks: Sdks = serde_json::from_str(&output).unwrap();

    if let Some(name) = &cli.name {
        sdks.sort_by(|a, b| a.build_id.cmp(&b.build_id));
        sdks.dedup_by(|a, b| a.build_id == b.build_id);
        let sdks_by_similarity = sdks
            .iter()
            .map(|sdk| {
                let s = similarity(&sdk.display_name, name.as_str());
                (sdk, s)
            })
            .collect::<Vec<_>>();
        let best = sdks_by_similarity
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        let result = if cli.frameworks {
            best.sdk_path.join("System/Library/Frameworks")
        } else {
            best.sdk_path.clone()
        };

        if cli.open {
            Command::new("open")
                .arg(result)
                .spawn()
                .expect("failed to execute process");
        } else if cli.reveal {
            Command::new("open")
                .arg("-R")
                .arg(result)
                .spawn()
                .expect("failed to execute process");
        } else if cli.copy {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(result.display().to_string()).unwrap();
        } else {
            println!("{}", result.display());
        }
    } else {
        sdks.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        // print the sdks
        let table = Table::new(sdks).to_string();
        println!("{}", table);
        // for sdk in sdks {
        //     println!("{:?}", sdk.product_name);
        // }
    }
}
