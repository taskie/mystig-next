use super::actor;

pub trait Scene: actor::Actor {
    fn next(&self) -> Option<&Scene>;
}
