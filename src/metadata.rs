#[derive(Debug)]
pub struct Chapter {
	pub start:u32,
	pub end:u32,
	pub title:String
}

impl Chapter {
	pub fn output(&self) -> String {
		format!(
			"[CHAPTER]\n\
			TIMEBASE=1/1000\n\
			START={}\n\
			END={}\n\
			title={}\n"
			,self.start,self.end,self.title)
	}
}
