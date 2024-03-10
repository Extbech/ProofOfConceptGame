fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        });
    };
    // Load tile sprites.
    let tile1 = asset_server.load_with_settings("environment/backgrounddetailed1.png", settings);
    let tile2 = asset_server.load_with_settings("environment/backgrounddetailed2.png", settings);
    let tile3 = asset_server.load_with_settings("environment/backgrounddetailed3.png", settings);
    let tile4 = asset_server.load_with_settings("environment/backgrounddetailed6.png", settings);

    commands.spawn((
        Collider::cuboid(MAP_SIZE_HALF * 4.0, 0.1, MAP_SIZE_HALF * 4.0),
        // EXPLANATION: see docs/physics.txt
        CollisionGroups::new(
            Group::from_bits(COLLISION_WORLD).unwrap(), // part of world(1)
            Group::all(),                               // interacts with all
        ),
        MaterialMeshBundle {
            // mesh: meshes.add(shape::Plane::from_size(MAP_SIZE_HALF * 4.4).into()),
            mesh: meshes.add(shape::Plane::from_size(MAP_SIZE_HALF * 2.0 + 15.0).into()),
            // material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            material: materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(tile2),
                    ..default()
                },
                extension: GroundMaterial {
                    // scale: vec2(13.0, 0.3),
                    scale: 13.0,
                    color_texture: tile1,
                    noise_scale: 0.3,
                    // filler: Default::default(),
                    // filler2: Default::default(),
                    // color_texture: todo!(),
                },
            }),
            transform: Transform::from_translation(vec3(0.0, -0.05, 0.0)),
            ..default()
        },
    ));
}
