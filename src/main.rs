use std::env;
use std::io;
use std::fs::{self, PathExt, DirEntry};
use std::path::Path;
use std::path::PathBuf;

fn visit_dirs(dir: &Path ) -> (io::Result<()>) 
{
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            println!("file {:?}",entry.path());
            try!(visit_dirs(&entry.path()))
        }
    Ok(())
}

fn main() {
    println!("Hello, rust_dev!");
    if let Some(arg1) = env::args().nth(1) {
        println!("The first argument is {}", arg1);

       // let zarb=PathBuf::from(arg1);
        let p=Path::new(&arg1);
        visit_dirs(&p );
    }else{
    	println!("needs [path] argument ");
    }

}