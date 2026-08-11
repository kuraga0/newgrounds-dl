use clap::Parser;
use colored::Colorize;
use downloader::Downloader;
use std::{env::temp_dir, path::Path};

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
}

fn convert_title(title: String) -> String {
	let entities: &[(char, &str)] = &[
		('&', "amp;"),
		('@', "at;"),
		('/', "-"),
		(' ', "-"),
		//
	];

	let mut out = String::with_capacity(title.len());
	'chars: for c in title.chars() {
		if c == ' ' {
			out.push('-');
			continue;
		}
		for &(ch, name) in entities {
			if c == ch {
				out.push_str(name);
				continue 'chars;
			}
		}
		out.push(c);
	}

	out
}

fn convert_url(url: String, name: String) -> String {
	let pos = url.rfind('/').unwrap() + 1;
	let num: u64 = url[pos..].parse().unwrap();
	let rounded = (num / 1000) * 1000;

	format!(
		"https://audio.ngfiles.com/{rounded}/{num}_{}.mp3",
		convert_title(name)
	)
}

fn get_page_body(url: String) -> Result<String, Box<dyn std::error::Error>> {
	Ok(ureq::get(url).call()?.body_mut().read_to_string()?)
}

struct SimpleReporterPrivate {
	last_update: std::time::Instant,
	max_progress: Option<u64>,
	message: String,
}
struct SimpleReporter {
	private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
}

impl SimpleReporter {
	fn create() -> std::sync::Arc<Self> {
		std::sync::Arc::new(Self {
			private: std::sync::Mutex::new(None),
		})
	}
}

impl downloader::progress::Reporter for SimpleReporter {
	fn setup(&self, max_progress: Option<u64>, message: &str) {
		let private = SimpleReporterPrivate {
			last_update: std::time::Instant::now(),
			max_progress,
			message: message.to_owned(),
		};

		let mut guard = self.private.lock().unwrap();
		*guard = Some(private);
	}

	fn progress(&self, current: u64) {
		if let Some(p) = self.private.lock().unwrap().as_mut() {
			let current_mb = current as f64 / 1_000_000.0;
			let max_bytes = p.max_progress.map_or_else(
				|| "{unknown}".to_owned(),
				|bytes| format!("{:.2}", bytes as f64 / 1_000_000.0),
			);
			if p.last_update.elapsed().as_millis() >= 1000 {
				println!(
					"test file: {:.2} of {} megabytes. [{}]",
					current_mb, max_bytes, p.message
				);
				p.last_update = std::time::Instant::now();
			}
		}
	}

	fn set_message(&self, message: &str) {
		println!("test file: Message changed to: {message}");
	}

	fn done(&self) {
		_ = self.private.lock().unwrap().take();
		println!("test file: [DONE]");
	}
}

fn main() {
	let args = Args::parse();

	println!("args: {:#?}", args);

	if args.parse_page {
		if args.title.is_some() {
			println!(
				"{} No need to specify the track title if `parse_page` is on (by default).",
				"WARN:".yellow()
			)
		}

		println!("Requesting page...");

		let body = match get_page_body(args.url) {
			Ok(b) => b,
			Err(e) => panic!("{} Request page error: {}", "ERROR:".red(), e),
		};

		let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
		let parser = dom.parser();
		let title = dom
			.query_selector("title")
			.unwrap()
			.next()
			.and_then(|h| h.get(parser))
			.map(|n| n.inner_text(parser));

		let audio = dom
			.query_selector("meta[property=\"og:audio\"]")
			.unwrap()
			.next()
			.and_then(|h| h.get(parser))
			.and_then(|n| n.as_tag())
			.and_then(|tag| tag.attributes().get("content").flatten())
			.map(|b| b.as_utf8_str().to_string());

		if audio.is_none() {
			panic!("{} Cannot find audio url.", "ERROR:".red());
		}
		let audio = audio.unwrap();

		println!("Found audio download link: '{}'", audio);

    if title.is_some() {
      println!("Found title: '{:?}'", title.unwrap());
    }
    else {
      println!("Cannot found title.");
    }

		let mut downloader = Downloader::builder()
			.download_folder(Path::new(&args.output_dir))
			.parallel_requests(1)
			.build()
			.unwrap();

		let dl = downloader::Download::new(&audio);

		// #[cfg(not(feature = "tui"))]
		let dl = dl.progress(SimpleReporter::create());

		let result = downloader.download(&[dl]).unwrap();

		for r in result {
			match r {
				Err(e) => println!("Error: {e}"),
				Ok(s) => {
					println!("Success: {s}");
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
		println!("parsed: '{}'", convert_url(args.url, title));
	}
}
