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