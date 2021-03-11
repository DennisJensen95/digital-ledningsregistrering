static LER_API_VERSION_4: &'static str = "v4";
static LER_API_VERSION_1: &'static str = "v1";

pub static LER_API_PRODUCTION: &'static str = "https://services.ler.dk";
pub static LER_API_TEST: &'static str = "https://services-extest.ler.dk";

pub struct LEREndpoints {
    pub basic_test: String,
    pub secure_test: String,
    pub request: String,
    pub request_id: String,
    pub request_received_id: String,
    pub dig_request: String,
    pub dig_request_answer_id: String,
    pub wire_package_id: String,
    pub wire_package_status_id: String,
    pub wire_package_status: String,
    pub wire_package_received_id: String,
    pub dig_request_receit_id: String,
    pub wire_package_map_show_id: String,
    pub dig_damage: String,
    pub interest_area: String,
    pub error_codes: String,
    pub error_code_id: String,
}

impl LEREndpoints {
    pub fn default(base_url: &str) -> LEREndpoints {
        LEREndpoints {
            basic_test: format!(
                "{}/{}/{}",
                base_url,
                &String::from("api"),
                &String::from("basicTest")
            ),
            secure_test: format!(
                "{}/{}/{}",
                base_url,
                &String::from("api"),
                &String::from("secureTest")
            ),
            request: format!(
                "{}/{}/{}/{}",
                base_url,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodning")
            ),
            request_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodning")
            ),
            request_received_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodningModtaget")
            ),
            dig_request: format!(
                "{}/{}/{}/{}",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergsel")
            ),
            dig_request_answer_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergsel")
            ),
            wire_package_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakke")
            ),
            wire_package_status: format!(
                "{}/{}/{}/{}",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeStatus")
            ),
            wire_package_status_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeStatus")
            ),
            wire_package_received_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeModtaget")
            ),
            dig_request_receit_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergselKvittering")
            ),
            wire_package_map_show_id: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeKortviser")
            ),
            dig_damage: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveskade")
            ),
            interest_area: format!(
                "{}/{}/{}/{}/",
                base_url,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("interesseomraade")
            ),
            error_codes: format!(
                "{}/{}/{}",
                base_url,
                &String::from("api"),
                &String::from("ErrorCodes")
            ),
            error_code_id: format!(
                "{}/{}/{}/",
                base_url,
                &String::from("api"),
                &String::from("errorcodes")
            ),
        }
    }
}
