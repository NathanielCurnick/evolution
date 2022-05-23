use crate::world::World;

#[test]
#[should_panic]
fn test_world_gen_panic() {
    World::new(10, 10, 1000);
}
