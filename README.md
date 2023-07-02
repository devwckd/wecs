# wecs

wecs (wckd-ecs) is a simple ECS library heavily based on [bevy_ecs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs/README.md).

### Motivation

 As part of my "Road to Rust GameDev" journey, I wanted to learn how stuff worked behind the scenes, so I started with the foundation of many game engines, the Entity-Component-System. I did it by devouring [bevy_ecs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs/README.md) and creating my own version of it, with similar inner workings and identical syntax, without all the optimization that I am yet to learn. I'll be using it on my personal projects and hopefully be incrementing on its features.  
  
### Code Example

#### - "Gravity" sim

```rust
use wecs::{query::Query, resource::Res, schedule::Schedule, world::World};

#[derive(Resource, Default)]
pub struct GravityManager {
    gravity: f32,
}

#[derive(Debug, Component)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let mut world = World::new();
    world.insert_resource(GravityManager { gravity: 10.0 });
    world.spawn_entity(Position {
        x: 0.0,
        y: 50.0,
        z: 0.0,
    });

    let mut schedule = Schedule::new()
        .with_system(update_system)
        .with_system(print_system);

    loop {
        schedule.run(&mut world);
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn update_system(query: Query<&mut Position>, manager: Res<GravityManager>) {
    for position in query {
        position.y -= manager.gravity;
    }
}

fn print_system(query: Query<&Position>) {
    for position in query {
        let Position {x, y, z} = position;
        println!("entity position is {x},{y},{z}");
    }
}
```

output:  
> entity position is 0,40,0  
> entity position is 0,30,0  
> entity position is 0,20,0  
> entity position is 0,10,0  
> entity position is 0,0,0  
> entity position is 0,-10,0  
> entity position is 0,-20,0  
> ...

#### - Events

```rust
use wecs::{schedule::Schedule, world::World, EventManager, EventReader, EventWriter};

struct CoolEvent {
    pub num: u32,
}

fn main() {
    let mut world = World::new();
    world.init_resource::<EventManager<CoolEvent>>();

    let mut schedule = Schedule::new();
    schedule.add_system(publish_events);
    schedule.add_system(consume_events);
    schedule.add_system(EventManager::<CoolEvent>::clear);

    schedule.run(&mut world);
}

fn publish_events(mut writer: EventWriter<CoolEvent>) {
    writer.dispatch_one(CoolEvent { num: 0 });
}

fn consume_events(reader: EventReader<CoolEvent>) {
    for event in reader {
        println!("CoolEvent's num is {}", event.num);
    }
}
```
output:
> CoolEvent's num is 0

