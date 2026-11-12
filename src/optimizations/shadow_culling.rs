use glam::{Mat4, Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct FrustumPlane {
    pub normal: Vec3,
    pub distance: f32,
}

impl FrustumPlane {
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

pub struct ShadowFrustumCuller {
    pub planes: [FrustumPlane; 6],
}

impl ShadowFrustumCuller {
    /// Estrae i 6 piani del frustum di proiezione delle ombre dalla matrice ViewProjection della luce
    pub fn from_view_projection(vp: &Mat4) -> Self {
        let cols = vp.to_cols_array_2d();
        let r0 = Vec4::from_array(cols[0]);
        let r1 = Vec4::from_array(cols[1]);
        let r2 = Vec4::from_array(cols[2]);
        let r3 = Vec4::from_array(cols[3]);

        let mut make_plane = |v: Vec4| -> FrustumPlane {
            let normal = Vec3::new(v.x, v.y, v.z);
            let len = normal.length();
            if len > 0.0 {
                FrustumPlane {
                    normal: normal / len,
                    distance: v.w / len,
                }
            } else {
                FrustumPlane {
                    normal: Vec3::Y,
                    distance: 0.0,
                }
            }
        };

        // Piani: Sinistra, Destra, Basso, Alto, Vicino, Lontano
        let planes = [
            make_plane(r3 + r0),
            make_plane(r3 - r0),
            make_plane(r3 + r1),
            make_plane(r3 - r1),
            make_plane(r3 + r2),
            make_plane(r3 - r2),
        ];

        Self { planes }
    }

    /// Controlla se l'AABB (Axis-Aligned Bounding Box) di un chunk (16x16x16) è visibile nel frustum ombre
    pub fn is_chunk_visible(&self, min: Vec3, max: Vec3) -> bool {
        for plane in &self.planes {
            let p = Vec3::new(
                if plane.normal.x > 0.0 { max.x } else { min.x },
                if plane.normal.y > 0.0 { max.y } else { min.y },
                if plane.normal.z > 0.0 { max.z } else { min.z },
            );

            if plane.distance_to_point(p) < 0.0 {
                return false;
            }
        }
        true
    }
}
