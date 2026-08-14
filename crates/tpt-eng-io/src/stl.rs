//! STL read/write functionality.
//!
//! Provides a small, format-agnostic model (`StlMesh`, `StlTriangle`, `StlVertex`)
//! plus convenience functions to read and write binary and ASCII STL files built on
//! top of the [`stl_io`] crate.

use crate::error::{Error, Result};
use stl_io::{self, IndexedMesh, IndexedTriangle, Normal, Triangle, Vertex};
use std::io::Write;
use std::path::Path;

/// A vertex in an STL mesh.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StlVertex {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
}

impl From<Vertex> for StlVertex {
    fn from(v: Vertex) -> Self {
        let [x, y, z] = v.0;
        Self { x, y, z }
    }
}

impl From<StlVertex> for Vertex {
    fn from(v: StlVertex) -> Self {
        Vertex::new([v.x, v.y, v.z])
    }
}

/// A triangle in an STL mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct StlTriangle {
    /// The normal vector.
    pub normal: StlVertex,
    /// The three vertices.
    pub vertices: [StlVertex; 3],
}

impl From<Triangle> for StlTriangle {
    fn from(t: Triangle) -> Self {
        Self {
            normal: StlVertex::from(t.normal),
            vertices: [
                StlVertex::from(t.vertices[0]),
                StlVertex::from(t.vertices[1]),
                StlVertex::from(t.vertices[2]),
            ],
        }
    }
}

impl From<StlTriangle> for Triangle {
    fn from(t: StlTriangle) -> Self {
        Self {
            normal: Normal::new([t.normal.x, t.normal.y, t.normal.z]),
            vertices: [
                Vertex::from(t.vertices[0]),
                Vertex::from(t.vertices[1]),
                Vertex::from(t.vertices[2]),
            ],
        }
    }
}

/// An STL mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct StlMesh {
    /// The triangles in the mesh.
    pub triangles: Vec<StlTriangle>,
}

impl StlMesh {
    /// Create a new empty STL mesh.
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    /// Create an STL mesh from triangles.
    pub fn from_triangles(triangles: Vec<StlTriangle>) -> Self {
        Self { triangles }
    }

    /// Get the number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Add a triangle to the mesh.
    pub fn add_triangle(&mut self, triangle: StlTriangle) {
        self.triangles.push(triangle);
    }
}

impl Default for StlMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl From<IndexedMesh> for StlMesh {
    fn from(mesh: IndexedMesh) -> Self {
        let triangles = mesh
            .faces
            .iter()
            .map(|face| StlTriangle {
                normal: StlVertex::from(face.normal),
                vertices: [
                    StlVertex::from(mesh.vertices[face.vertices[0]]),
                    StlVertex::from(mesh.vertices[face.vertices[1]]),
                    StlVertex::from(mesh.vertices[face.vertices[2]]),
                ],
            })
            .collect();
        Self { triangles }
    }
}

impl From<StlMesh> for IndexedMesh {
    fn from(mesh: StlMesh) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<IndexedTriangle> = Vec::new();

        for triangle in mesh.triangles {
            let v0_idx = vertices.len();
            vertices.push(Vertex::from(triangle.vertices[0]));
            let v1_idx = vertices.len();
            vertices.push(Vertex::from(triangle.vertices[1]));
            let v2_idx = vertices.len();
            vertices.push(Vertex::from(triangle.vertices[2]));

            faces.push(IndexedTriangle {
                normal: Normal::new([
                    triangle.normal.x,
                    triangle.normal.y,
                    triangle.normal.z,
                ]),
                vertices: [v0_idx, v1_idx, v2_idx],
            });
        }

        IndexedMesh { vertices, faces }
    }
}

/// Read an STL file (binary or ASCII).
pub fn read_stl<P: AsRef<Path>>(path: P) -> Result<StlMesh> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mesh = stl_io::read_stl(&mut reader)?;
    Ok(mesh.into())
}

/// Write an STL file in binary format.
pub fn write_stl<P: AsRef<Path>>(mesh: &StlMesh, path: P) -> Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    let indexed_mesh: IndexedMesh = mesh.clone().into();
    let triangles: Vec<Triangle> = indexed_mesh.into_triangle_vec();
    stl_io::write_stl(&mut writer, triangles.iter()).map_err(|e| Error::Stl(e.to_string()))?;
    Ok(())
}

/// Write an STL file in ASCII format.
///
/// `stl_io` only supports binary writing, so this emits the ASCII dialect directly.
pub fn write_stl_ascii<P: AsRef<Path>>(mesh: &StlMesh, path: P) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "solid tpt_eng_io").map_err(|e| Error::Stl(e.to_string()))?;
    for triangle in &mesh.triangles {
        writeln!(
            file,
            "  facet normal {} {} {}",
            triangle.normal.x, triangle.normal.y, triangle.normal.z
        )
        .map_err(|e| Error::Stl(e.to_string()))?;
        writeln!(file, "    outer loop").map_err(|e| Error::Stl(e.to_string()))?;
        for vertex in &triangle.vertices {
            writeln!(file, "      vertex {} {} {}", vertex.x, vertex.y, vertex.z)
                .map_err(|e| Error::Stl(e.to_string()))?;
        }
        writeln!(file, "    endloop").map_err(|e| Error::Stl(e.to_string()))?;
        writeln!(file, "  endfacet").map_err(|e| Error::Stl(e.to_string()))?;
    }
    writeln!(file, "endsolid tpt_eng_io").map_err(|e| Error::Stl(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn sample_mesh() -> StlMesh {
        let mut mesh = StlMesh::new();
        mesh.add_triangle(StlTriangle {
            normal: StlVertex {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            vertices: [
                StlVertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                StlVertex {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                StlVertex {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
        });
        mesh
    }

    #[test]
    fn test_stl_roundtrip_binary() {
        let mesh = sample_mesh();
        let file = NamedTempFile::new().unwrap();
        write_stl(&mesh, file.path()).unwrap();

        let loaded = read_stl(file.path()).unwrap();
        assert_eq!(mesh.triangle_count(), loaded.triangle_count());
        assert_eq!(mesh.triangles[0].normal, loaded.triangles[0].normal);
        assert_eq!(mesh.triangles[0].vertices, loaded.triangles[0].vertices);
    }

    #[test]
    fn test_stl_roundtrip_ascii() {
        let mesh = sample_mesh();
        let file = NamedTempFile::new().unwrap();
        write_stl_ascii(&mesh, file.path()).unwrap();

        let loaded = read_stl(file.path()).unwrap();
        assert_eq!(mesh.triangle_count(), loaded.triangle_count());
        assert_eq!(mesh.triangles[0].normal, loaded.triangles[0].normal);
        assert_eq!(mesh.triangles[0].vertices, loaded.triangles[0].vertices);
    }
}
