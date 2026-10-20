/*
*	Copyright (C) 2026 Kendall Tauser
*
*	This program is free software; you can redistribute it and/or modify
*	it under the terms of the GNU General Public License as published by
*	the Free Software Foundation; either version 2 of the License, or
*	(at your option) any later version.
*
*	This program is distributed in the hope that it will be useful,
*	but WITHOUT ANY WARRANTY; without even the implied warranty of
*	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*	GNU General Public License for more details.
*
*	You should have received a copy of the GNU General Public License along
*	with this program; if not, write to the Free Software Foundation, Inc.,
*	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use std::fs::read_to_string;

use crate::{config::FibServiceConfig, errors::FibServiceError};

#[derive(clap::Parser, Debug)]
pub(super) struct FibServiceArgs {
    /// Location where config file lives. Currently, only YAML inputs are supported.
    #[arg(short, long, env = "FIB_SERVICE_CONFIG_LOCATION")]
    pub(super) config_location: String,
}

impl FibServiceArgs {
    pub(super) fn get_config(&self) -> Result<FibServiceConfig, FibServiceError> {
        match self.config_location.is_empty() {
            true => Ok(FibServiceConfig::default()),
            false => Ok(yaml_serde::from_str(&read_to_string(
                &self.config_location,
            )?)?),
        }
    }
}
