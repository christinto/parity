// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#![allow(dead_code)]

extern crate ethcore_ipc as ipc;
extern crate ethcore_devtools as devtools;
extern crate semver;
extern crate nanomsg;
extern crate ethcore_ipc_nano as nanoipc;
extern crate ethcore_util as util;
extern crate ethcore_bytes as bytes;
#[macro_use] extern crate log;

pub mod service;
mod examples;
mod over_nano;
mod nested;
mod binary;
mod with_attrs;
