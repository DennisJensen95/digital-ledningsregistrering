// Application libraries
mod logger;

// Third party libraries
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;

async fn punchthrough() -> bool {
    let response = reqwest::get("https://services.ler.dk/api/basicTest")
        .await
        .unwrap();
    if response.status().is_success() {
        return true;
    } else {
        return false;
    }
}

#[tokio::main]
async fn main() {
    logger::init_logger();
    debug!("Succesfully initalized logger");
    punchthrough().await;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_puncthrough() {
        let result = punchthrough().await;
        assert_eq!(result, true);
    }
}
