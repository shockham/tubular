use caper::shader::default;
use caper::game::Game;

pub fn add_custom_shaders(game: &mut Game) {
    let mut shaders = &mut game.renderer.shaders;
    let display = &game.renderer.display;
    let _ = shaders.add_shader(display, "tubular",
                               default::gl330::VERT, tubular::FRAG,
                               tubular::GEOM, tubular::TESS_CONTROL,
                               tubular::TESS_EVAL);
}


mod tubular {
    // fragment shader
    pub const FRAG: &'static str =
        "
        #version 330

        uniform vec3 cam_pos;
        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        void main() {
            float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
            float dist = abs(distance(cam_pos, g_pos)) / 400.0;
            float norm_height = normalize(g_pos).y;
            float height = g_pos.y / 100.0;
            float col_val = clamp(norm_height, 0.1, 1.0);

            vec3 base_color = vec3(col_val);
            
            // water (first so that distance roll off doesn't affect)
            //base_color.gb /= max(smoothstep(0.0, 0.2, height), 0.0001);

            // fade to white on distance
            base_color = mix(base_color, vec3(1.0), dist);

            // color mode, comment out for more monochrome
            //base_color /= 0.5 + (normalize(g_normal) * 0.1);

            vec3 color = base_color * ((0.05 * lum) + (0.95 * dist));
            frag_output = vec4(color, 1.0);
        }
    ";

    // geometry shader
    pub const GEOM: &'static str =
        "
        #version 330

        layout(triangles) in;
        layout(triangle_strip, max_vertices=3) out;

        in vec3 te_normal[];
        in vec3 te_pos[];
        in vec2 te_texture[];

        out vec3 g_normal;
        out vec3 g_pos;
        out vec2 g_texture;

        vec3 calc_normal (vec3 p0, vec3 p1, vec3 p2) {
            return cross(p0 - p1, p0 - p2);
        }

        void main(void) {
            vec3 norm = calc_normal(te_pos[0], te_pos[1], te_pos[2]);

            for(int i = 0; i < gl_in.length(); i++){
                g_normal = norm;
                g_pos = te_pos[i];
                g_texture = te_texture[i];
                gl_Position = gl_in[i].gl_Position;
                EmitVertex();
            }
            EndPrimitive();
        }
    ";

    // tessellation control shader
    pub const TESS_CONTROL: &'static str =
        "
        #version 400

        layout(vertices = 3) out;

        in vec3 v_normal[];
        in vec2 v_texture[];

        out vec3 tc_normal[];
        out vec2 tc_texture[];

        uniform vec3 cam_pos;
        const float tess_level = 6.0;
        const float inner_range = 200.0;

        void main() {
            tc_normal[gl_InvocationID] = v_normal[gl_InvocationID];
            tc_texture[gl_InvocationID] = v_texture[gl_InvocationID];
            gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;

            float dist = abs(distance(cam_pos, vec3(gl_in[gl_InvocationID].gl_Position)));

            float inner_tess = 1.0 +
                (step(dist, inner_range) * (((inner_range - dist) / inner_range) * 10.0));

            gl_TessLevelOuter[0] = tess_level;
            gl_TessLevelOuter[1] = tess_level;
            gl_TessLevelOuter[2] = tess_level;
            gl_TessLevelInner[0] = inner_tess;
        }
    ";

    // tessellation evaluation shader
    pub const TESS_EVAL: &'static str =
        "
        #version 400

        uniform mat4 projection_matrix;
        uniform mat4 modelview_matrix;
        uniform float time;

        layout(triangles, equal_spacing, ccw) in;

        in vec3 tc_normal[];
        in vec2 tc_texture[];

        out vec3 te_normal;
        out vec3 te_pos;
        out vec2 te_texture;

        float rand (vec2 s) {
            return fract(sin(dot(s, vec2(12.9898, 78.233))) * 43758.5453);
        }

        float rand (vec3 s) {
            return fract(sin(dot(s, vec3(12.9898, 78.233, 54.1232))) * 4.5453);
        }

        vec3 tess_calc (vec3 one, vec3 two, vec3 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        vec3 calc_normal (vec3 p0, vec3 p1, vec3 p2) {
            return cross(p0 - p1, p0 - p2);
        }

        vec2 tex_calc (vec2 one, vec2 two, vec2 three) {
            return ((gl_TessCoord.x) * one) +
                            ((gl_TessCoord.y) * two) +
                            ((gl_TessCoord.z) * three);
        }

        void main () {
            te_normal = calc_normal(gl_in[0].gl_Position.xyz,
                gl_in[1].gl_Position.xyz,
                gl_in[2].gl_Position.xyz);

            vec3 position = tess_calc(gl_in[0].gl_Position.xyz,
                gl_in[1].gl_Position.xyz,
                gl_in[2].gl_Position.xyz);

            position.y += rand(normalize(position.xyz) + 
                cos(time)) * (0.9 + (0.1 * sin(time))) + 
                sin((time * 4.0) + position.x + position.z) * sin(time) * 8.0;

            position.x *= cos(time + position.x) * 2.0; 
            position.z *= sin(time + position.z) * 2.0;

            te_pos = position;

            vec2 texture = tex_calc(tc_texture[0], tc_texture[1], tc_texture[2]);
            te_texture = texture;

            gl_Position = projection_matrix *
                modelview_matrix *
                vec4(position, 1.0);
        }
    ";
}
