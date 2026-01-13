use pyo3::prelude::*;

struct LCS {
    calculated: Vec<usize>,
}

impl LCS {
    fn from_strings(s1: &str, s2: &str) -> Self {
        let mut previous = vec![0; s2.len() + 1];
        let mut current = vec![0; s2.len() + 1];
        for c1 in s1.chars() {
            for (j, c2) in s2.chars().enumerate() {
                if c1 == c2 {
                    current[j + 1] = previous[j] + 1;
                } else {
                    current[j + 1] = current[j].max(previous[j + 1]);
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        Self {
            calculated: previous,
        }
    }

    fn normalized_similarity_of_prefix(&self, prefix_length: usize) -> f64 {
        if prefix_length == 0 {
            return 1.0;
        }
        let lcs_length = self.calculated[prefix_length];
        (2.0 * lcs_length as f64) / (prefix_length + self.calculated.len() - 1) as f64
    }
}

#[pyfunction]
fn compute_lcs_similarity(s1: &str, s2: &str, queries: Vec<usize>) -> Vec<f64> {
    let lcs = LCS::from_strings(s1, s2);
    queries
        .into_iter()
        .map(|prefix_length| lcs.normalized_similarity_of_prefix(prefix_length))
        .collect()
}

#[pymodule(gil_used = false)]
fn lcs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_lcs_similarity, m)?)?;
    Ok(())
}
