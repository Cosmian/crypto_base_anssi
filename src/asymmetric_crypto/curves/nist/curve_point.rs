use std::ops::{Add, Sub};

use elliptic_curve::{
    sec1::{self, ToEncodedPoint},
    Curve, CurveArithmetic,
};

pub struct NistCurvePoint<C>(pub(crate) C::AffinePoint)
where
    C: Curve + CurveArithmetic,
    C::AffinePoint: sec1::ToEncodedPoint<C>,
    <C as Curve>::FieldBytesSize: sec1::ModulusSize;

impl<C> NistCurvePoint<C>
where
    C: Curve + CurveArithmetic,
    C::AffinePoint: sec1::ToEncodedPoint<C>,
    <C as Curve>::FieldBytesSize: sec1::ModulusSize,
{
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_encoded_point(true).as_bytes().to_vec()
    }
}

impl<C> Add<&NistCurvePoint<C>> for &NistCurvePoint<C>
where
    C: Curve + CurveArithmetic,
    C::AffinePoint: sec1::ToEncodedPoint<C>,
    <C as Curve>::FieldBytesSize: sec1::ModulusSize,
{
    type Output = NistCurvePoint<C>;

    fn add(self, rhs: &NistCurvePoint<C>) -> Self::Output {
        NistCurvePoint(
            C::ProjectivePoint::from(self.0)
                .add(C::ProjectivePoint::from(rhs.0))
                .into(),
        )
    }
}

impl<C> Sub<&NistCurvePoint<C>> for &NistCurvePoint<C>
where
    C: Curve + CurveArithmetic,
    C::AffinePoint: sec1::ToEncodedPoint<C>,
    <C as Curve>::FieldBytesSize: sec1::ModulusSize,
{
    type Output = NistCurvePoint<C>;

    fn sub(self, rhs: &NistCurvePoint<C>) -> Self::Output {
        NistCurvePoint(
            C::ProjectivePoint::from(self.0)
                .sub(C::ProjectivePoint::from(rhs.0))
                .into(),
        )
    }
}
