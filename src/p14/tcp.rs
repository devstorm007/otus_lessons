#[test]
fn test_tcp() {
    use std::error::Error;
    use std::io::Read;
    use std::net::{TcpListener, TcpStream};

    fn exec() -> Result<(), Box<dyn Error>> {
        let addr = "192.168.31.12:8095";
        let listener = TcpListener::bind(addr)?;
        for streamR in listener.incoming() {
            let mut stream = streamR?;
            println!("Client {} connected", stream.peer_addr()?);
            let mut buf: [u8; 20] = [0; 20];
            let taken = stream.read(&mut buf)?;
            let vec = buf.to_vec().into_iter().take(taken).collect();
            let st = String::from_utf8(vec)?;
            println!("Received: {st}");
        }

        Ok(())
    }

    exec().unwrap();
}

#[test]
fn test_tcp_stream() {
    use std::error::Error;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    fn exec() -> Result<(), Box<dyn Error>> {
        let addr = "192.168.31.12:8095";
        let mut stream = TcpStream::connect(addr)?;
        stream.write_all(b"Hey Manny!")?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("Mirror: {}", &buf);

        Ok(())
    }

    exec().unwrap();
}

#[test]
fn test_udp() {
    use std::error::Error;
    use std::net::UdpSocket;

    fn exec() -> Result<(), Box<dyn Error>> {
        let addr = "192.168.31.12:8095";
        let socket = UdpSocket::bind(addr)?;
        let mut buf = [0, 255];
        let (size, sender) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, sender);
        socket.send_to(b"Message bytes", sender)?;
        Ok(())
    }

    fn exec2() -> Result<(), Box<dyn Error>> {
        let addr = "192.168.31.12:8095";
        let socket = UdpSocket::bind(addr)?;
        let other_addr = "192.168.31.12:8095";
        socket.connect(other_addr)?;
        let mut buf = [0, 255];
        let size = socket.recv(&mut buf)?;
        println!("Received {} bytes from", size);
        socket.send(b"Message bytes")?;
        Ok(())
    }

    exec().unwrap();
}
