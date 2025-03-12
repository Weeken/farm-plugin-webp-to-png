#![deny(clippy::all)]

use farmfe_core::{
  config::{Config, Mode},
  plugin::Plugin,
  resource::{Resource, ResourceType},
};

use farmfe_macro_plugin::farm_plugin;

mod utils;
use crate::utils::{compress_png, convert_webp_to_png, get_png_bitmap, insert_resource};

#[farm_plugin]
pub struct FarmPluginWebpToPng {}

impl FarmPluginWebpToPng {
  fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmPluginWebpToPng {
  fn name(&self) -> &str {
    "FarmPluginWebpToPng"
  }

  fn render_resource_pot(
    &self,
    _param: &farmfe_core::plugin::PluginRenderResourcePotHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginRenderResourcePotHookResult>>
  {
    if matches!(_context.config.mode, Mode::Production) {
      if _param.content.contains(".webp") {
        return Ok(Some(
          farmfe_core::plugin::PluginRenderResourcePotHookResult {
            content: _param.content.replace(".webp", ".png"),
            source_map: None,
          },
        ));
      }
    }
    Ok(None)
  }

  fn finalize_resources(
    &self,
    _param: &mut farmfe_core::plugin::PluginFinalizeResourcesHookParams,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    if matches!(_context.config.mode, Mode::Production) {
      let mut resource_map_clone = _param.resources_map.clone();
      for (name, resource) in resource_map_clone.iter_mut() {
        if name.ends_with(".webp") {
          // println!("name: {}", name);
          // println!("resource: {:?}", resource);
          let png_name = name.replace(".webp", ".png");

          let png_bytes = convert_webp_to_png(&resource.bytes.as_slice(), name.clone());

          let png_resource = Resource {
            name: png_name.clone(),
            bytes: png_bytes,
            emitted: false,
            resource_type: ResourceType::Asset(".png".to_string()),
            origin: resource.origin.clone(),
            info: None,
          };
          insert_resource(_param.resources_map, png_name.clone(), png_resource);
          _param.resources_map.remove(name);
        }
        if name.ends_with(".png") {
          let png_bitmap = get_png_bitmap(resource.bytes.as_slice());
          let png_bytes = compress_png(png_bitmap.buffer, png_bitmap.width, png_bitmap.height);
          let png_resource = Resource {
            name: name.clone(),
            bytes: png_bytes,
            emitted: false,
            resource_type: ResourceType::Asset(".png".to_string()),
            origin: resource.origin.clone(),
            info: None,
          };
          _param.resources_map.remove(name);
          insert_resource(_param.resources_map, name.clone(), png_resource);
        }
      }
    }

    Ok(None)
  }
}
