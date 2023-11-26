pub trait IntoRapier {
    fn into_rapier(&self) -> rapier3d::prelude::Vector<f32>;
}

impl IntoRapier for Vec<f32> {
    fn into_rapier(&self) -> rapier3d::prelude::Vector<f32> {
        rapier3d::prelude::Vector::new(self[0], self[1], self[2])
    }
}
