// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use snarkvm::prelude::{
    narwhal::{Transmission, TransmissionID},
    Field,
    Network,
    Result,
};

pub trait LedgerService<N: Network>: Send + Sync {
    /// Returns `true` if the ledger contains the given certificate ID.
    fn contains_certificate(&self, certificate_id: &Field<N>) -> Result<bool>;

    /// Returns `true` if the ledger contains the given transmission ID.
    fn contains_transmission(&self, transmission_id: &TransmissionID<N>) -> Result<bool>;

    /// Returns the transmission for the given transmission ID, if it exists.
    fn get_transmission(&self, transmission_id: &TransmissionID<N>) -> Result<Option<Transmission<N>>>;
}