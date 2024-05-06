pub struct Square {
    pub(crate) height: f32, // pub(crate) define um metodo ou propriedade publica
    pub(crate) width: f32,
}

impl Square {
    pub(crate) fn calculate_area(&self) -> f32 {
        self.width * self.height
    }

    pub(crate) fn calculate_perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    pub(crate) fn is_perfect_square(&self) -> &'static str {
        if self.width == self.height {
            "Sim"
        } else {
            "Não"
        }
    }

    pub(crate) fn new_square(width: f32, height: f32) -> Self {
        Self { height, width }
    }
}
