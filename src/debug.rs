use bevy::prelude::*;
pub fn count<C: Component>(q: Query<(), With<C>>) {
    let mut count = 0;
    for _ in &q {
        count += 1;
    }
    println!("{}", count)
}