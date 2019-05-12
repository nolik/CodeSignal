mod task;

fn main() {
    println!(
        "{:?}",
        task::email_domain::findEmailDomain("\"much.more unusual\"@yahoo.com".to_string())
    );
}
