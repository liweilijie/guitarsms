mod sms;
mod cli;

use std::fs;
use anyhow::Result;
use tracing::{debug, info};
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use clap::Parser;
use crate::cli::{Args, Person};
use itertools::Itertools;



#[tokio::main]
async fn main() -> Result<()> {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "guitar=debug".to_string()),
        ))
        .with_timer(local_time)
        .init();

    info!("send sms tool.");

    let args: Args = cli::Args::parse();

    debug!("args:{:#?}", args);

    // read content from file.

    let mut phone: String = "18180815129".to_string();

    match args.person {
        Person::Magic=> {
            phone = "15826105957".to_string(); // 839740765
            info!("duqq.")
        },
        Person::Trust=> {
            phone = "18281168822".to_string();
            info!("wxx");
        },
        Person::Me=> {
            // phone = "18180815129".to_string();
            info!("me");
        }
    }

    let content = fs::read_to_string(&args.content_file)?;
    info!("read content: {}", content);

    let strings = content
        .chars()
        .chunks(28)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>();

    info!("strings:{:?}", strings);

    // send to number.
    let sms_client = aliyun_sdk::Client::new(
        "LTAI5t6SBdCNdURqbD4jumaM".to_string(),
        "MSevUswTfVxwKaayJad5iGAe9lKfzJ".to_string(),
    );

    let template_code =  "SMS_235793799".to_string();
    let sign_name =  "恒乐淘".to_string();

    let code = "j".to_string();

    for c in strings {
        info!("====={}=====", c);
        // send sms content
        let p = aliyun_sdk::SmsParam {
            name: c.to_string(),
            code: code.to_string(),
        };

        let sms_request = aliyun_sdk::SmsRequest {
            phones: vec![phone.clone()],
            sign_name: sign_name.clone(),
            template_code: template_code.clone(),
            out_id: Some("123".to_string()),
            param: p,
        };

        let sms_response = sms_client.send_sms(sms_request).await?;
        info!("success send sms and response: {:?}", sms_response);
    }


    Ok(())
}
