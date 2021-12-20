use bevy::prelude::*;

pub fn create_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1.0, 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0.0, 0.2, 0.2).into());

    spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 0.));
    spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 1.));
    spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 2.));
    spawn_queen(commands, white_material.clone(), queen_handle.clone(), Vec3::new(0., 0., 3.));
    spawn_king(commands, white_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(0., 0., 4.));
    spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0., 0., 5.));
    spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0., 0., 6.));
    spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0., 0., 7.));
    for i in 0..8 {
        spawn_pawn(commands, white_material.clone(), pawn_handle.clone(), Vec3::new(1.0, 0.0, i as f32));
    }

    spawn_rook( commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 0.),);
    spawn_knight( commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 1.),);
    spawn_bishop( commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 2.),);
    spawn_queen( commands, black_material.clone(), queen_handle.clone(), Vec3::new(7., 0., 3.),);
    spawn_king( commands, black_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(7., 0., 4.),);
    spawn_bishop( commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7., 0., 5.),);
    spawn_knight( commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7., 0., 6.),);
    spawn_rook( commands, black_material.clone(), rook_handle.clone(), Vec3::new(7., 0., 7.),);
    for i in 0..8 {
        spawn_pawn( commands, black_material.clone(), pawn_handle.clone(), Vec3::new(6., 0., i as f32),);
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_cross,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3
) {
    commands.spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(PbrBundle{
            mesh: mesh_1,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
        parent.spawn(PbrBundle {
            mesh: mesh_2,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
    });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}