#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod alfred;
mod cloudflare;
mod config;
mod utils;

use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};
use clap_complete::{generate_to, Shell};
use std::{fs, path::PathBuf};

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

fn build_cli<'help>() -> App<'help> {
    App::new("cf-alias")
        .about("CLI interface for Cloudflare Email Routing")
        .version(env!("CFA_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("alfred")
                .about("Commands for the Alfred extension")
                .subcommand(
                    SubCommand::with_name("clipboard")
                        .about("Copys email to clipboard.")
                        .arg(
                            Arg::with_name("email")
                                .short('e')
                                .long("email")
                                .help("Email to copy")
                                .takes_value(true)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Creates a new forwarding email")
                        .arg(
                            Arg::with_name("email-prefix")
                                .short('e')
                                .long("email-prefix")
                                .help("Forwarding email prefix to create")
                                .takes_value(true)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("create-list")
                        .about("Autocomplete command used for creating new emails")
                        .arg(
                            Arg::with_name("query")
                                .short('q')
                                .long("query")
                                .help("Command query or email prefix to be used when creating a new email")
                                .takes_value(true)
                                .required(false),
                        ),
                )
                .subcommand(SubCommand::with_name("manage").about("Opens the Cloudflare Email management UI."))
                .subcommand(SubCommand::with_name("list").about("List existing email forwarders")),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new forwarding email")
                .arg(
                    Arg::with_name("email-prefix")
                        .short('e')
                        .long("email-prefix")
                        .help("Forwarding email prefix to create")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("random")
                        .short('r')
                        .long("random")
                        .help("Generate a random email address")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("completion")
                .about("Generates and installs shell completions automatically")
                .arg(
                    Arg::with_name("shell")
                        .short('s')
                        .long("shell")
                        .help("Which shell to generate completions for (bash|zsh|fish)")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List existing email routes."))
}

async fn parse_cli() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("alfred", args)) => match args.subcommand() {
            Some(("create", run_matches)) => {
                let email_prefix = run_matches.value_of("email-prefix").unwrap().to_string();
                alfred::create(email_prefix).await?;
            }
            Some(("create-list", run_matches)) => {
                let query = run_matches.value_of("query").unwrap_or("").to_string();
                let res = alfred::create_list(query)?;
                println!("{}", res);
            }
            Some(("clipboard", run_matches)) => {
                let email = run_matches.value_of("email").unwrap().to_string();
                alfred::copy_to_clopboard(email);
            }
            Some(("manage", _)) => {
                alfred::open_manage()?;
            }
            Some(("list", _)) => {
                let emails = alfred::list_routes().await?;
                println!("{}", emails);
            }
            _ => {}
        },
        Some(("create", run_matches)) => {
            let mut email_prefix = run_matches
                .value_of("email-prefix")
                .unwrap_or("")
                .to_string();
            if run_matches.is_present("random") {
                email_prefix = "random".to_string();
            }
            let email = create(email_prefix).await?;
            println!("Created new email: {}", email);
        }
        Some(("completion", run_matches)) => {
            let shell_opt = run_matches.value_of("shell");
            let mut app = build_cli();
            let outdir = PathBuf::from("completions");
            fs::create_dir_all(&outdir).ok();

            let shells: Vec<Shell> = match shell_opt {
                Some("bash") => vec![Shell::Bash],
                Some("zsh") => vec![Shell::Zsh],
                Some("fish") => vec![Shell::Fish],
                _ => vec![Shell::Bash, Shell::Zsh, Shell::Fish],
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
        _ => {}
    }

    return Ok(());
}

#[tokio::main]
async fn main() {
    parse_cli().await.unwrap();
}
