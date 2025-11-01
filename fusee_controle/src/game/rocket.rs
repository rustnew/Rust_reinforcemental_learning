use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;
use crate::game::physics::PhysicsBody;
use crate::game::GameState;

#[derive(Component)]
pub struct Rocket {
    pub fuel: f32,
    pub max_fuel: f32,
    pub throttle: f32,
    pub engine_power: f32,
    pub rotation_speed: f32,
    pub has_crashed: bool,
    pub has_landed: bool,
    pub size_factor: f32, // Facteur de taille pour l'apprentissage progressif
}

#[derive(Component)]
pub struct RocketMainBody;

#[derive(Component)]
pub struct RocketFlame;

#[derive(Component)]
pub struct LandingLegs {
    pub deployed: bool,
    pub contact: bool,
}

#[derive(Component)]
pub struct RocketExplosion {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct RocketStats {
    pub altitude: f32,
    pub vertical_speed: f32,
    pub horizontal_speed: f32,
    pub angle: f32,
    pub fuel_percentage: f32,
    pub distance_to_target: f32,
    pub landing_score: f32,
    pub total_landings: u32,
    pub total_crashes: u32,
    pub consecutive_successes: u32, // Succès consécutifs pour l'apprentissage
}

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RocketStats {
                altitude: 0.0,
                vertical_speed: 0.0,
                horizontal_speed: 0.0,
                angle: 0.0,
                fuel_percentage: 1.0,
                distance_to_target: 0.0,
                landing_score: 0.0,
                total_landings: 0,
                total_crashes: 0,
                consecutive_successes: 0,
            })
            .add_systems(Startup, spawn_rocket)
            .add_systems(Update, (
                update_rocket_stats, 
                update_flame_visibility,
                check_landing_conditions,
                explosion_system,
                restart_system,
                auto_restart_timer,
            ));
    }
}

fn spawn_rocket(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    stats: Res<RocketStats>,
) {
    let mut rng = rand::thread_rng();
    
    // POSITION DE DÉPART ALÉATOIRE pour l'apprentissage RL
    let start_x = rng.gen_range(-100.0..100.0); // Position horizontale aléatoire
    let start_y = 200.0; // Hauteur fixe
    let start_rotation = rng.gen_range(-0.5..0.5); // Rotation initiale aléatoire
    
    // Taille adaptative basée sur les succès consécutifs
    let size_factor = 1.0 + (stats.consecutive_successes as f32 * 0.05).min(0.3); // +5% par succès, max +30%
    
    let rocket_entity = commands.spawn((
        TransformBundle::from(Transform::from_xyz(start_x, start_y, 1.0)
            .with_rotation(Quat::from_rotation_z(start_rotation))),
        RocketMainBody,
        Rocket {
            fuel: 100.0,
            max_fuel: 100.0,
            throttle: 0.0,
            engine_power: 600.0,
            rotation_speed: 1.2,
            has_crashed: false,
            has_landed: false,
            size_factor,
        },
        PhysicsBody {
            velocity: Vec2::new(0.0, 0.0),
            angular_velocity: 0.0,
        },
    )).id();

    let base_width = 20.0 * size_factor;
    let base_height = 60.0 * size_factor;
    let flame_width = 15.0 * size_factor;
    let flame_height = 30.0 * size_factor;
    let leg_width = 5.0 * size_factor;
    let leg_height = 20.0 * size_factor;

    commands.entity(rocket_entity).with_children(|parent| {
        // Corps principal de la fusée (taille adaptative)
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Box::new(base_width, base_height, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::rgb(1.0, 1.0, 1.0))),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));

        // Flamme du moteur (taille adaptative)
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Box::new(flame_width, flame_height, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.4, 0.0))),
                transform: Transform::from_xyz(0.0, -base_height/2.0 - flame_height/2.0 + 5.0, 0.0),
                ..default()
            },
            RocketFlame,
        ));

        // Jambes d'atterrissage (taille adaptative)
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Box::new(leg_width, leg_height, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7))),
                transform: Transform::from_xyz(-base_width/2.0 + 2.0, -base_height/2.0 + 5.0, 0.0),
                ..default()
            },
            LandingLegs { deployed: false, contact: false },
        ));

        parent.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Box::new(leg_width, leg_height, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7))),
                transform: Transform::from_xyz(base_width/2.0 - 2.0, -base_height/2.0 + 5.0, 0.0),
                ..default()
            },
            LandingLegs { deployed: false, contact: false },
        ));
    });

    println!("🚀 FUSÉE CRÉÉE - Position: ({:.1}, {:.1}), Rotation: {:.1}°, Taille: +{:.0}%", 
             start_x, start_y, start_rotation.to_degrees(), (size_factor - 1.0) * 100.0);
}

fn update_rocket_stats(
    rocket_query: Query<(&Transform, &Rocket, &PhysicsBody), With<RocketMainBody>>,
    mut stats: ResMut<RocketStats>,
) {
    if let Ok((transform, rocket, physics)) = rocket_query.get_single() {
        stats.altitude = transform.translation.y;
        stats.angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        stats.fuel_percentage = rocket.fuel / rocket.max_fuel;
        stats.vertical_speed = physics.velocity.y;
        stats.horizontal_speed = physics.velocity.x;
        stats.distance_to_target = (transform.translation.x.powi(2) + (transform.translation.y + 340.0).powi(2)).sqrt();
    }
}

fn update_flame_visibility(
    rocket_query: Query<&Rocket, With<RocketMainBody>>,
    mut flame_query: Query<&mut Visibility, With<RocketFlame>>,
) {
    if let (Ok(rocket), Ok(mut visibility)) = (rocket_query.get_single(), flame_query.get_single_mut()) {
        *visibility = if rocket.throttle > 0.0 && !rocket.has_crashed && !rocket.has_landed {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn check_landing_conditions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut rocket_query: Query<(&Transform, &mut Rocket, &PhysicsBody), With<RocketMainBody>>,
    mut stats: ResMut<RocketStats>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok((transform, mut rocket, physics)) = rocket_query.get_single_mut() {
        let rocket_bottom = transform.translation.y - (60.0 * rocket.size_factor / 2.0);
        
        // Vérifie si la fusée touche le sol (y = -340)
        if rocket_bottom <= -330.0 && !rocket.has_crashed && !rocket.has_landed {
            let angle_deg = transform.rotation.to_euler(EulerRot::XYZ).2.abs().to_degrees();
            let vertical_speed = physics.velocity.y.abs();
            let horizontal_speed = physics.velocity.x.abs();
            
            // ZONE D'ATTERRISSAGE OBLIGATOIRE (80 pixels de large)
            let landing_zone_x_min = -40.0;
            let landing_zone_x_max = 40.0;
            let landing_zone_y_min = -345.0;
            let landing_zone_y_max = -335.0;
            
            let in_landing_zone = transform.translation.x >= landing_zone_x_min && 
                                 transform.translation.x <= landing_zone_x_max &&
                                 transform.translation.y >= landing_zone_y_min && 
                                 transform.translation.y <= landing_zone_y_max;
            
            // CONDITIONS D'ATTERRISSAGE TRÈS STRICTES - INTERVALLE DE 10%
            let min_angle = 81.0;  // 90° - 9° (10% de 90°)
            let max_angle = 99.0;  // 90° + 9° (10% de 90°)
            let acceptable_angle = (min_angle..=max_angle).contains(&angle_deg);
            
            let good_vertical_speed = vertical_speed < 3.0;
            let good_horizontal_speed = horizontal_speed < 1.0;
            
            // VÉRIFICATION STRICTE - TOUTES les conditions doivent être respectées
            let perfect_landing = in_landing_zone && acceptable_angle && good_vertical_speed && good_horizontal_speed;
            
            if perfect_landing {
                // ATTERRISSAGE PARFAIT RÉUSSI
                rocket.has_landed = true;
                stats.total_landings += 1;
                stats.consecutive_successes += 1;
                
                // Calcul du score de précision
                let angle_deviation = (angle_deg - 90.0).abs();
                let angle_score = 1.0 - (angle_deviation / 9.0); // 9° = 10% de 90°
                let vertical_score = 1.0 - (vertical_speed / 3.0);
                let horizontal_score = 1.0 - (horizontal_speed / 1.0);
                let zone_score = if in_landing_zone { 1.0 } else { 0.0 };
                
                stats.landing_score = 100.0 * angle_score * vertical_score * horizontal_score * zone_score;
                
                println!("🎯 ATTERRISSAGE PARFAIT RÉUSSI!");
                println!("   • Zone: {:.1} (entre -40 et +40 ✓)", transform.translation.x);
                println!("   • Angle: {:.1}° (entre {}° et {}° ✓)", angle_deg, min_angle, max_angle);
                println!("   • Vitesse verticale: {:.1} m/s (< 3 m/s ✓)", vertical_speed);
                println!("   • Vitesse horizontale: {:.1} m/s (< 1 m/s ✓)", horizontal_speed);
                println!("   • Score: {:.1}/100", stats.landing_score);
                println!("   • Succès consécutifs: {}", stats.consecutive_successes);
                println!("   • Taille fusée: +{:.0}%", (rocket.size_factor - 1.0) * 100.0);
                
                *game_state = GameState::Landed;
                
            } else {
                // CRASH - AU MOINS une condition n'est pas respectée
                rocket.has_crashed = true;
                stats.total_crashes += 1;
                stats.consecutive_successes = 0;
                
                println!("💥 CRASH! Conditions non respectées:");
                
                // Détail exact des conditions échouées
                let mut crash_reasons = Vec::new();
                
                if !in_landing_zone { 
                    crash_reasons.push(format!("Hors zone d'atterrissage: {:.1} (doit être entre -40 et 40)", transform.translation.x));
                }
                if !acceptable_angle { 
                    crash_reasons.push(format!("Angle incorrect: {:.1}° (doit être entre {}° et {}°)", angle_deg, min_angle, max_angle));
                }
                if !good_vertical_speed { 
                    crash_reasons.push(format!("Vitesse verticale trop élevée: {:.1} m/s (> 3 m/s)", vertical_speed));
                }
                if !good_horizontal_speed { 
                    crash_reasons.push(format!("Vitesse horizontale trop élevée: {:.1} m/s (> 1 m/s)", horizontal_speed));
                }
                
                for reason in &crash_reasons {
                    println!("   • {}", reason);
                }
                
                println!("   • Succès consécutifs réinitialisés");
                
                *game_state = GameState::Crashed;
                
                // Crée une explosion
                spawn_explosion(&mut commands, &mut meshes, &mut materials, transform.translation);
            }
        }
        
        // Vérifie aussi les collisions latérales violentes (condition supplémentaire)
        let crash_speed = physics.velocity.length();
        if crash_speed > 25.0 && transform.translation.y < -300.0 && !rocket.has_crashed {
            println!("💥 CRASH! Impact trop violent: {:.1} m/s", crash_speed);
            rocket.has_crashed = true;
            stats.total_crashes += 1;
            stats.consecutive_successes = 0;
            *game_state = GameState::Crashed;
            spawn_explosion(&mut commands, &mut meshes, &mut materials, transform.translation);
        }
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
) {
    // Explosion principale
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::Circle::new(8.0).into())),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.8, 0.0))),
            transform: Transform::from_translation(position),
            ..default()
        },
        RocketExplosion {
            timer: Timer::from_seconds(1.5, TimerMode::Once),
        },
    ));

    // Particules d'explosion
    for i in 0..6 {
        let angle = (i as f32) * (std::f32::consts::TAU / 6.0);
        let dir = Vec2::new(angle.cos(), angle.sin());
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(4.0, 4.0)).into())),
                material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.3, 0.0))),
                transform: Transform::from_translation(position),
                ..default()
            },
            RocketExplosion {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            },
            PhysicsBody {
                velocity: dir * 60.0,
                angular_velocity: 8.0,
            },
        ));
    }
}

fn explosion_system(
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut RocketExplosion, &mut Transform, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut explosion, mut transform, material_handle) in explosion_query.iter_mut() {
        explosion.timer.tick(time.delta());
        
        let progress = explosion.timer.elapsed().as_secs_f32() / explosion.timer.duration().as_secs_f32();
        
        // Agrandit l'explosion
        transform.scale = Vec3::splat(1.0 + progress * 4.0);
        
        // Change la couleur (orange -> rouge -> transparent)
        if let Some(material) = materials.get_mut(&*material_handle) {
            let alpha = 1.0 - progress;
            material.color = Color::rgba(1.0, 0.5 - progress * 0.5, 0.0, alpha);
        }
        
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn restart_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    rocket_query: Query<Entity, With<RocketMainBody>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    stats: Res<RocketStats>,
) {
    if *game_state == GameState::Restarting {
        // Supprime l'ancienne fusée
        for entity in rocket_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Recrée la fusée avec les statistiques mises à jour
        spawn_rocket(commands, meshes, materials, stats);
        
        *game_state = GameState::Playing;
        println!("🔄 NOUVELLE PARTIE! Atterrissez dans la zone JAUNE.");
    }
}

fn auto_restart_timer(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut restart_timer: Local<f32>,
) {
    if *game_state == GameState::Crashed || *game_state == GameState::Landed {
        *restart_timer += time.delta_seconds();
        
        // Redémarrage automatique après 2 secondes
        if *restart_timer >= 2.0 {
            *game_state = GameState::Restarting;
            *restart_timer = 0.0;
        }
    } else {
        *restart_timer = 0.0;
    }
}