use crate::ler_endpoints;
use actix_web::client::Client;

pub async fn punch_through(endpoints: &ler_endpoints::LEREndpoints) -> bool {
    debug!("Checking for live connection");
    let client = Client::default();

    let response = client
        .get(&endpoints.basic_test)
        .header("User-Agent", "actix-web/3.0")
        .send() // <- Send request
        .await.unwrap(); 

    debug!("Response: {:?}", response);

    if response.status().is_success() {
    return true;
    } else {
    return false;
    }
}

pub async fn secure_punch_through(endpoints: &ler_endpoints::LEREndpoints) -> bool {
    debug!("Checking secure live connection");
    let client = Client::default();
    let response = client
        .get(&endpoints.basic_test)
        .header("User-Agent", "actix-web/3.0")
        .send() // <- Send request
        .await.unwrap(); 

    if response.status().is_success() {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use ler_endpoints::LER_API_PRODUCTION;

    #[actix_rt::test]
    async fn test_punch_through() {
        let endpoints = ler_endpoints::LEREndpoints::default(LER_API_PRODUCTION);
        assert!(punch_through(&endpoints).await);
    }

    #[actix_rt::test]
    async fn test_secure_punch_through() {
        let endpoints = ler_endpoints::LEREndpoints::default(LER_API_PRODUCTION);
        assert!(secure_punch_through(&endpoints).await);
    }
}