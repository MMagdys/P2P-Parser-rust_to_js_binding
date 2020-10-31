use neon::prelude::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;

pub mod file_formats;


fn read_file(file_name: &str) -> io::Result<[u8; 30]> {
  
  const BUFFER_SIZE: usize = 30;

  let mut f = File::open(&file_name)?;
  let mut buffer = [0; BUFFER_SIZE];

  f.read(&mut buffer[..])?;

  // println!("The bytes: {:?}", &buffer[..]);
  Ok(buffer)

}



fn hello(mut cx: FunctionContext) -> JsResult<JsString> {

    let my_file  = read_file("examples/video.mp4");


    let file_bytes ;
    match my_file {
        Ok(a) => { file_bytes =  a ;},
        Err(error) => {panic!("Error Parsing the file: {:?}", error);}
    };

    println!("{:?}", &file_bytes);

    let f = file_formats::ACEVid::new(file_bytes[0], file_bytes[1] as u16);
    f.show();

    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});