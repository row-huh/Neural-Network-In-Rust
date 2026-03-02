struct NeuralNetwork {
    weights: Vec<f64>
}

impl NeuralNetwork {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let weights = vec![rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)]


        Self {
            weights
        }
    }
}