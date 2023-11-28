pub mod math {
    use cgmath::{Matrix3, Rad};

  pub fn matrix34_to_matrix3(matrix: &[[f32; 4]; 3]) -> Matrix3<f32> {
    return Matrix3::new(
      matrix[0][0], matrix[0][1], matrix[0][2],
      matrix[1][0], matrix[1][1], matrix[1][2],
      matrix[2][0], matrix[2][1], matrix[2][2],
    );
  }

  pub fn decompose_rotation(matrix: Matrix3<f32>) -> (f32, f32, f32) {
    let pitch = Rad(-matrix[1][2].asin());
    let yaw = Rad(matrix[0][2].atan2(matrix[2][2]));
    let roll = Rad(matrix[1][0].atan2(matrix[1][1]));

    (yaw.0, pitch.0, roll.0)
  }
}