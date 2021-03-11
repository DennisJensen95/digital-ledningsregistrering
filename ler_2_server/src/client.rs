use crate::ler_endpoints;
extern crate reqwest;

pub async fn punch_through(endpoints: &ler_endpoints::LEREndpoints) -> bool {
    debug!("Checking for live connection");
    let response = reqwest::get(&endpoints.basic_test).await.unwrap();
    if response.status().is_success() {
        return true;
    } else {
        return false;
    }
}

pub async fn secure_punch_through(endpoints: &ler_endpoints::LEREndpoints) -> bool {
    debug!("Checking secure live connection");
    let response = reqwest::get(&endpoints.secure_test).await.unwrap();
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
    use ler_endpoints::LER_API_TEST;
    #[tokio::test]
    async fn test_punch_through() {
        let endpoints = ler_endpoints::LEREndpoints::default(LER_API_TEST);
        assert!(punch_through(&endpoints).await);
    }

    #[tokio::test]
    async fn test_secure_punch_through() {
        let endpoints = ler_endpoints::LEREndpoints::default(LER_API_TEST);
        assert!(secure_punch_through(&endpoints).await);
    }
}
