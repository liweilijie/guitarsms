mod cli;
mod sms;

use crate::cli::{Args, Person};
use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::fs;
use time::macros::format_description;
use time::UtcOffset;
use tracing::{debug, info};
use tracing_subscriber::fmt::time::OffsetTime;

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

    let mut phone: String = "13608025220".to_string();

    match args.person {
        Person::Magic => {
            phone = "15826105957".to_string();
            // 839740765  511304198809056228
            // (brother)杜川江(17794585581)
            // 1988.8.28(农历)
            info!("duqq.")
        }
        Person::Trust => {
            phone = "18281168822".to_string();
            // 1990.7.26(农历)(19-20点出生)
            // 513489537 nihao712
            // wx 账号Xj20225201210。  密码wxx521lj都是小写的
            // 讷河市双兴村长大
            // 四平市铁西区小串王烧烤店
            // 最重要的人：王振岩，于春华，肖蓓雅，肖虎妹
            // 王浚力：18644821004,13278146607
            info!("wxx");
        }
        Person::Me => {
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

    let template_code = "SMS_235793799".to_string();
    let sign_name = "恒乐淘".to_string();

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
