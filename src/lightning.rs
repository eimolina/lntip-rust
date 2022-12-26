use dotenv::dotenv;
use std::env;
use tonic_openssl_lnd::lnrpc::{AddInvoiceResponse, Invoice, PaymentHash, ChannelBalanceRequest, ChannelBalanceResponse, ListInvoiceRequest, ListInvoiceResponse};
use tonic_openssl_lnd::{LndClientError, LndLightningClient};

pub async fn connect() -> Result<LndLightningClient, LndClientError> {
    dotenv().ok();
    let port: u32 = env::var("LND_GRPC_PORT")
        .expect("LND_GRPC_PORT must be set")
        .parse()
        .expect("port is not u32");
    let host = env::var("LND_GRPC_HOST").expect("LND_GRPC_HOST must be set");
    let cert = env::var("LND_CERT_FILE").expect("LND_CERT_FILE must be set");
    let macaroon = env::var("LND_MACAROON_FILE").expect("LND_MACAROON_FILE must be set");
    // Connecting to LND requires only host, port, cert file, and macaroon file
    let client = tonic_openssl_lnd::connect_lightning(host, port, cert, macaroon)
        .await
        .expect("Failed connecting to LND");
    Ok(client)
}

pub async fn create_invoice(
    description: &str,
    amount: u32,
) -> Result<AddInvoiceResponse, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoice = Invoice {
        memo: description.to_string(),
        value: amount as i64,
        ..Default::default()
    };
    let invoice = client.add_invoice(invoice).await?.into_inner();
    Ok(invoice)
}

pub async fn get_list_invoice() -> Result<ListInvoiceResponse, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoices =client.list_invoices(ListInvoiceRequest{
        index_offset:0,
        num_max_invoices: 100,
        pending_only:false,
        reversed:false
    }).await?.into_inner();
    Ok(invoices)
}

pub async fn get_invoice(hash: &[u8]) -> Result<Invoice, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoice = client
        .lookup_invoice(PaymentHash {
            r_hash: hash.to_vec(),
            ..Default::default()
        })
        .await?
        .into_inner();
    Ok(invoice)
}

pub async fn get_balance() -> Result<ChannelBalanceResponse, LndClientError> {
    let mut client = connect().await.unwrap();
    let balance = client.channel_balance(ChannelBalanceRequest {}).await?.into_inner();
    Ok(balance)
}

pub async fn make_a_payment(
    invoice_payment_request: String
) -> Result<tonic_openssl_lnd::lnrpc::SendResponse, String> {
    let mut lightning_client = connect().await.unwrap();
    let send_response = lightning_client
        .send_payment_sync(tonic_openssl_lnd::lnrpc::SendRequest {
            payment_request: invoice_payment_request,
            ..Default::default()
        })
        .await
        .map_err(|e| format!("Error al enviar el pago: {:?}", e.message()))?
        .into_inner();
    if send_response.payment_preimage.is_empty() {
        return Err(format!(
            "Error de envío de pago: {:?}.",
            send_response.payment_error
        ));
    }
    Ok(send_response)
}