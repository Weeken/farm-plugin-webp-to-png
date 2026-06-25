#![deny(clippy::all)]

use farmfe_core::{
  config::{Config, Mode},
  plugin::Plugin,
  resource::{Resource, ResourceType},
};

use farmfe_macro_plugin::farm_plugin;
use serde::Deserialize;

mod utils;
use crate::utils::{compress_png, compress_webp, convert_webp_to_png, get_png_bitmap, insert_resource};

/// 插件配置,从 farm.config.ts 的插件 options 传入(JSON 字符串)
/// - `is_convert`:是否将 webp 转换为 png。默认 false,表示只压缩已有的 png;true 时才会把 webp 转换为 png
/// - `quality`:webp 有损重编码质量 0-100,is_convert=false 时生效。默认 80
#[derive(Deserialize, Default)]
#[serde(default)]
struct PluginOptions {
  is_convert: bool,
  quality: Option<f32>,
}

impl PluginOptions {
  /// 未传 quality 或超出范围时回退到 80
  const DEFAULT_QUALITY: f32 = 80.0;
}

#[farm_plugin]
pub struct FarmPluginWebpToPng {
  /// 是否执行 webp -> png 的转换;false 时只做压缩
  is_convert: bool,
  /// webp 有损重编码质量 0-100,is_convert=false 时用于压缩 webp
  quality: f32,
}

impl FarmPluginWebpToPng {
  fn new(_config: &Config, options: String) -> Self {
    let opts = if options.is_empty() {
      PluginOptions::default()
    } else {
      serde_json::from_str::<PluginOptions>(&options).unwrap_or_default()
    };
    let quality = match opts.quality {
      Some(quality) if (0.0..=100.0).contains(&quality) => quality,
      _ => PluginOptions::DEFAULT_QUALITY,
    };
    Self {
      is_convert: opts.is_convert,
      quality,
    }
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
    if matches!(_context.config.mode, Mode::Production) && self.is_convert {
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
      let resource_map_clone = _param.resources_map.clone();
      for (name, resource) in resource_map_clone.iter() {
        if name.ends_with(".webp") {
          if self.is_convert {
            // 转换为 png:生成 .png 资源并移除原 .webp
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
          } else {
            // 只压缩:解码 webp 为 RGBA,用 libwebp 有损重编码为更小的 webp(保留 .webp 文件名)
            let compressed_bytes = compress_webp(&resource.bytes.as_slice(), name.clone(), self.quality);
            let compressed_resource = Resource {
              name: name.clone(),
              bytes: compressed_bytes,
              emitted: false,
              resource_type: resource.resource_type.clone(),
              origin: resource.origin.clone(),
              info: None,
            };
            _param.resources_map.remove(name);
            insert_resource(_param.resources_map, name.clone(), compressed_resource);
          }
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
