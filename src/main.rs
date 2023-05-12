
mod control {
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    pub(crate) struct Configuration {
        pub port: u16,
    }

    pub(crate) struct Server {
        // server has a tcp connection
        listener: TcpListener,
    }

    impl Server {

        pub(crate) async fn start(configuration: Configuration) -> Self {
            Self {
                listener: TcpListener::bind(format!("0.0.0.0:{}", configuration.port)).await.unwrap(),
            }
        }

        pub(crate) async fn run(&self) {
            loop {
                let (socket, addr) = self.listener.accept().await.unwrap();
                println!("Accepted connection from {:?}", addr);

                tokio::spawn(async move {
                    process_socket(socket).await;
                });

            }
        }


    }

    async fn process_socket(mut socket: tokio::net::TcpStream) {
        loop {
            let mut buffer = [0; 1024];
            // read the request
            let message = socket.read(&mut buffer).await;
            // parse the request
            // send the response
            socket.write(&buffer).await;
        }
    }
}

struct Configuration {
    control: control::Configuration,
}


#[tokio::main]
async fn main() {
    let configuration = Configuration {
        control: control::Configuration {
            port: 8080,
        }
    };

    let server = control::Server::start(configuration.control).await;
    server.run().await;
}
