#![windows_subsystem = "windows"]
use std::f32::consts::PI;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, core_pipeline::bloom::BloomSettings,};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bouncing Balls".to_string(),
                width: 1024.,
                height: 768.,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(move_balls)
        .add_system(spawn_balls)
        .add_system(remove_ball)
        .run();
}

#[derive(Component)]
pub struct Ball{
    velocity: Vec3,
}

#[derive(Resource)]
pub struct BallsCounter{
    pub balls: i32
}

fn setup(mut commands: Commands,) {
    commands.spawn((Camera2dBundle{
        camera: Camera { 
            hdr: true,
            ..default()
        },
        ..default()
    },
    BloomSettings::default()));

    commands.insert_resource(BallsCounter{ balls: 0});
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut balls_counter: ResMut<BallsCounter>,
){
    let num = 100;
    if balls_counter.balls  == 0 {
        for _ in 0..num{
            let speed = rand::thread_rng().gen_range(50.0..300.0);
            let dir = rand::thread_rng().gen_range(0.0..2.0 * PI);
    
            let velocity = Vec3::new(speed * dir.cos(), speed * dir.sin(), 0.0);
    
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial{ 
                    color: Color::hsl(50.0, 1.0, 2.0),
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            })
            .insert(Ball{ velocity});
            balls_counter.balls += 1;
        }
    }
}


fn move_balls(mut query: Query<(&mut Transform, &Ball)>, time: Res<Time>,){
    for (mut tramsform, ball) in query.iter_mut(){
        tramsform.translation += ball.velocity * time.delta_seconds();
    } 

}

fn remove_ball(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform,), With<Ball>>,
    mut ball_counter: ResMut<BallsCounter>,
){
    for ( entity, transform) in query.iter_mut(){
        if distance(transform.translation.x, transform.translation.y) > 200.0 {
            commands.entity(entity).despawn();
            ball_counter.balls -= 1;
        }
    }
}

fn distance(x: f32, y: f32) -> f32{
    let distance = ((x * x) + (y * y)).sqrt();
    distance
}
