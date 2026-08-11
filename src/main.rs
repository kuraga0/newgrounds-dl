use clap::Parser;
use colored::Colorize;
use std::env::temp_dir;

use stream_download::http::HttpStream;
use stream_download::source::SourceStream;
use stream_download::{Settings, StreamDownload, storage::temp::TempStorageProvider};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	url: String,

	#[arg(short = 'p', long, default_value_t = true, action = clap::ArgAction::Set)]
	parse_page: bool,

	#[arg(short = 't', long, default_value = None)]
	title: Option<String>,

	#[arg(short, long, default_value = None)]
	output: Option<String>,

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

		if let Some(t) = title {
			println!("Found title: '{:?}'", t);
		} else {
			println!("Cannot found title.");
		}

		let settings = Settings::default();
		settings.on_progress(|stream: &HttpStream, state, _| {
			if let Some(total) = stream.content_length() {
				let progress = state.current_position as f32 / total as f32;
				println!("downloading: {}%", progress * 100.0);
			}
		});

		let mut reader = StreamDownload::new_http(audio.parse().unwrap(), TempStorageProvider::new(), settings);
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
