use std::{fs::{File, self}, env, io::Write};




mod time;
mod metadata;
mod parse;


fn main() {
    let args: Vec<String> = env::args().collect();


    let file = fs::read_to_string(&args[1]).expect("where da file at?");
    let mut chaps = file.lines()
        .map(parse::parse)
        .collect::<Vec<metadata::Chapter>>();
    
    //set end times
    for i in 0..chaps.len() {
        match chaps.get(i+1) {
            Some(next_chap) => chaps[i].end = next_chap.start,
            None => chaps[i].end = u32::MAX,
        }
    }

   
    let mut output = File::create("output.txt").expect("failed to create output file");
    write!(output, "{}", chaps.iter().map(metadata::Chapter::output).collect::<String>()).expect("failed to write to output file");


}
