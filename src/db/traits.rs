// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyid::TinyId;

use crate::types::api::{Count, Filter, Ordering};

pub struct SingleValueArgs {
    pub filter: Filter,
    pub order: Ordering,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MultiValueArgs {
    pub filter: Filter,
    pub order: Ordering,
    pub count: Count,
}

pub trait DataType: std::fmt::Debug + std::fmt::Display + Clone {
    type CreateDto;
    type UpdateDto;
    type DeleteDto;

    fn id(&self) -> TinyId;
}

pub trait DataSource {
    type Data: DataType;
    fn get_by_id(&self, id: TinyId) -> Option<Self::Data>;
    /// TODO: This is essentially `find` with the filtering available from `args`.
    fn get_all(&self, args: &MultiValueArgs) -> Vec<Self::Data>;
    fn get_one(&self, args: &SingleValueArgs) -> Option<Self::Data>;
}

pub trait AsyncDataSource {}
