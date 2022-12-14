mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() {
    let client = PrismaClient::_builder().build().await.unwrap();

    let users = client.user().find_many(Vec::new()).exec().await.unwrap();
    if users.len() == 0 {
        println!("--- No users found. Creating one");
        let user = client
            .user()
            .create("Hello World".to_string(), Vec::new())
            .exec()
            .await
            .unwrap();

        println!("{:?}", user);
    } else {
        println!("--- Users");
        println!("{:?}", users);
    }
}
