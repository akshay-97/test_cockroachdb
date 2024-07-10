use std::{any::Any, fmt::{format, Debug}, ops::Deref, sync::mpsc::TryIter};

use super::schema::users;
use diesel::{backend::Backend, 
        deserialize::{FromSql, FromSqlRow}, 
        expression::AsExpression, pg::Pg, prelude::Insertable, 
        serialize::ToSql, sql_types::Jsonb, PgConnection, 
        Queryable, Selectable, associations::HasTable,
        query_builder::{InsertStatement,IntoUpdateTarget, UpdateStatement, AsQuery, QueryFragment}
        , query_dsl::{LoadQuery, methods::FilterDsl}, RunQueryDsl, AsChangeset,
        Table, ExpressionMethods, BoolExpressionMethods};
use diesel::helper_types::Filter;
use serde::{de::DeserializeOwned, Deserialize, Serialize};


use diesel_models::*;
/*
where
    T: HasTable<Table = T> + Table + 'static + Debug,
    V: Debug + Insertable<T>,
    <T as QuerySource>::FromClause: QueryFragment<Pg> + Debug,
    <V as Insertable<T>>::Values: CanInsertInSingleQuery<Pg> + QueryFragment<Pg> + 'static,
    InsertStatement<T, <V as Insertable<T>>::Values>:
        AsQuery + LoadQuery<'static, PgConnection, R> + Send,
    R: Send + 'static,
*/
fn generic_insert<T, V,  R>(conn : &mut PgConnection, values : V) -> Result<(),()>
where
    T : diesel::associations::HasTable<Table = T> + diesel::Table + Debug,
    V : Debug + Insertable<T>,
    InsertStatement<T, <V as Insertable<T>>::Values> : LoadQuery<'static, PgConnection, R>,
    R: 'static
{
    let query = diesel::insert_into(<T as HasTable>::table()).values(values);
    let start = std::time::Instant::now();
    match query.get_result(conn)
    {
        Ok(_) => {
            let time_elapsed = start.elapsed();
            let table_name = std::any::type_name::<T>().rsplit("::").nth(1);
            println!("{:?} INSERT {}", table_name.unwrap_or("empty"), time_elapsed.as_millis());
            Ok(())
        },
        Err(e) => {
            println!("what is error {:?}", e);
            Err(())
        }, 
    }
}


fn generic_update<T, V, P>(conn : &mut PgConnection, predicate : P, values: V) -> Result<(),()>
where
    T: FilterDsl<P> + HasTable<Table = T> + Table + 'static,
    V: AsChangeset<Target = <Filter<T, P> as HasTable>::Table> + Debug,
    Filter<T, P>: IntoUpdateTarget,
    UpdateStatement<
        <Filter<T, P> as HasTable>::Table,
        <Filter<T, P> as IntoUpdateTarget>::WhereClause,
        <V as AsChangeset>::Changeset,
    >: AsQuery + QueryFragment<Pg> + diesel::query_builder::QueryId + 'static,
    P: 'static,

{
    let query: diesel::query_builder::UpdateStatement<<V as AsChangeset>::Target, _, <V as AsChangeset>::Changeset> = diesel::update(<T as HasTable>::table().filter(predicate)).set(values);
    let start = std::time::Instant::now();
    match query.execute(conn){
        Ok(_) => {
            let time_elapsed = start.elapsed();
            let table_name = std::any::type_name::<T>().rsplit("::").nth(1);
            println!("{:?} UPDATE {}", table_name.unwrap_or("empty"), time_elapsed.as_millis());
            Ok(())
        },
        Err(e) => {
            println!("what is error {:?}", e);
            Err(())}, 
    }
}


fn generic_find<T, P, R>(conn: &mut PgConnection, predicate: P) -> Result<(),()>
where
    T: FilterDsl<P> + HasTable<Table = T> + Table + 'static,
    Filter<T, P>: LoadQuery<'static, PgConnection, R> + QueryFragment<Pg> + 'static,
    R: 'static,
{
    let query = <T as HasTable>::table().filter(predicate);
    let start = std::time::Instant::now();
    match query.get_result(conn){
        Ok(_) => {
            let time_elapsed = start.elapsed();
            let table_name = std::any::type_name::<T>().rsplit("::").nth(1);
            println!("{:?} FIND {}", table_name.unwrap_or("empty"), time_elapsed.as_millis());
            Ok(())
        },
        Err(e) => {
            println!("what is error {:?}", e);
            Err(())
        },
    }
}

fn get_large_value () -> serde_json::Value{
    serde_json::json!({
        "amount": 6540,
        "currency": "USD",
        "confirm": false,
        "business_country":"US",
        "business_label":"default",
        "capture_method": "automatic",
        "capture_on": "2022-09-10T10:11:12Z",
        "amount_to_capture": 6540,
        "customer_id": "StripeCustomer",
        "email": "guest@example.com",
        "name": "John Doe",
        "phone": "999999999",
        "phone_country_code": "+65",
        "description": "Its my first payment request",
        "authentication_type": "no_three_ds",
        "return_url": "https://google.com",
        "payment_method": "card",
        "payment_method_type": "debit",
        "payment_method_data": {
            "card": {
                "card_number": "4242424242424242",
                "card_exp_month": "10",
                "card_exp_year": "25",
                "card_holder_name": "joseph Doe",
                "card_cvc": "123"
            }
        },
        //"setup_future_usage" : "off_session",
        "billing": {
            "address": {
                "line1": "1467",
                "line2": "Harrison Street",
                "line3": "Harrison Street",
                "city": "San Fransico",
                "state": "California",
                "zip": "94122",
                "country": "US",
                "first_name": "joseph",
                "last_name": "Doe"
            },
            "phone": {
                "number": "8056594427",
                "country_code": "+91"
            }
        },
        "shipping": {
            "address": {
                "line1": "1467",
                "line2": "Harrison Street",
                "line3": "Harrison Street",
                "city": "San Fransico",
                "state": "California",
                "zip": "94122",
                "country": "US",
                "first_name": "joseph",
                "last_name": "Doe"
            },
            "phone": {
                "number": "8056594427",
                "country_code": "+91"
            }
        },
        "statement_descriptor_name": "joseph",
        "statement_descriptor_suffix": "JS",
        "metadata": {
            "udf1": "value1",
            "new_customer": "true",
            "login_date": "2019-09-10T10:11:12Z"
        }
    })
}


fn get_large_value2() -> serde_json::Value {
    serde_json::json!({
        "merchant_id": "merchantasd",
        "locker_id": "m0010",
        "merchant_name": "NewAge Retailer",
        "merchant_details": {
          "primary_contact_person": "John Test",
          "primary_email": "JohnTest@test.com",
          "primary_phone": "sunt laborum",
          "secondary_contact_person": "John Test2",
          "secondary_email": "JohnTest2@test.com",
          "secondary_phone": "cillum do dolor id",
          "website": "www.example.com",
          "about_business": "Online Retail with a wide selection of organic products for North America",
          "address": {
            "line1": "1467",
            "line2": "Harrison Street",
            "line3": "Harrison Street",
            "city": "San Fransico",
            "state": "California",
            "zip": "94122",
            "country": "US"
          }
        },
        "return_url": "https://google.com/success",
        "webhook_details": {
          "webhook_version": 123124.12312412,
          "webhook_username": "ekart_retail",
          "webhook_password": "password_ekart@123",
          "payment_created_enabled": true,
          "payment_succeeded_enabled": true,
          "payment_failed_enabled": true
        },
        "routing_algorithm": {
          "type": "single",
          "data": "stripe"
        },
        "sub_merchants_enabled": false,
        "metadata": {
          "city": "NY",
          "unit": "245"
        },
        "primary_business_details": [
          {
            "country": "US",
            "business": "default"
          }
        ]
      })
}
fn payment_intent(i : String) -> PaymentIntentNew{
    PaymentIntentNew {
        payment_id: i.clone(),

        merchant_id: "foo".to_string(),

        status: diesel_models::enums::IntentStatus::Processing,
        amount: common_utils::types::MinorUnit::new(123124),
        currency: Some(diesel_models::enums::Currency::USD),
        amount_captured: None,
        customer_id: None,
        description: Some("randomeString12412953w23421".to_owned()),
        return_url: Some("randomeString12412953w23421".to_owned()),
        metadata: None,
        connector_id: Some("randomeString12412953w23421".to_owned()),
        shipping_address_id: Some("randomeString12412953w23421".to_owned()),
        billing_address_id: Some("randomeString12412953w23421".to_owned()),
        statement_descriptor_name: Some("randomeString12412953w23421".to_owned()),
        statement_descriptor_suffix: Some("randomeString12412953w23421".to_owned()),
        created_at: Some(time::PrimitiveDateTime::MAX),
        modified_at: Some(time::PrimitiveDateTime::MAX),
        last_synced: Some(time::PrimitiveDateTime::MAX),
        setup_future_usage: Some(diesel_models::enums::FutureUsage::OffSession),
        off_session: Some(false),
        client_secret: Some("randomeString12412953w23421".to_owned()),
        active_attempt_id: "asdasdas".to_string(),

        business_country: None,
        business_label: Some("randomeString12412953w23421".to_owned()),
        order_details: None,//Option<Vec<pii::SecretSerdeValue>>,
        allowed_payment_method_types: None, //Value 
        connector_metadata: Some(get_large_value()), //Value
        feature_metadata: Some(get_large_value()), //Value
        attempt_count: i16::MAX,
        profile_id: Some("randomeString12412953w23421".to_owned()),
        merchant_decision: Some("randomeString12412953w23421".to_owned()),
        payment_link_id: Some("randomeString12412953w23421".to_owned()),
        payment_confirm_source: None,
        updated_by: "asdasds".to_string(),

        surcharge_applicable: Some(false),
        request_incremental_authorization: None,
        incremental_authorization_allowed: Some(false),
        authorization_count: None,
        session_expiry: Some(time::PrimitiveDateTime::MAX),
        fingerprint_id: Some("randomeString12412953w23421".to_owned()),
        request_external_three_ds_authentication: Some(false),
        charges: None, //Option<pii::SecretSerdeValue>,
        frm_metadata: None, //Option<pii::SecretSerdeValue>,
    }
}

fn payment_method(i : String) -> PaymentMethodNew{
    PaymentMethodNew
    {
        customer_id: common_utils::id_type::CustomerId::default(),

        merchant_id: "foo".to_owned(),

        payment_method_id: format!("pm_id_{}",&i),
        payment_method: Some(diesel_models::enums::PaymentMethod::Card),//Option<storage_enums::PaymentMethod>,
        payment_method_type: Some(diesel_models::enums::PaymentMethodType::ApplePay),
        payment_method_issuer: Some("test_issuer".into()),
        payment_method_issuer_code: None,
        accepted_currency: Some(vec![diesel_models::enums::Currency::USD]),
        scheme: None,
        token: None,
        cardholder_name: None,
        issuer_name: None,
        issuer_country: None,
        payer_country: Some(vec!["land".into()]),
        is_stored: Some(true),
        swift_code: None,
        direct_debit_token: None,
        created_at: time::PrimitiveDateTime::MAX,
        last_modified: time::PrimitiveDateTime::MAX,
        metadata: None,
        payment_method_data: None,
        locker_id: None,
        last_used_at: time::PrimitiveDateTime::MAX,
        connector_mandate_details: None,
        customer_acceptance: None,
        status: diesel_models::enums::PaymentMethodStatus::Active,
        network_transaction_id: None,
        client_secret: None,
        payment_method_billing_address: None,
        updated_by: Some("test".into()),
    }
}

fn payment_attempt(i : String) -> PaymentAttemptNew{
    PaymentAttemptNew
    {
        payment_id: i.clone(),
        merchant_id:"foo".to_owned(),

        attempt_id: format!("attempt_{}", &i),
        status: diesel_models::enums::AttemptStatus::AuthenticationFailed,
        amount: i64::MAX,
        currency: Some(diesel_models::enums::Currency::USD),
        save_to_locker: Some(false),
        connector: Some("randomeString12412953w23421".to_owned()),
        error_message: Some("randomeString12412953w23421".to_owned()),
        offer_amount: Some(i64::MAX),
        surcharge_amount: Some(i64::MAX),
        tax_amount: Some(i64::MAX),
        payment_method_id: Some("randomeString12412953w23421".to_owned()),
        payment_method: Some(diesel_models::enums::PaymentMethod::BankDebit),
        // connector_transaction_id: Some("randomeString12412953w23421".to_owned()),
        capture_method: Some(enums::CaptureMethod::Automatic),
        //#[serde(default, with = "common_utils::custom_serde::iso8601::option")]
        capture_on: Some(time::PrimitiveDateTime::MAX),
        confirm: true,
        authentication_type: Some(diesel_models::enums::AuthenticationType::NoThreeDs),
        //#[serde(with = "common_utils::custom_serde::iso8601")]
        created_at: Some(time::PrimitiveDateTime::MAX),
       // #[serde(with = "common_utils::custom_serde::iso8601")]
        modified_at: Some(time::PrimitiveDateTime::MAX),
        //#[serde(default, with = "common_utils::custom_serde::iso8601::option")]
        last_synced: Some(time::PrimitiveDateTime::MAX),
        cancellation_reason: Some("randomeString12412953w23421".to_owned()),
        amount_to_capture: Some(i64::MAX),
        mandate_id: Some("randomeString12412953w23421".to_owned()),
        browser_info: None,
        error_code: Some("randomeString12412953w23421".to_owned()),
        payment_token: Some("randomeString12412953w23421".to_owned()),
        connector_metadata: Some(get_large_value()), //Value
        payment_experience: None,
        payment_method_type: None,
        payment_method_data: Some(get_large_value()), //Value
        business_sub_label: Some("randomeString12412953w23421".to_owned()),
        straight_through_algorithm: None,
        preprocessing_step_id: Some("randomeString12412953w23421".to_owned()),
        // providing a location to store mandate details intermediately for transaction
        mandate_details: None,
        error_reason: Some("randomeString12412953w23421".to_owned()),
        multiple_capture_count: None,
        // reference to the payment at connector side
        connector_response_reference_id: Some("randomeString12412953w23421".to_owned()),
        amount_capturable: i64::MAX,
        updated_by:"foo".to_owned(),

        merchant_connector_id: Some("randomeString12412953w23421".to_owned()),
        authentication_data: Some(get_large_value()), //Value
        encoded_data: Some("randomeString12412953w23421".to_owned()),
        unified_code: Some("randomeString12412953w23421".to_owned()),
        unified_message: Some("randomeString12412953w23421".to_owned()),
        net_amount: Some(i64::MAX),
        external_three_ds_authentication_attempted: None,
        authentication_connector: Some("randomeString12412953w23421".to_owned()),
        authentication_id: Some("randomeString12412953w23421".to_owned()),
        mandate_data: None,
        fingerprint_id: Some("randomeString12412953w23421".to_owned()),
        payment_method_billing_address_id: Some("randomeString12412953w23421".to_owned()),
        charge_id: Some("randomeString12412953w23421".to_owned()),
        client_source: Some("randomeString12412953w23421".to_owned()),
        client_version: Some("randomeString12412953w23421".to_owned()),

    }
}

fn payment_intent_update() -> PaymentIntentUpdate{
    PaymentIntentUpdate::ResponseUpdate { status: diesel_models::enums::IntentStatus::Succeeded
            , amount_captured: None
            , fingerprint_id: Some("123dasdasqwf121qwa".to_owned())
            , return_url: Some("qerty.com".to_string())
            , updated_by: "test_kvvv".to_owned()
            , incremental_authorization_allowed: Some(true)
        }
}

fn payment_attempt_update() -> PaymentAttemptUpdate{
    PaymentAttemptUpdate::ConfirmUpdate { amount: i64::default(),
 currency: diesel_models::enums::Currency::INR,
 status: diesel_models::enums::AttemptStatus::Authorized,
 authentication_type: Some(diesel_models::enums::AuthenticationType::NoThreeDs),
 capture_method: Some(diesel_models::enums::CaptureMethod::Scheduled),
 payment_method:None,
 browser_info: Some(get_large_value()),
 connector: None,
 payment_token: Some("asdg".to_owned()),
 payment_method_data: Some(get_large_value2()),
 payment_method_type: None,
 payment_experience: None,
 business_sub_label: None,
 straight_through_algorithm: Some(get_large_value2()),
 error_code: None,
 error_message: None,
 amount_capturable: None,
 surcharge_amount: None,
 tax_amount: None,
 fingerprint_id: None,
 updated_by: "test".to_owned(),
 merchant_connector_id: None,
 payment_method_id: None,
 external_three_ds_authentication_attempted: None,
 authentication_connector: None,
 authentication_id: None,
 payment_method_billing_address_id: None,
 client_source: None,
 client_version: None
}
}
pub fn create_payment_flow (conn : &mut PgConnection, ident : String) -> Result<(),()> {
    let _ = generic_insert::<_ , PaymentIntentNew, PaymentIntent>(conn, payment_intent(ident.clone())).expect("intent payment failed");

    let _ = generic_insert::<_, PaymentMethodNew, PaymentMethod>(conn, payment_method(ident.clone())).expect("method payment failed");
    generic_insert::<_, PaymentAttemptNew, PaymentAttempt>(conn, payment_attempt(ident.clone())).expect("attempt payment failed");
    Ok(())
}

pub fn fetch_payments_flow(conn : &mut PgConnection, ident: String) -> Result<(),()>{
    generic_find::<<PaymentIntent as HasTable>::Table, _, PaymentIntent>(conn, diesel_models::schema::payment_intent::merchant_id
        .eq("foo".to_owned())
        .and(diesel_models::schema::payment_intent::payment_id.eq(ident.clone()))).expect("payment_intent failed");

    generic_find::<<PaymentMethod as HasTable>::Table, _, PaymentMethod>(conn, diesel_models::schema::payment_methods::merchant_id
        .eq("foo".to_owned())
        .and(diesel_models::schema::payment_methods::payment_method_id.eq(format!("pm_id_{}",&ident)))).expect("pm failed");

    generic_find::<<PaymentAttempt as HasTable>::Table, _, PaymentAttempt>(conn, diesel_models::schema::payment_attempt::merchant_id
        .eq("foo".to_owned())
        .and(diesel_models::schema::payment_attempt::attempt_id.eq(format!("attempt_{}",&ident)))).expect("pm failed");
    
    Ok(())
}
pub fn update_payments(conn: &mut PgConnection , ident : String) -> Result<(),()>{
    generic_update::<<PaymentIntent as HasTable>::Table, PaymentIntentUpdateInternal, _,>(
        conn,
        diesel_models::schema::payment_intent::payment_id
            .eq(ident.clone())
            .and(diesel_models::schema::payment_intent::merchant_id.eq("foo".to_owned())),
        PaymentIntentUpdateInternal::from(payment_intent_update())).expect("payment intent update failed");
    
    generic_update::<<PaymentAttempt as HasTable>::Table,PaymentAttemptUpdateInternal,_>(
            conn,
            diesel_models::schema::payment_attempt::attempt_id
                .eq(format!("attempt_{}",&ident))
                .and(diesel_models::schema::payment_attempt::merchant_id.eq("foo".to_owned())),
            PaymentAttemptUpdateInternal::from(payment_attempt_update())
        ).expect("payment update failed");
    Ok(())

}
 
// #[derive(Debug, Queryable, Insertable,Selectable)]
// #[diesel(check_for_backend(Pg))]
// #[diesel(table_name = users)]
// pub struct User{
//     pub id :"foo".to_ownedNone,

//     pub job_id : i64,
//     #[diesel(deserialize_as = serde_json::Value)]
//     pub de : ArrJson,
//     // pub de : ArrJson<serde_json::Value>//Vec<serde_json::Value>
//     // //pub de : Vec<serde_json::Value>
// }

// pub struct UserNew{
//     pub id :"foo".to_ownedNone,

//     pub job_id : i64,
//     #[diesel(serialize_as = OptionalArrJson)]
//     pub de : Option<Vec<serde_json::Value>>
// }
// #[derive(Debug)]
// pub struct ArrJson<T: Debug + Serialize + DeserializeOwned>(pub Vec<T>);


// impl <T : Debug + Serialize + DeserializeOwned> TryInto<ArrJson<T>> for serde_json::Value{
//     type Error = ();

//     fn try_into(self) -> Result<ArrJson<T>, Self::Error> {
//         match self{
//             serde_json::Value::Array(t) => Ok(ArrJson(t)),
//             _ => Err(())
//         }
//     }
// }

// impl Into<serde_json::Value> for ArrJson{
//     fn into(self) -> serde_json::Value {
//         serde_json::Value::Array(self.0)
//     }
// }

// impl ToSql<Jsonb, Pg> for ArrJson{
//     fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
//         let output = serde_json::Value::Array(self.0.to_owned());
//         ToSql::<Jsonb, Pg>::to_sql(&output, &mut out.reborrow())
//     }
// }


// impl Queryable<Jsonb, Pg> for ArrJson
// where
//     serde_json::Value : FromSql<Jsonb, Pg>
// {
//     type Row = serde_json::Value;
    
//     fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
//         match row{
//             serde_json::Value::Array(t) => Ok(Self(t)),
//             e => Err(format!("Invalid type for Jsonb, expected Array {}", e).into())
//         }
//     }
// }
// impl FromSql<Jsonb, Pg> for ArrJson{
//     fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
//         let value = <serde_json::Value as FromSql<Jsonb,Pg>>::from_sql(bytes)?;
//         match value {
//             serde_json::Value::Array(t) => Ok(ArrJson(t)),
//             other => Err(format!("Non array json value found {}", other).into())
//         }
//     }
// }

// #[derive(Debug, AsExpression, FromSqlRow, Serialize, Deserialize)]
// #[serde(transparent)]
// #[diesel(sql_type = diesel::sql_types::Jsonb)]
// pub struct Jsonb<T: Serialize + Debug>(pub T);

// impl<T: Serialize + DeserializeOwned+ Debug> Deref for Jsonb<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl <T : Serialize + DeserializeOwned+ Debug + Clone> ToSql<Jsonb, Pg> for Jsonb<T> {
//     fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
//         let output = serde_json::to_value((*self).clone())?;
//         ToSql::<Jsonb, Pg>::to_sql(&output, &mut out.reborrow())
//     }
// }

// impl<T> FromSql<diesel::sql_types::Jsonb, Pg> for Jsonb<T>
// where
//     T: std::fmt::Debug + DeserializeOwned,
//     T: Serialize + Debug + Clone
// {
//     fn from_sql(bytes: diesel::pg::PgValue) -> diesel::deserialize::Result<Self> {
//         let value = <serde_json::Value as FromSql<diesel::sql_types::Jsonb, Pg>>::from_sql(bytes)?;
//         Ok(Jsonb(serde_json::from_value::<T>(value)?))
//     }
// }