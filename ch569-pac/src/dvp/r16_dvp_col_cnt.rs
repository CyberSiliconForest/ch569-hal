#[doc = "Register `R16_DVP_COL_CNT` reader"]
pub struct R(crate::R<R16_DVP_COL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_DVP_COL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_DVP_COL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_DVP_COL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_DVP_COL_CNT` reader - DVP receive fifo ready"]
pub type RB_DVP_COL_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DVP receive fifo ready"]
    #[inline(always)]
    pub fn rb_dvp_col_cnt(&self) -> RB_DVP_COL_CNT_R {
        RB_DVP_COL_CNT_R::new(self.bits)
    }
}
#[doc = "DVP col count value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_dvp_col_cnt](index.html) module"]
pub struct R16_DVP_COL_CNT_SPEC;
impl crate::RegisterSpec for R16_DVP_COL_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_dvp_col_cnt::R](R) reader structure"]
impl crate::Readable for R16_DVP_COL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R16_DVP_COL_CNT to value 0"]
impl crate::Resettable for R16_DVP_COL_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
