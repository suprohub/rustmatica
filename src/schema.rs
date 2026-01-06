use fastnbt::{IntArray, LongArray};
use mcdata::{util::BlockPos, GenericBlockEntity, GenericBlockState, GenericEntity};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

type CowStr = std::borrow::Cow<'static, str>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Litematic<
    'a,
    BlockState = GenericBlockState<'a>,
    Entity = GenericEntity<'a>,
    BlockEntity = GenericBlockEntity<'a>,
> where
    BlockState: mcdata::BlockState,
    Entity: mcdata::Entity,
    BlockEntity: mcdata::BlockEntity,
{
    #[serde(flatten)]
    pub regions: LitematicRegions<'a, BlockState, Entity, BlockEntity>,
    #[serde(flatten)]
    pub metadata: LitematicMetadata<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LitematicRegions<
    'a,
    BlockState = GenericBlockState<'a>,
    Entity = GenericEntity<'a>,
    BlockEntity = GenericBlockEntity<'a>,
> where
    BlockState: mcdata::BlockState,
    Entity: mcdata::Entity,
    BlockEntity: mcdata::BlockEntity,
{
    pub regions: HashMap<String, Region<'a, BlockState, Entity, BlockEntity>>,
    pub p: PhantomData<&'a ()>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LitematicMetadata<'a> {
    pub minecraft_data_version: i32,
    pub version: i32,
    pub sub_version: Option<i32>,
    pub metadata: Metadata<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Region<
    'a,
    BlockState = GenericBlockState<'a>,
    Entity = GenericEntity<'a>,
    BlockEntity = GenericBlockEntity<'a>,
> where
    BlockState: mcdata::BlockState,
    Entity: mcdata::Entity,
    BlockEntity: mcdata::BlockEntity,
{
    pub position: BlockPos,
    pub size: BlockPos,
    pub block_state_palette: Vec<BlockState>,
    pub block_states: LongArray,
    pub tile_entities: Vec<BlockEntity>,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pending_block_ticks: Vec<PendingBlockTick>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pending_fluid_ticks: Vec<PendingFluidTick>,
    pub p: PhantomData<&'a ()>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata<'a> {
    pub name: Cow<'a, str>,
    pub author: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub region_count: i32,
    pub total_volume: i32,
    pub total_blocks: i64,
    pub time_created: i64,
    pub time_modified: i64,
    pub enclosing_size: BlockPos,
    pub preview_image_data: Option<IntArray>,
}

/// A pending block tick.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct PendingBlockTick {
    pub block: CowStr,
    pub priority: i32,
    pub sub_tick: i64,
    pub time: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}

/// A pending fluid tick.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct PendingFluidTick {
    pub fluid: CowStr,
    pub priority: i32,
    pub sub_tick: i64,
    pub time: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}
