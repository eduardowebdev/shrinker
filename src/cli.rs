use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};

use crate::batch::{self, BatchOptions};
use crate::compress::{self, CompressOptions};
use crate::format::parse_target_format;
use crate::progress::Progress;
use crate::types::CompressionResult;
use crate::util::human_bytes;

#[derive(Parser, Debug)]
#[command(
    name = "shrinker",
    version,
    about = "Lossless image compression for everyone",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Compress one or more image files or directories
    Compress(CompressArgs),
    /// Print information about an image
    Info(InfoArgs),
}

#[derive(Args, Debug)]
pub struct CompressArgs {
    /// Input files or directories
    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,

    /// Output path (valid only with a single input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Overwrite input files in place
    #[arg(short = 'i', long)]
    pub in_place: bool,

    /// Recurse into directories
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Target format: auto, png, webp
    #[arg(short, long, default_value = "auto")]
    pub format: String,

    /// Suppress progress and per-file output
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    pub input: PathBuf,
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Compress(args) => run_compress(args),
        Command::Info(args) => run_info(args),
    }
}

fn run_compress(args: CompressArgs) -> Result<()> {
    if args.in_place && args.output.is_some() {
        bail!("--in-place and --output are mutually exclusive");
    }
    if args.output.is_some() && args.inputs.len() > 1 {
        bail!("--output is only valid with a single input");
    }

    let target = parse_target_format(&args.format)?;
    let options = CompressOptions {
        target,
        in_place: args.in_place,
        output: args.output.clone(),
    };

    let files = batch::collect(&args.inputs, args.recursive)?;
    if files.is_empty() {
        bail!("no input files matched");
    }

    let progress = Progress::new(files.len(), args.quiet);
    let results = batch::run(&files, &options, &progress);
    progress.finish();

    print_summary(&results, args.quiet);

    let failures = results.iter().filter(|(_, r)| r.is_err()).count();
    if failures > 0 {
        bail!("{failures} file(s) failed");
    }
    Ok(())
}

fn run_info(args: InfoArgs) -> Result<()> {
    let info = compress::inspect(&args.input)
        .with_context(|| format!("failed to inspect {}", args.input.display()))?;
    println!("path:    {}", info.path.display());
    println!("format:  {}", info.format.label());
    println!("width:   {}", info.width);
    println!("height:  {}", info.height);
    println!("size:    {}", human_bytes(info.bytes));
    Ok(())
}

fn print_summary(results: &[(PathBuf, Result<CompressionResult>)], quiet: bool) {
    let mut total_before = 0u64;
    let mut total_after = 0u64;
    for (path, result) in results {
        match result {
            Ok(r) => {
                total_before += r.before;
                total_after += r.after;
                if !quiet {
                    println!(
                        "{}: {} -> {} ({:.1}% saved)",
                        path.display(),
                        human_bytes(r.before),
                        human_bytes(r.after),
                        r.saved_percent()
                    );
                }
            }
            Err(e) => println!("{}: error: {e}", path.display()),
        }
    }
    if total_before > 0 {
        let total = CompressionResult {
            before: total_before,
            after: total_after,
        };
        println!(
            "total: {} -> {} ({:.1}% saved)",
            human_bytes(total.before),
            human_bytes(total.after),
            total.saved_percent()
        );
    }
}
