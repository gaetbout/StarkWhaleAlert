use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let mailcoach_api_token = std::env::var("COINCAP_API_KEY").expect("COINCAP_API_KEY must be set.");


    println!("Somethng {mailcoach_api_token}");
}
