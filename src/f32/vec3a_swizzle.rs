// Generated by swizzlegen. Do not edit.

use super::{Vec2, Vec3A, Vec4};

#[cfg(vec3a_f32)]
use super::Vec3;

#[cfg(all(vec3a_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec3a_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;

pub trait Vec3ASwizzles {
    fn xxxx(self) -> Vec4;
    fn xxxy(self) -> Vec4;
    fn xxxz(self) -> Vec4;
    fn xxyx(self) -> Vec4;
    fn xxyy(self) -> Vec4;
    fn xxyz(self) -> Vec4;
    fn xxzx(self) -> Vec4;
    fn xxzy(self) -> Vec4;
    fn xxzz(self) -> Vec4;
    fn xyxx(self) -> Vec4;
    fn xyxy(self) -> Vec4;
    fn xyxz(self) -> Vec4;
    fn xyyx(self) -> Vec4;
    fn xyyy(self) -> Vec4;
    fn xyyz(self) -> Vec4;
    fn xyzx(self) -> Vec4;
    fn xyzy(self) -> Vec4;
    fn xyzz(self) -> Vec4;
    fn xzxx(self) -> Vec4;
    fn xzxy(self) -> Vec4;
    fn xzxz(self) -> Vec4;
    fn xzyx(self) -> Vec4;
    fn xzyy(self) -> Vec4;
    fn xzyz(self) -> Vec4;
    fn xzzx(self) -> Vec4;
    fn xzzy(self) -> Vec4;
    fn xzzz(self) -> Vec4;
    fn yxxx(self) -> Vec4;
    fn yxxy(self) -> Vec4;
    fn yxxz(self) -> Vec4;
    fn yxyx(self) -> Vec4;
    fn yxyy(self) -> Vec4;
    fn yxyz(self) -> Vec4;
    fn yxzx(self) -> Vec4;
    fn yxzy(self) -> Vec4;
    fn yxzz(self) -> Vec4;
    fn yyxx(self) -> Vec4;
    fn yyxy(self) -> Vec4;
    fn yyxz(self) -> Vec4;
    fn yyyx(self) -> Vec4;
    fn yyyy(self) -> Vec4;
    fn yyyz(self) -> Vec4;
    fn yyzx(self) -> Vec4;
    fn yyzy(self) -> Vec4;
    fn yyzz(self) -> Vec4;
    fn yzxx(self) -> Vec4;
    fn yzxy(self) -> Vec4;
    fn yzxz(self) -> Vec4;
    fn yzyx(self) -> Vec4;
    fn yzyy(self) -> Vec4;
    fn yzyz(self) -> Vec4;
    fn yzzx(self) -> Vec4;
    fn yzzy(self) -> Vec4;
    fn yzzz(self) -> Vec4;
    fn zxxx(self) -> Vec4;
    fn zxxy(self) -> Vec4;
    fn zxxz(self) -> Vec4;
    fn zxyx(self) -> Vec4;
    fn zxyy(self) -> Vec4;
    fn zxyz(self) -> Vec4;
    fn zxzx(self) -> Vec4;
    fn zxzy(self) -> Vec4;
    fn zxzz(self) -> Vec4;
    fn zyxx(self) -> Vec4;
    fn zyxy(self) -> Vec4;
    fn zyxz(self) -> Vec4;
    fn zyyx(self) -> Vec4;
    fn zyyy(self) -> Vec4;
    fn zyyz(self) -> Vec4;
    fn zyzx(self) -> Vec4;
    fn zyzy(self) -> Vec4;
    fn zyzz(self) -> Vec4;
    fn zzxx(self) -> Vec4;
    fn zzxy(self) -> Vec4;
    fn zzxz(self) -> Vec4;
    fn zzyx(self) -> Vec4;
    fn zzyy(self) -> Vec4;
    fn zzyz(self) -> Vec4;
    fn zzzx(self) -> Vec4;
    fn zzzy(self) -> Vec4;
    fn zzzz(self) -> Vec4;
    fn xxx(self) -> Vec3A;
    fn xxy(self) -> Vec3A;
    fn xxz(self) -> Vec3A;
    fn xyx(self) -> Vec3A;
    fn xyy(self) -> Vec3A;
    fn xzx(self) -> Vec3A;
    fn xzy(self) -> Vec3A;
    fn xzz(self) -> Vec3A;
    fn yxx(self) -> Vec3A;
    fn yxy(self) -> Vec3A;
    fn yxz(self) -> Vec3A;
    fn yyx(self) -> Vec3A;
    fn yyy(self) -> Vec3A;
    fn yyz(self) -> Vec3A;
    fn yzx(self) -> Vec3A;
    fn yzy(self) -> Vec3A;
    fn yzz(self) -> Vec3A;
    fn zxx(self) -> Vec3A;
    fn zxy(self) -> Vec3A;
    fn zxz(self) -> Vec3A;
    fn zyx(self) -> Vec3A;
    fn zyy(self) -> Vec3A;
    fn zyz(self) -> Vec3A;
    fn zzx(self) -> Vec3A;
    fn zzy(self) -> Vec3A;
    fn zzz(self) -> Vec3A;
    fn xx(self) -> Vec2;
    fn xy(self) -> Vec2;
    fn xz(self) -> Vec2;
    fn yx(self) -> Vec2;
    fn yy(self) -> Vec2;
    fn yz(self) -> Vec2;
    fn zx(self) -> Vec2;
    fn zy(self) -> Vec2;
    fn zz(self) -> Vec2;
}

#[rustfmt::skip]
impl Vec3ASwizzles for Vec3A {
    #[inline]
    fn xxxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn xxxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn xxxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn xxyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn xxyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn xxyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn xxzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn xxzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn xxzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).0, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn xyxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn xyxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn xyxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn xyyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn xyyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn xyyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn xyzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn xyzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn xyzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).1, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn xzxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn xzxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn xzxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn xzyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn xzyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn xzyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn xzzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn xzzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn xzzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).0, (self.0).2, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn yxxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn yxxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn yxxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn yxyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn yxyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn yxyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn yxzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn yxzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn yxzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).0, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn yyxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn yyxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn yyxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn yyyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn yyyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn yyyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn yyzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn yyzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn yyzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).1, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn yzxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn yzxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn yzxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn yzyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn yzyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn yzyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn yzzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn yzzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn yzzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).1, (self.0).2, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn zxxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn zxxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn zxxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn zxyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn zxyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn zxyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn zxzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn zxzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn zxzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).0, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn zyxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn zyxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn zyxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn zyyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn zyyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn zyyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn zyzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn zyzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn zyzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).1, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn zzxx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn zzxy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_00_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn zzxz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_00_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn zzyx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn zzyy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn zzyz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_01_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn zzzx(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn zzzy(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn zzzz(self) -> Vec4 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec4((self.0).2, (self.0).2, (self.0).2, (self.0).2)
        }
    }
    #[inline]
    fn xxx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).0, (self.0).0))
        }
    }
    #[inline]
    fn xxy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).0, (self.0).1))
        }
    }
    #[inline]
    fn xxz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).0, (self.0).2))
        }
    }
    #[inline]
    fn xyx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).1, (self.0).0))
        }
    }
    #[inline]
    fn xyy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).1, (self.0).1))
        }
    }
    #[inline]
    fn xzx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).2, (self.0).0))
        }
    }
    #[inline]
    fn xzy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).2, (self.0).1))
        }
    }
    #[inline]
    fn xzz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_00))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).0, (self.0).2, (self.0).2))
        }
    }
    #[inline]
    fn yxx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).0, (self.0).0))
        }
    }
    #[inline]
    fn yxy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).0, (self.0).1))
        }
    }
    #[inline]
    fn yxz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).0, (self.0).2))
        }
    }
    #[inline]
    fn yyx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).1, (self.0).0))
        }
    }
    #[inline]
    fn yyy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).1, (self.0).1))
        }
    }
    #[inline]
    fn yyz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).1, (self.0).2))
        }
    }
    #[inline]
    fn yzx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).2, (self.0).0))
        }
    }
    #[inline]
    fn yzy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).2, (self.0).1))
        }
    }
    #[inline]
    fn yzz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_01))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).1, (self.0).2, (self.0).2))
        }
    }
    #[inline]
    fn zxx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).0, (self.0).0))
        }
    }
    #[inline]
    fn zxy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).0, (self.0).1))
        }
    }
    #[inline]
    fn zxz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).0, (self.0).2))
        }
    }
    #[inline]
    fn zyx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).1, (self.0).0))
        }
    }
    #[inline]
    fn zyy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).1, (self.0).1))
        }
    }
    #[inline]
    fn zyz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_01_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).1, (self.0).2))
        }
    }
    #[inline]
    fn zzx(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).2, (self.0).0))
        }
    }
    #[inline]
    fn zzy(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_01_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).2, (self.0).1))
        }
    }
    #[inline]
    fn zzz(self) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(Vec3((self.0).2, (self.0).2, (self.0).2))
        }
    }
    #[inline]
    fn xx(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).0, (self.0).0)
        }
    }
    #[inline]
    fn xy(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_00)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).0, (self.0).1)
        }
    }
    #[inline]
    fn xz(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_00)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).0, (self.0).2)
        }
    }
    #[inline]
    fn yx(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_01)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).1, (self.0).0)
        }
    }
    #[inline]
    fn yy(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_01)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).1, (self.0).1)
        }
    }
    #[inline]
    fn yz(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).1, (self.0).2)
        }
    }
    #[inline]
    fn zx(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_10)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).2, (self.0).0)
        }
    }
    #[inline]
    fn zy(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_01_10)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).2, (self.0).1)
        }
    }
    #[inline]
    fn zz(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec2::from(Vec3A(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_10)))
        }

        #[cfg(vec3a_f32)]
        {
            Vec2((self.0).2, (self.0).2)
        }
    }
}
