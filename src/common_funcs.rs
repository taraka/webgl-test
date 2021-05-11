use super::constants::*;
use nalgebra::{Perspective3};
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn get_updated_3d_y_values(curr_time: f32) -> Vec<f32> {
    let point_count_per_row = GRID_SIZE + 1;
    let mut y_vals: Vec<f32> = vec![0.; point_count_per_row * point_count_per_row];
    let half_grid: f32  = point_count_per_row as f32 / 2.0;
    let frequency_scale: f32 = 3.0 * std::f32::consts::PI;
    let y_scale = 0.15;

    let time_shift = curr_time / 500.0;

    for z in 0..point_count_per_row {
        for x in 0..point_count_per_row {
            let use_y_index = z * point_count_per_row + x;
            let scaled_x = frequency_scale * (x as f32 - half_grid) / half_grid;
            let scaled_z = frequency_scale * (z as f32 - half_grid) / half_grid;
            y_vals[use_y_index] = y_scale * ((scaled_x * scaled_x + scaled_z * scaled_z).sqrt() + time_shift).sin();
        }
    }

    y_vals
}

pub fn get_3d_projection_matrix(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_angle_x_axis: f32,
    rotation_angle_y_axis: f32,
) -> [f32; 16] {

    let rotation_x_axis: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, rotation_angle_x_axis.cos(), -rotation_angle_x_axis.sin(), 0.0,
        0.0, rotation_angle_x_axis.sin(), rotation_angle_x_axis.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    let rotation_y_axis: [f32; 16] = [
        rotation_angle_y_axis.cos(), 0.0, rotation_angle_y_axis.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -rotation_angle_y_axis.sin(), 0.0, rotation_angle_y_axis.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    let rotation_matrix = mult_matrix_4(rotation_x_axis, rotation_y_axis);

    let aspect: f32 = canvas_width / canvas_height;
    let scale_x = (right - left) / canvas_width;
    let scale_y = (top - bottom) / canvas_height;
    let scale = scale_y;

    let translation_matrix = translation_matrix(
        -1.0 + scale_x + 2.0 * left / canvas_width,
        -1.0 + scale_y + 2.0 * bottom / canvas_height,
        Z_PLANE,
    );

    let scale_matrix = scaling_matrix(scale, scale, 0.0);
    let rotation_scale = mult_matrix_4(rotation_matrix, scale_matrix);
    let combined_transform = mult_matrix_4(rotation_scale, translation_matrix);

    let perspective_matrix_tmp: Perspective3<f32> = Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
    let mut perspective: [f32; 16] = [0.0; 16];
    perspective.copy_from_slice(perspective_matrix_tmp.as_matrix().as_slice());

    mult_matrix_4(combined_transform, perspective)
}


pub fn get_position_grid_n_by_n(n: usize) -> (Vec<f32>, Vec<u16>) {
    let n_plus_one = n + 1;
    let mut positions: Vec<f32> = vec![0.; 3 * n_plus_one * n_plus_one];
    let mut indices: Vec<u16> = vec![0; 6 * n * n];

    let graph_layout_width: f32 = 2.;
    let square_size: f32 = graph_layout_width / n as f32;

    for z in 0..n_plus_one {
        for x in 0..n_plus_one {
            let start_pos_i = 3 * (z * n_plus_one + x);
            positions[start_pos_i + 0] = -1. + (x as f32) * square_size;
            positions[start_pos_i + 1] = 0.;
            positions[start_pos_i + 2] = -1. + (z as f32) * square_size;

            if z < n && x < n {
                let start_index_i = 6 * (z * n + x);
                let vertex_index_top_left = (z * n_plus_one + x) as u16;
                let vertex_index_bottom_left = vertex_index_top_left + n_plus_one as u16;
                let vertex_index_top_right = vertex_index_top_left + 1;
                let vertex_index_bottom_right = vertex_index_bottom_left + 1;

                indices[start_index_i + 0] = vertex_index_top_left;
                indices[start_index_i + 1] = vertex_index_bottom_left;
                indices[start_index_i + 2] = vertex_index_bottom_right;
                indices[start_index_i + 3] = vertex_index_top_left;
                indices[start_index_i + 4] = vertex_index_bottom_right;
                indices[start_index_i + 5] = vertex_index_top_right;
            }
        }
    }

    (positions, indices)
}



pub fn link_program(
    gl: &GL,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

        let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vert_source).unwrap();
        let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

        gl.attach_shader(&program, &vert_shader);
        gl.attach_shader(&program, &frag_shader);
        gl.link_program(&program);

        if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) {
            Ok(program)
        }
        else {
            Err(gl.get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unable to get program info log")))
        }
}

fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
            Ok(shader)
        }
        else {
            Err(gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unable to get shader info log")))
        }

}


pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut mat = [0.0; 16];

    mat[0] = 1.0;
    mat[5] = 1.0;
    mat[10] = 1.0;
    mat[15] = 1.0;

    mat[12] = tx;
    mat[13] = ty;
    mat[14] = tz;

    mat
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut mat = [0.0; 16];

    mat[0] = sx;
    mat[5] = sy;
    mat[10] = sz;
    mat[15] = 1.0;

    mat
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut mat = [0.0; 16];

    mat[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    mat[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    mat[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    mat[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    mat[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    mat[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    mat[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    mat[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    mat[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    mat[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    mat[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    mat[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    mat[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    mat[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    mat[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    mat[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    mat
}