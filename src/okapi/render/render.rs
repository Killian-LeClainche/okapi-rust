pub enum RenderAttributes {
    POS, POS_COLOR, POS_TEXTURE, POS_COLOR_TEXTURE, POS_NORMAL_TEXTURE, POS_COLOR_NORMAL_TEXTURE
}

pub struct RenderArray {
    gl_mode: i32,
    draw_mode: i32,
    vertices: i32,
    attributes: RenderAttributes,
    vbo: i32,
    vao: i32
}

pub struct RenderElement {
    gl_mode: i32,
    draw_mode: i32,
    vertices: i32,
    attributes: RenderAttributes,
    vbo: i32,
    vao: i32,
    ibo: i32
}

pub struct Shader {
    shader: i32,
    vertex_shader: i32,
    fragment_shader: i32
}

pub struct Renders {

}