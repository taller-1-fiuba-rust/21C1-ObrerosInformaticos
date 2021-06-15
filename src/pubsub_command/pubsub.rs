use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::pubsub::PublisherSubscriber;


use std::sync::{Arc, Mutex, MutexGuard};
use regex::Regex;

pub fn run(
    pubsub: Arc<Mutex<PublisherSubscriber>>,
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
) -> Result<(), &'static str> {

    let mut locked_pubsub = pubsub.lock().ok().ok_or("Failed to lock")?;
    let subcommand = arguments[0].clone().string()?;

    match &subcommand.to_lowercase()[..] {
        "numsub" => numsub(&mut locked_pubsub, arguments[1..]
                               .iter()
                               .map(|x| x.clone().string().unwrap())
                               .collect::<Vec<String>>(), builder),
        "channels" => channels(&mut locked_pubsub, if arguments.len() == 2 { arguments[1].clone().string()? } else { "*".to_string() }, builder)?,
        _ => { return Err("Unknown subcommand"); }
    }

    Ok(())
}

fn numsub(pubsub: &mut MutexGuard<PublisherSubscriber>, channels: Vec<String>, builder: &mut ResponseBuilder) {
    let mut arr = Vec::new();
    for channel in channels {
        arr.push(ProtocolType::String(channel.clone()));
        arr.push(ProtocolType::Integer(pubsub.subscriber_count(&channel) as i64));
    }
    builder.add(ProtocolType::Array(arr));
}

fn channels(pubsub: &mut MutexGuard<PublisherSubscriber>, pattern_str: String, builder: &mut ResponseBuilder) -> Result<(), &'static str> {
    let pattern = format!("^{}$", pattern_str.replace("?", "."));
    let re = Regex::new(&pattern).ok().ok_or("Error parsing the regex")?;
    let channels = pubsub.get_channels();

    builder.add(ProtocolType::Array(
        channels
            .into_iter()
            .filter(move |x| re.is_match(x))
            .map(ProtocolType::String)
            .collect(),
    ));
    Ok(())
}

