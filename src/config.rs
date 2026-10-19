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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct FibServiceConfig {
    /// Host to listen on.
    #[serde(default)]
    pub(super) listen_host: String,

    /// Port to listen on.
    #[serde(default)]
    pub(super) listen_port: u16,

    /// The supported max fibonnaci number for this service. Tradeoff
    pub(super) max_supported_number: u32,
}

impl Default for FibServiceConfig {
    fn default() -> Self {
        Self {
            listen_host: "0.0.0.0".into(),
            listen_port: 8080,
            max_supported_number: 32768,
        }
    }
}
