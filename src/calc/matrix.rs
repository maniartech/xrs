// MMult Returns the matrix product of two matrices
pub fn m_mult(matrix1: Vec<Vec<f64>>, matrix2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Checking if the matrices are compatible for multiplication
    if matrix1[0].len() != matrix2.len() {
        panic!("The matrices are not compatible for multiplication");
    }

    // Creating a new matrix to store the result
    let mut result: Vec<Vec<f64>> = vec![vec![0.0; matrix2[0].len()]; matrix1.len()];
    for i in 0..matrix1.len() {
        for j in 0..matrix2[0].len() {
            for k in 0..matrix2.len() {
                result[i][j] += matrix1[i][k] * matrix2[k][j]
            }
        }
    }
    result
}

// m_unit Returns the unit matrix of the given size
pub fn m_unit(size: usize) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
    for i in 0..size {
        result[i][i] = 1.0;
    }
    result
}

// instead of taking the vectors, shadowing them, and then returning the result, we can directly take the ownership of the vectors and return the result
pub fn m_minor(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = vec![vec![0.0; matrix.len() - 1]; matrix.len() - 1];
    let mut i: usize = 0;
    let mut j: usize = 0;
    for x in 0..matrix.len() {
        if x == row {
            continue;
        }
        for y in 0..matrix.len() {
            if y == col {
                continue;
            }
            result[i][j] = matrix[x][y];
            j += 1;
        }
        i += 1;
        j = 0;
    }
    result
}

// m_determ Returns the determinant of a matrix
pub fn m_determ(matrix: &Vec<Vec<f64>>) -> f64 {
    let len: usize = matrix.len();

    // Checking the matrix is empty
    if len == 0 {
        return 0.0;
    }

    // Checking if the length of vectors are correct
    for i in 0..len {
        if matrix[i].len() != len {
            panic!("The matrix is not square");
        }
    }

    if len == 1 {
        return matrix[0][0];
    }

    if len == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    let mut det: f64 = 0.0;

    for i in 0..len {
        let minor_det: f64 = m_determ(&m_minor(&matrix, 0, i));
        det += matrix[0][i] * minor_det * if i % 2 == 0 { 1.0 } else { -1.0 };
    }
    det
}

pub fn m_inverse(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let det: f64 = m_determ(&matrix);
    if det == 0.0 {
        panic!("The matrix is not invertible");
    }

    let len: usize = matrix.len();

    for i in 0..len {
        if matrix[i].len() != len {
            panic!("The matrix is not square");
        }
    }

    let mut minor_det: Vec<Vec<f64>> = vec![vec![0.0; len]; len];
    for i in 0..len {
        for j in 0..len {
            let minor: Vec<Vec<f64>> = m_minor(&matrix, i, j);
            minor_det[i][j] = m_determ(&minor) * if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        }
    }

    let mut inverse: Vec<Vec<f64>> = vec![vec![0.0; len]; len];

    for i in 0..len {
        for j in 0..len {
            inverse[j][i] = minor_det[i][j] / det;
        }
    }

    inverse
}
