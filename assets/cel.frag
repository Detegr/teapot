#version 140

in vec3 camera_normal;
in vec3 camera_lightdir;
in vec3 lightpos;
in vec3 eyedir_to_camera;
in vec3 vertex_world;
out vec4 color;

void main() {
	vec3 n = normalize(camera_normal);
	vec3 l = normalize(camera_lightdir);
	vec3 e = normalize(eyedir_to_camera);
	vec3 r = reflect(-camera_lightdir, n);
	float cosAlpha = clamp(dot(e,r), 0, 1);
	float cosTheta = clamp(dot(n,l), 0, 1);
	vec3 ambient = vec3(0.01, 0.0, 0.01);
	vec3 diffusecolor = vec3(1.0, 0.0, 1.0);
	vec3 specularcolor = vec3(0.9, 0.5, 0.9);
	vec3 lightcolor = vec3(1.0, 1.0, 1.0);
	float distance = length(lightpos - vertex_world);
	float lightpower = 0.75;
	color = vec4(
			ambient +
			(diffusecolor * lightcolor * 10.0 * cosTheta / (distance*distance)) +
			(specularcolor * lightcolor * 1.0 * pow(cosAlpha, 1) / (distance*distance))
			, 1.0);
}
