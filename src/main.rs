use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

// ID3v2.4.0 Docs: http://id3.org/id3v2.4.0-structure
fn main() -> io::Result<()> {
    let mut mp3_path = PathBuf::from(r"/Users/RyanB/Desktop/mp3_id3_test_files");
    mp3_path.push("02-cashout");
    // mp3_path.push("04-epic_problem");
    mp3_path.set_extension("mp3");

    let mut mp3_file = File::open(mp3_path).expect("Failed to open file");

    let mut tag = Tag::new();
    tag.parse_header(&mut mp3_file);
    // mp3_file.read(&mut tag.header).expect("Failed to read buffer");

    for i in 0..tag.header.identifier.len() {
        println!("{}", tag.header.identifier[i] as char);
    }

    println!("Version: v{:?}.{:?}", tag.header.version[0], tag.header.version[1]);
    println!("Flags: {:?}", tag.header.flags);
    println!("Size: {} bytes", tag.header.size[0] + tag.header.size[1] + tag.header.size[2] +
             tag.header.size[3]);
    Ok(())
}

pub struct Tag {
    pub header: Header,
}

impl Tag {
    pub fn new() -> Tag {
        Tag {
            header: Header::new(),
        }
    }

    pub fn parse_header(&mut self, file_path: &mut File) {
       file_path.read(&mut self.header.identifier).expect("Can't read ID"); 
       file_path.read(&mut self.header.version).expect("Can't read Version"); 
       file_path.read(&mut self.header.flags).expect("Can't read Flags"); 
       file_path.read(&mut self.header.size).expect("Can't read Size"); 
    }
}

pub struct Header {
    pub identifier: [u8; 3],
    pub version: [u8; 2],
    pub flags: [u8; 1], // Use bit masking to uncover which flags are set
    pub size: [u8; 4],

}

impl Header {
    fn new() -> Header {
        Header {
            identifier: [0; 3],
            version: [0; 2],
            flags: [0; 1],
            size: [0; 4],
        }
    }



}
