use literalize::literal;

#[literal(2_147_483_647)]
pub struct I32Max;

#[literal(2_147_483_648)]
pub struct AboveI32Max;

#[literal(-2_147_483_648)]
pub struct I32Min;

#[literal(-2_147_483_649)]
pub struct BelowI32Min;
