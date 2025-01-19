1. 添加必要依赖库
```toml
tonic = "0.12"
tokio = {version = "1", features = ["rt-multi-thread", "macros"]}
prost = "0.13"
```
2. 编写 proto协议
```proto
syntax = "proto3";
package helloworld;

#可以用于空参数，或者空返回值
import "google/protobuf/empty.proto";

service Greeter {
    // SayHello rpc 接受 HelloRequests 并返回 HelloReplies
    rpc SayHello (HelloRequest) returns (HelloReply);

    rpc Exit (google.protobuf.Empty) returns (google.protobuf.Empty);
}

message HelloRequest {
    // 请求消息中包含要问候的名称
    string name = 1;
}

message HelloReply {
    // 回复包含问候语
    string message = 1;
}
```
3. 根目录创建build.rs
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // println!(">>> out_dir: {:?}", out_dir);
    // tonic_build::configure()
    //     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
    //     .compile_protos(&["proto/helloworld.proto"], &["proto"])
    //     .unwrap();
}
```
4. build proto,生成依赖文件
```sh
    cargo build
```

5. Cargo.toml 分离客户端和服务端，便于测试验证
```toml
[[bin]] # 用来运行 HelloWorld gRPC 服务器的可执行文件
name = "helloworld-server"
path = "src/server.rs"

[[bin]] # 用来运行 HelloWorld gRPC 客户端的可执行文件
name = "helloworld-client"
path = "src/client.rs"
```

6. 编写服务端代码
```rust
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

7. 编写客户端程序
```rust
use hello_world::{greeter_client::GreeterClient, HelloReply, HelloRequest};

//保持和 proto中的命名空间一致
pub mod hello_world{
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest{
        name: "Tonic".into()
    });

    let response = client.say_hello(request).await?;

    println!("HelloReply: {:?}", response.into_inner().message);

    let exit_response = client.exit(()).await?;

    println!("Exit: {:?}", exit_response);

    Ok(())
}
```

8. 运行服务端程序
```sh
    cargo run --bin helloworld-server
```
9. 运行客户端程序
```sh
    cargo run --bin helloworld-client
```