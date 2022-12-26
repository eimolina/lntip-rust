use crate::lightning;
use hex::FromHex;
use rocket::serde::{json::Json, Serialize};
use tonic_openssl_lnd::lnrpc::{Amount,invoice::InvoiceState};

#[derive(Serialize, Default)]
pub struct InvoiceResponse {
    value: i64,
    payment_request: String,
    hash: String,
    paid: bool,
    preimage: String,
    memo: String,
    creation_date: i64,
    settle_date: i64,
    state: i32,
    settled: bool,
    settlet_index: u64,
    settle_time: i64,
    amt_paid_msat: i64
}

#[derive(Serialize, Default)]
pub struct WalletBalance {
    pub balance: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
}

#[derive(Serialize, Default)]
pub struct ListInvoices{
    invoices: Vec<InvoiceResponse>,
    last_index_offset: u64,
    first_index_offset: u64,
}

#[derive(Serialize, Default)]
pub struct PaymentResponse{
    success: bool,
    error: Option<String>
}

#[get("/hola/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hola, tienes {} años y te llamas {}!", age, name)
}

#[get("/create_invoice/<description>/<amount>")]
pub async fn create_invoice(description: &str, amount: u32) -> Json<InvoiceResponse> {
    let invoice = lightning::create_invoice(description, amount)
        .await
        .unwrap();

    let hash_str = invoice
        .r_hash
        .iter()
        .map(|h| format!("{h:02x}"))
        .collect::<Vec<String>>()
        .join("");

    Json(InvoiceResponse {
        payment_request: invoice.payment_request,
        hash: hash_str,
        ..Default::default()
    })
}

#[get("/get_wallet_balance")]
pub async fn get_wallet_balance() -> Json<WalletBalance> {
    let response = lightning::get_balance().await.unwrap();
    Json(WalletBalance {
        balance: response.local_balance.as_ref().unwrap_or(&Amount {sat: 0, msat: 0}).sat,
        local_balance: response.local_balance.as_ref().unwrap_or(&Amount {sat: 0, msat: 0}).sat,
        remote_balance: response.remote_balance.as_ref().unwrap_or(&Amount {sat: 0, msat: 0}).sat,
    })
}

#[get("/invoices")]
pub async fn lookup_invoices() -> Json<ListInvoices> {
    let data = lightning::get_list_invoice().await.unwrap();
    Json(ListInvoices{
        invoices: data.invoices.iter().map(|i| InvoiceResponse{
            value: i.value,
            memo: i.memo.clone(),
            creation_date: i.creation_date,
            settle_date: i.settle_date,
            state: i.state,
            settled: if i.state == InvoiceState::Settled as i32 {true} else {false},
            ..Default::default()
        }).collect(),
        first_index_offset: data.first_index_offset,
        last_index_offset: data.last_index_offset
    })
}

#[get("/invoices/<hash>")]
pub async fn lookup_invoice(hash: &str) -> Json<InvoiceResponse> {
    let hash = <[u8; 32]>::from_hex(hash).expect("Decoding failed");
    let invoice = lightning::get_invoice(&hash).await.unwrap();
    let mut preimage = String::new();
    let mut paid = false;
    if let Some(state) = InvoiceState::from_i32(invoice.state) {
        if state == InvoiceState::Settled {
            paid = true;
            preimage = invoice
                .r_preimage
                .iter()
                .map(|h| format!("{h:02x}"))
                .collect::<Vec<String>>()
                .join("");
        }
    }
    Json(InvoiceResponse {
        paid,
        preimage,
        payment_request: invoice.payment_request,
        value: invoice.value,
        memo: invoice.memo,
        settle_date: invoice.settle_date,
        state: invoice.state,
        ..Default::default()
    })
}

#[post("/payment", data = "<payment_request>")]
pub async fn payment(payment_request: String) -> Json<PaymentResponse> {
    match lightning::make_a_payment(payment_request).await {
        Ok(send_response) => Json(PaymentResponse{success: true, error: None }),
        Err(error) => Json(PaymentResponse { success: false, error: Some(error) }),
    }
}