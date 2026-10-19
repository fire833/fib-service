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

use std::process::exit;

use clap::Parser;

use crate::cli::FibServiceArgs;

mod api;
mod cli;
mod config;
mod errors;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = FibServiceArgs::parse();
    let conf = match args.get_config() {
        Ok(conf) => conf,
        Err(e) => {
            println!("failed to load config: {e}");
            exit(1);
        }
    };

    api::start_server(conf).await;
}
