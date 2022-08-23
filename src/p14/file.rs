#[test]
fn test_cursor() {
    use std::error::Error;
    use std::io::{BufRead, Cursor};

    fn exec_cursor() -> Result<(), Box<dyn Error>> {
        let data: &[u8; 20] = b"First\r\nSecond\r\nThird";
        let cursor = Cursor::new(data);
        //let _ = cursor.;
        let lines = cursor.lines();
        for (i, line) in lines.enumerate() {
            println!("{}: {}", i, line?)
        }

        println!("{:?}", data);

        Ok(())
    }
    exec_cursor();
}

#[test]
fn test_stdin() {
    use std::error::Error;
    use std::io::prelude::*;
    use std::io::{BufRead, Read, Write};
    use std::path::Path;
    use std::{fs, io, mem};

    fn exec() -> Result<(), Box<dyn Error>> {
        let mut s = String::new();
        io::stdin().read_line(&mut s);
        println!("{s}");

        let mut buf = [0; 20];
        let red = io::stdin().read(&mut buf);
        println!("{buf:?}");

        let input = io::stdin();
        let mut lock = input.lock();

        lock.read_line(&mut s)?;
        println!("{s}");

        lock.read_line(&mut s)?;
        println!("{s}");

        Ok(())
    }
    exec();
}

#[test]
fn test_stdout() {
    use std::error::Error;
    use std::io::prelude::*;
    use std::io::{BufRead, Read, Write};
    use std::path::Path;
    use std::{fs, io, mem};

    fn exec() -> Result<(), Box<dyn Error>> {
        io::stdout().write_all(b"Hello");
        let mut buf = [b'x'; 10];
        let written = io::stdout().write(&mut buf);

        let output = io::stdout();
        let mut lock = output.lock();
        writeln!(lock, "First: {}", 1);
        writeln!(lock, "Second: {}", 2);
        println!("Third");

        Ok(())
    }
    exec();
}

#[test]
fn test_bufread() {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufReader, BufWriter};
    use std::path::Path;

    fn buf_input(input: impl Read) -> impl BufRead {
        BufReader::new(input)
    }

    let file = File::open(Path::new("hello.txt")).unwrap();
    let mut buffered_file = buf_input(&file);

    let lines = buffered_file.consume(10);
    println!("{:?}", lines);

    fn buf_output(input: impl Write) -> impl Write {
        BufWriter::new(input)
    }
    let mut buffer = File::create("foo.txt").unwrap();
    let mut buffered_write_file = buf_output(&buffer);
    buffered_write_file.write("some data".as_bytes());
    let written = buffered_write_file.flush();
    println!("{:?}", written);
}

#[test]
fn test_file() {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn exec() -> Result<(), Box<dyn Error>> {
        let path = Path::new("hello.txt");
        let display = path.display();

        let mut file = File::open(&path)?;

        let mut s = String::new();
        file.read_to_string(&mut s)?;
        print!("{} contains:\n{}", display, s);

        Ok(())
    }

    exec().unwrap();

    /*let path = Path::new("hello.txt");
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }*/

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

#[test]
fn test_file2() {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufRead, Read, Write};
    use std::path::Path;
    use std::{fs, io, mem};

    fn exec() -> Result<(), Box<dyn Error>> {
        let path = Path::new("fan.txt");
        let display = path.display();

        let mut file = File::create(path)?;
        file.write_all("Test new file".as_bytes())?;
        mem::drop(file);

        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;
        println!("File content:\n{}", content);
        fs::remove_file(path)?;

        Ok(())
    }

    exec().unwrap();
}

#[test]
fn test_file3() {
    use std::error::Error;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::{BufRead, Read, Write};
    use std::path::Path;
    use std::{fs, io, mem};

    fn exec() -> Result<(), Box<dyn Error>> {
        let path = Path::new("fan.txt");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all("Test new file".as_bytes())?;
        mem::drop(file);

        let mut content = String::new();
        File::open(path)?.read_to_string(&mut content)?;
        println!("File content:\n{}", content);
        fs::remove_file(path)?;

        Ok(())
    }

    exec().unwrap();
}
