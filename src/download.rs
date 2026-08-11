struct SimpleReporterPrivate {
	last_update: std::time::Instant,
	max_progress: Option<u64>,
	message: String,
}

pub struct SimpleReporter {
	private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
}

impl SimpleReporter {
	pub fn create() -> std::sync::Arc<Self> {
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
