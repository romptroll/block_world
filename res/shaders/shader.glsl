#shader vertex
#version 330 core

layout (location = 0) in vec3 aPos; // the position variable has attribute position 0
layout (location = 1) in vec2 aUV; // the position variable has attribute position 0
layout (location = 2) in vec3 aCol; // the position variable has attribute position 0

uniform mat4 u_mat;


out vec4 vertexColor; // specify a color output to the fragment shader
out vec2 uv;

void main()
{
    gl_Position = vec4(aPos, 1.0) * u_mat; // see how we directly give a vec3 to vec4's constructor
    vertexColor = vec4(aCol, 1.0); // set the output variable to a dark-red color
    uv = aUV;
}

#shader fragment

#version 330 core

out vec4 FragColor;

in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)
in vec2 uv;

uniform sampler2D u_texture;

void main()
{
    vec4 texColor = texture(u_texture, uv) * vertexColor;
    FragColor = texColor;
}