use std::net::{TcpListener, TcpStream};



fn main() {
    //println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();
    for stream in listener.incoming() {
    	match stream {
        	Ok(stream) => {
            	println!("¡Nuevo cliente!");
        	}
            	Err(e) => {
                	println!("Conexión fallida")
            	}
    	}
	}
}
