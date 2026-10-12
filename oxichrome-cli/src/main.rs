use clap::{Parser, Subcommand};
use oxichrome_build::Browser;

mod commands;

#[derive(Parser)]
#[command(name = "cargo-oxichrome", bin_name = "cargo")]
struct Cli {
    #[command(subcommand)]
    command: CargoSubcommand,
}

#[derive(Subcommand)]
enum CargoSubcommand {
    Oxichrome(OxichromeArgs),
}

#[derive(Parser)]
struct OxichromeArgs {
    #[command(subcommand)]
    command: OxichromeCommand,
}

#[derive(Subcommand)]
enum OxichromeCommand {
    Build {
        #[arg(long)]
        release: bool,
        #[arg(
            long,
            default_value = "chromium",
            value_parser = parse_browser,
            help = "Target browser: chromium (alias: chrome), firefox, or safari"
        )]
        target: Browser,
    },
    Clean,
    New {
        #[arg(default_value = "my-extension")]
        name: String,
    },
}

fn parse_browser(target: &str) -> Result<Browser, String> {
    match target {
        "chromium" | "chrome" => Ok(Browser::Chromium),
        "firefox" => Ok(Browser::Firefox),
        "safari" => Ok(Browser::Safari),
        other => Err(format!(
            "unknown target browser: {other} (expected \"chromium\", \"firefox\", or \"safari\")"
        )),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CargoSubcommand::Oxichrome(args) => match args.command {
            OxichromeCommand::Build { release, target } => {
                commands::build::run(release, target)?;
            }
            OxichromeCommand::Clean => commands::clean::run()?,
            OxichromeCommand::New { name } => commands::new::run(&name)?,
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_targets() {
        for (target, expected) in [
            ("chromium", Browser::Chromium),
            ("chrome", Browser::Chromium),
            ("firefox", Browser::Firefox),
            ("safari", Browser::Safari),
        ] {
            let cli = Cli::try_parse_from([
                "cargo", "oxichrome", "build", "--release", "--target", target,
            ])
            .unwrap();
            let CargoSubcommand::Oxichrome(args) = cli.command;
            let OxichromeCommand::Build { release, target } = args.command else {
                panic!("expected build command");
            };
            assert!(release);
            assert_eq!(target, expected);
        }
    }

    #[test]
    fn test_default_build_target() {
        let cli = Cli::try_parse_from(["cargo", "oxichrome", "build"]).unwrap();
        let CargoSubcommand::Oxichrome(args) = cli.command;
        let OxichromeCommand::Build { release, target } = args.command else {
            panic!("expected build command");
        };
        assert!(!release);
        assert_eq!(target, Browser::Chromium);
    }

    #[test]
    fn test_unknown_build_target() {
        let error =
            Cli::try_parse_from(["cargo", "oxichrome", "build", "--target", "unknown"])
                .err()
                .expect("unknown targets must be rejected");
        assert!(error.to_string().contains("unknown target browser"));
        assert!(error.to_string().contains("safari"));
    }
}
