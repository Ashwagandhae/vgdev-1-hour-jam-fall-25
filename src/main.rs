use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_bomb_spawning);
    app.add_systems(Update, move_player);
    app.add_systems(Update, make_bullet);
    app.add_systems(Update, move_bullet);
    app.add_systems(Update, move_enemy);
    app.add_systems(Update, collid_bullet);
    app.add_systems(Update, spawn_bombs);
    app.add_systems(Update, kill_playr);
    app.run();
}

const X_EXTENT: f32 = 900.;

#[derive(Component)]
struct Player {
    alvie: bool,
}

#[derive(Component)]
struct Enemy {
    helth: usize,
}

#[derive(Component)]
struct Bullet {
    direction: Vec2,
}

#[derive(Component)]
struct Score {
    score: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(CircularSegment::new(50.0, 1.25)),
        meshes.add(Ellipse::new(25.0, 50.0)),
        meshes.add(Annulus::new(25.0, 50.0)),
        meshes.add(Capsule2d::new(25.0, 50.0)),
        meshes.add(Rhombus::new(75.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(RegularPolygon::new(50.0, 6)),
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )),
        meshes.add(Segment2d::new(
            Vec2::new(-50.0, 50.0),
            Vec2::new(50.0, -50.0),
        )),
        meshes.add(Polyline2d::new(vec![
            Vec2::new(-50.0, 50.0),
            Vec2::new(0.0, -50.0),
            Vec2::new(50.0, 50.0),
        ])),
    ];
    let num_shapes = shapes.len();

    // for (i, shape) in shapes.into_iter().enumerate() {
    // Distribute colors evenly across the rainbow.

    // }
    let color = Color::hsl(360. * 0 as f32 / num_shapes as f32, 0.95, 0.7);

    commands.spawn((
        Player { alvie: true },
        // Mesh2d(shapes[0].clone()),
        // MeshMaterial2d(materials.add(color)),
        Sprite::from_image(asset_server.load("bevy.png")),
        Transform::from_xyz(
            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
            0.0, 0.0, 0.0,
        ),
        Score { score: 0 },
    ));

    for i in 0..20 {}

    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("hello\nbevy!"),
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(Justify::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
        TextFont::from_font_size(67.0),
        Score { score: 0 },
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &Player), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (mut trans, pla) = query.single_mut().unwrap();
    if (!pla.alvie) {
        return;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        trans.translation.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        trans.translation.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        trans.translation.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        trans.translation.x += 1.0;
    }
}

fn make_bullet(
    mut query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
) {
    let mut player = query.single().unwrap();

    let mut y_direction = 0.0;
    if keyboard.pressed(KeyCode::ArrowDown) {
        y_direction = -1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        y_direction = 1.0;
    }
    let mut x_direction = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        x_direction = -1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        x_direction = 1.0;
    }

    if keyboard.just_pressed(KeyCode::Space) && (x_direction != 0.0 || y_direction != 0.0) {
        let mut spir = Sprite::from_image(asset_server.load("crab.png"));
        spir.custom_size = Some(Vec2::new(50.0, 50.0));
        commands.spawn((
            spir,
            Bullet {
                direction: Vec2::new(x_direction, y_direction),
            },
            Transform::from_translation(player.translation),
        ));
    }
}

fn move_bullet(mut query: Query<(&mut Transform, &Bullet), With<Bullet>>) {
    for (mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.direction.extend(0.0) * 2.0;
    }
}

fn move_enemy(
    query: Query<&Transform, With<Player>>,
    mut query_en: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    let mut player = query.single().unwrap();

    for (mut transform, enemy) in query_en.iter_mut() {
        let towards_player =
            (player.translation.truncate() - transform.translation.truncate()).normalize();
        transform.translation += towards_player.extend(0.0);
    }
}

fn collid_bullet(
    mut query_p: Query<&Player, With<Player>>,

    mut commands: Commands,
    query: Query<(&Transform, &Bullet, Entity)>,
    query_en: Query<(&Transform, &Enemy, Entity)>,
    mut query_score: Query<(&mut Score, &mut Text)>,
) {
    if !query_p.single().unwrap().alvie {
        return;
    }
    let (mut score, mut tex) = query_score.single_mut().unwrap();
    for (en_tra, en, eid) in query_en.iter() {
        for (bul_trans, b, bid) in query.iter() {
            if en_tra.translation.distance(bul_trans.translation) < 50.0 {
                commands.get_entity(eid).unwrap().despawn();
                commands.get_entity(bid).unwrap().despawn();
                score.score += 1;
                *tex = Text::from(format!("Inferiro game engines defeated: {}", score.score));
            }
        }
    }
}

#[derive(Resource)]
struct BombsSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn spawn_bombs(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<BombsSpawnConfig>,
    mut asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());

    if config.timer.just_finished() {
        let i = rand::random_range(0.0..20.0);
        let angle = (i as f32) / 20.0 * 2.0 * PI;
        let mut sp = Sprite::from_image(asset_server.load(format!("enemy{}.png", i as usize % 4)));
        sp.custom_size = Some(Vec2::new(100.0, 100.0));
        commands.spawn((
            Enemy { helth: 3 },
            sp,
            Transform::from_xyz(angle.sin() * 400.0, angle.cos() * 400.0, 0.0),
        ));
    }
}

fn setup_bomb_spawning(mut commands: Commands) {
    commands.insert_resource(BombsSpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}

fn kill_playr(
    mut query_p: Query<(&mut Player, Entity, &Transform), With<Player>>,

    mut commands: Commands,
    query_en: Query<(&Transform, &Enemy, Entity)>,
    mut query_score: Query<(&mut Score, &mut Text)>,
) {
    let (mut play, pid, pt) = query_p.single_mut().unwrap();

    if (!play.alvie) {
        return;
    }
    let (mut score, mut tex) = query_score.single_mut().unwrap();
    for (en_trans, b, eid) in query_en.iter() {
        if pt.translation.distance(en_trans.translation) < 50.0 {
            commands.get_entity(eid).unwrap().despawn();
            // score.score += 1;
            play.alvie = false;
            *tex = Text::from(format!("GAME OVER"));
        }
    }
}
