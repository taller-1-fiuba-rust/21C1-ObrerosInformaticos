use crate::protocol::response::ResponseBuilder;
use crate::protocol::types::ProtocolType;
use crate::storage::data_storage::DataStorage;
use std::sync::Arc;

pub fn run(
    builder: &mut ResponseBuilder,
    arguments: Vec<ProtocolType>,
    data: Arc<DataStorage>,
) -> Result<(), &'static str> {
    if arguments.len() < 2 {
        return Err("ERR wrong number of arguments for 'scard' command");
    }
    let key = arguments[0].clone().string()?;

    let string_arguments: Vec<String> = arguments
        .into_iter()
        .map(|x| x.string())
        .collect::<Result<_, _>>()?;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::ProtocolType;
    use crate::storage::data_storage::DataStorage;
    use crate::storage::data_storage::Value;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn scard() {

    }
}
