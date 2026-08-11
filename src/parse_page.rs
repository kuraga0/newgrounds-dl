use colored::Colorize;

pub struct Track {
	pub title: Option<String>,
  // url.mp3
	pub audio: Option<String>,
}

fn get_page_body(url: String) -> Result<String, Box<dyn std::error::Error>> {
	Ok(ureq::get(url).call()?.body_mut().read_to_string()?)
}

pub fn parse_page(url: String) -> Track {
	let body = match get_page_body(url) {
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
		.map(|n| n.inner_text(parser).to_string());

	let audio = dom
		.query_selector("meta[property=\"og:audio\"]")
		.unwrap()
		.next()
		.and_then(|h| h.get(parser))
		.and_then(|n| n.as_tag())
		.and_then(|tag| tag.attributes().get("content").flatten())
		.map(|b| b.as_utf8_str().to_string());

  Track {
    title,
    audio
  }
}
