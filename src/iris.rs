use std::ops::Index;
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Iris {
    //    id: u32,
    sepal_length_cm: f32,
    sepal_width_cm: f32,
    petal_length_cm: f32,
    petal_width_cm: f32,
    pub species: String,
}

impl Index<usize> for Iris {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.sepal_length_cm,
            1 => &self.sepal_width_cm,
            2 => &self.petal_length_cm,
            3 => &self.petal_width_cm,
            _ => panic!("Invalid index"),
        }
    }
}

impl Iris {
    pub fn new() -> Iris {
        Iris {
            sepal_length_cm: 0.0,
            sepal_width_cm: 0.0,
            petal_length_cm: 0.0,
            petal_width_cm: 0.0,
            species: String::new(),
        }
    }

    pub fn len(&self) -> usize {
        4
    }

    pub fn as_vec(&self) -> Vec<f32> {
        vec![
            self.sepal_length_cm,
            self.sepal_width_cm,
            self.petal_length_cm,
            self.petal_width_cm,
        ]
    }

    pub fn as_array(&self) -> [f32; 4] {
        [
            self.sepal_length_cm,
            self.sepal_width_cm,
            self.petal_length_cm,
            self.petal_width_cm,
        ]
    }
}

impl IntoIterator for Iris {
    type Item = f32;
    type IntoIter = IrisIterator;

    fn into_iter(self) -> Self::IntoIter {
        IrisIterator {
            iris: self,
            index: 0,
        }
    }
}

pub struct IrisIterator {
    iris: Iris,
    index: usize,
}

impl Iterator for IrisIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.index < self.iris.len() {
            let result = Some(self.iris[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}
