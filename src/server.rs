use tokio_stream::wrappers::TcpListenerStream;
use tonic::{transport::Server, Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

//代表的proto中的命名空间
pub mod hello_world{
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug)]
pub struct MyGreeter{}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn exit(
        &self,
        request: Request<()>
    ) -> Result<Response<()>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        //执行退出程序
        // 创建一个异步任务延迟退出，让 gRPC 响应完成
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            std::process::exit(0);
        });

        // 返回一个空响应
        Ok(Response::new(()))
    }
}

fn get_available_port() -> u16 {
    std::net::TcpListener::bind("0.0.0.0:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 方式 1 先获取空闲的端口，再拼接 addr 地址，后再使用 serve绑定
    // 缺点，server绑定时，也会再执行一次 tcplicense，判断端口是否空闲，再绑定，多一次消耗
    // let addr = format!("[::1]:{}", get_available_port()).parse().unwrap();
    // let tcp = std::net::TcpListener::bind("0.0.0.0:0").unwrap().incoming();
    let greeter = MyGreeter::default();

    // 方式 2 通过tokio TcpListener 动态绑定
    // 缺点 多引入tokio-stream 库，还没有判定启动性能差异
    let listener = tokio::net::TcpListener::bind("[::1]:0").await?;

    println!("GreeterServer listening on {}", listener.local_addr()?.port());

    let incommint = TcpListenerStream::new(listener);

    Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve_with_incoming(incommint)
    .await?;

    Ok(())
}