use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    // 创建TCP连接
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 获取流数据
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 流处理方法
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 创建1024字节缓冲区
    let mut buffer = [0; 1024];
    let resut = stream.read(&mut buffer);
    // 模式匹配读取的结果
    match resut {
        // 读取正常
        Ok(_) => {
            println!("input stream is  ok!");
        }
        // 读取有误，panic退出
        Err(error) => {
            println!("input stream is error!");
            panic!("Problem opening the file: {:?}", error)
        }
    }


    // 打印信息
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // 定义数组
    let get = b"GET / HTTP/1.1\r\n";
    // 定义tatus_lien并返回文件名
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // 读取文件
    let contents = fs::read_to_string(filename).unwrap();
    // 设置文件title
    let response = format!("{}{}", status_line, contents);
    // 返回内容
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
