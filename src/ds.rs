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
fn generic_insert<T, V,  R>(conn : &mut PgConnection, values : V) -> Result<(), ()>
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

fn generic_update<T, V, P>(conn : &mut PgConnection, predicate : P, values: V) -> Result<(), ()>
where
    T: FilterDsl<P> + HasTable<Table = T> + Table + 'static,
    V: AsChangeset<Target = <Filter<T, P> as HasTable>::Table> + Debug + 'static,
    Filter<T, P>: IntoUpdateTarget + 'static,
    UpdateStatement<
        <Filter<T, P> as HasTable>::Table,
        <Filter<T, P> as IntoUpdateTarget>::WhereClause,
        <V as AsChangeset>::Changeset,
    >: AsQuery + LoadQuery<'static, PgConnection, P> + QueryFragment<Pg> + Clone,
    P: 'static,

{
    let query: diesel::query_builder::UpdateStatement<<V as AsChangeset>::Target, _, <V as AsChangeset>::Changeset> = diesel::update(<T as HasTable>::table().filter(predicate)).set(values);
    match query.get_result(conn){
        Ok(_) => Ok(()),
        Err(e) => {
            println!("what is error {:?}", e);
            Err(())}, 
    }
}


fn generic_find<T, P, R>(conn: &mut PgConnection, predicate: P) -> Result<(), ()>
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
fn payment_method(i : String) -> PaymentMethodNew{
    PaymentMethodNew
    {
        customer_id: common_utils::id_type::CustomerId::default(),
        merchant_id: "foo".to_owned(),
        payment_method_id: format!("pm_id_{}",&i),
        payment_method: Some(common_enums::PaymentMethod::Card),//Option<storage_enums::PaymentMethod>,
        payment_method_type: Some(common_enums::PaymentMethodType::ApplePay),
        payment_method_issuer: Some("test_issuer".into()),
        payment_method_issuer_code: None,
        accepted_currency: Some(vec![common_enums::Currency::USD]),
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
        status: common_enums::PaymentMethodStatus::Active,
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
        status: common_enums::AttemptStatus::AuthenticationFailed,
        amount: i64::MAX,
        currency: Some(common_enums::Currency::USD),
        save_to_locker: Some(false),
        connector: None, //Option<String>,
        error_message: None, //Option<String>,
        offer_amount: Some(i64::MAX),
        surcharge_amount: Some(i64::MAX),
        tax_amount: Some(i64::MAX),
        payment_method_id: None, //Option<String>,
        payment_method: Some(common_enums::PaymentMethod::BankDebit),
        // connector_transaction_id: None, //Option<String>,
        capture_method: Some(enums::CaptureMethod::Automatic),
        //#[serde(default, with = "common_utils::custom_serde::iso8601::option")]
        capture_on: Some(time::PrimitiveDateTime::MAX),
        confirm: true,
        authentication_type: Some(common_enums::AuthenticationType::NoThreeDs),
        //#[serde(with = "common_utils::custom_serde::iso8601")]
        created_at: Some(time::PrimitiveDateTime::MAX),
       // #[serde(with = "common_utils::custom_serde::iso8601")]
        modified_at: Some(time::PrimitiveDateTime::MAX),
        //#[serde(default, with = "common_utils::custom_serde::iso8601::option")]
        last_synced: Some(time::PrimitiveDateTime::MAX),
        cancellation_reason: None, //Option<String>,
        amount_to_capture: Some(i64::MAX),
        mandate_id: None, //Option<String>,
        browser_info: None,
        error_code: None, //Option<String>,
        payment_token: None, //Option<String>,
        connector_metadata: None,
        payment_experience: None,
        payment_method_type: None,
        payment_method_data: None,
        business_sub_label: None, //Option<String>,
        straight_through_algorithm: None,
        preprocessing_step_id: None, //Option<String>,
        // providing a location to store mandate details intermediately for transaction
        mandate_details: None,
        error_reason: None, //Option<String>,
        multiple_capture_count: None,
        // reference to the payment at connector side
        connector_response_reference_id: None, //Option<String>,
        amount_capturable: i64::MAX,
        updated_by:"foo".to_owned(),
        merchant_connector_id: None, //Option<String>,
        authentication_data: None,
        encoded_data: None, //Option<String>,
        unified_code: None, //Option<String>,
        unified_message: None, //Option<String>,
        net_amount: Some(i64::MAX),
        external_three_ds_authentication_attempted: None,
        authentication_connector: None, //Option<String>,
        authentication_id: None, //Option<String>,
        mandate_data: None,
        fingerprint_id: None, //Option<String>,
        payment_method_billing_address_id: None, //Option<String>,
        charge_id: None, //Option<String>,
        client_source: None, //Option<String>,
        client_version: None, //Option<String>

    }
}

pub fn create_payment_flow (conn : &mut PgConnection, ident : String) -> Result<(), ()> {
    let mut payment_intent = PaymentIntentNew::default();
    payment_intent.created_at = Some(time::PrimitiveDateTime::MAX);
    payment_intent.modified_at = Some(time::PrimitiveDateTime::MAX);
    payment_intent.payment_id = ident.clone();
    payment_intent.merchant_id = "foo".to_owned();
    let _ = generic_insert::<_ , PaymentIntentNew, PaymentIntent>(conn, payment_intent).expect("intent payment failed");

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
pub fn create_trackers() -> Result<(), ()>{
    Ok(())
}
 
// #[derive(Debug, Queryable, Insertable,Selectable)]
// #[diesel(check_for_backend(Pg))]
// #[diesel(table_name = users)]
// pub struct User{
//     pub id :"foo".to_owned(),
//     pub job_id : i64,
//     #[diesel(deserialize_as = serde_json::Value)]
//     pub de : ArrJson,
//     // pub de : ArrJson<serde_json::Value>//Vec<serde_json::Value>
//     // //pub de : Vec<serde_json::Value>
// }

// pub struct UserNew{
//     pub id :"foo".to_owned(),
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