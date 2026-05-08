use core::fmt;
use std::error::Error;

/// Errors returned by validated geometry constructors.
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryError {
    /// A geometry component must be finite.
    ///
    /// The type name, component name, and invalid value are included for
    /// diagnostics.
    NonFiniteComponent {
        /// The type that rejected the invalid component.
        type_name: &'static str,
        /// The component that rejected the invalid value.
        component: &'static str,
        /// The invalid component value.
        value: f64,
    },
    /// A circle radius cannot be negative.
    ///
    /// The invalid radius value is included for diagnostics.
    NegativeRadius(f64),
    /// A circle radius must be finite.
    ///
    /// The invalid radius value is included for diagnostics.
    NonFiniteRadius(f64),
    /// A tolerance must be finite.
    ///
    /// The invalid tolerance value is included for diagnostics.
    NonFiniteTolerance(f64),
    /// A tolerance cannot be negative.
    ///
    /// The invalid tolerance value is included for diagnostics.
    NegativeTolerance(f64),
    /// Distinct points were required but identical points were supplied.
    IdenticalPoints,
    /// A non-zero direction vector was required.
    ZeroDirectionVector,
    /// An axis-aligned bounding box corner ordering was invalid.
    InvalidBounds {
        /// The minimum x coordinate.
        min_x: f64,
        /// The minimum y coordinate.
        min_y: f64,
        /// The maximum x coordinate.
        max_x: f64,
        /// The maximum y coordinate.
        max_y: f64,
    },
}

impl GeometryError {
    pub(crate) const fn non_finite_component(
        type_name: &'static str,
        component: &'static str,
        value: f64,
    ) -> Self {
        Self::NonFiniteComponent {
            type_name,
            component,
            value,
        }
    }

    pub(crate) const fn validate_tolerance(tolerance: f64) -> Result<f64, Self> {
        if !tolerance.is_finite() {
            return Err(Self::NonFiniteTolerance(tolerance));
        }

        if tolerance < 0.0 {
            return Err(Self::NegativeTolerance(tolerance));
        }

        Ok(tolerance)
    }
}

impl fmt::Display for GeometryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteComponent {
                type_name,
                component,
                value,
            } => write!(
                formatter,
                "{type_name} {component} component must be finite, got {value}"
            ),
            Self::NegativeRadius(value) => {
                write!(formatter, "circle radius must be non-negative, got {value}")
            },
            Self::NonFiniteRadius(value) => {
                write!(formatter, "circle radius must be finite, got {value}")
            },
            Self::NonFiniteTolerance(value) => {
                write!(formatter, "tolerance must be finite, got {value}")
            },
            Self::NegativeTolerance(value) => {
                write!(formatter, "tolerance must be non-negative, got {value}")
            },
            Self::IdenticalPoints => write!(formatter, "points must be distinct"),
            Self::ZeroDirectionVector => write!(formatter, "direction vector must be non-zero"),
            Self::InvalidBounds {
                min_x,
                min_y,
                max_x,
                max_y,
            } => write!(
                formatter,
                "aabb min must not exceed max, got min=({min_x}, {min_y}) and max=({max_x}, {max_y})"
            ),
        }
    }
}

impl Error for GeometryError {}
