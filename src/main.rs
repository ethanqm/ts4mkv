use std::{
        fs::{File, self}, 
        env, 
        io::Write, 
        process::Command
};


mod time;
mod metadata;
mod parse;


fn main() {
    let args: Vec<String> = env::args().collect();


    let file = fs::read_to_string(&args[1]).expect("Where da timestamp file at?");
    let mut chaps: Vec<metadata::Chapter> = file.lines()
        .map(parse::parse_line)
        .collect();
    
    //set end times
    for i in 0..chaps.len() {
        match chaps.get(i+1) {
            Some(next_chap) => chaps[i].end = next_chap.start,
            None => chaps[i].end = u32::MAX,
        }
    }

    //ffmpeg extract video metadata
    Command::new("ffmpeg").arg("-i")
                                .arg(&args[2]) //video file path
                                .arg("-f")
                                .arg("ffmetadata")
                                .arg("metadata.txt")//@TODO: make unique to avoid clash
                                .output()
                                .expect("Failed to extract video metadata with ffmpeg");

    // read arg[1] and write to output.txt
    // let mut output = File::create("output.txt").expect("failed to create output file");
    // write!(output, "{}", chaps.iter().map(metadata::Chapter::output).collect::<String>()).expect("failed to write to output file");

}
