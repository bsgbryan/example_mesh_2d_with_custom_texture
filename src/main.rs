use std::f32::consts::PI;

use bevy::{
  DefaultPlugins,
  app::{
    App,
    Startup,
  },
  asset::{
    Assets,
    RenderAssetUsages,
  },
  core_pipeline::core_2d::Camera2d,
  ecs::system::{
    Commands,
    ResMut,
  },
  image::Image,
  math::{
    UVec3,
    Vec2,
    ops,
    // primitives::Circle,
  },
  render::{
    mesh::{
      Indices,
      Mesh,
      Mesh2d,
      PrimitiveTopology,
    },
    render_resource::{
      Extent3d,
      TextureDimension,
      TextureFormat,
    },
  },
  sprite::{
    ColorMaterial,
    MeshMaterial2d,
  },
  transform::components::Transform,
};

use noise::{
  NoiseFn,
  Simplex,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, startup)
    .run();
}

fn startup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let mut shape = Mesh::new(
    PrimitiveTopology::TriangleList,
    RenderAssetUsages::RENDER_WORLD,
  );

  let mut uv_positions = vec![[0.5, 0.5]];
  let mut vertex_positions = vec![[0.0, 0.0, 0.0]];
  let segments = rand::random_range(9..18);

  for i in 0..segments {
    let a = i as f32 * PI / (segments as f32 * 0.5);

    let uv = Vec2::from_angle(-a).mul_add(Vec2::splat(0.5), Vec2::splat(0.5));
    uv_positions.push([uv.x, uv.y]);

    let min = rand::random_range(0.8..0.925);
    let max = rand::random_range(1.025..1.2);
    let j = rand::random_range(min..max);
    let r = 50.0 * j;

    vertex_positions.push([r * ops::sin(a), r * ops::cos(a), 0.0]);
  }

  shape.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
  shape.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_positions);

  let mut indices = vec![0, 1, segments];
  for i in 2..=segments {
    indices.extend_from_slice(&[0, i, i - 1]);
  }
  shape.insert_indices(Indices::U32(indices));

  commands.spawn((
    Mesh2d(meshes.add(shape)),
    // Mesh2d(meshes.add(Circle::new(50.0))),
    MeshMaterial2d(
      materials.add(
        ColorMaterial {
          texture: Some(images.add(
            texture(
              50.0,
              &Simplex::new(rand::random_range(0..u32::MAX)),
            )
          )),
          ..Default::default()
        }
      )
    ),
    Transform::from_xyz(0.0, 0.0, 0.0),
  ));

  commands.spawn(Camera2d);
}

fn texture(
  size:   f32,
  noise: &Simplex,
) -> Image {
  let magnitude = rand::random_range(100.25..100.75);

  generate(
    size,
    |x: u32, y: u32| -> [u8; 4] {
      let nx = ((x as f32 / size) * magnitude) as f64;
      let ny = ((y as f32 / size) * magnitude) as f64;
      let val = (((noise.get([nx, ny, 10.14]) + 1.0) * 0.5) * 255.0).round() as u8;

      [val, val, val, val]
    }
  )
}

pub fn generate<F: Fn(u32, u32) -> [u8; 4]>(
  size: f32,
  callback: F,
) -> Image {
  let h = size.round() as u32;
  let w = size.round()  as u32;

  let mut image = Image::new_fill(
    Extent3d {
      height: h,
      width:  w,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[0, 0, 0, 255],
    TextureFormat::Rgba8UnormSrgb,
    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
  );

  for x in 0..w {
    for y in 0..h {
      if let Some(pixel_bytes) = image.pixel_bytes_mut(UVec3::new(x, y, 0)) {
        let processed = callback(x, y);

        pixel_bytes[0] = processed[0];
        pixel_bytes[1] = processed[1];
        pixel_bytes[2] = processed[2];
        pixel_bytes[3] = processed[3];
      }
    }
  }

  image
}
