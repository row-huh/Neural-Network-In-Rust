struct NeuralNetwork {
    weights: Vec<f64>
    bias f64
}

// output = activation(weights* inputs + bias)

impl NeuralNetwork {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        weights = vec![rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)]


        Self {
            weights,
            bias: rng.gen_range(0.0..1.0)
        }
    }

    // parameter is the object itself and a vector of type f64 with 2 items (which are the inputs)

    fn predict(&Self, input:&[f64, 2]) -> f64 {
        for (i, weight) in Self.weights.iter().enumerate() {
            sum += input[i] * weight;
        }

        sigmoid(sum)
    }
}


fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}