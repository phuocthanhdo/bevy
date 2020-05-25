mod asset_server;
mod assets;
#[cfg(feature = "filesystem_watcher")]
pub mod filesystem_watcher;
mod handle;
mod load_request;
mod loader;

pub use asset_server::*;
pub use assets::*;
pub use handle::*;
pub use load_request::*;
pub use loader::*;

use bevy_app::{AppBuilder, AppPlugin};
use bevy_component_registry::RegisterComponent;
use legion::prelude::IntoSystem;

pub mod stage {
    pub const LOAD_ASSETS: &str = "load_assets";
    pub const ASSET_EVENTS: &str = "asset_events";
}

#[derive(Default)]
pub struct AssetPlugin;

impl AppPlugin for AssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before(bevy_app::stage::PRE_UPDATE, stage::LOAD_ASSETS)
            .add_stage_after(bevy_app::stage::POST_UPDATE, stage::ASSET_EVENTS)
            .init_resource::<AssetServer>()
            .register_property_type::<HandleId>();
        #[cfg(feature = "filesystem_watcher")]
        app.add_system_to_stage(
            stage::LOAD_ASSETS,
            AssetServer::filesystem_watcher_system.system(),
        );
    }
}
