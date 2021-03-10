use crate::ler_endpoints;

extern crate reqwest;

pub async fn punch_through() -> bool {
    debug!("Checking for live connection");
    let response = reqwest::get("https://services.ler.dk/api/basicTest")
        .await
        .unwrap();
    if response.status().is_success() {
        return true;
    } else {
        return false;
    }
}

pub async fn secure_punch_through() -> bool {
    debug!("Checking secure live connection");
    let response = reqwest::get("https://services.ler.dk/api/secureTest")
        .await
        .unwrap();
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

    #[tokio::test]
    async fn test_punch_through() {
        assert!(punch_through().await);
    }

    #[tokio::test]
    async fn test_secure_punch_through() {
        assert!(secure_punch_through().await);
    }
}
