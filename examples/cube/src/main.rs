use std::ffi::c_void;

use vitagl_sys::*;

fn main() {
    let colors: [f32; 12] = [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0]; // Colors for a face
    let vertices_front: [f32; 12] = [
        -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
    ]; // Front Face
    let vertices_back: [f32; 12] = [
        -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    ]; // Back Face
    let vertices_left: [f32; 12] = [
        -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
    ]; // Left Face
    let vertices_right: [f32; 12] = [
        0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,
    ]; // Right Face
    let vertices_top: [f32; 12] = [
        -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5,
    ]; // Top Face
    let vertices_bottom: [f32; 12] = [
        -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    ]; // Bottom Face

    let indices: [u16; 36] = [
        0, 1, 2, 1, 2, 3, // Front
        4, 5, 6, 5, 6, 7, // Back
        8, 9, 10, 9, 10, 11, // Left
        12, 13, 14, 13, 14, 15, // Right
        16, 17, 18, 17, 18, 19, // Top
        20, 21, 22, 21, 22, 23, // Bottom
    ];

    let mut color_array = [0.; 12 * 6];
    for i in 0..12 * 6 {
        color_array[i] = colors[i % 12];
    }

    let mut vertex_array = vec![];
    vertex_array.extend_from_slice(&vertices_front);
    vertex_array.extend_from_slice(&vertices_back);
    vertex_array.extend_from_slice(&vertices_left);
    vertex_array.extend_from_slice(&vertices_right);
    vertex_array.extend_from_slice(&vertices_top);
    vertex_array.extend_from_slice(&vertices_bottom);

    unsafe {
        vglInit(0x800000);
        vglWaitVblankStart(GL_TRUE as u8);
        glClearColor(0., 0., 0., 0.);

        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        gluPerspective(90., 960. / 544., 0.01, 100.);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        glTranslatef(0., 0., -3.); // Centering the cube

        // Enabling depth test
        glEnable(GL_DEPTH_TEST);
        glDepthFunc(GL_LESS);

        loop {
            // Clear color and depth buffers
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            // Drawing our cube with vertex arrays
            glEnableClientState(GL_VERTEX_ARRAY);
            glEnableClientState(GL_COLOR_ARRAY);
            glVertexPointer(3, GL_FLOAT, 0, vertex_array.as_ptr() as *const c_void);
            glColorPointer(3, GL_FLOAT, 0, color_array.as_ptr() as *const c_void);
            glRotatef(1., 0., 0., 1.); // Rotating cube at each frame by 1 on axis x and axis w
            glRotatef(0.5, 0., 1., 0.); // Rotating cube at each frame by 0.5 on axis x and 1.0 on axis z
            glDrawElements(
                GL_TRIANGLES,
                6 * 6,
                GL_UNSIGNED_SHORT,
                indices.as_ptr() as *const c_void,
            );
            glDisableClientState(GL_VERTEX_ARRAY);
            glDisableClientState(GL_COLOR_ARRAY);

            // Performing buffer swap
            vglSwapBuffers(GL_FALSE as u8);
        }

        vglEnd();
    }
}
