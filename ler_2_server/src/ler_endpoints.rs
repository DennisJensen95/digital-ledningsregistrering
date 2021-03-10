static LER_API_VERSION_4: &'static str = "v4";
static LER_API_VERSION_1: &'static str = "v1";

static LER_API_PRODUCTION: &'static str = "https://services.ler.dk/";
static LER_API_TEST: &'static str = "https://services-extest.ler.dk/";

struct LEREndpoints {
    basicTest: String,
    secureTest: String,
    request: String,
    requestId: String,
    requestReceivedId: String,
    digRequest: String,
    digRequestAnswerId: String,
    wirePackageId: String,
    wirePackageStatusId: String,
    wirePackageStatus: String,
    wirePackageReceivedId: String,
    digRequestReceitId: String,
    wirePackageMapShowId: String,
    digDamage: String,
    interestArea: String,
    errorCodes: String,
    errorCodeId: String,
}

impl Default for LEREndpoints {
    fn default() -> Self {
        LEREndpoints {
            basicTest: format!(
                "{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                &String::from("basicTest")
            ),
            secureTest: format!(
                "{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                &String::from("basicTest")
            ),
            request: format!(
                "{}/{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodning")
            ),
            requestId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodning")
            ),
            requestReceivedId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_4,
                &String::from("anmodningModtaget")
            ),
            digRequest: format!(
                "{}/{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergsel")
            ),
            digRequestAnswerId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergsel")
            ),
            wirePackageId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakke")
            ),
            wirePackageStatus: format!(
                "{}/{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeStatus")
            ),
            wirePackageStatusId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeStatus")
            ),
            wirePackageReceivedId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeModtaget")
            ),
            digRequestReceitId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveforespoergselKvittering")
            ),
            wirePackageMapShowId: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("ledningspakkeKortviser")
            ),
            digDamage: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("graveskade")
            ),
            interestArea: format!(
                "{}/{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                LER_API_VERSION_1,
                &String::from("interesseomraade")
            ),
            errorCodes: format!(
                "{}/{}/{}",
                LER_API_PRODUCTION,
                &String::from("api"),
                &String::from("ErrorCodes")
            ),
            errorCodeId: format!(
                "{}/{}/{}/",
                LER_API_PRODUCTION,
                &String::from("api"),
                &String::from("errorcodes")
            ),
        }
    }
}

pub static ENDPOINTS: LEREndpoints = LEREndpoints::default();
