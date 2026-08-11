use clap::Parser;
use colored::Colorize;

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
