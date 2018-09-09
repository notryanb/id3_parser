use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;


fn main() -> io::Result<()> {
    let mut mp3_path = PathBuf::from(r"/Users/RyanB/Desktop/mp3_id3_test_files");
    // mp3_path.push("02-cashout");
    mp3_path.push("04-epic_problem");
    mp3_path.set_extension("mp3");

    let mut mp3_file = File::open(mp3_path).expect("Failed to open file");
    let mut buffer = [0; 48];

    mp3_file.read(&mut buffer).expect("Failed to read buffer");

    for i in 0..buffer.len() {
        println!("{}", buffer[i] as char);
    }

    Ok(())
}
// 73, 68, 51, 3, 0,0,0,0,15,109,84
