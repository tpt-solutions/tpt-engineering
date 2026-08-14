//! OBJ read/write functionality.
//!
//! Provides a small, format-agnostic model (`ObjMesh`, `ObjFace`, `ObjVertex`,
//! `ObjNormal`, `ObjTexCoord`) plus convenience functions to read and write Wavefront
//! OBJ files built on top of the [`obj`] crate.

use crate::error::{Error, Result};
use obj::{self, IndexTuple, Obj, ObjData, SimplePolygon};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

/// A vertex in an OBJ mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjVertex {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
}

impl From<[f32; 3]> for ObjVertex {
    fn from(v: [f32; 3]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl From<ObjVertex> for [f32; 3] {
    fn from(v: ObjVertex) -> Self {
        [v.x, v.y, v.z]
    }
}

/// A texture coordinate in an OBJ mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjTexCoord {
    /// U coordinate.
    pub u: f32,
    /// V coordinate.
    pub v: f32,
}

impl From<[f32; 2]> for ObjTexCoord {
    fn from(t: [f32; 2]) -> Self {
        Self { u: t[0], v: t[1] }
    }
}

impl From<ObjTexCoord> for [f32; 2] {
    fn from(t: ObjTexCoord) -> Self {
        [t.u, t.v]
    }
}

/// A normal in an OBJ mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjNormal {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl From<[f32; 3]> for ObjNormal {
    fn from(n: [f32; 3]) -> Self {
        Self {
            x: n[0],
            y: n[1],
            z: n[2],
        }
    }
}

impl From<ObjNormal> for [f32; 3] {
    fn from(n: ObjNormal) -> Self {
        [n.x, n.y, n.z]
    }
}

/// A face in an OBJ mesh.
///
/// Indices are stored 0-based (as in Rust) and converted to the 1-based OBJ
/// representation on write.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjFace {
    /// Vertex indices (0-based).
    pub vertex_indices: Vec<usize>,
    /// Texture coordinate indices (0-based, optional).
    pub tex_coord_indices: Vec<usize>,
    /// Normal indices (0-based, optional).
    pub normal_indices: Vec<usize>,
}

/// An OBJ mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjMesh {
    /// The vertices in the mesh.
    pub vertices: Vec<ObjVertex>,
    /// The texture coordinates.
    pub tex_coords: Vec<ObjTexCoord>,
    /// The normals.
    pub normals: Vec<ObjNormal>,
    /// The faces.
    pub faces: Vec<ObjFace>,
    /// The name of the object (optional).
    pub name: Option<String>,
    /// The material library (optional).
    pub material_lib: Option<String>,
}

impl ObjMesh {
    /// Create a new empty OBJ mesh.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
            name: None,
            material_lib: None,
        }
    }

    /// Get the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of faces.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Add a vertex.
    pub fn add_vertex(&mut self, vertex: ObjVertex) {
        self.vertices.push(vertex);
    }

    /// Add a texture coordinate.
    pub fn add_tex_coord(&mut self, tex_coord: ObjTexCoord) {
        self.tex_coords.push(tex_coord);
    }

    /// Add a normal.
    pub fn add_normal(&mut self, normal: ObjNormal) {
        self.normals.push(normal);
    }

    /// Add a face.
    pub fn add_face(&mut self, face: ObjFace) {
        self.faces.push(face);
    }
}

impl Default for ObjMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Obj> for ObjMesh {
    fn from(obj: Obj) -> Self {
        let mut mesh = Self::new();

        for position in &obj.data.position {
            mesh.vertices.push(ObjVertex::from(*position));
        }
        for texture in &obj.data.texture {
            mesh.tex_coords.push(ObjTexCoord::from(*texture));
        }
        for normal in &obj.data.normal {
            mesh.normals.push(ObjNormal::from(*normal));
        }
        for object in &obj.data.objects {
            for group in &object.groups {
                for poly in &group.polys {
                    let vertex_indices = poly.0.iter().map(|it| it.0).collect();
                    let tex_coord_indices = poly.0.iter().filter_map(|it| it.1).collect();
                    let normal_indices = poly.0.iter().filter_map(|it| it.2).collect();
                    mesh.faces.push(ObjFace {
                        vertex_indices,
                        tex_coord_indices,
                        normal_indices,
                    });
                }
            }
        }

        mesh.name = obj.data.objects.first().map(|o| o.name.clone());
        mesh.material_lib = obj.data.material_libs.first().map(|m| m.filename.clone());

        mesh
    }
}

impl From<ObjMesh> for Obj {
    fn from(mesh: ObjMesh) -> Self {
        let material_libs = mesh
            .material_lib
            .into_iter()
            .map(obj::Mtl::new)
            .collect();

        let mut object = obj::Object::new(mesh.name.unwrap_or_else(|| "default".to_string()));
        let mut group = obj::Group::new("default".to_string());
        for face in mesh.faces {
            let max_len = face
                .vertex_indices
                .len()
                .max(face.tex_coord_indices.len())
                .max(face.normal_indices.len());
            let mut tuples = Vec::with_capacity(max_len);
            for i in 0..max_len {
                let v = face.vertex_indices.get(i).copied().unwrap_or(0);
                let vt = face.tex_coord_indices.get(i).copied();
                let vn = face.normal_indices.get(i).copied();
                tuples.push(IndexTuple(v, vt, vn));
            }
            group.polys.push(SimplePolygon(tuples));
        }
        object.groups.push(group);

        let data = ObjData {
            position: mesh.vertices.iter().map(|v| [v.x, v.y, v.z]).collect(),
            texture: mesh.tex_coords.iter().map(|t| [t.u, t.v]).collect(),
            normal: mesh.normals.iter().map(|n| [n.x, n.y, n.z]).collect(),
            objects: vec![object],
            material_libs,
        };

        Obj {
            data,
            path: PathBuf::new(),
        }
    }
}

/// Read an OBJ file.
pub fn read_obj<P: AsRef<Path>>(path: P) -> Result<ObjMesh> {
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let data = ObjData::load_buf(&mut reader).map_err(|e| Error::Obj(e.to_string()))?;
    let obj = Obj {
        data,
        path: PathBuf::new(),
    };
    Ok(obj.into())
}

/// Write an OBJ file.
///
/// Serialized manually so that the `v//vn` (vertex + normal, no texture) form is emitted
/// correctly. The `obj` crate's `IndexTuple` display is ambiguous in that case, so it cannot be
/// relied on for round-tripping normals without texture coordinates.
pub fn write_obj<P: AsRef<Path>>(mesh: &ObjMesh, path: P) -> Result<()> {
    let mut file = std::fs::File::create(path)?;

    let w = |e: std::io::Error| Error::Obj(e.to_string());

    writeln!(file, "# Generated by tpt-eng-io").map_err(w)?;

    for vertex in &mesh.vertices {
        writeln!(file, "v {} {} {}", vertex.x, vertex.y, vertex.z).map_err(w)?;
    }
    for tex_coord in &mesh.tex_coords {
        writeln!(file, "vt {} {}", tex_coord.u, tex_coord.v).map_err(w)?;
    }
    for normal in &mesh.normals {
        writeln!(file, "vn {} {} {}", normal.x, normal.y, normal.z).map_err(w)?;
    }
    if let Some(name) = &mesh.name {
        writeln!(file, "o {}", name).map_err(w)?;
    }
    if let Some(material_lib) = &mesh.material_lib {
        writeln!(file, "mtllib {}", material_lib).map_err(w)?;
    }

    for face in &mesh.faces {
        let mut line = String::from("f");
        let count = face.vertex_indices.len();
        for i in 0..count {
            let v = face.vertex_indices[i] + 1;
            let vt = face.tex_coord_indices.get(i).copied();
            let vn = face.normal_indices.get(i).copied();
            match (vt, vn) {
                (Some(vt), Some(vn)) => line.push_str(&format!(" {}/{}/{}", v, vt + 1, vn + 1)),
                (Some(vt), None) => line.push_str(&format!(" {}/{}", v, vt + 1)),
                (None, Some(vn)) => line.push_str(&format!(" {}//{}", v, vn + 1)),
                (None, None) => line.push_str(&format!(" {}", v)),
            }
        }
        writeln!(file, "{}", line).map_err(w)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_obj_roundtrip() {
        let mut mesh = ObjMesh::new();
        mesh.name = Some("test_cube".to_string());

        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        });

        mesh.add_face(ObjFace {
            vertex_indices: vec![0, 1, 2, 3],
            tex_coord_indices: vec![],
            normal_indices: vec![],
        });

        let file = NamedTempFile::new().unwrap();
        write_obj(&mesh, file.path()).unwrap();

        let loaded = read_obj(file.path()).unwrap();
        assert_eq!(mesh.vertex_count(), loaded.vertex_count());
        assert_eq!(mesh.face_count(), loaded.face_count());
        assert_eq!(mesh.name, loaded.name);
        assert_eq!(mesh.vertices, loaded.vertices);
        assert_eq!(mesh.faces, loaded.faces);
    }

    #[test]
    fn test_obj_roundtrip_with_normals() {
        let mut mesh = ObjMesh::new();

        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        mesh.add_vertex(ObjVertex {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        mesh.add_normal(ObjNormal {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

        mesh.add_face(ObjFace {
            vertex_indices: vec![0, 1, 2],
            tex_coord_indices: vec![],
            normal_indices: vec![0, 0, 0],
        });

        let file = NamedTempFile::new().unwrap();
        write_obj(&mesh, file.path()).unwrap();

        let loaded = read_obj(file.path()).unwrap();
        assert_eq!(mesh.vertices, loaded.vertices);
        assert_eq!(mesh.normals, loaded.normals);
        assert_eq!(mesh.faces, loaded.faces);
    }
}
