use crate::prelude::*;

pub fn monster_ai(q: Query<(&Position, &Viewshed), With<Monster>>) {
    for (pos, viewshed) in q.iter() {
        println!("Monster considers their own existence")
    }
}
