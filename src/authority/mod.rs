/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*!
  The catalog module has the types necessary for storing and caching authoritative records.
 */
use ::op::ResponseCode;

pub type UpdateResult<T> = Result<T, ResponseCode>;

#[derive(RustcDecodable, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ZoneType { Master, Slave, Hint, Forward }

mod authority;
mod catalog;

pub use self::authority::Authority;
pub use self::catalog::Catalog;

#[cfg(test)]
pub use self::authority::authority_tests;