use crate::{mscope::PropertyScope, mselector::PropertySelector};
use coreaudio_sys::AudioObjectPropertyAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AudioObjPropAddress {
    select: PropertySelector,
    scope: PropertyScope,
    element: u32,
}

impl AudioObjPropAddress {
    pub const fn new(select: PropertySelector, scope: PropertyScope) -> Self {
        Self::new_with_element(select, scope, 0)
    }

    pub const fn new_with_element(
        select: PropertySelector,
        scope: PropertyScope,
        element: u32,
    ) -> Self {
        Self {
            select,
            scope,
            element,
        }
    }

    pub const fn selector(&self) -> PropertySelector {
        self.select
    }

    pub const fn scope(&self) -> PropertyScope {
        self.scope
    }

    pub const fn element(&self) -> u32 {
        self.element
    }

    pub fn set_element(&mut self, new: u32) {
        self.element = new;
    }
}

impl From<AudioObjPropAddress> for AudioObjectPropertyAddress {
    fn from(value: AudioObjPropAddress) -> Self {
        AudioObjectPropertyAddress {
            mSelector: value.select.into(),
            mScope: value.scope.into(),
            mElement: value.element,
        }
    }
}
