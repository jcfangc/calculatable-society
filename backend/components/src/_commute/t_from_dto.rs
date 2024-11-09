// src/common/from_dto.rs
pub trait FromDTO<DTO> {
    fn from_dto(dto: DTO) -> Self;
}
