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

use std::{fmt::Display, io};

pub(crate) enum FibServiceError {
    IOError(io::Error),
    SerdeYamlError(yaml_serde::Error),
}

impl Display for FibServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FibServiceError::IOError(error) => write!(f, "io: {error}"),
            FibServiceError::SerdeYamlError(error) => write!(f, "yaml: {error}"),
        }
    }
}

impl From<io::Error> for FibServiceError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<yaml_serde::Error> for FibServiceError {
    fn from(value: yaml_serde::Error) -> Self {
        Self::SerdeYamlError(value)
    }
}
