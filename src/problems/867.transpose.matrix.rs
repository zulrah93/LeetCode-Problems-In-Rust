impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Get the number of rows and columns so we don't have to write matrix.len all over the place
        let rows = matrix.len();
        let columns = matrix[0].len();

        // We are going to initialize the matrix size to be the columns by rows; since we are transposing the original matrix
        let mut transposed_matrix = vec![vec![0; rows]; columns]; // Space Complexity: O(N)

        // Time Complexitiy: O(N^2) -- For a small input size this should suffice
        for i in 0..rows {
            for j in 0..columns {
                transposed_matrix[j][i] = matrix[i][j];
            }
        }

        transposed_matrix
    }
}
