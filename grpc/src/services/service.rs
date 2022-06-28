use tonic::transport::Channel;


pub trait Service {
    type Client;
    type Server;

    fn get_server() -> Self::Server;
    fn get_client(channel: Channel) -> Self::Client;
}
