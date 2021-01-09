use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

// reference: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn main() {
    // 使用 TcpListener 来创建一个简单的 server，绑定到指定的端口，more details: https://doc.rust-lang.org/std/net/struct.TcpListener.html
    // unwrap 是 Rust 提供的一个快速的错误处理方法，当返回值为 None 时会 panic, 这样能很好的避免空指针异常
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // 迭代服务器端收到的请求，该方法永远不会返回 None，此方法相当于循环调用 accept
    for stream in listener.incoming() {
        match stream {
            // 状态 OK 时才处理链接请求
            Ok(stream) => {
                // debug 打印信息
                println!("Got a new connection");
                // 见方法定义
                handle_connection(stream);
            }
            // 错误直接退出程序。。
            Err(e) => panic!("Connection failed: {}", e),
        }
    }
    
}

// 处理客户端的连接请求
fn handle_connection(mut stream: TcpStream) {
    // 申请 1k 大小的数组
    let mut buffer = [0; 1024];
    // 读取客户端（输入流）的数据
    stream.read(&mut buffer).unwrap();
    // 打印调试信息
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // 将输入信息原样写入返回
    stream.write(&buffer).unwrap();
    // 刷新流输出到客户端
    stream.flush().unwrap();
}
