# tpt-eng-sections

Cross-section properties for the TPT engineering ecosystem.

Every section implements the [`Section`](section::Section) trait, which exposes
area, centroid, centroidal second moments, elastic/plastic section moduli, and
the torsional constant. Properties are assembled into a single
[`SectionProperties`](properties::SectionProperties) bundle via
[`Section::properties`](section::Section::properties).

Supported section types ([`shapes`](shapes)):

- [`Rectangle`](shapes::Rectangle)
- [`Circle`](shapes::Circle)
- [`Tube`](shapes::Tube) (circular hollow)
- [`ISection`](shapes::ISection)
- [`Channel`](shapes::Channel)
- [`Angle`](shapes::Angle)
- [`CustomPolygon`](polygon::CustomPolygon) (arbitrary simply-connected polygon)

Composite sections (I-section, channel, angle) are evaluated by rectangle
decomposition ([`compose`](compose)); arbitrary polygons use exact
Green's-theorem formulas for area/centroid/second moments, with plastic moduli
and torsion computed on a grid confined to the polygon.

All quantities are reported in the section's own consistent length units; the
caller is responsible for unit consistency (integration with `tpt-eng-units` is
deferred).

Geometry integration with `tpt-eng3` (`tpt-eng-geometry`) is deferred until that
repository/crate exists.

## License

Dual-licensed under [MIT](../../LICENSE-MIT) OR
[Apache-2.0](../../LICENSE-APACHE).
