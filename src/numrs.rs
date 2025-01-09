pub struct NdArray<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
}

impl<T> NdArray<T> {
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product(), "Data size must match shape.");
        let strides = shape
            .iter()
            .rev()
            .scan(1, |acc, &dim| {
                let stride = *acc;
                *acc *= dim;
                Some(stride)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        Self { data, shape, strides }
    }
}

impl NdArray<f64> {
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Self::new(vec![0.0; size], shape)
    }
}
