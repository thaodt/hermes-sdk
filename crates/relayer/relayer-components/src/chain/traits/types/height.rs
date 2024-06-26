/*!
   Trait definition for [`HasHeightType`].
*/

use core::fmt::Display;

use cgp_core::prelude::*;

#[derive_component(HeightTypeComponent, ProvideHeightType<Chain>)]
pub trait HasHeightType: Async {
    /**
       The height of the chain, which should behave like natural numbers.

       By default, the height only contains the `Ord` constraint, and does
       not support operations like addition.

       We can impose additional constraints at the use site of `HasChainTypes`.
       However doing so may impose limitations on which concrete types
       the `Height` type can be.

       By keeping the abstract type minimal, we can for example use
       `u8` or `u128` as the `Height` type during testing, and use the
       more complex Cosmos height type during production.
    */
    type Height: Ord + Display + Async + Clone;
}

#[derive_component(HeightFieldComponent, HeightFieldGetter<Chain>)]
pub trait HasHeightFields: HasHeightType {
    fn revision_number(height: &Self::Height) -> u64;

    fn revision_height(height: &Self::Height) -> u64;
}

#[derive_component(HeightIncrementerComponent, HeightIncrementer<Chain>)]
pub trait CanIncrementHeight: HasHeightType + HasErrorType {
    fn increment_height(height: &Self::Height) -> Result<Self::Height, Self::Error>;
}

#[derive_component(GenesisHeightGetterComponent, GenesisHeightGetter<Chain>)]
pub trait HasGenesisHeight: HasHeightType {
    fn genesis_height(&self) -> Self::Height;
}
