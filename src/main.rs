#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod alfred;
mod cloudflare;
mod config;
mod utils;

use std::io;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Arg, Command};
use clap_complete::{generate_to, Shell};

async fn list_routes() -> Result<String> {
    let routes = cloudflare::list_routes().await?;
    let mut emails = routes
        .iter()
        .map(|e| return e.email.to_owned())
        .collect::<Vec<String>>();
    emails.sort();
    return Ok(emails.join("\n"));
}

async fn create(email_prefix: String) -> Result<String> {
    let email = utils::get_email(email_prefix)?;
    cloudflare::create_route(email.to_owned()).await?;
    return Ok(email);
}

fn build_cli() -> Command {
    return Command::new("cf-alias")
        .about("CLI interface for Cloudflare Email Routing")
        .version(env!("CFA_VERSION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("alfred")
                .about("Commands for the Alfred extension")
                .subcommand(
                    Command::new("clipboard")
                        .about("Copys email to clipboard.")
                        .arg(
                            Arg::new("email")
                                .short('e')
                                .long("email")
                                .help("Email to copy")
                                .required(true)
                                .num_args(1),
                        ),
                )
                .subcommand(
                    Command::new("create")
                        .about("Creates a new forwarding email")
                        .arg(
                            Arg::new("email-prefix")
                                .short('e')
                                .long("email-prefix")
                                .help("Forwarding email prefix to create")
                                .required(true)
                                .num_args(1),
                        ),
                )
                .subcommand(
                    Command::new("create-list")
                        .about("Autocomplete command used for creating new emails")
                        .arg(
                            Arg::new("query")
                                .short('q')
                                .long("query")
                                .help("Command query or email prefix to be used when creating a new email")
                                .required(false)
                                .num_args(1),
                        ),
                )
                .subcommand(
                    Command::new("manage")
                        .about("Opens the Cloudflare Email management UI.")
                )
                .subcommand(
                    Command::new("list")
                        .about("List existing email forwarders")
                ),
        )
        .subcommand(
            Command::new("create")
                .about("Creates a new forwarding email")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("email-prefix")
                        .short('e')
                        .long("email-prefix")
                        .help("Forwarding email prefix to create")
                        .required(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("random")
                        .short('r')
                        .long("random")
                        .help("Generate a random email address")
                        .required(false)
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("completion")
                .about("Generates and installs shell completions automatically")
                .arg(
                    Arg::new("shell")
                        .short('s')
                        .long("shell")
                        .help("Which shell to generate completions for.")
                        .value_parser(clap::builder::EnumValueParser::<Shell>::new())
                        .required(false)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List existing email routes.")
        );
}

async fn parse_cli() -> Result<()> {
    let matches = build_cli().get_matches();
    match matches.subcommand() {
        Some(("alfred", args)) => {
            match args.subcommand() {
                Some(("create", run_matches)) => {
                    let email_prefix = run_matches
                        .get_one::<String>("email-prefix")
                        .unwrap()
                        .to_string();
                    alfred::create(email_prefix).await?;
                }
                Some(("create-list", run_matches)) => {
                    let default_res = "".to_string();
                    let query = run_matches
                        .get_one::<String>("query")
                        .unwrap_or_else(|| return &default_res)
                        .to_string();
                    let res = alfred::create_list(query)?;
                    println!("{}", res);
                }
                Some(("clipboard", run_matches)) => {
                    let email = run_matches
                        .get_one::<String>("email")
                        .unwrap()
                        .to_string();
                    alfred::copy_to_clopboard(email);
                }
                Some(("manage", _)) => {
                    alfred::open_manage()?;
                }
                Some(("list", _)) => {
                    let emails = alfred::list_routes().await?;
                    println!("{}", emails);
                }
                _ => unreachable!(),
            }
        }
        Some(("create", run_matches)) => {
            let default_res = "".to_string();
            let mut email_prefix = run_matches
                .get_one::<String>("email-prefix")
                .unwrap_or_else(|| return &default_res)
                .to_string();
            if run_matches.contains_id("random") {
                email_prefix = "random".to_string();
            }
            let email = create(email_prefix).await?;
            println!("Created new email: {}", email);
        }
        Some(("completion", run_matches)) => {
            use clap_complete::{generate, Shell};
            let mut app = build_cli();
            let outdir = PathBuf::from("completions");
            fs::create_dir_all(&outdir).ok();
            let shells = if let Some(shell) = run_matches.get_one::<Shell>("shell").copied() {
                vec![shell]
            } else {
                vec![Shell::Bash, Shell::Zsh, Shell::Fish]
            };
            for shell in shells {
                let target = generate_to(shell, &mut app, "cf-alias", &outdir)
                    .expect("failed to write completion");
                eprintln!("Generated completion: {}", target.display());
                if shell == Shell::Zsh {
                    let sys_path = PathBuf::from("/usr/share/zsh/site-functions/_cf-alias");
                    if fs::copy(&target, &sys_path).is_ok() {
                        eprintln!("Installed Zsh completion to {}", sys_path.display());
                    } else {
                        eprintln!(
                            "No permission to write {}, leaving file in {:?}",
                            sys_path.display(),
                            outdir
                        );
                    }
                }
            }
        }
        Some(("list", _)) => {
            let emails = list_routes().await?;
            println!("{}", emails);
        }
        _ => unreachable!(),
    }
    return Ok(());
}

#[tokio::main]
async fn main() {
    parse_cli().await.unwrap();
}
