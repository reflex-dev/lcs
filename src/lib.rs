use pyo3::prelude::*;

struct LCS {
    length_of_source: usize,
    calculated_table: Vec<usize>,
}

impl LCS {
    fn from_strings(source: &str, target: &str) -> Self {
        let mut previous = vec![0; target.len() + 1];
        let mut current = vec![0; target.len() + 1];
        for c1 in source.chars() {
            for (j, c2) in target.chars().enumerate() {
                if c1 == c2 {
                    current[j + 1] = previous[j] + 1;
                } else {
                    current[j + 1] = current[j].max(previous[j + 1]);
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        Self {
            length_of_source: source.len(),
            calculated_table: previous,
        }
    }

    fn normalized_similarity_of_prefix(&self, length_of_prefix_of_target: usize) -> f64 {
        if length_of_prefix_of_target == 0 {
            return 1.0;
        }
        let lcs_length = self.calculated_table[length_of_prefix_of_target];
        (2.0 * lcs_length as f64) / (length_of_prefix_of_target + self.length_of_source) as f64
    }
}

#[pyfunction]
fn compute_lcs_similarity(
    source: &str,
    target: &str,
    length_of_prefixes_of_target: Vec<usize>,
) -> Vec<f64> {
    let lcs = LCS::from_strings(source, target);
    length_of_prefixes_of_target
        .into_iter()
        .map(|length_of_prefix_of_target| {
            lcs.normalized_similarity_of_prefix(length_of_prefix_of_target)
        })
        .collect()
}

#[pymodule(gil_used = false)]
fn fast_lcs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_lcs_similarity, m)?)?;
    Ok(())
}
