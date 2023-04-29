#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64
}

// FIXME: macro this junk

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x - other.x < f64::EPSILON &&
            self.y - other.y < f64::EPSILON &&
            self.z - other.z < f64::EPSILON
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x - other.x < f64::EPSILON &&
            self.y - other.y < f64::EPSILON &&
            self.z - other.z < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{Point, Vector};

    #[test]
    fn test_add() {
        assert_eq!(Point {
            x: 3.0, y: -2.0, z: 5.0
        } + Vector {
            x: -2.0, y: 3.0, z: 1.0
        }, Point {
            x: 1.0, y: 1.0, z: 6.0
        });

        assert_eq!(Vector {
            x: 3.0, y: -2.0, z: 5.0
        } + Vector {
            x: -2.0, y: 3.0, z: 1.0
        }, Vector {
            x: 1.0, y: 1.0, z: 6.0
        })
    }

    #[test]
    fn test_sub() {
        assert_eq!(Point {
            x: 3.0, y: 2.0, z: 1.0
        } - Point {
            x: 5.0, y: 6.0, z: 7.0
        }, Vector {
            x: -2.0, y: -4.0, z: -4.0
        });

        assert_eq!(Point {
            x: 3.0, y: 2.0, z: 1.0
        } - Vector {
            x: 5.0, y: 6.0, z: 7.0
        }, Point {
            x: -2.0, y: -4.0, z: -4.0
        });

        assert_eq!(Vector {
            x: 3.0, y: 2.0, z: 1.0
        } - Vector {
            x: 5.0, y: 6.0, z: 7.0
        }, Vector {
            x: -2.0, y: -4.0, z: -4.0
        })
    }
}