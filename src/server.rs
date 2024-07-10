use axum::routing::{get};
use axum::extract::{State, Path};
use crate::ds::{create_payment_flow, fetch_payments_flow};
use crate::pool::Pool;

pub async fn mk_server(pool : Pool) -> anyhow::Result<axum::Router>{
    Ok(axum::Router::new()
        .route("/payment_init/:payment_id", get(start_payment))
        .route("/show_payment/:payment_id", get(show_payment))
        .route("/update_payment/:payment_id", get(update_payment))
        .with_state(pool))
}


async fn start_payment(State(state) : State<Pool>, Path(payment_id) : Path<String>)
    -> Result<axum::Json<()>, ()>
{   
    //println!("start_payment");
    state.execute(|c| create_payment_flow(c,payment_id)).await;
    Ok(axum::Json(()))
}

async fn show_payment(State(state) : State<Pool>, Path(payment_id) : Path<String>)
    -> Result<axum::Json<()>, ()>
{
    state.execute(|c| fetch_payments_flow(c, payment_id)).await;
    Ok(axum::Json(()))
}

async fn update_payment(State(state) : State<Pool> , Path(payment_id) : Path<String>)
    -> Result<axum::Json<()>, ()>
{
    Ok(axum::Json(()))
}