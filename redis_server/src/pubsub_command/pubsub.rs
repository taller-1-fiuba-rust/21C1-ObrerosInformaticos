use redis_protocol::response::ResponseBuilder;
use redis_protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;
use regex::Regex;
use std::sync::Arc;

/// The PUBSUB command is an introspection command that allows to inspect the state of the Pub/Sub subsystem.
pub fn run(
    pubsub: Arc<PublisherSubscriber>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {
    let subcommand = arguments[0].clone().string()?;

    match &subcommand.to_lowercase()[..] {
        "numsub" => numsub(
            pubsub,
            arguments[1..]
                .iter()
                .map(|x| x.clone().string().unwrap())
                .collect::<Vec<String>>(),
            builder,
        )?,
        "channels" => channels(
            pubsub,
            if arguments.len() == 2 {
                arguments[1].clone().string()?
            } else {
                "*".to_string()
            },
            builder,
        )?,
        _ => {
            return Err("Unknown subcommand");
        }
    }

    Ok(())
}

fn numsub(
    pubsub: Arc<PublisherSubscriber>,
    channels: Vec<String>,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    let mut arr = Vec::new();
    for channel in channels {
        arr.push(ProtocolType::String(channel.clone()));
        arr.push(ProtocolType::Integer(
            (pubsub.subscriber_count(&channel)?) as i64,
        ));
    }
    builder.add(ProtocolType::Array(arr));
    Ok(())
}

fn channels(
    pubsub: Arc<PublisherSubscriber>,
    pattern_str: String,
    builder: &mut ResponseBuilder,
) -> Result<(), &'static str> {
    let pattern = format!("^{}$", pattern_str.replace("?", "."));
    let re = Regex::new(&pattern).ok().ok_or("Error parsing the regex")?;
    let channels = pubsub.get_channels()?;

    builder.add(ProtocolType::Array(
        channels
            .into_iter()
            .filter(move |x| re.is_match(x))
            .map(ProtocolType::String)
            .collect(),
    ));
    Ok(())
}
