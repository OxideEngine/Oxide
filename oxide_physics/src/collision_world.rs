
pub struct CollisionWorld {
    pub broad_phase: BroadPhaseObject,
}

// type aliasing
pub BroadPhaseObject = Box<dyn Shape>
