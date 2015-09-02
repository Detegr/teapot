#version 140

uniform mat4 m_perspective;
uniform mat4 m_view;
uniform mat4 m_model;
uniform vec3 in_lightpos;

in vec3 in_position;
in vec3 in_normal;

out vec3 camera_lightdir;
out vec3 camera_normal;
out vec3 eyedir_to_camera;
out vec3 lightpos;
out vec3 normal;
out vec3 vertex_world;

void main() {
	normal = in_normal;
	lightpos = in_lightpos;
	vertex_world = (m_model * vec4(in_position, 1)).xyz;
	vec3 camera_vertexpos = (m_view * vec4(vertex_world,1)).xyz;
	eyedir_to_camera = vec3(0,0,0) - camera_vertexpos;
	camera_lightdir = (m_view * vec4(lightpos, 1)).xyz + eyedir_to_camera;
	camera_normal = (m_view * m_model * vec4(in_normal, 0)).xyz;
	gl_Position = m_perspective * vec4(camera_vertexpos, 1.0);
}
