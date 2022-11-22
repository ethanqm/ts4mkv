pub struct Timestamp {
	pub hours:u32,
	pub minutes:u32,
	pub seconds:u32
}

impl Timestamp {
	pub fn to_ms(&self) -> u32 {
		self.hours * 3_600_000 
		+ self.minutes * 60_000 
		+ self.seconds * 1000
	}
	pub fn from_str(text: &str) -> Timestamp {
		let parts: Vec<u32> = text.split(':')
								  .map(|x| x.parse::<u32>().unwrap())
								  .collect();
		match parts.len() {
			2 => Timestamp { hours: 0,        minutes: parts[0], seconds: parts[1] },
			3 => Timestamp { hours: parts[0], minutes: parts[1], seconds: parts[2] },
			_ => panic!("Unable to parse timestamp: '{}'.\n\
                        Accepting only 'mm:ss' or 'hh:mm:ss' at the beginning of a line",text)
		}
	}
}
