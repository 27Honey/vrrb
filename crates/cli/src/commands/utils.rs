use std::path::PathBuf;

use crate::{commands::node::RunOpts, result::CliError};

pub fn read_node_config_from_file(config_file_path: PathBuf) -> crate::result::Result<RunOpts> {
    let path_str = config_file_path.to_str().unwrap_or_default();

    let node_config = RunOpts::from_file(path_str)
        .map_err(|err| CliError::Other(format!("failed to read config file: {err}")))?;

    Ok(node_config)
}

// TODO: fix state I/O && test writing txns to state
