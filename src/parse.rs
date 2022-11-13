use crate::metadata::Chapter;
use crate::time::Timestamp;



pub fn parse_line(line:&str) -> Chapter {
	let pair = line.split_once(' ').unwrap();
	let ms = Timestamp::from_str(pair.0).to_ms();
	
	Chapter{start:ms, end:0, title:String::from(pair.1)}
}
