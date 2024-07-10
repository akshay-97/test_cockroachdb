use diesel::table;
//use diesel::sql_types::*;
// #[router_derive::diesel_enum(storage_type = "db_enum")]
// #[serde(rename_all = "snake_case")]
// #[strum(serialize_all = "snake_case")]
// pub enum IntentStatus {
//     Succeeded,
//     Failed,
//     Cancelled,
//     Processing,
//     RequiresCustomerAction,
//     RequiresMerchantAction,
//     RequiresPaymentMethod,
//     #[default]
//     RequiresConfirmation,
//     RequiresCapture,
//     PartiallyCaptured,
//     PartiallyCapturedAndCapturable,
// }

// #[router_derive::diesel_enum(storage_type = "db_enum")]
// pub enum Currency {
//     AED,
//     USD,
// }

// #[router_derive::diesel_enum(storage_type = "db_enum")]
// #[serde(rename_all = "snake_case")]
// #[strum(serialize_all = "snake_case")]
// pub enum FutureUsage {
//     OffSession,
//     #[default]
//     OnSession,
// }

// #[router_derive::diesel_enum(storage_type = "db_enum")]
// #[serde(rename_all = "snake_case")]
// #[strum(serialize_all = "snake_case")]
// pub enum PaymentSource {
//     #[default]
//     MerchantServer,
//     Postman,
//     Dashboard,
//     Sdk,
//     Webhook,
//     ExternalAuthenticator,
// }


// #[router_derive::diesel_enum(storage_type = "db_enum")]
// pub enum CountryAlpha2 {
//     AF, AX, AL, DZ, AS, AD, AO, AI, AQ, AG, AR, AM, AW, AU, AT,
//     AZ, BS, BH, BD, BB, BY, BE, BZ, BJ, BM, BT, BO, BQ, BA, BW,
//     BV, BR, IO, BN, BG, BF, BI, KH, CM, CA, CV, KY, CF, TD, CL,
//     CN, CX, CC, CO, KM, CG, CD, CK, CR, CI, HR, CU, CW, CY, CZ,
//     DK, DJ, DM, DO, EC, EG, SV, GQ, ER, EE, ET, FK, FO, FJ, FI,
//     FR, GF, PF, TF, GA, GM, GE, DE, GH, GI, GR, GL, GD, GP, GU,
//     GT, GG, GN, GW, GY, HT, HM, VA, HN, HK, HU, IS, IN, ID, IR,
//     IQ, IE, IM, IL, IT, JM, JP, JE, JO, KZ, KE, KI, KP, KR, KW,
//     KG, LA, LV, LB, LS, LR, LY, LI, LT, LU, MO, MK, MG, MW, MY,
//     MV, ML, MT, MH, MQ, MR, MU, YT, MX, FM, MD, MC, MN, ME, MS,
//     MA, MZ, MM, NA, NR, NP, NL, NC, NZ, NI, NE, NG, NU, NF, MP,
//     NO, OM, PK, PW, PS, PA, PG, PY, PE, PH, PN, PL, PT, PR, QA,
//     RE, RO, RU, RW, BL, SH, KN, LC, MF, PM, VC, WS, SM, ST, SA,
//     SN, RS, SC, SL, SG, SX, SK, SI, SB, SO, ZA, GS, SS, ES, LK,
//     SD, SR, SJ, SZ, SE, CH, SY, TW, TJ, TZ, TH, TL, TG, TK, TO,
//     TT, TN, TR, TM, TC, TV, UG, UA, AE, GB, UM, UY, UZ, VU,
//     VE, VN, VG, VI, WF, EH, YE, ZM, ZW,
//     #[default]
//     US
// }

// #[router_derive::diesel_enum(storage_type = "db_enum")]
// #[serde(rename_all = "snake_case")]
// #[strum(serialize_all = "snake_case")]
// pub enum RequestIncrementalAuthorization {
//     True,
//     False,
//     #[default]
//     Default,
// }

table! {
    users {
        id -> Text,
        job_id -> Int8,
        de -> Jsonb
    }
}

// table! {
//     payment_intent (payment_id, merchant_id) {
//         #[max_length = 64]
//         payment_id -> Varchar,
//         #[max_length = 64]
//         merchant_id -> Varchar,
//         status -> IntentStatus,
//         amount -> Int8,
//         currency -> Nullable<Currency>,
//         amount_captured -> Nullable<Int8>,
//         #[max_length = 64]
//         customer_id -> Nullable<Varchar>,
//         #[max_length = 255]
//         description -> Nullable<Varchar>,
//         #[max_length = 255]
//         return_url -> Nullable<Varchar>,
//         metadata -> Nullable<Jsonb>,
//         #[max_length = 64]
//         connector_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         shipping_address_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         billing_address_id -> Nullable<Varchar>,
//         #[max_length = 255]
//         statement_descriptor_name -> Nullable<Varchar>,
//         #[max_length = 255]
//         statement_descriptor_suffix -> Nullable<Varchar>,
//         created_at -> Timestamp,
//         modified_at -> Timestamp,
//         last_synced -> Nullable<Timestamp>,
//         setup_future_usage -> Nullable<FutureUsage>,
//         off_session -> Nullable<Bool>,
//         #[max_length = 128]
//         client_secret -> Nullable<Varchar>,
//         #[max_length = 64]
//         active_attempt_id -> Varchar,
//         business_country -> Nullable<CountryAlpha2>,
//         #[max_length = 64]
//         business_label -> Nullable<Varchar>,
//         order_details -> Nullable<Array<Nullable<Jsonb>>>,
//         allowed_payment_method_types -> Nullable<Jsonb>,
//         connector_metadata -> Nullable<Jsonb>,
//         feature_metadata -> Nullable<Jsonb>,
//         attempt_count -> Int2,
//         #[max_length = 64]
//         profile_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         merchant_decision -> Nullable<Varchar>,
//         #[max_length = 255]
//         payment_link_id -> Nullable<Varchar>,
//         payment_confirm_source -> Nullable<PaymentSource>,
//         #[max_length = 32]
//         updated_by -> Varchar,
//         surcharge_applicable -> Nullable<Bool>,
//         request_incremental_authorization -> Nullable<RequestIncrementalAuthorization>,
//         incremental_authorization_allowed -> Nullable<Bool>,
//         authorization_count -> Nullable<Int4>,
//         session_expiry -> Nullable<Timestamp>,
//         #[max_length = 64]
//         fingerprint_id -> Nullable<Varchar>,
//         request_external_three_ds_authentication -> Nullable<Bool>,
//         charges -> Nullable<Jsonb>,
//         frm_metadata -> Nullable<Jsonb>,
//     }
// }


// table! {
//     payment_attempt (attempt_id, merchant_id) {
//         #[max_length = 64]
//         payment_id -> Varchar,
//         #[max_length = 64]
//         merchant_id -> Varchar,
//         #[max_length = 64]
//         attempt_id -> Varchar,
//         status -> AttemptStatus,
//         amount -> Int8,
//         currency -> Nullable<Currency>,
//         save_to_locker -> Nullable<Bool>,
//         #[max_length = 64]
//         connector -> Nullable<Varchar>,
//         error_message -> Nullable<Text>,
//         offer_amount -> Nullable<Int8>,
//         surcharge_amount -> Nullable<Int8>,
//         tax_amount -> Nullable<Int8>,
//         #[max_length = 64]
//         payment_method_id -> Nullable<Varchar>,
//         payment_method -> Nullable<Varchar>,
//         #[max_length = 128]
//         connector_transaction_id -> Nullable<Varchar>,
//         capture_method -> Nullable<CaptureMethod>,
//         capture_on -> Nullable<Timestamp>,
//         confirm -> Bool,
//         authentication_type -> Nullable<AuthenticationType>,
//         created_at -> Timestamp,
//         modified_at -> Timestamp,
//         last_synced -> Nullable<Timestamp>,
//         #[max_length = 255]
//         cancellation_reason -> Nullable<Varchar>,
//         amount_to_capture -> Nullable<Int8>,
//         #[max_length = 64]
//         mandate_id -> Nullable<Varchar>,
//         browser_info -> Nullable<Jsonb>,
//         #[max_length = 255]
//         error_code -> Nullable<Varchar>,
//         #[max_length = 128]
//         payment_token -> Nullable<Varchar>,
//         connector_metadata -> Nullable<Jsonb>,
//         #[max_length = 50]
//         payment_experience -> Nullable<Varchar>,
//         #[max_length = 64]
//         payment_method_type -> Nullable<Varchar>,
//         payment_method_data -> Nullable<Jsonb>,
//         #[max_length = 64]
//         business_sub_label -> Nullable<Varchar>,
//         straight_through_algorithm -> Nullable<Jsonb>,
//         preprocessing_step_id -> Nullable<Varchar>,
//         mandate_details -> Nullable<Jsonb>,
//         error_reason -> Nullable<Text>,
//         multiple_capture_count -> Nullable<Int2>,
//         #[max_length = 128]
//         connector_response_reference_id -> Nullable<Varchar>,
//         amount_capturable -> Int8,
//         #[max_length = 32]
//         updated_by -> Varchar,
//         #[max_length = 32]
//         merchant_connector_id -> Nullable<Varchar>,
//         authentication_data -> Nullable<Jsonb>,
//         encoded_data -> Nullable<Text>,
//         #[max_length = 255]
//         unified_code -> Nullable<Varchar>,
//         #[max_length = 1024]
//         unified_message -> Nullable<Varchar>,
//         net_amount -> Nullable<Int8>,
//         external_three_ds_authentication_attempted -> Nullable<Bool>,
//         #[max_length = 64]
//         authentication_connector -> Nullable<Varchar>,
//         #[max_length = 64]
//         authentication_id -> Nullable<Varchar>,
//         mandate_data -> Nullable<Jsonb>,
//         #[max_length = 64]
//         fingerprint_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         payment_method_billing_address_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         charge_id -> Nullable<Varchar>,
//         #[max_length = 64]
//         client_source -> Nullable<Varchar>,
//         #[max_length = 64]
//         client_version -> Nullable<Varchar>,
//     }
// }