use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct ScreenBoundary;

#[derive(Component)]
pub struct LandingPad {
    pub position: Vec2,
    pub width: f32,
}

#[derive(Component)]
pub struct LandingZone {
    pub x_min: f32,
    pub x_max: f32,
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_environment)
           .add_systems(Update, keep_rocket_on_screen);
    }
}

fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Sol vert foncé
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::Box::new(2000.0, 20.0, 0.0).into())),
            material: materials.add(ColorMaterial::from(Color::rgb(0.1, 0.5, 0.1))),
            transform: Transform::from_xyz(0.0, -350.0, 0.0),
            ..default()
        },
        Ground,
    ));

    // ZONE D'ATTERRISSAGE ÉLARGIE (80 pixels de large au lieu de 60)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::Box::new(80.0, 10.0, 0.0).into())), // 80 au lieu de 60
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 0.0))), // JAUNE VIF
            transform: Transform::from_xyz(0.0, -340.0, 1.0),
            ..default()
        },
        LandingZone {
            x_min: -40.0, // ÉLARGI
            x_max: 40.0,  // ÉLARGI
        },
    ));

    // Plateforme grise (visuelle seulement)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::Box::new(100.0, 8.0, 0.0).into())), // Légèrement plus large
            material: materials.add(ColorMaterial::from(Color::rgb(0.4, 0.4, 0.4))),
            transform: Transform::from_xyz(0.0, -340.0, 0.5),
            ..default()
        },
        LandingPad {
            position: Vec2::new(0.0, -340.0),
            width: 100.0,
        },
    ));

    create_screen_boundaries(&mut commands, &mut meshes, &mut materials);
}

fn create_screen_boundaries(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let screen_width = 1200.0;
    let screen_height = 800.0;
    let boundary_thickness = 50.0;

    // Limites invisibles autour de l'écran
    let boundaries = [
        (-screen_width / 2.0 - boundary_thickness / 2.0, 0.0, boundary_thickness, screen_height * 2.0), // Gauche
        (screen_width / 2.0 + boundary_thickness / 2.0, 0.0, boundary_thickness, screen_height * 2.0), // Droite
        (0.0, screen_height / 2.0 + boundary_thickness / 2.0, screen_width * 2.0, boundary_thickness), // Haut
    ];

    for (x, y, width, height) in boundaries {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Box::new(width, height, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            ScreenBoundary,
        ));
    }
}

fn keep_rocket_on_screen(
    mut rocket_query: Query<&mut Transform, (With<crate::game::rocket::RocketMainBody>, Without<ScreenBoundary>)>,
) {
    if let Ok(mut transform) = rocket_query.get_single_mut() {
        let screen_width = 1200.0;
        let screen_height = 800.0;
        
        // Empêche de sortir des côtés
        transform.translation.x = transform.translation.x.clamp(-screen_width / 2.0 + 40.0, screen_width / 2.0 - 40.0);
        
        // Empêche de sortir par le haut
        transform.translation.y = transform.translation.y.clamp(-screen_height, screen_height / 2.0 - 50.0);
    }
}