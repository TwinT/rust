//! Simulating files one step at a time

#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed, 
}

/// Represents a file with a name, data, and state
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

fn open(f: &mut File) -> bool{
    true
}

fn close(f: &mut File) -> bool{
    true
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File{
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
     
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    } 
    
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize{
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }

}


fn main(){
    let f2_data= vec![114, 117, 115, 116, 33];

    let mut f2 = File::new_with_data("f2.txt", &f2_data);
    
    let mut buffer: Vec<u8> = Vec::new();

    open(&mut f2);
    let f2_length = f2.read(&mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{}", f2);
    println!("{}", text);
}