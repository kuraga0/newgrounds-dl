use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	url: String,

	#[arg(short = 'n', long, default_value = None)]
	name: Option<String>,

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

	let name = args.name.clone().unwrap_or_else(|| {
		println!("getting track name...");
		String::new()
	});

	println!("parsed: '{}'", convert_url(args.url, name));
}
