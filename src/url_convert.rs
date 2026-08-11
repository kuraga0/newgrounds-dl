pub fn convert_title(title: String) -> String {
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

pub fn convert_url(url: String, name: String) -> String {
	let pos = url.rfind('/').unwrap() + 1;
	let num: u64 = url[pos..].parse().unwrap();
	let rounded = (num / 1000) * 1000;

	format!(
		"https://audio.ngfiles.com/{rounded}/{num}_{}.mp3",
		convert_title(name)
	)
}
