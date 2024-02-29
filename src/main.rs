use std::iter::Iterator;
use std::string::ToString;

use clap::Parser;
use url::Url;

use crate::url_history::UrlHistoryItem;

mod url_history;

/// Banner shown at startup
fn declutter_banner() -> String {
    String::from(
        "\
██████╗ ███████╗ ██████╗██╗     ██╗   ██╗████████╗████████╗███████╗██████╗
██╔══██╗██╔════╝██╔════╝██║     ██║   ██║╚══██╔══╝╚══██╔══╝██╔════╝██╔══██╗
██║  ██║█████╗  ██║     ██║     ██║   ██║   ██║      ██║   █████╗  ██████╔╝
██║  ██║██╔══╝  ██║     ██║     ██║   ██║   ██║      ██║   ██╔══╝  ██╔══██╗
██████╔╝███████╗╚██████╗███████╗╚██████╔╝   ██║      ██║   ███████╗██║  ██║
╚═════╝ ╚══════╝ ╚═════╝╚══════╝ ╚═════╝    ╚═╝      ╚═╝   ╚══════╝╚═╝  ╚═╝",
    )
}

/// Print the current configuration to stderr
fn declutter_print_cfg(cfg: &Config) {
    if cfg.quiet {
        return;
    }

    eprintln!("{}", declutter_banner());

    if cfg.allow_extensions.is_empty() {
        eprintln!("declutter: allow all extensions");
    } else {
        eprintln!("declutter: allowed extensions : {:?}", cfg.allow_extensions);
    }

    if cfg.deny_extensions.is_empty() {
        eprintln!("declutter: denies no extensions");
    } else {
        eprintln!("declutter: denies extensions : {:?}", cfg.deny_extensions);
    }

    if cfg.allow_duplicates {
        eprintln!("declutter: allow duplicates")
    } else {
        eprintln!("declutter: deny duplicates")
    }
}

#[derive(Parser)]
#[command(
    version,
    about = "Use filters and matchers conditions against a list of URLs 🧨"
)]
struct Config {
    #[arg(
        short = 'a',
        long = "allow",
        default_values_t = config_default_whitelist(),
        help_heading = "Matchers ✅",
        help = "Extensions that should be matched. Can be omitted to only deny extensions"
    )]
    allow_extensions: Vec<String>,

    #[arg(
        short = 'd',
        long = "deny",
        default_values_t = config_default_blacklist(),
        help_heading = "Filters 🚫",
        help = "Extensions to always block. Can be omitted to allow all extensions"
    )]
    deny_extensions: Vec<String>,

    #[arg(
        long = "dup",
        default_value_t = false,
        help_heading = "Filters 🚫",
        help = "allow duplicate urls"
    )]
    allow_duplicates: bool,

    #[arg(
        short = 'q',
        long = "quiet",
        default_value_t = false,
        help_heading = "Debug 🐛",
        help = "Do not print anything to stderr"
    )]
    quiet: bool,
}

fn config_default_whitelist() -> Vec<String> {
    return vec![];
}

fn config_default_blacklist() -> Vec<String> {
    return vec![
        String::from("jpg"),
        String::from("jpeg"),
        String::from("png"),
        String::from("svg"),
        String::from("ico"),
        String::from("gif"),
        String::from("webp"),
        String::from("bmp"),
        String::from("css"),
        String::from("scss"),
        String::from("tif"),
        String::from("tiff"),
        String::from("ttf"),
        String::from("otf"),
        String::from("woff"),
        String::from("woff2"),
        String::from("eot"),
        String::from("pdf"),
        String::from("mp3"),
        String::from("mp4"),
        String::from("avi"),
    ];
}

fn url_parse_extension(url: &Url) -> Option<String> {
    let segments = url.path_segments()?;
    let filename = segments.last()?;
    let start = filename.rfind('.')?;

    if start == filename.len() {
        return None;
    }

    Some(String::from(&filename[(start + 1)..]))
}

fn declutter_url(url: &str, history: &Vec<UrlHistoryItem>, cfg: &Config) -> Option<UrlHistoryItem> {
    return match Url::parse(url) {
        Ok(url) => {
            // Verify if URL extension is allowed, if any
            if let Some(extension) = url_parse_extension(&url) {
                if !cfg.allow_extensions.is_empty() && !cfg.allow_extensions.contains(&extension) {
                    // extension not in allow list
                    return None;
                } else if cfg.deny_extensions.contains(&extension) {
                    // extension denied
                    return None;
                }
            }

            let item = UrlHistoryItem::from(url);

            if !cfg.allow_duplicates && history.contains(&item) {
                return None;
            }

            return Some(item);
        }

        // URL could not be parsed
        Err(..) => None,
    };
}

fn declutter(cfg: &Config) {
    let mut history = vec![];

    for url in std::io::stdin().lines() {
        let url = url.unwrap();
        if let Some(item) = declutter_url(url.as_str(), &history, &cfg) {
            println!("{}", url);
            history.push(item)
        }
    }
}

fn main() {
    let cfg = Config::parse();
    declutter_print_cfg(&cfg);
    declutter(&cfg);
}
