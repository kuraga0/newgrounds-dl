use clap::Parser;
use colored::Colorize;
use downloader::Downloader;

use std::env::args;
use std::path::Path;
use std::path::PathBuf;

use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

mod download;
mod parse_page;
mod url_convert;
use crate::download::SimpleReporter;
use crate::url_convert::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	url: String,

	#[arg(short = 'p', long, default_value_t = true, action = clap::ArgAction::Set)]
	parse_page: bool,

	#[arg(short = 't', long, default_value = None)]
	title: Option<String>,

	#[arg(short, long, default_value_t = ".".to_string())]
	output_dir: String,

	#[arg(short = 'b', long, default_value_t = false)]
	open_in_browser: bool,

	#[arg(short, long, action = clap::ArgAction::Count)]
	verbose: u8,
}

fn check_path_and_panic(path: PathBuf) {
	if path.exists() {
		panic!("{} File already exists: {}", "ERROR:".red(), path.display());
	}
}

fn open_and_exit(url: &str) {
	open::that(url).unwrap_or_default();
	std::process::exit(0);
}

fn main() {
	let args = Args::parse();

	let level = match args.verbose {
		0 => LevelFilter::WARN,
		1 => LevelFilter::INFO,
		2 => LevelFilter::DEBUG,
		_ => LevelFilter::TRACE,
	};

	tracing_subscriber::fmt().with_max_level(level).init();

	debug!("args: {:#?}", args);

	if args.parse_page {
		if args.title.is_some() {
			println!(
				"{} No need to specify the track title if `parse_page` is on (by default).",
				"WARN:".yellow()
			)
		}

		info!("Requesting page...");

		let track = parse_page::parse_page(args.url);

		if track.audio.is_none() {
			panic!("{} Cannot find audio url.", "ERROR:".red());
		}
		let audio = track.audio.unwrap();

		info!("Found audio download link: '{}'", audio);

		if let Some(t) = track.title {
			info!("Found title: '{}'", t);
		} else {
			info!("Cannot found title.");
		}

		if args.open_in_browser {
			open_and_exit(&audio);
		}

		let filename = audio.rsplit('/').next().unwrap().rsplit("?").last();

		if let Some(f) = filename {
			check_path_and_panic(PathBuf::from(&args.output_dir).join(f));
		}

		println!("Downloading {}", filename.unwrap());

		let mut downloader = Downloader::builder()
			.download_folder(Path::new(&args.output_dir))
			.parallel_requests(1)
			.build()
			.unwrap();

		let dl = downloader::Download::new(&audio);
		let dl = dl.progress(SimpleReporter::create());

		let result = downloader.download(&[dl]).unwrap();

		for r in result {
			match r {
				Err(e) => println!("{} {e}", "ERROR:".red()),
				Ok(s) => {
					info!("Success: {s}");
				}
			}
		}
	} else {
		let title = args.title.clone().unwrap_or_else(|| {
			panic!(
				"{} If `parse_page` is {}, you need to specify the track title.",
				"ERROR:".red(),
				"false".italic()
			)
		});

		let audio = convert_url(args.url, title);

		info!("parsed: '{}'", audio);

		if args.open_in_browser {
			open_and_exit(&audio);
		}

		let filename = audio.rsplit('/').next().unwrap().rsplit("?").last();

		if let Some(f) = filename {
			check_path_and_panic(PathBuf::from(&args.output_dir).join(f));
		}

		let mut downloader = Downloader::builder()
			.download_folder(Path::new(&args.output_dir))
			.parallel_requests(1)
			.build()
			.unwrap();

		let dl = downloader::Download::new(&audio);
		let dl = dl.progress(SimpleReporter::create());

		let result = downloader.download(&[dl]).unwrap();

		for r in result {
			match r {
				Err(e) => {
					println!("{} {e}", "ERROR:".red());
					std::fs::remove_file(filename.unwrap()).unwrap();
				}
				Ok(s) => {
					info!("Success: {s}");
				}
			}
		}
	}
}
