use std::fmt;

#[derive(Debug)]

// access-controlled cryptographically encoded video Format (ACEVid) (acv)
pub struct ACEVid {
	version: u8,
	starting_addr: u16,
}

// Methods
impl ACEVid {
	// add code here
	pub fn show(&self){
		println!("Version: {}\nStarting addr: {}", self.version,
			self.starting_addr);
	}
}

// Related Functions
impl ACEVid {
	pub fn new(version: u8, starting_addr: u16) -> ACEVid{
		ACEVid{
			version,
			starting_addr
		}
	}
}

impl fmt::Display for ACEVid{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Meta-data\n\tFormat: ACEVid\n\tVersion: {}", 
			self.version)
	}
}
