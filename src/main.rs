use std::{
        fs::{File, self}, 
        env, 
        io::Write, 
        process::Command
};
use regex::Regex;

mod time;
mod metadata;
mod parse;


fn main() {
    let header = ";FFMETADATA1\n";//basically overrides the metadata tho
    let args: Vec<String> = env::args().collect();
    let video_path = &args[2];

    let re = Regex::new(r"^[./]*(.*)").unwrap();
    let video_name = re.captures(video_path).expect("could not decipher video file name").get(1).unwrap().as_str();


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

    // read arg[1] and write to output.txt
    let mut outf = File::create("output.txt").expect("failed to create output file");
    let meta_out = format!("{}{}", header, chaps.iter().map(metadata::Chapter::output).collect::<String>());
    write!(outf, "{}",meta_out).expect("failed to write to output file");

    //ffmpeg extract video metadata
    Command::new("ffmpeg").arg("-i")
                                .arg(video_path)
                                .arg("-i")
                                .arg("output.txt") //should maybe be unique
                                .arg("-map_metadata")
                                .arg("0")
                                .arg("-codec")
                                .arg("copy")
                                .arg(format!("{}{}","STAMPED",video_name))
                                .status()
                                .expect("Failed to extract video metadata with ffmpeg");


}
