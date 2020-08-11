use crate::rust_tracer::math::point::Point;
use crate::rust_tracer::math::ray::Ray;
use crate::rust_tracer::intersection::Intersection;
use crate::rust_tracer::{materials::material::Material, math::{Vec4, Mat4}, lights::point_light::PointLight, utils::math::reflection};
use super::color::Color;


pub struct Sphere {
    pub transform: Mat4,
    pub id: i32,
    pub material: Material,

    inverse_transform: Mat4
}

impl Sphere {
    pub fn new(transform: Mat4, id: i32, material: Material) -> Sphere {
        let inverse_transform = transform.try_inverse();

        match inverse_transform {
            Some(x) => {
                Sphere {transform, id, material, inverse_transform: x}
            },
            None => {
                Sphere { transform, id, material, inverse_transform: Mat4::identity()}
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = Ray::transform(&ray, &self.inverse_transform);
        let mut intersections = Vec::with_capacity(2);
        let sphere_to_ray = &transformed_ray.origin - &Point::new(Vec4::new(0.0, 0.0, 0.0, 1.0));
        let a = transformed_ray.direction.dot(&(transformed_ray.direction));
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            intersections
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 > t2 {
                intersections.push(Intersection { t: t2, object_intersected: self.id });
                intersections.push(Intersection { t: t1, object_intersected: self.id });
            } else {
                intersections.push(Intersection { t: t1, object_intersected: self.id });
                intersections.push(Intersection { t: t2, object_intersected: self.id });
            }
            intersections
        }
    }

    pub fn normal_at(&self, point: &Point) -> Vec4 {
        let object_point = &self.inverse_transform * &point.point;
        let object_normal = object_point - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let mut world_normal = &self.inverse_transform.transpose() * object_normal;
        world_normal.w = 0.0;
        return world_normal.normalize();
    }

    pub fn phong_lighting(&self, point: &Point, light: &PointLight, eye: &Point) -> Color {
        let eye_v = point - eye;
        let normal_v = self.normal_at(point);
        let light_v = point.point - light.position.point;
        let light_v = light_v.normalize();

        let effective_color = &self.material.color * &light.intensity;
        let ambient = &effective_color * self.material.ambient;

        let light_dot_normal = light_v.dot(&normal_v);

        let mut diffuse = Color::new(Vec4::new(0.0, 0.0, 0.0, 0.0));
        let mut specular = Color::new(Vec4::new(0.0, 0.0, 0.0, 0.0));

        if light_dot_normal >= 0.0 {
            diffuse = &effective_color * (self.material.diffuse * light_dot_normal);
            let reflect_v = reflection(&-light_v, &normal_v);
            let reflect_dot_eye = reflect_v.dot(&eye_v);

            if reflect_dot_eye > 0.0 {
                let factor = self.material.shininess.powf(reflect_dot_eye);
                specular = &light.intensity * (self.material.specular * factor);
            }
        }
        return ambient + diffuse;
    }
}