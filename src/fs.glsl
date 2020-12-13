in vec2 v_texpos;

uniform sampler2D image;

out vec4 frag_color;

void main() {
        frag_color = texture(image, v_texpos);
}
