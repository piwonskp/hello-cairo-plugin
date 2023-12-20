use std::env::{self, current_dir};

use anyhow::Result;
use camino::{Utf8PathBuf};
use clap::Args;
use scarb::compiler::CompilerRepository;
use scarb::core::{Config, TargetKind};
use scarb::ops;
use clap::{Parser, Subcommand};


mod plugin;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        about = "Builds output to be later run using cairo-run"
    )]
    Build(BuildArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build(args) => run(args),
    }?;
    Ok(())
}


#[derive(Args, Debug)]
pub struct BuildArgs {
    #[clap(help = "Source directory")]
    path: Option<Utf8PathBuf>,
}

pub fn run(args: BuildArgs) -> Result<()> {
    let source_dir = match args.path {
        Some(path) => get_absolute_path(path),
        None => Utf8PathBuf::from_path_buf(current_dir().unwrap()).unwrap(),
    };

    let mut compilers = CompilerRepository::std();
    compilers.add(Box::new(plugin::compiler::HelloCompiler)).unwrap();

    let cairo_plugins = plugin::plugin::CairoPluginRepository::new();
    let repo: scarb::compiler::plugin::CairoPluginRepository = cairo_plugins.into();

    let manifest_path = source_dir.join("Scarb.toml");
    let config = Config::builder(manifest_path)
        .log_filter_directive(env::var_os("SCARB_LOG"))
        .compilers(compilers)
        .cairo_plugins(repo)
        .build()
        .unwrap();


    let ws = ops::read_workspace(config.manifest_path(), &config).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });

    let opts = ops::CompileOpts {
        include_targets: vec![],
        exclude_targets: vec![TargetKind::TEST.clone()],
    };
    
    ops::compile(ws.members().map(|p| p.id).collect(), opts, &ws)
}

fn get_absolute_path(path: Utf8PathBuf) -> Utf8PathBuf {
    if path.is_absolute() {
        path
    } else {
        relative_to_absolute_path(path)
    }
}

fn relative_to_absolute_path(path: Utf8PathBuf) -> Utf8PathBuf {
    let mut current_path = current_dir().unwrap();
    current_path.push(path);
    Utf8PathBuf::from_path_buf(current_path).unwrap()
}
