
pub struct Settings {
    pub timestamp_filepath: String,
    pub video_filepath: String
}

impl Settings {
    fn new(ts:String, vd:String) -> Self {
        Settings { timestamp_filepath: ts, video_filepath: vd }
    }
}

pub fn parse_cli_args(args: std::env::Args) -> Settings {
    let mut out = Settings::new("".to_string(),"".to_string());
    let args_vec = args.collect::<Vec<String>>();
    let mut args_iter = args_vec.iter().skip(1);
    
    loop {
        match args_iter.next() {
            Some(arg) => match arg.as_str() {
                "-t" => out.timestamp_filepath = args_iter.next().unwrap().to_string(),
                "-v" => out.video_filepath = args_iter.next().unwrap().to_string(),
                "-h" => {
                    println!("Usage:\n\
                    \t-t\tPath to file containing timestamps and titles\n\
                    \t\t\t'(hh:)mm:ss TITLE' on each line, hours optional, 0:00 required\n\
                    \t-v\tPath to video file\n\
                    \t-h\tView help");
                    std::process::exit(0);
                },
                x => panic!("Unknown cli argument: {}", x)
            },
            None => break,
        }
    }
    out
}