#[cfg(test)]
mod socket_tests {
    use std::{
        fs,
        io::{Read, Write},
        net::{TcpListener, TcpStream, UdpSocket},
        os::unix::net::{UnixListener, UnixStream},
        thread,
        time::Duration,
    };

    // ---------------- TCP 测试 ----------------
    #[test]
    fn test_tcp_socket() {
        let tcp_addr = "127.0.0.1:9000";

        // TCP 服务端
        let server = thread::spawn(move || {
            let listener = TcpListener::bind(tcp_addr).unwrap();
            let (mut stream, addr) = listener.accept().unwrap();
            println!("[TCP Server] 连接来自 {}", addr);

            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("[TCP Server] 收到: {}", String::from_utf8_lossy(&buf[..n]));

            stream.write_all(b"Hello TCP Client!").unwrap();
        });

        // TCP 客户端
        thread::sleep(Duration::from_millis(100));
        let client = thread::spawn(move || {
            let mut stream = TcpStream::connect(tcp_addr).unwrap();
            stream.write_all(b"Hello TCP Server!").unwrap();

            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("[TCP Client] 收到: {}", String::from_utf8_lossy(&buf[..n]));
        });

        server.join().unwrap();
        client.join().unwrap();
    }

    // ---------------- UDP 测试 ----------------
    #[test]
    fn test_udp_socket() {
        let udp_server_addr = "127.0.0.1:9001";
        let udp_client_addr = "127.0.0.1:0";

        // UDP 服务端
        let server = thread::spawn(move || {
            let socket = UdpSocket::bind(udp_server_addr).unwrap();
            println!("[UDP Server] 监听 {}", udp_server_addr);

            let mut buf = [0u8; 1024];
            let (n, src) = socket.recv_from(&mut buf).unwrap();
            println!(
                "[UDP Server] 接收到 {}: {}",
                src,
                String::from_utf8_lossy(&buf[..n])
            );

            socket.send_to(b"Hello UDP Client!", src).unwrap();
        });

        // UDP 客户端
        thread::sleep(Duration::from_millis(100));
        let client = thread::spawn(move || {
            let socket = UdpSocket::bind(udp_client_addr).unwrap();
            socket.connect(udp_server_addr).unwrap();
            socket.send(b"Hello UDP Server!").unwrap();

            let mut buf = [0u8; 1024];
            let n = socket.recv(&mut buf).unwrap();
            println!("[UDP Client] 收到: {}", String::from_utf8_lossy(&buf[..n]));
        });

        server.join().unwrap();
        client.join().unwrap();
    }

    // ---------------- Unix Domain Socket 测试 ----------------
    #[test]
    fn test_unix_socket() {
        let path = "/tmp/rust_unix_test.sock";
        let _ = fs::remove_file(path);

        // Unix Socket 服务端
        let server = thread::spawn(move || {
            let listener = UnixListener::bind(path).unwrap();
            let (mut stream, _) = listener.accept().unwrap();

            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("[Unix Server] 收到: {}", String::from_utf8_lossy(&buf[..n]));

            stream.write_all(b"Hello Unix Client!").unwrap();
        });

        // Unix Socket 客户端
        thread::sleep(Duration::from_millis(100));
        let client = thread::spawn(move || {
            let mut stream = UnixStream::connect(path).unwrap();
            stream.write_all(b"Hello Unix Server!").unwrap();

            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("[Unix Client] 收到: {}", String::from_utf8_lossy(&buf[..n]));
        });

        server.join().unwrap();
        client.join().unwrap();
    }
}
