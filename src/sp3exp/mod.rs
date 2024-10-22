use std::net::UdpSocket;

fn run_server(addr: &str) {
    let socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0u8; 4];  // byte buffer

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("rx: {} bytes from {}", amt, src);

        let num = u32::from_be_bytes(buf);
        println!("BE: {}", num);

        let num_le = u32::from_le(num);
        println!("LE: {}", num_le);
    }
}
