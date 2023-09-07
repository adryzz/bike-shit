#[derive(Debug, Clone, Copy)]
pub struct Gearbox<const I: usize, const O: usize> {
    pub input: [f32; I],
    pub output: [f32; O],
    pub current_input: usize,
    pub current_output: usize,
}

impl<const I: usize, const O: usize> Gearbox<I, O>
{
    pub fn new(input: [f32; I], output: [f32; O]) -> Self {
        Self {
            input,
            output,
            current_input: 0,
            current_output: 0,
        }
    }

    pub fn transform(&self, val: f32) -> f32 {
        let multiplier = self.input[self.current_input] / self.output[self.current_output];
        val * multiplier
    }

    pub fn set_input_gear(&mut self, gear: usize) -> Result<(), GearboxError> {
        if gear >= self.input.len() {
            return Err(GearboxError::InvalidGear(gear));
        }
        self.current_input = gear;
        Ok(())
    }

    pub fn set_output_gear(&mut self, gear: usize) -> Result<(), GearboxError> {
        if gear >= self.output.len() {
            return Err(GearboxError::InvalidGear(gear));
        }
        self.current_output = gear;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GearboxError {
    InvalidGear(usize),
}