struct NeuralNetwork {
    weights: Vec<f64>
    bias f64
    learningRate: f64
}

// output = activation(weights* inputs + bias)

impl NeuralNetwork {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        weights = vec![rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)]


        Self {
            weights,
            bias: rng.gen_range(0.0..1.0),
            learningRate: 0.1
        }
    }

    // parameter is the object itself and a vector of type f64 with 2 items (which are the inputs)

    fn predict(&Self, input:&[f64, 2]) -> f64 {
        for (i, weight) in Self.weights.iter().enumerate() {
            sum += input[i] * weight;
        }

        sigmoid(sum)
    }


    fn train(&mut Self, inputs:Vec<[f64; 2]>, outputs: Vec<f64>, epochs: usize) {
        for _ in 0..epochs {
            for (i, input) in inputs.iter().enumerate() {
                let output = self.predict(input);

                let error = outputs[i] - output

                // finding gradient of sigmoid function
                let delta = derivative(output)

                for j in 0..self.weights.len() {
                    self.weights[j] += self.learningRate * error * input[j] * delta
                }

                bias = learningRate * error * delta
            }
        }
    }
}


fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}


fn derivative(x: f64) -> f64 {
    x * (1.0 - x)
}