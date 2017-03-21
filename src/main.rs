extern crate libloading as lib;

fn main() {
    load().unwrap();
}

fn load() -> lib::Result<()> {
    let dylib = try!(lib::Library::new("target/debug/libfoo.dylib"));
    let f: lib::Symbol<fn()> = unsafe { try!(dylib.get(b"func\0")) };


    println!("BEFORE");
    f();
    println!("AFTER");

    Ok(())
}
