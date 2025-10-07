use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use teloxide::prelude::*;
use reqwest::header::{AUTHORIZATION, HeaderValue};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use dashmap::DashMap;

#[derive(Debug, Clone)]
enum UserState {
    Idle,
    WaitingReference,
    ReferenceNotFound,
}

#[derive(Serialize)]
struct LinkRequest {
    reference: String,
    amount: u64,
    currency: String,
    payment_method: String,
    description: String,
    redirect_url: String,
    ipn_url: String,
    customer_data: CustomerData,
}

#[derive(Serialize)]
struct CustomerData {
    legal_doc: String,
    legal_doc_type: String,
    phone_code: String,
    phone_number: String,
    email: String,
    full_name: String,
}

#[derive(Deserialize)]
struct LinkResponse {
    code: String,
    status: String,
    message: String,
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    ticket: String,
    date: String,
    payment_url: String,
    transaction: Transaction,
}

#[derive(Deserialize, Debug)]
struct Transaction {
    reference: String,
    amount: u32,
    currency: String,
    payment_method: String,
    redirect_url: String,
    ipn_url: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting tg-paylink-bot");

    dotenvy::dotenv().ok();
    let bot = Bot::from_env();

    let sessions = Arc::new(DashMap::<i64, UserState>::new());

    Dispatcher::builder(
        bot.clone(),
        Update::filter_message().endpoint({
            let sessions = sessions.clone();
            move |bot: Bot, msg: Message| {
                let sessions = sessions.clone();
                async move {
                    let chat_id = msg.chat.id.0;
                    let text = msg.text().unwrap_or("").trim().to_string();

                    match text.as_str() {
                        "/start" | "ayuda" => {
                            bot.send_message(msg.chat.id,
                                "🔗 Envíame: /pay para iniciar el proceso de pago"
                            ).await?;
                        }
                        "/pay" => {
                            sessions.insert(chat_id, UserState::WaitingReference);
                            bot.send_message(msg.chat.id, "🔗 Ingresa la referencia de pago:").await?;
                        }
                        _ => {
                            match sessions.get(&chat_id).map(|r| r.clone()).unwrap_or(UserState::Idle) {
                                UserState::WaitingReference => {
                                    let reference = text.clone();
                                    bot.send_message(msg.chat.id, format!("🔍 Buscando pago para referencia: {}", reference)).await?;
                                    
                                    // Simular verificación de referencia (aquí puedes conectar con tu base de datos)
                                    if reference.to_lowercase() == "abc" {
                                        // Referencia no existe
                                        bot.send_message(msg.chat.id, "❌ Referencia 'ABC' no encontrada en el sistema.\n\n🔗 Por favor, ingresa una referencia válida:").await?;
                                        sessions.insert(chat_id, UserState::ReferenceNotFound);
                                    } else {
                                        // Referencia válida, generar pago
                                        let amount = rand::thread_rng().gen_range(10000..100000);
                                        
                                        match create_pay_link(amount, &reference, chat_id).await {
                                            Ok(url) => {
                                                bot.send_message(msg.chat.id, format!("✅ Link de pago generado:\n💰 Monto: ${} COP\n🔗 Link: {}", amount, url)).await?;
                                            }
                                            Err(e) => {
                                                bot.send_message(msg.chat.id, format!("❌ Error al generar el link: {}", e)).await?;
                                            }
                                        }
                                        sessions.insert(chat_id, UserState::Idle);
                                    }
                                }
                                UserState::ReferenceNotFound => {
                                    let reference = text.clone();
                                    bot.send_message(msg.chat.id, format!("🔍 Verificando nueva referencia: {}", reference)).await?;
                                    
                                    // Verificar la nueva referencia
                                    if reference.to_lowercase() == "abc" {
                                        // Sigue siendo inválida
                                        bot.send_message(msg.chat.id, "❌ La referencia 'ABC' sigue siendo inválida.\n\n🔗 Por favor, ingresa una referencia diferente:").await?;
                                        // Mantener en el mismo estado
                                    } else {
                                        // Nueva referencia válida
                                        let amount = rand::thread_rng().gen_range(10000..100000);
                                        
                                        match create_pay_link(amount, &reference, chat_id).await {
                                            Ok(url) => {
                                                bot.send_message(msg.chat.id, format!("✅ ¡Perfecto! Link de pago generado:\n💰 Monto: ${} COP\n🔗 Link: {}", amount, url)).await?;
                                            }
                                            Err(e) => {
                                                bot.send_message(msg.chat.id, format!("❌ Error al generar el link: {}", e)).await?;
                                            }
                                        }
                                        sessions.insert(chat_id, UserState::Idle);
                                    }
                                }
                                UserState::Idle => {
                                    bot.send_message(msg.chat.id, "Usa /pay para iniciar el proceso de pago.").await?;
                                }
                            }
                        }
                    }

                    Ok::<(), anyhow::Error>(())
                }
            }
        }),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}

async fn create_pay_link(amount: u64, _reference: &str, _chat_id: i64) -> Result<String> {
    let api_url = env::var("GATEWAY_API_URL")?;
    let user = env::var("GATEWAY_USER")?;
    let password = env::var("GATEWAY_PASSWORD")?;
    let token = env::var("GATEWAY_TOKEN")?;

    let req = LinkRequest {
        reference: String::from(format!("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ{}", rand::thread_rng().gen_range(1..1000000))),
        amount: amount,
        currency: String::from("COP"),
        payment_method: String::from("ALL_METHODS"),
        description: String::from("Payment from telegram user"),
        redirect_url: String::from("https://google.com/"),
        ipn_url: String::from("https://google.com/"),
        customer_data: CustomerData {
            legal_doc: String::from("1102184491"),
            legal_doc_type: String::from("CC"),
            phone_code: String::from("57"),
            phone_number: String::from("3133243232"),
            email: String::from("John-Doe@test.com"),
            full_name: String::from("John Doe"),
        },
    };

    let credentials = format!("{}:{}", user, password);
    let encoded_credentials = general_purpose::STANDARD.encode(credentials);
    let auth_header_value = format!("Basic {}", encoded_credentials);
    let auth_header = HeaderValue::from_str(&auth_header_value)?;
    
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/payin", api_url))
        .header("Token-Top", token)
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, auth_header)
        .json(&req)
        .send()
        .await?
        .error_for_status()?;

    let data: LinkResponse = res.json().await?;
    println!("API response body: {:?}", data.data);

    Ok(data.data.payment_url)
}
