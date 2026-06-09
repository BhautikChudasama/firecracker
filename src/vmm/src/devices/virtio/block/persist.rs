// Copyright 2023 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::vhost_user::persist::VhostUserBlockState;
use super::virtio::persist::VirtioBlockState;
use crate::devices::virtio::transport::VirtioInterrupt;
use crate::vstate::memory::GuestMemoryMmap;

/// Block device state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockState {
    Virtio(VirtioBlockState),
    VhostUser(VhostUserBlockState),
}

impl BlockState {
    pub fn is_activated(&self) -> bool {
        match self {
            BlockState::Virtio(virtio_block_state) => virtio_block_state.virtio_state.activated,
            BlockState::VhostUser(vhost_user_block_state) => false,
        }
    }
}

/// Auxiliary structure for creating a device when resuming from a snapshot.
///
/// `block_path_overrides` is an optional drive_id → new backing-file
/// path map supplied at `PUT /snapshot/load` time. When a drive being
/// restored has an entry here, its `disk_path` from the snapshot is
/// ignored and the override is opened instead. Lets the caller relocate
/// a drive without rewriting the binary vmstate — useful for snapshot
/// clones (fork), cross-host migrations whose drive paths don't exist
/// on the destination, and any scenario where the snapshot was taken at
/// path A but the live file lives at path B. When no override is
/// present, behaviour is unchanged: the snapshot's serialized path is
/// used verbatim.
#[derive(Debug, Default)]
pub struct BlockConstructorArgs {
    pub mem: GuestMemoryMmap,
    pub block_path_overrides: HashMap<String, String>,
}
