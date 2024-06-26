#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout(binding = 2) uniform HudUBO {
    mat4 scale;
} ubo;

// input is a struct TexturedVertex
layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 tex_coord;

layout (location = 0) out vec2 o_tex_coord;

void main() {
    gl_Position = ubo.scale * pos;
    o_tex_coord = tex_coord;
}
