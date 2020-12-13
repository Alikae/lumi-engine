in vec2 position;
in vec2 texpos;

uniform vec3 camera;

out vec2 v_texpos;

void main() {
    v_texpos = texpos;

    vec2 new_position = (position - camera.xy) * camera.z;

    gl_Position = vec4(new_position, 0., 1.);
}

