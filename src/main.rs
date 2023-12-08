use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);

                udp_socket.send_to(&[], source)?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
